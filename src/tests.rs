//! Comprehensive test suite for the `c0nst` transformation engine.
//!
//! This module contains extensive tests that verify the correctness of the AST
//! transformation logic for converting between stable and nightly const trait syntax.
//!
//! ## Test Structure
//!
//! The tests are organized using `rstest` parameterized testing to ensure that
//! each transformation case is tested for both target environments:
//!
//! - **Nightly Target**: Transforms `#[c0nst]` to native const syntax
//! - **Stable Target**: Removes `#[c0nst]` attributes and wrapper bounds
//!
//! ## Test Categories
//!
//! ### Basic Item Transformations
//! - Functions: `#[c0nst] fn` → `const fn` / `fn`
//! - Traits: `#[c0nst] trait` → `const trait` / `trait`
//! - Implementations: `#[c0nst] impl` → `impl const` / `impl`
//!
//! ### Wrapper Trait Bounds
//! - Required const: `T: c0nst<Trait>` → `T: const Trait` / `T: Trait`
//! - Conditional const: `T: ?c0nst<Trait>` → `T: [const] Trait` / `T: Trait`
//!
//! ## Test Methodology
//!
//! The tests use string normalization (removing whitespace) to focus on semantic
//! correctness rather than formatting details.

#![cfg(test)]

use crate::xform::{Annotation, Target, Transform};
use syn::Item;

