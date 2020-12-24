use std::{
    io::{self, Read, Write},
    hash::{Hash, Hasher},
    collections::hash_map::DefaultHasher,
};
use crate::Config;

// Type

/// Runtime type base trait.
pub trait TypeBase {
    /// Align of dynamic type.
    fn align(&self, config: &Config) -> usize;

    /// Size of type.
    /// Returns `None` if the type is dynamically sized.
    fn size(&self, config: &Config) -> Option<usize>;
}

/// Runtime type abstract trait.
/// *Shouldn't be implemented directly for user types, use `TypeSpec` instead.*
pub trait TypeDyn: TypeBase {
    /// Hash the type with abstract hasher.
    fn hash_dyn(&self, hasher: &mut dyn Hasher);

    /// Clones abstract type.
    fn clone_dyn(&self) -> TypeBox;

    /// Loads the abstract instance.
    fn load_dyn(&self, config: &Config, src: &mut dyn Read) -> io::Result<InstBox>;
}

/// Runtime type trait.
pub trait Type: TypeBase + TypeDyn + Clone + Hash + 'static {
    /// Instance of the type.
    type Inst: Inst;

    /// Type unique identifier.
    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    /// Loads the instance from abstract loader.
    /// 
    /// The amount of bytes to read from `src` always equal to `self.size()` for statically-sized types,
    /// but may vary for dynamically-sized types.
    fn load<R: Read + ?Sized>(&self, config: &Config, src: &mut R) -> io::Result<Self::Inst>;
}

/// Trait for specific runtime type.
/// *This is the trait to be implemented for your own types.*
pub trait TypeSpec: Type {}

impl<T> TypeDyn for T where T: TypeSpec {
    fn hash_dyn(&self, hasher: &mut dyn Hasher) {
        let mut hb = Box::new(hasher);
        self.hash(&mut hb);
    }
    fn clone_dyn(&self) -> TypeBox {
        Box::new(self.clone())
    }
    fn load_dyn(&self, config: &Config, src: &mut dyn Read) -> io::Result<InstBox> {
        self.load(config, src).map(|x| Box::new(x) as Box::<dyn InstDyn>)
    }
}


// Instance

/// Instance of a runtime type basic trait.
pub trait InstBase {
    /// Size of instance.
    /// 
    /// Should be equal to `self.type_().size().unwrap()` if the instance is statically sized.
    fn size(&self, config: &Config) -> usize;
}

/// Instance of a runtime type abstract trait.
/// *Shouldn't be implemented directly for user type instances, use `InstSpec` instead.*
pub trait InstDyn: InstBase {
    /// Returns an abstract type of the instance.
    fn type_dyn(&self) -> TypeBox;

    /// Stores the instance to abstract writer.
    /// 
    /// The `dst` should have capacity to store at least `self.size()` bytes.
    fn store_dyn(&self, config: &Config, dst: &mut dyn Write) -> io::Result<()>;
}

/// Instance of a runtime type trait.
pub trait Inst: InstBase + InstDyn + 'static {
    /// Type of the instance.
    type Type: Type;

    /// Returns the type of the instance.
    fn type_(&self) -> Self::Type;

    /// Stores the instance to bytes.
    /// The bytes should have at least
    fn store<W: Write + ?Sized>(&self, config: &Config, dst: &mut W) -> io::Result<()>;
}

/// Trait for specific runtime type instance.
/// *This is the trait to be implemented for your own type instances.*
pub trait InstSpec: Inst {}

impl<T> InstDyn for T where T: InstSpec {
    fn type_dyn(&self) -> TypeBox {
        Box::new(self.type_())
    }
    fn store_dyn(&self, config: &Config, dst: &mut dyn Write) -> io::Result<()> {
        self.store(config, dst)
    }
}


// Box

pub type TypeBox = Box<dyn TypeDyn>;
pub type InstBox = Box<dyn InstDyn>;

