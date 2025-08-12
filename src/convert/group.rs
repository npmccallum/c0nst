use proc_macro2::{Delimiter, Group, Ident, TokenStream, TokenTree};

use crate::Target;

impl super::Convert for Group {
    type Output = Option<TokenTree>;

    fn convert(self, target: Target) -> Self::Output {
        let mut stream = self.stream().into_iter();
        let first = stream.next();
        let second = stream.next();

        match (self.delimiter(), first, second) {
            (Delimiter::Bracket, Some(TokenTree::Ident(ident)), None) if ident == "c0nst" => {
                match target {
                    Target::Stable => None,
                    Target::Nightly => Some(TokenTree::Group(Group::new(
                        Delimiter::Bracket,
                        TokenTree::Ident(Ident::new("const", ident.span())).into(),
                    ))),
                }
            }

            _ => {
                let mut output = TokenStream::new();

                for token in self.stream() {
                    if let Some(converted) = token.convert(target) {
                        output.extend(std::iter::once(converted));
                    }
                }

                Some(TokenTree::Group(Group::new(self.delimiter(), output)))
            }
        }
    }
}
