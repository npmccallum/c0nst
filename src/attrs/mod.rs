mod attribute;

/// Extension trait for checking if a single attribute matches a name
pub trait IsAttribute {
    fn is_attribute(&self, name: &str) -> bool;
}

/// Extension trait for checking attributes by name
pub trait HasAttribute {
    fn has_attribute(&self, name: &str) -> bool;
}
