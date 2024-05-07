#![deny(
    //panic prevention lints
    clippy::indexing_slicing,
    clippy::assertions_on_constants,
    clippy::await_holding_refcell_ref,
    clippy::diverging_sub_expression,
    clippy::expect_used,
    clippy::fallible_impl_from,
    clippy::get_last_with_len,
    clippy::get_unwrap,
    clippy::get_unwrap,
    clippy::arithmetic_side_effects,
    clippy::match_on_vec_items,
    clippy::match_wild_err_arm,
    clippy::missing_panics_doc,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::string_slice,
    clippy::empty_loop,
    clippy::correctness,
    clippy::needless_borrow,
    clippy::cast_lossless,
    clippy::obfuscated_if_else,
    clippy::index_refutable_slice,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    clippy::exit,
    clippy::todo,
    clippy::expect_used,
    clippy::unimplemented,
    clippy::manual_strip,
)]

extern crate nom;
pub mod parser;

#[macro_use]
extern crate serde_derive;
