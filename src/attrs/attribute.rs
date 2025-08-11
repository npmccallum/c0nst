use syn::{Attribute, Meta};

use super::{HasAttribute, IsAttribute};

impl IsAttribute for Attribute {
    fn is_attribute(&self, name: &str) -> bool {
        if let Meta::Path(path) = &self.meta {
            path.is_ident(name)
        } else {
            false
        }
    }
}

impl HasAttribute for [Attribute] {
    fn has_attribute(&self, name: &str) -> bool {
        self.iter().any(|attr| attr.is_attribute(name))
    }
}
