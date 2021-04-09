use proc_macro2::TokenStream;
use quote::quote;
use synstructure::{decl_derive, Structure};

fn entity_derive(s: Structure) -> TokenStream {
    let _body = s.fold(false, |acc, bi| quote!{
        #acc || types::Entity::interesting(#bi)
    });

    let align = s.fold(1usize, |acc, bi| quote!{
        num_integer::lcm(#acc, types::Entity::align())
    });

    s.bound_impl(quote!(types::Entity), quote!{
        //fn interesting(&self) -> bool {
        //    match *self {
        //        #body
        //    }
        //}

        fn align(cfg: &types::Config) -> usize {
            #align
        }

        fn size(&self, cfg: &types::Config) -> usize {
            0
        }

        fn load<R: types::io::CntRead>(cfg: &types::Config, src: &mut R) -> std::io::Result<Self> {
            Err(std::io::ErrorKind::Other.into())
        }

        fn store<W: types::io::CntWrite>(&self, cfg: &types::Config, dst: &mut W) -> std::io::Result<()> {
            Err(std::io::ErrorKind::Other.into())
        }

        fn source(cfg: &types::Config) -> types::SourceInfo {
            types::SourceInfo::new(String::new(), String::new())
        }
    })
}

#[macro_export]
decl_derive!([Entity] => entity_derive);

#[test]
fn expand_entity() {
    synstructure::test_derive!{
        entity_derive {
            struct A {
                x: i32,
            }
        }
        expands to {}
    }
}
