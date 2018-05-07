/*!

WebAssembly bindings.

*/

use super::ast::Node;
use alloc::Vec;
use core::{self, mem, slice};

// This is required by `wee_alloc` and `no_std`.
#[lang = "panic_fmt"]
extern "C" fn panic_fmt(_args: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    unsafe {
        core::intrinsics::abort();
    }
}

// This is the definition of `std::ffi::c_void`, but wasm runs without std.
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum c_void {
    #[doc(hidden)] 
    __variant1,

    #[doc(hidden)] 
    __variant2
}

#[no_mangle]
pub extern "C" fn alloc(capacity: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(capacity);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    return pointer as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn root(pointer: *mut u8, length: usize) -> *mut u8 {
    unsafe {
        let input = slice::from_raw_parts(pointer, length);
        let mut output = vec![];

        if let Ok((_remaining, nodes)) = super::root(input) {
            let mut number_of_nodes = 0;

            for node in nodes {
                output.extend(node.into_bytes());
                number_of_nodes += 1;
            }

            output.insert(0, number_of_nodes as u8);
        }

        let pointer = output.as_mut_ptr();
        mem::forget(output);

        pointer
    }
}

impl<'a> Node<'a> {
    fn into_bytes(&self) -> Vec<u8> {
        match *self {
            Node::Block { name, attributes, ref children } => {
                let node_type = 1u8;
                let name_length = name.0.len() + name.1.len() + 1;
                let attributes_length = match attributes {
                    Some(attributes) => attributes.len(),
                    None             => 2
                };

                let number_of_children = children.len();
                let children: Vec<u8> =
                    children
                        .iter()
                        .flat_map(
                            |ref node| {
                                node.into_bytes()
                            }
                        )
                        .collect();

                let mut output = Vec::with_capacity(4 + name_length + attributes_length + number_of_children);

                output.push(node_type);
                output.push(name_length as u8);
                output.push(attributes_length as u8);
                output.push(number_of_children as u8);

                output.extend(name.0);
                output.push(b'/');
                output.extend(name.1);

                if let Some(attributes) = attributes {
                    output.extend(attributes);
                } else {
                    output.extend(&b"{}"[..]);
                }

                output.extend(children);

                output
            },

            Node::Phrase(phrase) => {
                let node_type = 2u8;
                let phrase_length = phrase.len();

                let mut output = Vec::with_capacity(2 + phrase_length);

                output.push(node_type);
                output.push(phrase_length as u8);

                output.extend(phrase);

                output
            }
        }
    }
}
