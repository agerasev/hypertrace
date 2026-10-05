use crate::{
    light::LightSampler,
    shader::{EncodedObject, Geometry, Modules, Result, Transform},
    Object,
};

/// Explicitly enable direct-light sampling for the objects inside this wrapper.
/// The proposal's frame is this wrapper's local frame, independently of nested
/// shape or object maps. Usually put `Mapped` outside `Sampled` to move both.
/// Groups share the proposal but remain separately selected emitter instances.
#[derive(Clone, Debug)]
pub struct Sampled<T, L> {
    pub inner: T,
    pub sampler: L,
}
impl<T, L> Sampled<T, L> {
    pub fn new(inner: T, sampler: L) -> Self {
        Self { inner, sampler }
    }
}
impl<G: Geometry, T: Object<G>, L: LightSampler<G>> Object<G> for Sampled<T, L> {
    fn shader_modules() -> Result<Modules<G>> {
        let mut modules = T::shader_modules()?;
        modules.lights.push(L::shader()?);
        Ok(modules)
    }
    fn encode_objects(
        &self,
        outer: Transform<G>,
        output: &mut Vec<EncodedObject<G>>,
    ) -> Result<()> {
        let mut sampling = self.sampler.encode()?;
        sampling.map = outer.chain(&sampling.map)?;
        let mut children = Vec::new();
        self.inner.encode_objects(outer, &mut children)?;
        for child in &mut children {
            anyhow::ensure!(
                child.sampling.is_none(),
                "nested light sampling wrappers are ambiguous"
            );
            child.sampling = Some(sampling.clone());
        }
        output.extend(children);
        Ok(())
    }
}
