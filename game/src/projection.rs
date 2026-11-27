//! First-person projection for unit-curvature hyperbolic triangle meshes.
use anyhow::{Context, Result, ensure};
use ccgeom::{Hyperboloid3, Space3};
use hypertrace_scene::Transform;
use wgame::{
    Library,
    gfx::{BakedScene, Camera, DepthMode, Scene, Target},
    glam::{Mat4, Vec3, Vec4, camera::rh::proj::directx},
    shader::Attribute,
    shapes::{
        Mesh, Polygon,
        material::{MaterialBinding, MaterialMesh, MeshShader},
        prelude::ShapeFill,
        shader::{ShaderConfig, Vertex},
    },
};
use wgpu::util::DeviceExt;

use crate::{
    atmosphere::{AirBindings, Atmosphere},
    config::AmbientLight,
    world::{InfinitePlane, Material, Object, Objects, Sun},
};

mod tiles;
use tiles::{TileCache, TileDrawing};

pub(crate) type Pose = Transform<Hyperboloid3>;

pub(crate) struct Shader {
    sky: MeshShader<()>,
    air: AirBindings,
    mesh: MeshShader<Instance>,
    tile_mesh: MeshShader<Instance>,
    tile_cache: TileCache,
    plane: MeshShader<Instance>,
    tile_plane: MeshShader<Instance>,
    plane_quad: Polygon,
    light: MaterialBinding,
    sun_pose: Pose,
}

#[derive(Clone, Attribute)]
struct Instance {
    real: Vec4,
    imag: Vec4,
    sun_plane: Vec4,
    reflection: Vec3,
    transmission: Vec3,
}

fn instance_capacity(device: &wgpu::Device) -> u64 {
    let stride = wgame::shapes::shader::InstanceData::<Instance>::bindings().size();
    (device.limits().max_buffer_size / stride).min(u32::MAX.into())
}

#[derive(Attribute)]
struct Varyings {
    eye_point: Vec4,
    local_klein: Vec3,
    smooth_normal: Vec3,
    sun_plane: Vec4,
    reflection: Vec3,
    transmission: Vec3,
    emission: Vec3,
}

pub(crate) fn camera_matrix(
    (width, height): (u32, u32),
    tan_half_fov: f64,
    near_distance: f64,
) -> Mat4 {
    // For embedded (W,X,Y,Z), (X,Y,Z,W) is the homogeneous Klein point.
    // Its screen direction is (X,Y)/-Z. Along a ray, Klein radius = tanh(d).
    // An infinite far plane leaves the whole unit Klein ball in range, including
    // when a distant vertex rounds exactly onto the ideal boundary.
    directx::perspective_infinite(
        (2.0 * tan_half_fov.atan()) as f32,
        width as f32 / height as f32,
        near_distance.tanh() as f32,
    )
}

