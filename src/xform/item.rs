use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Item;

use super::{Annotation, Target, Transform};

impl Annotation for Item {
    fn can_m0rph(&self) -> Result<(), syn::Error> {
        match self {
            Item::Mod(item) if item.content.is_some() => Ok(()),
            Item::Trait(..) => Ok(()),
            Item::Impl(..) => Ok(()),
            Item::Fn(..) => Ok(()),
            Item::Struct(..) => Ok(()),
            Item::Enum(..) => Ok(()),
            Item::Union(..) => Ok(()),
            Item::Type(..) => Ok(()),
            _ => Err(syn::Error::new_spanned(
                self,
                "cannot use `#[m0rph]` in this context",
            )),
        }
    }

    fn can_c0nst(&self) -> Result<(), syn::Error> {
        match self {
            Item::Trait(..) => Ok(()),
            Item::Impl(..) => Ok(()),
            Item::Fn(..) => Ok(()),
            _ => Err(syn::Error::new_spanned(
                self,
                "cannot use `#[c0nst]` in this context",
            )),
        }
    }
}

/// Implementation for Items
impl Transform for Item {
    fn transform(&self, target: Target) -> TokenStream {
        match self {
            Item::Trait(item_trait) => item_trait.transform(target),
            Item::Impl(item_impl) => item_impl.transform(target),
            Item::Fn(item_fn) => item_fn.transform(target),
            Item::Struct(item_struct) => item_struct.transform(target),
            Item::Enum(item_enum) => item_enum.transform(target),
            Item::Union(item_union) => item_union.transform(target),
            Item::Type(item_type) => item_type.transform(target),
            Item::Mod(item_mod) => item_mod.transform(target),
            _ => self.to_token_stream(), // Other items use default behavior
        }
    }
}
