use std::{
    io::{self, Read, Write},
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
};


/// Runtime type base trait.
pub trait TypeBase {
    /// Align of dynamic type.
    fn align(&self) -> usize;

    /// Size of type.
    /// Returns `None` if the type is dynamically sized.
    fn size(&self) -> Option<usize>;
}


/// Runtime type abstract trait.
pub trait TypeDyn: TypeBase {
    /// Hash the type with abstract hasher.
    fn hash_dyn(&self, hasher: &mut Box<dyn Hasher>);

    /// Type unique identifier.
    fn id(&self) -> u64 {
        let mut hasher: Box<dyn Hasher> = Box::new(DefaultHasher::new());
        self.hash_dyn(&mut hasher);
        return hasher.finish();
    }

    /// Clones abstract type.
    fn clone_dyn(&self) -> Box<dyn TypeDyn>;

    /// Loads the abstract instance.
    fn load_dyn(&self, src: &mut Box<dyn Read>) -> io::Result<Box<dyn InstDyn>>;
}

/// Runtime type trait.
pub trait Type: 'static + TypeBase + TypeDyn + Clone + Hash {
    /// Instance of the type.
    type Inst: Inst;

    /// Loads the instance from abstract loader.
    /// 
    /// The amount of bytes to read from `src` always equal to `self.size()` for statically-sized types,
    /// but may vary for dynamically-sized types.
    fn load<R: Read>(&self, src: &mut R) -> io::Result<Self::Inst>;
}

impl<T> TypeDyn for T where T: Type {
    fn hash_dyn(&self, hasher: &mut Box<dyn Hasher>) {
        self.hash(hasher);
    }
    fn clone_dyn(&self) -> Box<dyn TypeDyn> {
        Box::new(self.clone())
    }
    fn load_dyn(&self, src: &mut Box<dyn Read>) -> io::Result<Box<dyn InstDyn>> {
        self.load(src).map(|x| Box::new(x) as Box::<dyn InstDyn>)
    }
}

/// Instance of a runtime type basic trait.
pub trait InstBase {
    /// Size of instance.
    /// 
    /// Should be equal to `self.type_().size().unwrap()` if the instance is statically sized.
    fn size(&self) -> usize;
}

/// Instance of a runtime type abstract trait.
pub trait InstDyn: InstBase {
    /// Returns an abstract type of the instance.
    fn type_dyn(&self) -> Box<dyn TypeDyn>;

    /// Stores the instance to abstract writer.
    /// 
    /// The `dst` should have capacity to store at least `self.size()` bytes.
    fn store_dyn(&self, dst: &mut Box<dyn Write>) -> io::Result<()>;
}

/// Instance of a runtime type trait.
pub trait Inst: 'static + InstBase + InstDyn {
    /// Type of the instance.
    type Type: Type;

    /// Returns the type of the instance.
    fn type_(&self) -> Self::Type;

    /// Stores the instance to bytes.
    /// The bytes should have at least
    fn store<W: Write>(&self, dst: &mut W) -> io::Result<()>;
}

impl<T> InstDyn for T where T: Inst {
    fn type_dyn(&self) -> Box<dyn TypeDyn> {
        Box::new(self.type_())
    }
    fn store_dyn(&self, dst: &mut Box<dyn Write>) -> io::Result<()> {
        self.store(dst)
    }
}


/// Type which instance doesn't store anything.
pub trait EmptyType: Type {}

impl<T> TypeBase for T where T: EmptyType {
    fn align(&self) -> usize {
        1
    }
    fn size(&self) -> Option<usize> {
        Some(0)
    }
}

/// Instance doesn't store anything.
pub trait EmptyInst: Inst {}

impl<T> InstBase for T where T: EmptyInst {
    fn size(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        any::TypeId
    };

    #[derive(Debug, Clone)]
    struct DummyType {}

    #[derive(Debug, Clone)]
    struct DummyInst {}

    impl EmptyType for DummyType {}
    impl Type for DummyType {
        type Inst = DummyInst;
        fn load<R: Read>(&self, _: &mut R) -> io::Result<Self::Inst> {
            Ok(DummyInst {})
        }
    }
    impl Hash for DummyType {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            TypeId::of::<Self>().hash(hasher);
        }
    }

    impl EmptyInst for DummyInst {}
    impl Inst for DummyInst {
        type Type = DummyType;
        fn type_(&self) -> DummyType {
            DummyType {}
        }
        fn store<W: Write>(&self, _: &mut W) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn empty() {
        let (dty, din) = (DummyType{}, DummyInst{});
        assert_eq!(dty.id(), din.type_().id());
        assert_eq!(dty.align(), 1);
        assert_eq!(dty.size().unwrap(), 0);
        assert_eq!(din.size(), 0);
    }
    #[test]
    fn empty_dyn() {
        let (dty, din) = (Box::new(DummyType{}) as Box<dyn TypeDyn>, Box::new(DummyInst{}) as Box<dyn InstDyn>);
        assert_eq!(dty.id(), din.type_dyn().id());
    }
}