pub(crate) fn shader(
    library: &Library,
    sun: &Sun,
    ambient: AmbientLight,
    atmosphere: Option<Atmosphere>,
) -> Result<Shader> {
    ensure!(
        [sun.radiance, ambient.night, ambient.day]
            .into_iter()
            .flatten()
            .all(|x| x.is_finite() && x >= 0.0),
        "invalid lighting radiance"
    );
    let state = library.state();
    let device = state.device();
    let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Surface lighting"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new(48),
            },
            count: None,
        }],
    });
    let colors = [
        Vec3::from_array(sun.radiance).extend(0.0).to_array(),
        Vec3::from_array(ambient.night)
            .extend(ambient.response_factor)
            .to_array(),
        Vec3::from_array(ambient.day)
            .extend(ambient.response_exponent)
            .to_array(),
    ];
    let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Surface lighting"),
        contents: bytemuck::cast_slice(&colors),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let uniforms = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Surface lighting"),
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    });
    let display = if state.format().is_srgb() {
        "linear"
    } else {
        "encode_srgb(linear)"
    };
    let surface_vertex = |eye_point: &str| {
        format!(
            "
            let local = model_matrix * vertex.position;
            let placed = place_vertex(instance.real, instance.imag, local);
            output.eye_point = {eye_point};
            output.position = view_matrix * output.eye_point;
            output.local_klein = local.xyz;
            // Spatial Lorentz normals are also Klein tangent-plane covectors.
            let a = model_matrix[0].xyz;
            let b = model_matrix[1].xyz;
            let c = model_matrix[2].xyz;
            output.smooth_normal = mat3x3<f32>(cross(b,c), cross(c,a), cross(a,b)) * vertex.normal;
            output.sun_plane = instance.sun_plane;
            output.reflection = instance.reflection;
            output.transmission = instance.transmission;
            output.emission = instance.tex_color.rgb;
            output.color = view_color * vertex.color;
        "
        )
    };
    let mut config = ShaderConfig {
        module_source: concat!(
            include_str!("projection.wgsl"),
            "\n",
            include_str!("lighting.wgsl"),
            "\n",
            include_str!("atmosphere.wgsl")
        )
        .into(),
        varying: Varyings::bindings(),
        vertex_source: surface_vertex("placed"),
        fragment_color_source: format!(
            "let linear = display_linear(color.rgb * illuminate(input) + air_to_surface(input.eye_point)); color = vec4<f32>({display}, color.a);"
        ),
        ..Default::default()
    };
    let air = AirBindings::new(device, atmosphere, sun)?;
    let layouts = [layout, air.layout.clone()];
    let shapes = library.shapes().state();
    let sky_config = ShaderConfig {
        vertex_source: "
            let ndc = vertex.position.xz;
            output.position = vec4<f32>(ndc, 1.0, 1.0);
            output.eye_point = vec4<f32>(ndc.x/view_matrix[0][0], ndc.y/view_matrix[1][1], -1.0, 0.0);
        ".into(),
        fragment_color_source: format!(
            "let linear = display_linear(color.rgb + air_glow(input.eye_point.xyz, 0.0)); color = vec4<f32>({display}, 1.0);"
        ),
        ..config.clone()
    };
    let sky = MeshShader::new(shapes, sky_config, &layouts)?;
    let surface_shaders = |mut surface: ShaderConfig| -> Result<_> {
        let global = MeshShader::new(shapes, surface.clone(), &layouts)?;
        surface.vertex_source =
            surface_vertex("place_vertex(air.tile_real, air.tile_imag, placed)");
        Ok((global, MeshShader::new(shapes, surface, &layouts)?))
    };
    let (mesh, tile_mesh) = surface_shaders(config.clone())?;
    // The two triangles cover a square. Only its open unit disk is in the plane.
    config
        .fragment_color_source
        .push_str("\nif dot(input.local_klein, input.local_klein) >= 1.0 { discard; }");
    let (plane, tile_plane) = surface_shaders(config)?;
    Ok(Shader {
        sky,
        air,
        mesh,
        tile_mesh,
        tile_cache: TileCache::default(),
        plane,
        tile_plane,
        plane_quad: plane_quad(library),
        light: MaterialBinding::Uniforms(uniforms),
        sun_pose: sun.pose,
    })
}

fn plane_quad(library: &Library) -> Polygon {
    let shapes = library.shapes();
    let vertices = [(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0)]
        .map(|(x, z)| Vertex::new(Vec4::new(x, 0.0, z, 1.0), Vec3::Z).with_normal(Vec3::ZERO));
    shapes.mesh(Mesh::from_arrays(
        shapes.state(),
        &vertices,
        Some(&[0, 1, 2, 0, 2, 3]),
    ))
}

fn projective_pair(map: Pose) -> Result<[Vec4; 2]> {
    let rows = map.components()?;
    // The sandwich is quadratic in the pair. A common scale on its two
    // quaternions only changes the homogeneous weight of the resulting point.
    // Scaling in f64 before upload keeps the products bounded for distant poses.
    let scale = rows
        .iter()
        .flatten()
        .copied()
        .map(f64::abs)
        .fold(0.0, f64::max);
    Ok(rows.map(|row| Vec4::from_array(row.map(|v| (v / scale) as f32))))
}

