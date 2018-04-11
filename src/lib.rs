#![feature(alloc)]
#![cfg_attr(feature = "wasm", feature(proc_macro, wasm_custom_section, wasm_import_module, global_allocator))]

#![no_std]
#[macro_use]
extern crate alloc;
#[macro_use] extern crate nom;
#[cfg(feature = "wasm")] extern crate wasm_bindgen;
#[cfg(feature = "wasm")] extern crate wee_alloc;
#[cfg_attr(test, macro_use)] extern crate serde_json;

pub mod ast;
#[macro_use] pub mod combinators;
pub mod parser;
#[cfg(feature = "wasm")] pub mod wasm;


#[cfg(feature = "wasm")] use wee_alloc::WeeAlloc;
use alloc::Vec;

#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


/// Represent the type of the input elements.
pub type InputElement = u8;

/// Represent the type of the input.
pub type Input<'a> = &'a [InputElement];

/// Test
pub fn root(input: Input) -> Result<(Input, Vec<ast::Block>), nom::Err<Input>> {
    parser::block_list(input)
}
