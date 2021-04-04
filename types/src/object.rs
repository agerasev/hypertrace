use crate::traits::{
    Type, SizedType, TypeDyn, SizedTypeDyn,
    Value, SizedValue, ValueDyn, SizedValueDyn,
};


pub trait ObjectType: Type
where
    Self::Value: Object<Type = Self>
{
    fn name(&self) -> String;
    fn source(&self) -> String;
}

pub trait SizedObjectType: ObjectType + SizedType + Type
where
    Self::Value: SizedObject<Type = Self>
{
}

macro_rules! def_object_type_dyn {
    ($T:ident, $V:ident) => {
        fn name_dyn(&self) -> String;
        fn source_dyn(&self) -> String;
        fn into_object_type_dyn(self: Box<Self>) -> Box<dyn ObjectTypeDyn>;
    };
}

macro_rules! def_sized_object_type_dyn {
    ($T:ident, $V:ident) => {
        fn into_sized_object_type_dyn(self: Box<Self>) -> Box<dyn SizedObjectTypeDyn>;
    };
}

pub trait ObjectTypeDyn: 'static {
    def_type_dyn!(ObjectTypeDyn, ObjectDyn);
    def_object_type_dyn!(ObjectTypeDyn, ObjectDyn);
}

pub trait SizedObjectTypeDyn: 'static {
    def_type_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    def_sized_type_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    def_object_type_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    def_sized_object_type_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
}

macro_rules! impl_object_type_dyn {
    ($T:ident, $V:ident) => {
        fn name_dyn(&self) -> String {
            self.name()
        }
        fn source_dyn(&self) -> String {
            self.source()
        }

        fn into_object_type_dyn(self: Box<Self>) -> Box<dyn ObjectTypeDyn> {
            self
        }
    };
}

macro_rules! impl_sized_object_type_dyn {
    ($T:ident, $V:ident) => {
        fn into_sized_object_type_dyn(self: Box<Self>) -> Box<dyn SizedObjectTypeDyn> {
            self
        }
    };
}

impl<T> ObjectTypeDyn for T
where
    T: ObjectType,
    T::Value: Object,
{
    impl_type_dyn!(ObjectTypeDyn, ObjectDyn);
    impl_object_type_dyn!(ObjectTypeDyn, ObjectDyn);
}

impl<T> SizedObjectTypeDyn for T
where
    T: SizedObjectType,
    T::Value: SizedObject,
{
    impl_type_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    impl_sized_type_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    impl_object_type_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    impl_sized_object_type_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
}

macro_rules! impl_object_type_box {
    ($T:ident, $V:ident) => {
        impl ObjectType for Box<dyn $T> {
            fn name(&self) -> String {
                self.as_ref().name_dyn()
            }
            fn source(&self) -> String {
                self.as_ref().source_dyn()
            }
        }
    };
}

macro_rules! impl_sized_object_type_box {
    ($T:ident, $V:ident) => {
        impl SizedObjectType for Box<dyn $T> {}
    };
}

impl_type_box!(ObjectTypeDyn, ObjectDyn);
impl_object_type_box!(ObjectTypeDyn, ObjectDyn);

impl_type_box!(SizedObjectTypeDyn, SizedObjectDyn);
impl_sized_type_box!(SizedObjectTypeDyn, SizedObjectDyn);
impl_object_type_box!(SizedObjectTypeDyn, SizedObjectDyn);
impl_sized_object_type_box!(SizedObjectTypeDyn, SizedObjectDyn);


pub trait Object: Value
where
    Self::Type: ObjectType<Value = Self>
{
}

pub trait SizedObject: Object + SizedValue + Value
where
    Self::Type: SizedObjectType<Value = Self>
{
}

macro_rules! def_object_dyn {
    ($T:ident, $V:ident) => {
        fn into_object_dyn(self: Box<Self>) -> Box<dyn ObjectDyn>;
    };
}

macro_rules! def_sized_object_dyn {
    ($T:ident, $V:ident) => {
        fn into_sized_object_dyn(self: Box<Self>) -> Box<dyn SizedObjectDyn>;
    };
}

pub trait ObjectDyn: 'static {
    def_value_dyn!(ObjectTypeDyn, ObjectDyn);
    def_object_dyn!(ObjectTypeDyn, ObjectDyn);
}

pub trait SizedObjectDyn: 'static {
    def_value_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    def_sized_value_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    def_object_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    def_sized_object_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
}

macro_rules! impl_object_dyn {
    ($T:ident, $V:ident) => {
        fn into_object_dyn(self: Box<Self>) -> Box<dyn ObjectDyn> {
            self
        }
    };
}

macro_rules! impl_sized_object_dyn {
    ($T:ident, $V:ident) => {
        fn into_sized_object_dyn(self: Box<Self>) -> Box<dyn SizedObjectDyn> {
            self
        }
    };
}

impl<V> ObjectDyn for V
where
    V: Object,
    V::Type: ObjectType,
{
    impl_value_dyn!(ObjectTypeDyn, ObjectDyn);
    impl_object_dyn!(ObjectTypeDyn, ObjectDyn);
}

impl<V> SizedObjectDyn for V
where
    V: SizedObject,
    V::Type: SizedObjectType,
{
    impl_value_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    impl_sized_value_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    impl_object_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
    impl_sized_object_dyn!(SizedObjectTypeDyn, SizedObjectDyn);
}

macro_rules! impl_object_box {
    ($T:ident, $V:ident) => {
        impl Object for Box<dyn $V> {}
    };
}

macro_rules! impl_sized_object_box {
    ($T:ident, $V:ident) => {
        impl SizedObject for Box<dyn $V> {}
    };
}

impl_value_box!(ObjectTypeDyn, ObjectDyn);
impl_object_box!(ObjectTypeDyn, ObjectDyn);

impl_value_box!(SizedObjectTypeDyn, SizedObjectDyn);
impl_sized_value_box!(SizedObjectTypeDyn, SizedObjectDyn);
impl_object_box!(SizedObjectTypeDyn, SizedObjectDyn);
impl_sized_object_box!(SizedObjectTypeDyn, SizedObjectDyn);
