use std::{
    hash::{Hasher}
};

/// Dynamic type trait.
pub trait Type {
    /// Hash the type.
    fn hash_dyn(&self, )

    /// Type unique identifier.
    fn id(&self) -> u64 {

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
    fn size(&self) -> usize;

    /// Returns dynamic type of the instance.
    fn type_dyn(&self) -> Box<dyn Type>;

    /// Stores the instance to bytes.
    /// The bytes should have at least
    fn store_dyn(&self, dst: &mut dyn Write) -> usize;
}

pub trait EmptyType: Type {

}

    fn size(&self) -> Option<usize> {
        Some(0)
    }
    fn align(&self) -> usize {
        1
    }
}
