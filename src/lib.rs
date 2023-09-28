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
    clippy::integer_arithmetic,
    clippy::match_on_vec_items,
    clippy::match_wild_err_arm,
    clippy::missing_panics_doc,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    // maybe we should use?
    clippy::string_slice,
    // additional lints
    clippy::empty_loop
)]

extern crate nom;
pub mod parser;

#[macro_use]
extern crate serde_derive;
