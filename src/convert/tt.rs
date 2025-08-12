use proc_macro2::TokenTree;

use crate::Target;

impl super::Convert for TokenTree {
    type Output = Option<TokenTree>;

    fn convert(self, target: Target) -> Self::Output {
        match self {
            Self::Ident(ident) => ident.convert(target),
            Self::Group(group) => group.convert(target),
            token => Some(token),
        }
    }
}
