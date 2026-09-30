use crate::{
    object::tiling::{self, TileSurface, Tiling},
    shader::{EncodedObject, Geometry, Modules, Result, Transform},
    Material, Object,
};

/// A surface with a statically compatible intrinsic pattern and tile materials.
/// The border may have a different material type from the tiles. Use `Mapped`
/// around this object to move its shape and material coordinates together.
#[derive(Clone, Debug)]
pub struct Tiled<S, P, M, B, const N: usize> {
    pub shape: S,
    pub pattern: P,
    pub materials: [M; N],
    pub border_material: B,
}
impl<S, P, M, B, const N: usize> Tiled<S, P, M, B, N> {
    pub fn new(shape: S, pattern: P, materials: [M; N], border_material: B) -> Self {
        Self {
            shape,
            pattern,
            materials,
            border_material,
        }
    }
}
impl<G, S, P, M, B, const N: usize> Object<G> for Tiled<S, P, M, B, N>
where
    G: Geometry,
    S: TileSurface<G>,
    P: Tiling<S::Domain>,
    M: Material<G>,
    B: Material<G>,
{
    fn shader_modules() -> Result<Modules<G>> {
        Ok(Modules {
            shapes: vec![S::shader()?],
            materials: vec![tiling::tiled_schema(
                tiling::selector::<G, S, P>()?,
                vec![M::shader()?; N],
                B::shader()?,
            )?],
            libraries: vec![],
        })
    }
    fn encode_objects(
        &self,
        outer: Transform<G>,
        output: &mut Vec<EncodedObject<G>>,
    ) -> Result<()> {
        let (cell, width) = self.pattern.parameters();
        output.push(EncodedObject {
            map: outer,
            shape: self.shape.encode()?,
            material: tiling::tiled(
                tiling::selector::<G, S, P>()?,
                self.materials
                    .iter()
                    .map(M::encode)
                    .collect::<Result<_>>()?,
                self.border_material.encode()?,
                cell,
                width,
            )?,
        });
        Ok(())
    }
}
