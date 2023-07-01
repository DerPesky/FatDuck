#![deny(
    absolute_paths_not_starting_with_crate,
    future_incompatible,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    non_ascii_idents,
    nonstandard_style,
    noop_method_call,
    pointer_structural_match,
    private_in_public,
    rust_2018_idioms,
    unused_qualifications
)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    // missing_debug_implementations,
    // missing_docs,
    clippy::dbg_macro,
    clippy::print_stderr,
    clippy::print_stdout
)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]
mod chess;
mod neural;
mod search;
mod time;
mod utils;
pub(crate) mod pblczero {
    include!(concat!(env!("OUT_DIR"), "/pblczero.rs"));
}