/// In-plane translation to the perpendicular foot of the reference origin.
/// Its local coordinates are (X,0,Z)/sqrt(1+Y²) for an origin at (W,X,Y,Z).
fn plane_foot(relative: Pose) -> Result<Pose> {
    let [_, x, y, z] = relative.inverse()?.apply_vector([1.0, 0.0, 0.0, 0.0]);
    let radial = x.hypot(z);
    if radial == 0.0 {
        return Ok(Pose::identity());
    }
    let distance = (radial / y.hypot(1.0)).asinh();
    let axis = [x / radial, 0.0, z / radial];
    Pose::from_isometry(
        Space3::<f64, -1>::unit()
            .translation(axis.into(), distance)
            .context("invalid plane footprint")?,
    )
}

impl Shader {
    fn shaded_plane(
        &self,
        pipeline: &MeshShader<Instance>,
        plane: &Object<InfinitePlane>,
        relative: Pose,
        sun: Pose,
        air: &MaterialBinding,
    ) -> Result<MaterialMesh<Instance>> {
        // Center the disk at the eye's foot for global planes, or the tile
        // center's foot for instances baked with a ground tile.
        let foot = plane_foot(relative)?;
        let sun_in_plane =
            if plane.material.reflection == [0.0; 3] && plane.material.transmission == [0.0; 3] {
                Pose::identity()
            } else {
                foot.inverse()?.chain(&plane.pose.inverse()?.chain(&sun)?)?
            };
        self.shaded_mesh(
            &self.plane_quad,
            pipeline,
            relative.chain(&foot)?,
            sun_in_plane,
            plane.material,
            air,
        )
    }

    fn shaded_mesh(
        &self,
        polygon: &Polygon,
        pipeline: &MeshShader<Instance>,
        relative: Pose,
        sun_in_object: Pose,
        material: Material,
        air: &MaterialBinding,
    ) -> Result<MaterialMesh<Instance>> {
        material.validate()?;
        let [w, x, y, z] = sun_in_object.apply_vector([0.0, 0.0, 1.0, 0.0]);
        let sun_plane = Vec4::new(x as f32, y as f32, z as f32, w as f32);
        ensure!(
            sun_plane.is_finite(),
            "Sun plane exceeds shader coordinate range"
        );
        let [real, imag] = projective_pair(relative)?;
        let attributes = Instance {
            real,
            imag,
            sun_plane,
            reflection: Vec3::from_array(material.reflection),
            transmission: Vec3::from_array(material.transmission),
        };
        // Using Wgame's built-in instance color for emission keeps the vertex
        // layout within WebGPU's guaranteed 16 attribute locations.
        let emission = Vec3::from_array(material.emission);
        let material = pipeline.material(vec![self.light.clone(), air.clone()], attributes)?;
        Ok(polygon.fill_color(emission).with_material(&material))
    }
}

pub(crate) struct Drawing {
    sky: Option<BakedScene>,
    planes: BakedScene,
    meshes: BakedScene,
    tiles: TileDrawing,
}

impl Drawing {
    #[cfg(test)]
    pub(crate) fn ground_batch_count(&self) -> usize {
        self.tiles
            .planes()
            .chain(self.tiles.meshes())
            .map(BakedScene::len)
            .sum()
    }

    pub(crate) fn draw(&self, target: &mut impl Target, camera: &Camera) {
        self.tiles.upload(target.encoder());
        if let Some(sky) = &self.sky {
            target.render(camera, sky);
        }
        // Both passes use the target's depth buffer so a plane can occlude a mesh.
        target.render_iter(
            camera,
            std::iter::once(&self.planes).chain(self.tiles.planes()),
        );
        target.render_iter(
            camera,
            std::iter::once(&self.meshes).chain(self.tiles.meshes()),
        );
    }
}

