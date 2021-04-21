macro_rules! impl_entity_unwrap {
    ($T:ty) => {
        fn align(cfg: &crate::Config) -> usize {
            <$T>::align(cfg)
        }
        fn size(&self, cfg: &crate::Config) -> usize {
            <Self as Into<$T>>::into(*self).size(cfg)
        }
        fn load<R: crate::io::CntRead>(cfg: &crate::Config, src: &mut R) -> std::io::Result<Self> {
            <$T>::load(cfg, src).map(|x| x.into())
        }
        fn store<W: crate::io::CntWrite>(&self, cfg: &crate::Config, dst: &mut W) -> std::io::Result<()> {
            <Self as Into<$T>>::into(*self).store(cfg, dst)
        }
        fn entity_source(cfg: &crate::Config) -> crate::source::SourceInfo {
            <$T>::entity_source(cfg)
        }
    };
}

macro_rules! impl_sized_entity_unwrap {
    ($T:ty) => {
        fn type_size(cfg: &crate::Config) -> usize {
            <$T>::type_size(cfg)
        }
    };
}
