use proc_macro2::{Group, TokenStream, TokenTree};

use super::rule::Rule;
use super::subslice::Subslice;
use crate::Target;

impl super::Convert for TokenStream {
    type Output = TokenStream;

    fn convert(self, target: Target) -> Self::Output {
        let mut tokens: Vec<_> = self.into_iter().collect();

        for rule in Rule::RULES {
            let pattern = rule.pattern();
            while let Some(index) = tokens.find_subslice(&pattern) {
                // Remove the matched pattern.
                for _ in 0..pattern.len() {
                    tokens.remove(index);
                }

                // Insert the replacement tokens.
                if target == Target::Nightly {
                    for token in rule.nightly().drain(..).rev() {
                        tokens.insert(index, token);
                    }
                }
            }
        }

        let mut output = Self::new();

        for token in tokens {
            let token = match token {
                TokenTree::Group(group) => TokenTree::Group(Group::new(
                    group.delimiter(),
                    group.stream().convert(target),
                )),
                tt => tt,
            };

            output.extend(std::iter::once(token));
        }

        output
    }
}
