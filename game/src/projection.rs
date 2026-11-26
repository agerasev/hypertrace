//! First-person projection for unit-curvature hyperbolic triangle meshes.
use anyhow::Result;
use ccgeom::Hyperboloid3;
use hypertrace_scene::Transform;
use wgame::{
    Library,
    gfx::Scene,
    glam::{Mat4, Vec4, camera::rh::proj::directx},
    shapes::{material::MeshShader, shader::ShaderConfig},
};

use crate::world::Object;

pub(crate) type Pose = Transform<Hyperboloid3>;
pub(crate) type Shader = MeshShader<RelativeMap>;

#[derive(Clone, wgame::shader::Attribute)]
pub(crate) struct RelativeMap {
    real: Vec4,
    imag: Vec4,
}

/// Physical distance along the optical axis to the near clipping plane, in curvature radii.
const NEAR_DISTANCE: f64 = 0.02;

pub(crate) fn camera_matrix((width, height): (u32, u32), tan_half_fov: f64) -> Mat4 {
    // For embedded (W,X,Y,Z), (X,Y,Z,W) is the homogeneous Klein point.
    // Its screen direction is (X,Y)/-Z. Along a ray, Klein radius = tanh(d).
    // An infinite far plane leaves the whole unit Klein ball in range, including
    // when a distant vertex rounds exactly onto the ideal boundary.
    directx::perspective_infinite(
        (2.0 * tan_half_fov.atan()) as f32,
        width as f32 / height as f32,
        NEAR_DISTANCE.tanh() as f32,
    )
}

pub(crate) fn shader(library: &Library) -> Result<Shader> {
    MeshShader::new(
        library.shapes().state(),
        ShaderConfig {
            module_source: include_str!("projection.wgsl").into(),
            vertex_source: "output.position = view_matrix * place_vertex(instance.real, instance.imag, vertex.position);".into(),
            ..Default::default()
        },
        &[],
    )
}

fn relative_map(view: Pose, object: Pose) -> Result<RelativeMap> {
    let rows = view.chain(&object)?.components()?;
    // The sandwich is quadratic in the pair. A common scale on its two
    // quaternions only changes the homogeneous weight of the resulting point.
    // Scaling in f64 before upload keeps the products bounded for distant poses.
    let scale = rows
        .iter()
        .flatten()
        .copied()
        .map(f64::abs)
        .fold(0.0, f64::max);
    let [real, imag] = rows.map(|row| Vec4::from_array(row.map(|v| (v / scale) as f32)));
    Ok(RelativeMap { real, imag })
}

pub(crate) fn prepare(objects: &[Object], camera: Pose, shader: &Shader) -> Result<Scene> {
    let view = camera.inverse()?;
    let mut scene = Scene::default();
    for object in objects {
        let material = shader.material(vec![], relative_map(view, object.pose)?)?;
        scene.add(&object.mesh.with_material(&material));
    }
    Ok(scene)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ccgeom::Geometry3;

    #[test]
    fn upload_removes_large_shared_world_motion_before_rounding() -> Result<()> {
        let camera = Pose::from_isometry(Hyperboloid3::shift_y(12.0))?;
        let offset = Pose::identity().move_local([0.15, 0.04, -0.7], [0.2, 0.1, -0.3], 1.0)?;
        let relative = relative_map(camera.inverse()?, camera.chain(&offset)?)?;
        let local = relative_map(Pose::identity(), offset)?;
        assert!(relative.real.abs_diff_eq(local.real, 1e-6));
        assert!(relative.imag.abs_diff_eq(local.imag, 1e-6));
        Ok(())
    }

    #[test]
    fn distant_boost_stays_bounded_and_projects_toward_the_ideal_point() -> Result<()> {
        let distance: f64 = 14.0;
        let map = relative_map(
            Pose::identity(),
            Pose::from_isometry(Hyperboloid3::shift_z(-distance))?,
        )?;
        // A pure Z boost has a scalar real quaternion and a single imaginary
        // imag component. Its point action on the origin gives (W,0,0,Z).
        let a = map.real.x;
        let b = map.imag.w;
        let point = Vec4::new(0.0, 0.0, 2.0 * a * b, a * a + b * b);
        assert_eq!(a, 1.0);
        assert!(b.abs() <= 1.0);
        assert!((point.z / point.w + distance.tanh() as f32).abs() < 1e-6);
        let clip = camera_matrix((640, 480), 1.0) * point;
        assert!(clip.z > 0.0 && clip.z < clip.w);
        Ok(())
    }

    #[test]
    fn pinhole_matches_intrinsic_angles_and_depth_orders_forward_rays() {
        let projection = camera_matrix((800, 400), 0.5);
        // Unit-speed origin rays have P(d) = (cosh(d), sinh(d) * unit_direction).
        let forward = |d: f64| projection * Vec4::new(0.0, 0.0, -d.sinh() as f32, d.cosh() as f32);
        assert!(forward(-0.5).w < 0.0);
        assert!(forward(NEAR_DISTANCE / 2.0).z < 0.0);
        assert!(forward(NEAR_DISTANCE).z.abs() < 1e-7);
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