#[rstest::rstest]
#[case::basic_items(
    "#[c0nst] fn test() -> i32 { 42 }",
    "const fn test() -> i32 { 42 }",
    "fn test() -> i32 { 42 }"
)]
#[case::basic_trait(
    "#[c0nst] trait MyTrait { fn method(&self) -> i32; }",
    "const trait MyTrait { fn method(&self) -> i32; }",
    "trait MyTrait { fn method(&self) -> i32; }"
)]
#[case::basic_impl(
    "#[c0nst] impl MyTrait for MyType { fn method(&self) -> i32 { 42 } }",
    "impl const MyTrait for MyType { fn method(&self) -> i32 { 42 } }",
    "impl MyTrait for MyType { fn method(&self) -> i32 { 42 } }"
)]
#[case::comprehensive_wrapper_traits(
    "#[c0nst] fn test<T: c0nst<Clone> + ?c0nst<Send> + c0nst<From<u64>>>() -> T where T: c0nst<Default> { T::default() }",
    "const fn test<T: const Clone + [const] Send + const From<u64>>() -> T where T: const Default { T::default() }",
    "fn test<T: Clone + Send + From<u64>>() -> T where T: Default { T::default() }"
)]
#[case::complex_with_modifiers(
    "#[derive(Debug)] #[c0nst] pub unsafe fn test<'a, const N: usize, T: Clone + c0nst<Send>>() -> Result<T, String> where T: c0nst<Default> { Ok(T::default()) }",
    "#[derive(Debug)] pub const unsafe fn test<'a, const N: usize, T: Clone + const Send>() -> Result<T, String> where T: const Default { Ok(T::default()) }",
    "#[derive(Debug)] pub unsafe fn test<'a, const N: usize, T: Clone + Send>() -> Result<T, String> where T: Default { Ok(T::default()) }"
)]
#[case::trait_methods(
    "trait MyTrait { #[c0nst] fn method(&self) -> i32 { 42 } }",
    "trait MyTrait { const fn method(&self) -> i32 { 42 } }",
    "trait MyTrait { fn method(&self) -> i32 { 42 } }"
)]
#[case::impl_methods(
    "impl MyTrait for i32 { #[c0nst] fn method(&self) -> i32 { 42 } }",
    "impl MyTrait for i32 { const fn method(&self) -> i32 { 42 } }",
    "impl MyTrait for i32 { fn method(&self) -> i32 { 42 } }"
)]
#[case::generic_impl_comprehensive(
    "#[c0nst] impl<T: c0nst<Clone>> From<T> for MyType<T> where T: ?c0nst<Send> { fn from(t: T) -> Self { MyType(t) } }",
    "impl<T: const Clone> where T: [const] Send const From<T> for MyType<T> { fn from(t: T) -> Self { MyType(t) } }",
    "impl<T: Clone> where T: Send From<T> for MyType<T> { fn from(t: T) -> Self { MyType(t) } }"
)]
#[case::struct_comprehensive(
    "struct MyStruct<T: Clone + c0nst<Send>, U: ?c0nst<Sync>> where T: c0nst<Default> { t: T, u: U }",
    "struct MyStruct<T: Clone + const Send, U: [const] Sync> where T: const Default { t: T, u: U }",
    "struct MyStruct<T: Clone + Send, U: Sync> where T: Default { t: T, u: U }"
)]
#[case::tuple_struct(
    "struct MyStruct<T: c0nst<Clone>>(T);",
    "struct MyStruct<T: const Clone>(T);",
    "struct MyStruct<T: Clone>(T);"
)]
#[case::enum_comprehensive(
    "enum MyEnum<T: c0nst<Clone> + ?c0nst<Send>> where T: c0nst<Default> { Variant(T), Other }",
    "enum MyEnum<T: const Clone + [const] Send> where T: const Default { Variant(T), Other }",
    "enum MyEnum<T: Clone + Send> where T: Default { Variant(T), Other }"
)]
#[case::union_with_bounds(
    "union MyUnion<T: c0nst<Copy>> where T: c0nst<Clone> { field: T }",
    "union MyUnion<T: const Copy> where T: const Clone { field: T }",
    "union MyUnion<T: Copy> where T: Clone { field: T }"
)]
#[case::type_alias_comprehensive(
    "type MyType<T: c0nst<Clone> + ?c0nst<Send>> where T: c0nst<Default> = Vec<T>;",
    "type MyType<T: const Clone + [const] Send> where T: const Default = Vec<T>;",
    "type MyType<T: Clone + Send> where T: Default = Vec<T>;"
)]
#[case::invalid_c0nst_patterns(
    "fn test<T: c0nst>() {}",
    "fn test<T: c0nst>() {}",
    "fn test<T: c0nst>() {}"
)]
#[case::c0nst_multiple_args_fallback(
    "fn test<T: c0nst<Clone, Send>>() {}",
    "fn test<T: const Clone>() {}",
    "fn test<T: Clone>() {}"
)]
#[case::async_const_ordering(
    "#[c0nst] async fn test() -> i32 { 42 }",
    "const async fn test() -> i32 { 42 }",
    "async fn test() -> i32 { 42 }"
)]
#[case::doc_comments(
    "/// Documentation\n#[c0nst] fn test() -> i32 { 42 }",
    "#[doc = \"Documentation\"] const fn test() -> i32 { 42 }",
    "#[doc = \"Documentation\"] fn test() -> i32 { 42 }"
)]
#[case::unsupported_const_item(
    "const MY_CONST: i32 = 42;",
    "const MY_CONST: i32 = 42;",
    "const MY_CONST: i32 = 42;"
)]
#[case::unsupported_static_item(
    "static MY_STATIC: &str = \"hello\";",
    "static MY_STATIC: &str = \"hello\";",
    "static MY_STATIC: &str = \"hello\";"
)]
#[case::unsupported_use_item(
    "use std::collections::HashMap;",
    "use std::collections::HashMap;",
    "use std::collections::HashMap;"
)]
#[case::unsupported_extern_crate(
    "extern crate serde;",
    "extern crate serde;",
    "extern crate serde;"
)]
#[case::unsupported_macro_rules(
    "macro_rules! my_macro { () => {}; }",
    "macro_rules! my_macro { () => {}; }",
    "macro_rules! my_macro { () => {}; }"
)]
#[case::unsupported_module_item(
    "mod my_module { fn test() {} }",
    "mod my_module { fn test() {} }",
    "mod my_module { fn test() {} }"
)]
#[case::impl_item_type_alias(
    "impl MyTrait for MyType { type AssocType = i32; }",
    "impl MyTrait for MyType { type AssocType = i32; }",
    "impl MyTrait for MyType { type AssocType = i32; }"
)]
#[case::trait_item_type_alias(
    "trait MyTrait { type AssocType; }",
    "trait MyTrait { type AssocType; }",
    "trait MyTrait { type AssocType; }"
)]
#[case::trait_method_with_default(
    "trait MyTrait { #[c0nst] fn method(&self) -> i32 { 42 } }",
    "trait MyTrait { const fn method(&self) -> i32 { 42 } }",
    "trait MyTrait { fn method(&self) -> i32 { 42 } }"
)]
#[case::lifetime_bounds(
    "fn test<'a, T: 'a>() where T: 'a {}",
    "fn test<'a, T: 'a>() where T: 'a {}",
    "fn test<'a, T: 'a>() where T: 'a {}"
)]
#[case::invalid_c0nst_bare(
    "fn test<T: c0nst>() {}",
    "fn test<T: c0nst>() {}",
    "fn test<T: c0nst>() {}"
)]
#[case::invalid_c0nst_parentheses(
    "fn test<T: c0nst(Clone)>() {}",
    "fn test<T: c0nst(Clone)>() {}",
    "fn test<T: c0nst(Clone)>() {}"
)]
#[case::invalid_c0nst_empty(
    "fn test<T: c0nst<>>() {}",
    "fn test<T: c0nst<>>() {}",
    "fn test<T: c0nst<>>() {}"
)]
#[case::multi_segment_path(
    "fn test<T: std::marker::Send>() {}",
    "fn test<T: std::marker::Send>() {}",
    "fn test<T: std::marker::Send>() {}"
)]
#[case::trait_with_supertraits(
    "trait MyTrait: Clone + Send { fn method(&self); }",
    "trait MyTrait: Clone + Send { fn method(&self); }",
    "trait MyTrait: Clone + Send { fn method(&self); }"
)]
#[case::single_where_predicate(
    "struct MyStruct<T> where T: Clone { field: T }",
    "struct MyStruct<T> where T: Clone { field: T }",
    "struct MyStruct<T> where T: Clone { field: T }"
)]
#[case::multiple_where_predicates(
    "struct MyStruct<T, U> where T: Clone, U: Send { t: T, u: U }",
    "struct MyStruct<T, U> where T: Clone, U: Send { t: T, u: U }",
    "struct MyStruct<T, U> where T: Clone, U: Send { t: T, u: U }"
)]
#[case::lifetime_where_predicates(
    "fn test<'a, 'b>() where 'a: 'b {}",
    "fn test<'a, 'b>() where 'a: 'b {}",
    "fn test<'a, 'b>() where 'a: 'b {}"
)]
#[case::mod_without_content("mod empty;", "mod empty;", "mod empty;")]
fn test_transformations(#[case] input: &str, #[case] nightly: &str, #[case] stable: &str) {
    let nightly = nightly.replace(&[' ', '\n', '\t'][..], "");
    let stable = stable.replace(&[' ', '\n', '\t'][..], "");
    let item: Item = syn::parse_str(input).expect("Failed to parse input");

    // Nightly Output
    let result = item
        .transform(Target::Nightly)
        .to_string()
        .replace(&[' ', '\n', '\t'][..], "");
    assert_eq!(
        result, nightly,
        "Expected nightly output: {nightly}\nGot: {result}"
    );

    // Stable Output
    let result = item
        .transform(Target::Stable)
        .to_string()
        .replace(&[' ', '\n', '\t'][..], "");
    assert_eq!(
        result, stable,
        "Expected stable output: {stable}\nGot: {result}"
    );
}

