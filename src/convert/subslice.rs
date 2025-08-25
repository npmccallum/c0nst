use proc_macro2::TokenTree;

pub trait Subslice {
    fn find_subslice(&self, needle: &Self) -> Option<usize>;
}

impl Subslice for [TokenTree] {
    fn find_subslice(&self, needle: &Self) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }

        if needle.len() > self.len() {
            return None;
        }

        self.windows(needle.len()).position(|window| {
            window
                .iter()
                .zip(needle.iter())
                .all(|(a, b)| a.to_string() == b.to_string())
        })
    }
}
