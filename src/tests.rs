//! Test suite for the TokenStream-based transformation engine.
//!
//! This module tests the core transformation logic without relying on syn's AST parsing.

#![cfg(test)]

use proc_macro2::TokenStream;

use crate::{Convert, Target};

#[rstest::rstest]
#[case::basic_items(
    "const fn test() -> i32 { 42 }",
    "const fn test() -> i32 { 42 }",
    "const fn test() -> i32 { 42 }"
)]
#[case::basic_trait(
    "c0nst trait MyTrait { fn method(&self) -> i32; }",
    "const trait MyTrait { fn method(&self) -> i32; }",
    "trait MyTrait { fn method(&self) -> i32; }"
)]
#[case::basic_impl(
    "impl c0nst MyTrait for MyType { fn method(&self) -> i32 { 42 } }",
    "impl const MyTrait for MyType { fn method(&self) -> i32 { 42 } }",
    "impl MyTrait for MyType { fn method(&self) -> i32 { 42 } }"
)]
#[case::comprehensive_wrapper_traits(
    "fn test<T>() where T: c0nst Trait + [c0nst] Other {}",
    "fn test<T>() where T: const Trait + [const] Other {}",
    "fn test<T>() where T: Trait + Other {}"
)]
#[case::bare_function_types(
    "type Callback = fn(impl c0nst Send + [c0nst] Sync);",
    "type Callback = fn(impl const Send + [const] Sync);",
    "type Callback = fn(impl Send + Sync);"
)]
#[case::impl_self_type(
    "impl c0nst Clone for MyType { fn clone(&self) -> Self { todo!() } }",
    "impl const Clone for MyType { fn clone(&self) -> Self { todo!() } }",
    "impl Clone for MyType { fn clone(&self) -> Self { todo!() } }"
)]
#[case::array_types(
    "type ArrayType = [impl c0nst Send; 10];",
    "type ArrayType = [impl const Send; 10];",
    "type ArrayType = [impl Send; 10];"
)]
#[case::slice_types(
    "type SliceType = [impl c0nst Send];",
    "type SliceType = [impl const Send];",
    "type SliceType = [impl Send];"
)]
#[case::reference_types(
    "type RefType = &impl c0nst Send;",
    "type RefType = &impl const Send;",
    "type RefType = &impl Send;"
)]
#[case::mutable_reference_types(
    "type MutRefType = &mut impl c0nst Send;",
    "type MutRefType = &mut impl const Send;",
    "type MutRefType = &mut impl Send;"
)]
#[case::tuple_types(
    "type TupleType = (impl c0nst Send, impl [c0nst] Sync);",
    "type TupleType = (impl const Send, impl [const] Sync);",
    "type TupleType = (impl Send, impl Sync);"
)]
#[case::parenthesized_types(
    "type ParenType = (impl c0nst Send);",
    "type ParenType = (impl const Send);",
    "type ParenType = (impl Send);"
)]
#[case::trait_object_types(
    "type TraitObjType = dyn c0nst Send + [c0nst] Sync;",
    "type TraitObjType = dyn const Send + [const] Sync;",
    "type TraitObjType = dyn Send + Sync;"
)]
#[case::non_type_generic_arguments(
    "fn test<const N: usize>() -> [i32; N] { [0; N] }",
    "fn test<const N: usize>() -> [i32; N] { [0; N] }",
    "fn test<const N: usize>() -> [i32; N] { [0; N] }"
)]
#[case::lifetime_generic_arguments(
    "fn test<'a, T>() -> &'a T { todo!() }",
    "fn test<'a, T>() -> &'a T { todo!() }",
    "fn test<'a, T>() -> &'a T { todo!() }"
)]
#[case::multiple_constraints_in_where(
    "fn test<T>() where T: c0nst Clone + c0nst Send + [c0nst] Sync {}",
    "fn test<T>() where T: const Clone + const Send + [const] Sync {}",
    "fn test<T>() where T: Clone + Send + Sync {}"
)]
#[case::lifetime_arguments_only(
    "fn test<'a>() -> &'a str { \"\" }",
    "fn test<'a>() -> &'a str { \"\" }",
    "fn test<'a>() -> &'a str { \"\" }"
)]
#[case::const_arguments_only(
    "fn test<const N: usize>() -> [u8; N] { [0; N] }",
    "fn test<const N: usize>() -> [u8; N] { [0; N] }",
    "fn test<const N: usize>() -> [u8; N] { [0; N] }"
)]
#[case::bare_fn_with_named_params(
    "type NamedFn = fn(x: impl c0nst Send, y: &dyn [c0nst] Sync) -> i32;",
    "type NamedFn = fn(x: impl const Send, y: &dyn [const] Sync) -> i32;",
    "type NamedFn = fn(x: impl Send, y: &dyn Sync) -> i32;"
)]
#[case::impl_without_trait(
    "impl c0nst MyStruct { fn new() -> Self { todo!() } }",
    "impl const MyStruct { fn new() -> Self { todo!() } }",
    "impl MyStruct { fn new() -> Self { todo!() } }"
)]
#[case::impl_with_negative_trait(
    "impl !Send for MyType {}",
    "impl !Send for MyType {}",
    "impl !Send for MyType {}"
)]
#[case::comprehensive_generics(
    "c0nst fn test<T: c0nst Clone + [c0nst] Send + c0nst From<u64>>() -> T where T: c0nst Default { T::default() }",
    "const fn test<T: const Clone + [const] Send + const From<u64>>() -> T where T: const Default { T::default() }",
    "fn test<T: Clone + Send + From<u64>>() -> T where T: Default { T::default() }"
)]
#[case::complex_with_modifiers(
    "#[derive(Debug)] pub c0nst unsafe fn test<'a, const N: usize, T: Clone + c0nst Send>() -> Result<T, String> where T: c0nst Default { Ok(T::default()) }",
    "#[derive(Debug)] pub const unsafe fn test<'a, const N: usize, T: Clone + const Send>() -> Result<T, String> where T: const Default { Ok(T::default()) }",
    "#[derive(Debug)] pub unsafe fn test<'a, const N: usize, T: Clone + Send>() -> Result<T, String> where T: Default { Ok(T::default()) }"
)]
#[case::trait_methods(
    "trait MyTrait { c0nst fn method(&self) -> i32 { 42 } }",
    "trait MyTrait { const fn method(&self) -> i32 { 42 } }",
    "trait MyTrait { fn method(&self) -> i32 { 42 } }"
)]
#[case::impl_methods(
    "impl MyTrait for i32 { c0nst fn method(&self) -> i32 { 42 } }",
    "impl MyTrait for i32 { const fn method(&self) -> i32 { 42 } }",
    "impl MyTrait for i32 { fn method(&self) -> i32 { 42 } }"
)]
#[case::generic_impl_comprehensive(
    "impl<T: c0nst Clone> c0nst From<T> for MyType<T> where T: [c0nst] Send { fn from(t: T) -> Self { MyType(t) } }",
    "impl<T: const Clone> const From<T> for MyType<T> where T: [const] Send { fn from(t: T) -> Self { MyType(t) } }",
    "impl<T: Clone> From<T> for MyType<T> where T: Send { fn from(t: T) -> Self { MyType(t) } }"
)]
#[case::struct_comprehensive(
    "struct MyStruct<T: Clone + c0nst Send, U: [c0nst] Sync> where T: c0nst Default { t: T, u: U }",
    "struct MyStruct<T: Clone + const Send, U: [const] Sync> where T: const Default { t: T, u: U }",
    "struct MyStruct<T: Clone + Send, U: Sync> where T: Default { t: T, u: U }"
)]
#[case::tuple_struct(
    "struct MyStruct<T: c0nst Clone>(T);",
    "struct MyStruct<T: const Clone>(T);",
    "struct MyStruct<T: Clone>(T);"
)]
#[case::enum_comprehensive(
    "enum MyEnum<T: c0nst Clone + [c0nst] Send> where T: c0nst Default { Variant(T), Other }",
    "enum MyEnum<T: const Clone + [const] Send> where T: const Default { Variant(T), Other }",
    "enum MyEnum<T: Clone + Send> where T: Default { Variant(T), Other }"
)]
#[case::union_with_bounds(
    "union MyUnion<T: c0nst Copy> where T: c0nst Clone { field: T }",
    "union MyUnion<T: const Copy> where T: const Clone { field: T }",
    "union MyUnion<T: Copy> where T: Clone { field: T }"
)]
#[case::type_alias_comprehensive(
    "type MyType<T: c0nst Clone + [c0nst] Send> where T: c0nst Default = Vec<T>;",
    "type MyType<T: const Clone + [const] Send> where T: const Default = Vec<T>;",
    "type MyType<T: Clone + Send> where T: Default = Vec<T>;"
)]
#[case::async_const_ordering(
    "const async fn test() -> i32 { 42 }",
    "const async fn test() -> i32 { 42 }",
    "const async fn test() -> i32 { 42 }"
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
    "trait MyTrait { c0nst fn method(&self) -> i32 { 42 } }",
    "trait MyTrait { const fn method(&self) -> i32 { 42 } }",
    "trait MyTrait { fn method(&self) -> i32 { 42 } }"
)]
#[case::lifetime_bounds(
    "fn test<'a, T: 'a>() where T: 'a {}",
    "fn test<'a, T: 'a>() where T: 'a {}",
    "fn test<'a, T: 'a>() where T: 'a {}"
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
#[case::impl_return(
    "c0nst fn foo() -> impl c0nst Into<usize> { 8usize }",
    "const fn foo() -> impl const Into<usize> { 8usize }",
    "fn foo() -> impl Into<usize> { 8usize }"
)]
#[case::associated_type_bounds(
    "fn test<T>() where T::Item: c0nst Send {}",
    "fn test<T>() where T::Item: const Send {}",
    "fn test<T>() where T::Item: Send {}"
)]
#[case::trait_with_simple_associated_types(
    "trait MyTrait { type Item; }",
    "trait MyTrait { type Item; }",
    "trait MyTrait { type Item; }"
)]
#[case::impl_with_simple_associated_types(
    "impl MyTrait for MyType { type Item = String; }",
    "impl MyTrait for MyType { type Item = String; }",
    "impl MyTrait for MyType { type Item = String; }"
)]
#[case::parenthesized_path_args(
    "type FnType = dyn Fn(i32) -> i32;",
    "type FnType = dyn Fn(i32) -> i32;",
    "type FnType = dyn Fn(i32) -> i32;"
)]
#[case::lifetime_in_associated_type(
    "fn test<'a, T>() where T: Iterator<Item = &'a str> {}",
    "fn test<'a, T>() where T: Iterator<Item = &'a str> {}",
    "fn test<'a, T>() where T: Iterator<Item = &'a str> {}"
)]
#[case::generic_arg_const(
    "fn test<const N: usize>() {}",
    "fn test<const N: usize>() {}",
    "fn test<const N: usize>() {}"
)]
#[case::generic_arg_lifetime("fn test<'a>() {}", "fn test<'a>() {}", "fn test<'a>() {}")]
#[case::generic_arg_type(
    "fn test<T: c0nst Clone>() {}",
    "fn test<T: const Clone>() {}",
    "fn test<T: Clone>() {}"
)]
#[case::generic_arg_type_with_constraint(
    "struct Foo<T: c0nst Default>(T);",
    "struct Foo<T: const Default>(T);",
    "struct Foo<T: Default>(T);"
)]
#[case::complex_generic_args(
    "fn test<T, U>() where T: c0nst Clone, U: c0nst Default {}",
    "fn test<T, U>() where T: const Clone, U: const Default {}",
    "fn test<T, U>() where T: Clone, U: Default {}"
)]
#[case::nested_generic_types(
    "type VecType = Vec<Box<String>>;",
    "type VecType = Vec<Box<String>>;",
    "type VecType = Vec<Box<String>>;"
)]
#[case::type_path_with_generics(
    "type MyPath = std::collections::HashMap<String, usize>;",
    "type MyPath = std::collections::HashMap<String, usize>;",
    "type MyPath = std::collections::HashMap<String, usize>;"
)]
#[case::type_path_nested_generics(
    "type ComplexPath = Vec<Option<String>>;",
    "type ComplexPath = Vec<Option<String>>;",
    "type ComplexPath = Vec<Option<String>>;"
)]
#[case::type_path_with_multiple_segments(
    "type FullyQualified = std::sync::Arc<String>;",
    "type FullyQualified = std::sync::Arc<String>;",
    "type FullyQualified = std::sync::Arc<String>;"
)]
#[case::associated_type_in_generics(
    "fn test<T: Iterator<Item = String>>() {}",
    "fn test<T: Iterator<Item = String>>() {}",
    "fn test<T: Iterator<Item = String>>() {}"
)]
#[case::associated_type_with_complex_type(
    "fn test<T: Iterator<Item = Box<String>>>() {}",
    "fn test<T: Iterator<Item = Box<String>>>() {}",
    "fn test<T: Iterator<Item = Box<String>>>() {}"
)]
#[case::multiple_associated_types(
    "fn test<T: Iterator<Item = String> + ExactSizeIterator<Item = String>>() {}",
    "fn test<T: Iterator<Item = String> + ExactSizeIterator<Item = String>>() {}",
    "fn test<T: Iterator<Item = String> + ExactSizeIterator<Item = String>>() {}"
)]
#[case::associated_type_with_lifetime(
    "fn test<'a, T: Iterator<Item = &'a str>>() {}",
    "fn test<'a, T: Iterator<Item = &'a str>>() {}",
    "fn test<'a, T: Iterator<Item = &'a str>>() {}"
)]
#[case::impl_trait_in_associated_type(
    "trait MyTrait { type Output = impl Clone; }",
    "trait MyTrait { type Output = impl Clone; }",
    "trait MyTrait { type Output = impl Clone; }"
)]
#[case::complex_associated_type_bound(
    "fn test<T>() where T: Iterator, T::Item: c0nst Clone {}",
    "fn test<T>() where T: Iterator, T::Item: const Clone {}",
    "fn test<T>() where T: Iterator, T::Item: Clone {}"
)]
#[case::function_pointer_parenthesized(
    "type FnPointer = fn(i32, String) -> usize;",
    "type FnPointer = fn(i32, String) -> usize;",
    "type FnPointer = fn(i32, String) -> usize;"
)]
#[case::const_generic_argument(
    "type Array<const N: usize> = [i32; N];",
    "type Array<const N: usize> = [i32; N];",
    "type Array<const N: usize> = [i32; N];"
)]
#[case::associated_const_argument(
    "fn test<T>() where T: IntoIterator {}",
    "fn test<T>() where T: IntoIterator {}",
    "fn test<T>() where T: IntoIterator {}"
)]
#[case::explicit_assoc_type_syntax(
    "fn test<T: Iterator<Item=String>>() {}",
    "fn test<T: Iterator<Item=String>>() {}",
    "fn test<T: Iterator<Item=String>>() {}"
)]
#[case::assoc_type_with_const_bound(
    "fn test<T: Iterator<Item: c0nst Clone>>() {}",
    "fn test<T: Iterator<Item: const Clone>>() {}",
    "fn test<T: Iterator<Item: Clone>>() {}"
)]
#[case::test_assoc_type_transformation(
    "fn generic_with_assoc<T: Iterator<Item = impl c0nst Clone>>() {}",
    "fn generic_with_assoc<T: Iterator<Item = impl const Clone>>() {}",
    "fn generic_with_assoc<T: Iterator<Item = impl Clone>>() {}"
)]
#[case::simple_impl_trait_const(
    "fn test() -> impl c0nst Clone { 42i32 }",
    "fn test() -> impl const Clone { 42i32 }",
    "fn test() -> impl Clone { 42i32 }"
)]
#[case::function_pointer_with_parenthesized_args(
    "type FnPtr = dyn Fn(i32) -> String;",
    "type FnPtr = dyn Fn(i32) -> String;",
    "type FnPtr = dyn Fn(i32) -> String;"
)]
#[case::nested_const_in_where_clause_assoc_type(
    "fn test<T>() where T: Iterator, T::Item: Clone + c0nst Send {}",
    "fn test<T>() where T: Iterator, T::Item: Clone + const Send {}",
    "fn test<T>() where T: Iterator, T::Item: Clone + Send {}"
)]
#[case::associated_const_in_generics(
    "type T = Vec<{MyTrait::CONST}>;",
    "type T = Vec<{MyTrait::CONST}>;",
    "type T = Vec<{MyTrait::CONST}>;"
)]
#[case::closure_param_types(
    "fn test() { let _closure = |x: impl c0nst Send| x; }",
    "fn test() { let _closure = |x: impl const Send| x; }",
    "fn test() { let _closure = |x: impl Send| x; }"
)]
#[case::box_dyn_trait_bounds(
    "type BoxedTrait = Box<dyn c0nst Send + c0nst Sync>;",
    "type BoxedTrait = Box<dyn const Send + const Sync>;",
    "type BoxedTrait = Box<dyn Send + Sync>;"
)]
#[case::nested_generic_complex(
    "type Complex = HashMap<String, Vec<Box<dyn c0nst Send>>>;",
    "type Complex = HashMap<String, Vec<Box<dyn const Send>>>;",
    "type Complex = HashMap<String, Vec<Box<dyn Send>>>;"
)]
#[case::function_pointer_return_impl(
    "type FnPtr = fn() -> impl c0nst Clone;",
    "type FnPtr = fn() -> impl const Clone;",
    "type FnPtr = fn() -> impl Clone;"
)]
#[case::higher_ranked_trait_bounds(
    "fn test<F>() where F: for<'a> Fn(&'a str) + c0nst Send {}",
    "fn test<F>() where F: for<'a> Fn(&'a str) + const Send {}",
    "fn test<F>() where F: for<'a> Fn(&'a str) + Send {}"
)]
#[case::raw_pointer_types(
    "type RawPtr = *const dyn c0nst Send;",
    "type RawPtr = *const dyn const Send;",
    "type RawPtr = *const dyn Send;"
)]
#[case::async_return_bounds(
    "c0nst async fn test() -> impl c0nst Send + c0nst Future<Output = i32> { async { 42 } }",
    "const async fn test() -> impl const Send + const Future<Output = i32> { async { 42 } }",
    "async fn test() -> impl Send + Future<Output = i32> { async { 42 } }"
)]
#[case::associated_type_projection(
    "fn test<T>() where <T as Iterator>::Item: c0nst Clone {}",
    "fn test<T>() where <T as Iterator>::Item: const Clone {}",
    "fn test<T>() where <T as Iterator>::Item: Clone {}"
)]
#[case::nested_closure_bounds(
    "fn test() { let _f = || -> impl c0nst Clone { 42 }; }",
    "fn test() { let _f = || -> impl const Clone { 42 }; }",
    "fn test() { let _f = || -> impl Clone { 42 }; }"
)]
#[case::pin_box_future(
    "type PinnedFuture = Pin<Box<dyn c0nst Future<Output = String>>>;",
    "type PinnedFuture = Pin<Box<dyn const Future<Output = String>>>;",
    "type PinnedFuture = Pin<Box<dyn Future<Output = String>>>;"
)]
#[case::trait_object_with_lifetime(
    "type TraitObj<'a> = &'a (dyn c0nst Send + c0nst Sync);",
    "type TraitObj<'a> = &'a (dyn const Send + const Sync);",
    "type TraitObj<'a> = &'a (dyn Send + Sync);"
)]
#[case::complex_where_clause_projection(
    "fn test<T>() where T::Output: c0nst Into<String> + c0nst Send {}",
    "fn test<T>() where T::Output: const Into<String> + const Send {}",
    "fn test<T>() where T::Output: Into<String> + Send {}"
)]
#[case::nested_impl_trait_bounds(
    "type NestedImpl = Box<dyn Fn() -> impl c0nst Iterator<Item = impl c0nst Clone>>;",
    "type NestedImpl = Box<dyn Fn() -> impl const Iterator<Item = impl const Clone>>;",
    "type NestedImpl = Box<dyn Fn() -> impl Iterator<Item = impl Clone>>;"
)]
#[case::const_block("c0nst { 32 }", "const { 32 }", "{ 32 }")]
#[case::const_destruct(
    "c0nst trait MyTrait { type Foo: c0nst Destruct; }",
    "const trait MyTrait { type Foo: const core::marker::Destruct; }",
    "trait MyTrait { type Foo; }"
)]
#[case::maybe_const_destruct(
    "c0nst trait MyTrait { type Foo: [c0nst] Destruct; }",
    "const trait MyTrait { type Foo: [const] core::marker::Destruct; }",
    "trait MyTrait { type Foo; }"
)]
#[case::trailing_const_destruct(
    "c0nst trait MyTrait { type Foo: Clone + c0nst Destruct; }",
    "const trait MyTrait { type Foo: Clone + const core::marker::Destruct; }",
    "trait MyTrait { type Foo: Clone; }"
)]
#[case::trailing_maybe_const_destruct(
    "c0nst trait MyTrait { type Foo: Clone + [c0nst] Destruct; }",
    "const trait MyTrait { type Foo: Clone + [const] core::marker::Destruct; }",
    "trait MyTrait { type Foo: Clone; }"
)]
#[case::destruct_bound_single(
    "impl<T: [c0nst] Destruct> MyTrait for T {}",
    "impl<T: [const] core::marker::Destruct> MyTrait for T {}",
    "impl<T> MyTrait for T {}"
)]
#[case::destruct_bound_multiple_first(
    "impl<T: [c0nst] Destruct + Clone + Copy> MyTrait for T {}",
    "impl<T: [const] core::marker::Destruct + Clone + Copy> MyTrait for T {}",
    "impl<T: Clone + Copy> MyTrait for T {}"
)]
#[case::destruct_bound_multiple_middle(
    "impl<T: Copy + [c0nst] Destruct + Clone> MyTrait for T {}",
    "impl<T: Copy + [const] core::marker::Destruct + Clone> MyTrait for T {}",
    "impl<T: Copy + Clone> MyTrait for T {}"
)]
#[case::destruct_bound_multiple_end(
    "impl<T: Clone + Copy + [c0nst] Destruct> MyTrait for T {}",
    "impl<T: Clone + Copy + [const] core::marker::Destruct> MyTrait for T {}",
    "impl<T: Clone + Copy> MyTrait for T {}"
)]
fn test_transformations(
    #[case] input: &str,
    #[case] nightly_expected: &str,
    #[case] stable_expected: &str,
) {
    let nightly = nightly_expected.replace(&[' ', '\n', '\t'][..], "");
    let stable = stable_expected.replace(&[' ', '\n', '\t'][..], "");
    let input: TokenStream = input.parse().expect("Failed to parse input");
    eprintln!("Input: {input:#?}");

    // Nightly Output
    let result = input
        .clone()
        .convert(Target::Nightly)
        .to_string()
        .replace(&[' ', '\n', '\t'][..], "");
    assert_eq!(result, nightly);

    // Stable Output
    let result = input
        .convert(Target::Stable)
        .to_string()
        .replace(&[' ', '\n', '\t'][..], "");
    assert_eq!(result, stable);
}
