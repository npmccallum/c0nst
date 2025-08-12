use proc_macro2::TokenStream;

use crate::Target;

impl super::Convert for TokenStream {
    type Output = TokenStream;

    fn convert(self, target: Target) -> Self::Output {
        let mut output = TokenStream::new();

        for token in self {
            if let Some(converted) = token.convert(target) {
                output.extend(std::iter::once(converted));
            }
        }

        output
    }
}