pub(crate) fn prepare(objects: &Objects, camera: Pose, shader: &Shader) -> Result<Drawing> {
    let view = camera.inverse()?;
    let atmosphere = shader.air.words(view)?;
    let air = shader.air.binding(&atmosphere);
    let sky = if let Some(background) = shader.air.sky_background() {
        let material = shader
            .sky
            .material(vec![shader.light.clone(), air.clone()], ())?;
        let mut sky = Scene::default();
        sky.add(
            &shader
                .plane_quad
                .fill_color(background)
                .with_material(&material)
                .depth(DepthMode::Overlay),
        );
        Some(sky.bake())
    } else {
        None
    };
    let mut planes = Scene::default();
    for plane in &objects.planes {
        let mesh = shader.shaded_plane(
            &shader.plane,
            plane,
            view.chain(&plane.pose)?,
            shader.sun_pose,
            &air,
        )?;
        planes.add(&mesh);
    }
    let mut meshes = Scene::default();
    for object in &objects.meshes {
        let mesh = shader.shaded_mesh(
            &object.shape,
            &shader.mesh,
            view.chain(&object.pose)?,
            object.pose.inverse()?.chain(&shader.sun_pose)?,
            object.material,
            &air,
        )?;
        meshes.add(&mesh);
    }
    Ok(Drawing {
        sky,
        planes: planes.bake(),
        meshes: meshes.bake(),
        tiles: shader
            .tile_cache
            .prepare(shader, &objects.tiles, view, atmosphere)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccgeom::Geometry3;

    #[test]
    fn upload_removes_large_shared_world_motion_before_rounding() -> Result<()> {
        let camera = Pose::from_isometry(Hyperboloid3::shift_y(12.0))?;
        let offset = Pose::identity().move_local([0.15, 0.04, -0.7], [0.2, 0.1, -0.3], 1.0)?;
        let relative = projective_pair(camera.inverse()?.chain(&camera.chain(&offset)?)?)?;
        let local = projective_pair(offset)?;
        assert!(relative[0].abs_diff_eq(local[0], 1e-6));
        assert!(relative[1].abs_diff_eq(local[1], 1e-6));
        Ok(())
    }

    #[test]
    fn distant_boost_stays_bounded_and_projects_toward_the_ideal_point() -> Result<()> {
        let distance: f64 = 14.0;
        let [real, imag] = projective_pair(Pose::from_isometry(Hyperboloid3::shift_z(-distance))?)?;
        // A pure Z boost has a scalar real quaternion and a single imaginary
        // imag component. Its point action on the origin gives (W,0,0,Z).
        let a = real.x;
        let b = imag.w;
        let point = Vec4::new(0.0, 0.0, 2.0 * a * b, a * a + b * b);
        assert_eq!(a, 1.0);
        assert!(b.abs() <= 1.0);
        assert!((point.z / point.w + distance.tanh() as f32).abs() < 1e-6);
        let clip = camera_matrix((640, 480), 1.0, 0.01) * point;
        assert!(clip.z > 0.0 && clip.z < clip.w);
        Ok(())
    }

    #[test]
    fn pinhole_matches_intrinsic_angles_and_depth_orders_forward_rays() {
        let near_distance = 0.02;
        let projection = camera_matrix((800, 400), 0.5, near_distance);
        // Unit-speed origin rays have P(d) = (cosh(d), sinh(d) * unit_direction).
        let forward = |d: f64| projection * Vec4::new(0.0, 0.0, -d.sinh() as f32, d.cosh() as f32);
        assert!(forward(-0.5).w < 0.0);
        assert!(forward(near_distance / 2.0).z < 0.0);
        assert!(forward(near_distance).z.abs() < 1e-7);
        let mut previous_depth = 0.0;
        for distance in [0.1_f64, 1.0, 3.0, 7.0] {
            let clip = forward(distance);
            let depth = clip.z / clip.w;
            assert!(previous_depth < depth && depth < 1.0);
            previous_depth = depth;
            let radial = distance.sinh() as f32 / 1.5_f32.sqrt();
            let p = Vec4::new(0.5 * radial, -0.5 * radial, -radial, distance.cosh() as f32);
            let clip = projection * p;
            assert!((clip.x / clip.w - 0.5).abs() < 1e-6);
            assert!((clip.y / clip.w + 1.0).abs() < 1e-6);
        }
    }
}
