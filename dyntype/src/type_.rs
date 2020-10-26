use std::{
    io::{Read, Write},
    hash::{Hasher}, collections::hash_map::DefaultHasher,
};

/// Dynamic type trait.
pub trait Type {
    /// Hash the type.
    fn hash_dyn(&self, hasher: &mut dyn Hasher);

    /// Type unique identifier.
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash_dyn(&mut hasher);
        return hasher.finish();
    }

    /// Clones type.
    fn clone_dyn(&self) -> Box<dyn Type>;

    /// Align of dynamic type.
    fn align(&self) -> usize;

    /// Size of type.
    /// Returns `None` if the type is dynamically sized.
    fn size(&self) -> Option<usize>;

    /// Loads the instance.
    fn load_dyn(&self, src: &mut dyn Read) -> Box<dyn Instance>;
}

/// Instance of a dynamic type.
pub trait Instance {
    /// Size of instance.
    /// Should be equal to `self.type_().size().unwrap()` if the instance is statically sized.
    fn size(&self) -> usize;

    /// Returns the type of the instance.
    fn type_dyn(&self) -> Box<dyn Type>;

    /// Stores the instance to bytes.
    /// The bytes should have at least
    fn store_dyn(&self, dst: &mut dyn Write);
}


/// Dynamic type that doesn't store anything.
pub trait EmptyType: Type {
    /// Hash the type.
    fn hash_dyn(&self, hasher: &mut dyn Hasher);

    /// Clones type.
    fn clone_dyn(&self) -> Box<dyn Type>;

    /// Returns empty instance.
    fn instantiate_dyn(&self) -> Box<dyn Instance>;
}

impl<T> Type for T where T: EmptyType {
    fn hash_dyn(&self, hasher: &mut dyn Hasher) {
        <Self as EmptyType>::hash_dyn(self, hasher)
    }

    fn clone_dyn(&self) -> Box<dyn Type> {
        <Self as EmptyType>::clone_dyn(self)
    }

    fn align(&self) -> usize {
        1
    }
    fn size(&self) -> Option<usize> {
        Some(0)
    }

    fn load_dyn(&self, _: &mut dyn Read) -> Box<dyn Instance> {
        <Self as EmptyType>::instantiate_dyn(self)
    }
}

/// Instance of an `EmptyType`.
pub trait EmptyInstance: Instance {
    /// Returns the type of the instance.
    fn type_dyn(&self) -> Box<dyn Type>;
}

impl<T> Instance for T where T: EmptyInstance {
    fn size(&self) -> usize {
        0
    }

    fn type_dyn(&self) -> Box<dyn Type> {
        <Self as EmptyInstance>::type_dyn(self)
    }

    fn store_dyn(&self, _: &mut dyn Write) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        any::TypeId,
        hash::Hash
    };

    struct DummyType {}
    struct DummyInstance {}

    impl EmptyType for DummyType {
        fn hash_dyn(&self, hasher: &mut dyn Hasher) {
            TypeId::of::<DummyType>().hash(hasher);
        }
        fn clone_dyn(&self) -> Box<dyn Type> {
            Box::new(DummyType {})
        }
        fn instantiate_dyn(&self) -> Box<dyn Instance> {
            Box::new(DummyInstance {})
        }
    }

    impl EmptyInstance for DummyInstance {
        fn type_dyn(&self) -> Box<dyn Type> {
            Box::new(DummyType {})
        }
    }

    #[test]
    fn empty() {
        assert_eq!(DummyType{}.id(), DummyInstance{}.type_dyn().id()); 
    }
}