#[rstest::rstest]
// Supported item types (should return true)
#[case::embedded_module("mod embedded { fn test() {} }", true)]
#[case::trait_item("trait MyTrait { fn method(&self); }", true)]
#[case::impl_block("impl MyTrait for MyType { fn method(&self) {} }", true)]
#[case::function("fn my_function() -> i32 { 42 }", true)]
#[case::struct_item("struct MyStruct { field: i32 }", true)]
#[case::enum_item("enum MyEnum { Variant1, Variant2(i32) }", true)]
#[case::union_item("union MyUnion { field1: i32, field2: f32 }", true)]
#[case::type_alias("type MyType = Vec<i32>;", true)]
// Unsupported item types (should return false)
#[case::const_item("const MY_CONST: i32 = 42;", false)]
#[case::static_item("static MY_STATIC: i32 = 42;", false)]
#[case::use_item("use std::collections::HashMap;", false)]
#[case::extern_crate("extern crate serde;", false)]
#[case::macro_rules("macro_rules! my_macro { () => {}; }", false)]
fn test_can_adapt(#[case] input: &str, #[case] expected_can_adapt: bool) {
    let item: Item = syn::parse_str(input).expect("Failed to parse input");
    assert_eq!(
        item.can_m0rph().is_ok(),
        expected_can_adapt,
        "Item type should {} be adaptable: {}",
        if expected_can_adapt { "" } else { "not" },
        input
    );
}
