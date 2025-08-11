use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::GenericParam;

use super::{Target, Transform};

/// Implementation for GenericParam
impl Transform for GenericParam {
    fn transform(&self, target: Target) -> TokenStream {
        match self {
            GenericParam::Type(type_param) => type_param.transform(target),
            _ => self.to_token_stream(),
        }
    }
}
