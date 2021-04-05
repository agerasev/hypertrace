use types::{
    ObjectType, Object, EmptyValue,
    wrap_def, wrap_impl, wrap_impl_sized, wrap_impl_unit,
};

wrap_def!(SphereType, Sphere, EmptyValue);
wrap_impl!(SphereType, Sphere, EmptyValue);
wrap_impl_sized!(SphereType, Sphere, EmptyValue);
wrap_impl_unit!(SphereType, Sphere, EmptyValue);

impl ObjectType for SphereType {
    fn name(&self) -> String {
        String::from("sphere")
    }
    fn source(&self) -> String {
        String::from("#include <shape/eu/shapes.hh>")
    }
}

impl Object for Sphere {}
