use proc_macro2::{TokenStream, TokenTree};

pub struct Rule<'a>(&'a str, &'a str);

impl Rule<'_> {
    pub const RULES: &'static [Rule<'static>] = &[
        Rule("[c0nst] Destruct +", "[const] core::marker::Destruct +"),
        Rule("+ [c0nst] Destruct", "+ [const] core::marker::Destruct"),
        Rule(": [c0nst] Destruct", ": [const] core::marker::Destruct"),
        Rule("c0nst Destruct +", "const core::marker::Destruct +"),
        Rule("+ c0nst Destruct", "+ const core::marker::Destruct"),
        Rule(": c0nst Destruct", ": const core::marker::Destruct"),
        Rule("[c0nst]", "[const]"),
        Rule("c0nst", "const"),
    ];

    pub fn pattern(&self) -> Vec<TokenTree> {
        let stream: TokenStream = self.0.parse().expect("valid pattern");
        stream.into_iter().collect()
    }

    pub fn nightly(&self) -> Vec<TokenTree> {
        let stream: TokenStream = self.1.parse().expect("valid nightly");
        stream.into_iter().collect()
    }
}
