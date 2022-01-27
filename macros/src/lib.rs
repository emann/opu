// #![deny(clippy::correctness)]
// #![deny(clippy::style)]
// #![deny(clippy::complexity)]
// #![deny(clippy::perf)]
// #![warn(clippy::pedantic)]
// #![deny(
// absolute_paths_not_starting_with_crate,
// anonymous_parameters,
// bad_style,
// const_err,
// dead_code,
// keyword_idents,
// improper_ctypes,
// macro_use_extern_crate,
// meta_variable_misuse, // May have false positives
// missing_abi,
// missing_debug_implementations, // can affect compile time/code size
// missing_docs,
// rustdoc::missing_doc_code_examples,
// no_mangle_generic_items,
// non_shorthand_field_patterns,
// noop_method_call,
// overflowing_literals,
// path_statements,
// patterns_in_fns_without_body,
// pointer_structural_match,
// private_in_public,
// semicolon_in_expressions_from_macros,
// //single_use_lifetimes,
// trivial_casts,
// trivial_numeric_casts,
// unaligned_references,
// unconditional_recursion,
// unreachable_pub,
// unsafe_code,
// unused,
// unused_allocation,
// unused_comparisons,
// unused_extern_crates,
// unused_import_braces,
// unused_lifetimes,
// unused_parens,
// unused_qualifications,
// unused_results,
// variant_size_differences,
// while_true
// )]
//
// mod subdirs;
//
// #[allow(unused_extern_crates)]
// extern crate proc_macro;
//
// use proc_macro::TokenStream;
// use syn::DeriveInput;
//
// #[proc_macro_derive(OP1Subdir, attributes(parse))]
// pub fn derive_parsing(input: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(input as DeriveInput);
//     parse::derive_parsing(&input)
//         .unwrap_or_else(|err| err.to_compile_error())
//         .into()
// }
//
// //
// // impl TryFrom<PathBuf> for Album {
// //     type Error = ();
// //
// //     fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
// //         path.try_exists().map_or_else(Error::MissingDir, Self)
// //     }
// // }