impl TypeBase for TypeBox {
    fn align(&self, config: &Config) -> usize {
        self.as_ref().align(config)
    }
    fn size(&self, config: &Config) -> Option<usize> {
        self.as_ref().size(config)
    }
}
impl TypeDyn for TypeBox {
    fn hash_dyn(&self, hasher: &mut dyn Hasher) {
        self.as_ref().hash_dyn(hasher);
    }
    fn clone_dyn(&self) -> TypeBox {
        self.as_ref().clone_dyn()
    }
    fn load_dyn(&self, config: &Config, src: &mut dyn Read) -> io::Result<InstBox> {
        self.as_ref().load_dyn(config, src)
    }
}
impl Type for TypeBox {
    type Inst = InstBox;

    fn load<R: Read + ?Sized>(&self, config: &Config, src: &mut R) -> io::Result<Self::Inst> {
        let mut rb = Box::new(src) as Box<dyn Read>;
        self.as_ref().load_dyn(config, &mut rb)
    }
}
impl Clone for TypeBox {
    fn clone(&self) -> Self {
        self.as_ref().clone_dyn()
    }
}
impl Hash for TypeBox {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let mut hb = Box::new(hasher);
        self.as_ref().hash_dyn(&mut hb);
    }
}

impl InstBase for InstBox {
    fn size(&self, config: &Config) -> usize {
        self.as_ref().size(config)
    }
}
impl InstDyn for InstBox {
    fn type_dyn(&self) -> TypeBox {
        self.as_ref().type_dyn()
    }
    fn store_dyn(&self, config: &Config, dst: &mut dyn Write) -> io::Result<()> {
        self.as_ref().store_dyn(config, dst)
    }
}
impl Inst for InstBox {
    type Type = TypeBox;

    fn type_(&self) -> Self::Type {
        self.as_ref().type_dyn()
    }
    fn store<W: Write + ?Sized>(&self, config: &Config, dst: &mut W) -> io::Result<()> {
        let mut wb = Box::new(dst) as Box<dyn Write>;
        self.as_ref().store_dyn(config, &mut wb)
    }
}


// Empty

/// Type which instance doesn't store anything.
pub trait EmptyType: TypeSpec {}

impl<T> TypeBase for T where T: EmptyType {
    fn align(&self, _: &Config) -> usize {
        1
    }
    fn size(&self, _: &Config) -> Option<usize> {
        Some(0)
    }
}

/// Instance that doesn't store anything.
pub trait EmptyInst: InstSpec {}

impl<T> InstBase for T where T: EmptyInst {
    fn size(&self, _: &Config) -> usize {
        0
    }
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;
    use crate::{Endianness, AddressWidth};

    const CONFIG: Config = Config {
        endianness: Endianness::Little,
        address_width: AddressWidth::X64,
        double_support: true,
    };

    #[derive(Debug, Clone)]
    struct DummyType {}

    #[derive(Debug, Clone)]
    struct DummyInst {}

    impl EmptyType for DummyType {}
    impl TypeSpec for DummyType {}
    impl Type for DummyType {
        type Inst = DummyInst;
        fn load<R: Read + ?Sized>(&self, _: &Config, _: &mut R) -> io::Result<Self::Inst> {
            Ok(DummyInst {})
        }
    }
    impl Hash for DummyType {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            TypeId::of::<Self>().hash(hasher);
        }
    }

    impl EmptyInst for DummyInst {}
    impl InstSpec for DummyInst {}
    impl Inst for DummyInst {
        type Type = DummyType;
        fn type_(&self) -> DummyType {
            DummyType {}
        }
        fn store<W: Write + ?Sized>(&self, _: &Config, _: &mut W) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn empty() {
        let (dty, din) = (DummyType{}, DummyInst{});
        assert_eq!(dty.id(), din.type_().id());
        assert_eq!(dty.align(&CONFIG), 1);
        assert_eq!(dty.size(&CONFIG).unwrap(), 0);
        assert_eq!(din.size(&CONFIG), 0);
    }
    #[test]
    fn empty_dyn() {
        let (dty, din) = (Box::new(DummyType{}) as TypeBox, Box::new(DummyInst{}) as InstBox);
        assert_eq!(dty.id(), din.type_().id());
    }
}
