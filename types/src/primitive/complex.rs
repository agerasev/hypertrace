use crate::{
    primitive::{PrimScal, PrimVec},
    Config, Entity, SizedEntity, source::{SourceInfo, Sourced},
};
use vecmat::{Vector, Complex, Quaternion};


impl<T: PrimScal> Entity for Complex<T> {
    impl_entity_unwrap!(Vector<T, 2>);
}

impl<T: PrimScal> SizedEntity for Complex<T> {
    impl_sized_entity_unwrap!(Vector<T, 2>);
}

impl<T: PrimScal> Sourced for Complex<T> {
    fn source(cfg: &Config) -> SourceInfo {
        Self::entity_source(cfg)
    }
}

impl<T: PrimScal> PrimVec for Complex<T> {}


impl<T: PrimScal> Entity for Quaternion<T> {
    impl_entity_unwrap!(Vector<T, 4>);
}

impl<T: PrimScal> SizedEntity for Quaternion<T> {
    impl_sized_entity_unwrap!(Vector<T, 4>);
}

impl<T: PrimScal> Sourced for Quaternion<T> {
    fn source(cfg: &Config) -> SourceInfo {
        Self::entity_source(cfg)
    }
}

impl<T: PrimScal> PrimVec for Quaternion<T> {}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::HOST_CONFIG,
        io::{EntityReader, EntityWriter, TestBuffer},
    };

    #[test]
    fn complex() {
        let mut buf = TestBuffer::new();
        let sv = Complex::<i32>::from([1, 2]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        let dv = buf.reader().read_entity(&HOST_CONFIG).unwrap();
        assert_eq!(sv, dv);
    }

    #[test]
    fn quaternion() {
        let mut buf = TestBuffer::new();
        let sv = Quaternion::<i32>::from([1, 2, 3, 4]);
        buf.writer().write_entity(&HOST_CONFIG, &sv).unwrap();
        let dv = buf.reader().read_entity(&HOST_CONFIG).unwrap();
        assert_eq!(sv, dv);
    }
}
