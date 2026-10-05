use crate::{
    shader::{EncodedObject, Geometry, Modules, Result, Transform},
    Object,
};

impl<G: Geometry> Object<G> for () {
    fn shader_modules() -> Result<Modules<G>> {
        Ok(Modules {
            lights: vec![],
            shapes: vec![],
            materials: vec![],
            libraries: vec![],
        })
    }

    fn encode_objects(&self, _: Transform<G>, _: &mut Vec<EncodedObject<G>>) -> Result<()> {
        Ok(())
    }
}

macro_rules! tuple {
    ($( $type:ident : $index:tt ),+) => {
        impl<G: Geometry, $( $type: Object<G> ),+> Object<G> for ($( $type, )+) {
            fn shader_modules() -> Result<Modules<G>> {
                let mut modules = Modules { lights: vec![], shapes: vec![], materials: vec![], libraries: vec![] };
                $(
                    let child = $type::shader_modules()?;
                    modules.extend(child);
                )+
                Ok(modules)
            }

            fn encode_objects(&self, outer: Transform<G>, output: &mut Vec<EncodedObject<G>>) -> Result<()> {
                $( self.$index.encode_objects(outer, output)?; )+
                Ok(())
            }
        }
    };
}

tuple!(A:0);
tuple!(A:0, B:1);
tuple!(A:0, B:1, C:2);
tuple!(A:0, B:1, C:2, D:3);
tuple!(A:0, B:1, C:2, D:3, E:4);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7, J:8);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7, J:8, K:9);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7, J:8, K:9, L:10);
tuple!(A:0, B:1, C:2, D:3, E:4, F:5, H:6, I:7, J:8, K:9, L:10, M:11);
