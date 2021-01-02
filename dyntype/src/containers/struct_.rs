use crate::{io::*, traits::*, utils::*, Config};
use std::{
    collections::BTreeSet,
    hash::{Hash, Hasher},
    io,
};

struct StructBaseType;

fn unique_names<T>(fields: &[(T, String)]) -> usize {
    fields
        .iter()
        .map(|(_, name)| name.clone())
        .collect::<BTreeSet<_>>()
        .len()
}

#[derive(Clone, Debug)]
pub struct StructType<T: SizedType>
where
    T::Value: SizedValue,
{
    fields: Vec<(T, String)>,
}

impl<T: SizedType> StructType<T>
where
    T::Value: SizedValue,
{
    pub fn new(fields: Vec<(T, String)>) -> Option<Self> {
        if fields.len() == unique_names(&fields) {
            Some(Self { fields })
        } else {
            None
        }
    }
    pub fn fields(&self) -> &Vec<(T, String)> {
        &self.fields
    }
    pub fn fields_mut(&mut self) -> &mut Vec<(T, String)> {
        &mut self.fields
    }
    pub fn into_fields(self) -> Vec<(T, String)> {
        self.fields
    }
}

impl<T: SizedType> Type for StructType<T>
where
    T::Value: SizedValue,
{
    type Value = Struct<T::Value>;

    fn align(&self, cfg: &Config) -> usize {
        self.fields
            .iter()
            .map(|(type_, _)| type_.align(cfg))
            .max()
            .unwrap_or(1)
    }

    fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        type_id::<StructBaseType>().hash(&mut hasher);
        for (type_, name) in self.fields.iter() {
            type_.id().hash(&mut hasher);
            name.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn load<R: CountingRead + ?Sized>(&self, cfg: &Config, src: &mut R) -> io::Result<Self::Value> {
        let mut fields = Vec::<(T::Value, String)>::new();
        assert!(
            src.position() % self.align(cfg) == 0,
            "Stream is not aligned"
        );
        for (type_, name) in self.fields() {
            src.align(type_.align(cfg))?;
            fields.push((src.read_value(cfg, type_)?, name.clone()));
        }
        src.align(self.align(cfg))?;
        Ok(Self::Value::new(fields).unwrap())
    }
}

impl<T: SizedType> SizedType for StructType<T>
where
    T::Value: SizedValue,
{
    fn size(&self, cfg: &Config) -> usize {
        let mut size = 0;
        for (type_, _) in self.fields() {
            size = upper_multiple(size, type_.align(cfg)) + type_.size(cfg);
        }
        size = upper_multiple(size, self.align(cfg));
        size
    }
}

#[derive(Clone)]
pub struct Struct<V: SizedValue>
where
    V::Type: SizedType,
{
    fields: Vec<(V, String)>,
}

impl<V: SizedValue> Struct<V>
where
    V::Type: SizedType,
{
    pub fn new(fields: Vec<(V, String)>) -> Option<Self> {
        if fields.len() == unique_names(&fields) {
            Some(Self { fields })
        } else {
            None
        }
    }

    pub fn fields(&self) -> &Vec<(V, String)> {
        &self.fields
    }
    pub fn fields_mut(&mut self) -> &mut Vec<(V, String)> {
        &mut self.fields
    }
    pub fn into_fields(self) -> Vec<(V, String)> {
        self.fields
    }
}

impl<V: SizedValue> Value for Struct<V>
where
    V::Type: SizedType,
{
    type Type = StructType<V::Type>;

    fn size(&self, cfg: &Config) -> usize {
        self.type_().size(cfg)
    }

    fn type_(&self) -> Self::Type {
        Self::Type::new(
            self.fields()
                .iter()
                .map(|(item, name)| (item.type_(), name.clone()))
                .collect(),
        )
        .unwrap()
    }

    fn store<W: CountingWrite + ?Sized>(&self, cfg: &Config, dst: &mut W) -> io::Result<()> {
        assert!(
            dst.position() % self.align(cfg) == 0,
            "Stream is not aligned"
        );
        for (item, _) in self.fields() {
            dst.align(item.align(cfg))?;
            dst.write_value(cfg, item)?;
        }
        dst.align(self.align(cfg))?;
        Ok(())
    }
}

impl<V: SizedValue> SizedValue for Struct<V> where V::Type: SizedType {}

#[macro_export]
macro_rules! def_struct {
    ( $V:ident : $T:ident { $( $f:ident : $I:ty ),* $(,)? }) => {
        #[derive(Clone, Default, Debug)]
        pub struct $T {
            $( pub $f: <$I as $crate::traits::Value>::Type, )*
        }

        impl $crate::Type for $T {
            type Value = $V;

            fn align(&self, cfg: &$crate::Config) -> usize {
                [ $( self.$f.align(cfg) ),* ]
                .iter().cloned().max().unwrap_or(1)
            }

            fn id(&self) -> u64 {
                use std::hash::{Hash, Hasher};
                let mut hasher = $crate::DefaultHasher::new();
                $crate::type_id::<StructBaseType>().hash(&mut hasher);
                $(
                    self.$f.id().hash(&mut hasher);
                    stringify!($f).hash(&mut hasher);
                )*
                hasher.finish()
            }

            fn load<R: $crate::io::CountingRead + ?Sized>(&self, cfg: &$crate::Config, src: &mut R) -> std::io::Result<Self::Value> {
                use $crate::io::ValueReader;
                assert!(src.position() % self.align(cfg) == 0, "Stream is not aligned");
                let value = $V {
                    $( $f: {
                        src.align(self.$f.align(cfg))?;
                        src.read_value(cfg, &self.$f)?
                    }, )*
                };
                src.align(self.align(cfg))?;
                Ok(value)
            }
        }

        impl $crate::SizedType for $T {
            fn size(&self, cfg: &$crate::Config) -> usize {
                use $crate::traits::Type;
                let mut size = 0;
                $(
                    size = $crate::utils::upper_multiple(size, self.$f.align(cfg)) + self.$f.size(cfg);
                )*
                size = $crate::utils::upper_multiple(size, self.align(cfg));
                size
            }
        }

        impl $crate::UnitType for $T
        where
            $( <$I as $crate::Value>::Type: $crate::UnitType ),*
        {}

        #[derive(Clone, Debug, PartialEq)]
        pub struct $V {
            $( pub $f: $I, )*
        }

        impl $crate::Value for $V {
            type Type = $T;

            fn size(&self, cfg: &$crate::Config) -> usize {
                use $crate::SizedType;
                self.type_().size(cfg)
            }

            fn type_(&self) -> Self::Type {
                $T {
                    $( $f: self.$f.type_(), )*
                }
            }

            fn store<W: $crate::io::CountingWrite + ?Sized>(&self, cfg: &$crate::Config, dst: &mut W) -> std::io::Result<()> {
                use $crate::io::ValueWriter;
                assert!(dst.position() % self.align(cfg) == 0, "Stream is not aligned");
                $(
                    dst.align(self.$f.align(cfg))?;
                    dst.write_value(cfg, &self.$f)?;
                )*
                dst.align(self.align(cfg))?;
                Ok(())
            }
        }

        impl $crate::SizedValue for $V {}

        impl $crate::UnitValue for $V
        where
            $( $I: $crate::UnitValue ),*
        {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::*, primitive::*};
    use std::fmt::Debug;

    def_struct!(TestStruct: TestStructType {
        one: u8,
        two: i32,
        three: usize,
    });

    #[test]
    fn ids() {
        let sty = TestStructType::default();
        let dty = StructType::<Box<dyn SizedTypeDyn>>::new(vec![
            (Box::new(U8Type), "one".into()),
            (Box::new(I32Type), "two".into()),
            (Box::new(UsizeType), "three".into()),
        ])
        .unwrap();
        assert_eq!(sty.id(), dty.id())
    }

    #[test]
    fn store_load_static() {
        let st = TestStruct {
            one: 1,
            two: 2,
            three: 3,
        };
        let mut buf = TestBuffer::new();
        buf.writer().write_value(&HOST_CONFIG, &st).unwrap();
        assert_eq!(
            st,
            buf.reader()
                .read_value(&HOST_CONFIG, &TestStructType::default())
                .unwrap()
        );
    }

    fn assert_eq_dyn<V: Value + PartialEq + Debug>(a: &dyn SizedValueDyn, b: &dyn SizedValueDyn) {
        assert_eq!(
            a.as_any().downcast_ref::<V>().unwrap(),
            b.as_any().downcast_ref::<V>().unwrap(),
        );
    }

    #[test]
    fn store_load() {
        let st = Struct::<Box<dyn SizedValueDyn>>::new(vec![
            (Box::<u8>::new(1), "one".into()),
            (Box::<i32>::new(2), "two".into()),
            (Box::<usize>::new(3), "three".into()),
        ])
        .unwrap();
        let mut buf = TestBuffer::new();
        buf.writer().write_value(&HOST_CONFIG, &st).unwrap();
        let ty = StructType::<Box<dyn SizedTypeDyn>>::new(vec![
            (Box::new(U8Type), "one".into()),
            (Box::new(I32Type), "two".into()),
            (Box::new(UsizeType), "three".into()),
        ])
        .unwrap();
        let ps = st
            .into_fields()
            .into_iter()
            .zip(
                buf.reader()
                    .read_value(&HOST_CONFIG, &ty)
                    .unwrap()
                    .into_fields()
                    .into_iter(),
            )
            .map(|((sv, sn), (dv, dn))| {
                assert_eq!(sn, dn);
                (sv, dv)
            })
            .collect::<Vec<_>>();
        assert_eq!(ps.len(), 3);
        assert_eq_dyn::<u8>(ps[0].0.as_ref(), ps[0].1.as_ref());
        assert_eq_dyn::<i32>(ps[1].0.as_ref(), ps[1].1.as_ref());
        assert_eq_dyn::<usize>(ps[2].0.as_ref(), ps[2].1.as_ref());
    }
}
