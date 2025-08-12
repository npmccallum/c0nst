use proc_macro2::{Ident, TokenTree};

use crate::Target;

impl super::Convert for Ident {
    type Output = Option<TokenTree>;

    fn convert(self, target: Target) -> Self::Output {
        if self != "c0nst" {
            return Some(TokenTree::Ident(self));
        }

        match target {
            Target::Stable => None,
            Target::Nightly => Some(TokenTree::Ident(Ident::new("const", self.span()))),
        }
    }
}
