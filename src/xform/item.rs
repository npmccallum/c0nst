use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Item;

use super::{Adaptable, Target, Transform};

impl Adaptable for Item {
    fn can_adapt(&self) -> bool {
        match self {
            Item::Mod(item) => item.can_adapt(),
            Item::Trait(item) => item.can_adapt(),
            Item::Impl(item) => item.can_adapt(),
            Item::Fn(item) => item.can_adapt(),
            Item::Struct(item) => item.can_adapt(),
            Item::Enum(item) => item.can_adapt(),
            Item::Union(item) => item.can_adapt(),
            Item::Type(item) => item.can_adapt(),
            _ => false, // Other items are not adaptable
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
