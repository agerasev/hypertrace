macro_rules! impl_named_unwrap {
    ($T:ty) => {
        fn name() -> (String, String) {
            <$T>::name()
        }
    };
}

macro_rules! impl_entity_unwrap {
    ($T:ty) => {
        fn align(cfg: &crate::Config) -> usize {
            <$T>::align(cfg)
        }
        fn size(&self, cfg: &crate::Config) -> usize {
            <Self as Into<$T>>::into(*self).size(cfg)
        }
        fn min_size(cfg: &crate::Config) -> usize {
            <$T>::min_size(cfg)
        }
        fn is_dyn_sized() -> bool {
            <$T>::is_dyn_sized()
        }
        fn load<R: crate::io::CntRead>(cfg: &crate::Config, src: &mut R) -> std::io::Result<Self> {
            <$T>::load(cfg, src).map(|x| x.into())
        }
        fn store<W: crate::io::CntWrite>(
            &self,
            cfg: &crate::Config,
            dst: &mut W,
        ) -> std::io::Result<()> {
            <Self as Into<$T>>::into(*self).store(cfg, dst)
        }
    };
}

macro_rules! impl_sized_entity_unwrap {
    ($T:ty) => {
        fn static_size(cfg: &crate::Config) -> usize {
            <$T>::static_size(cfg)
        }
    };
}

macro_rules! impl_sourced_unwrap {
    ($T:ty) => {
        fn source(cfg: &crate::Config) -> crate::source::SourceTree {
            <$T>::source(cfg)
        }
    };
}
