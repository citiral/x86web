use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use goblin::{error, Object};
use std::ops::{Index, IndexMut};
use crate::*;

mod elf;

const PAGE_SIZE: u32 = 4096;

pub struct Page {
    pub mem: [u8; PAGE_SIZE as usize],
}

impl Page {
    pub fn new() -> Page {
        Page {
            mem: [0; PAGE_SIZE as usize],
        }
    }
}

impl Index<u32> for Page {
    type Output = u8;
    fn index(&self, s: u32) -> &u8 {
        return &self.mem[s as usize];
    }
}

impl IndexMut<u32> for Page {
    fn index_mut(&mut self, s: u32) -> &mut u8 {
        return &mut self.mem[s as usize];
    }
}

#[wasm_bindgen]
pub struct Memory {
    pages: HashMap<u32, Page>,
}

impl Memory {
    pub fn get_page_for_address(&mut self, address: u32) -> &mut Page {
        return self.pages.entry(address / PAGE_SIZE).or_insert(Page::new());
    }

    pub fn get_at(&self, address: u32) -> u8 {
        if let Some(page) = self.pages.get(&(address / PAGE_SIZE)) {
            page[address % PAGE_SIZE]
        } else {
            0
        }
    }

    pub fn set_at(&mut self, address: u32, value: u8) {
        self.get_page_for_address(address)[address % PAGE_SIZE] = value;
    }
}

#[wasm_bindgen]
impl Memory {
    pub fn new() -> Memory {
        Memory {
            pages: HashMap::new()
        }
    }

    pub fn load_and_link(&mut self, file: &js_sys::Uint8Array) -> u32 {
        console_log!("Linking file.\n");

        let buffer = clone_uint8_array_as_native(file);
        if let Result::Ok(parsed) = Object::parse(&buffer) {
            match parsed {
                Object::Elf(elf) => {
                    console_log!("found elf file.");
                    return match self.link_elf(&buffer, &elf) {
                        Ok(entry) => {
                            console_log!("linked elf file.");
                            entry
                        },
                        Err(s)  => {
                            console_log!("Failed linking elf file: {}", s);
                            0
                        }
                    }
                },
                Object::Unknown(magic) => {
                    console_log!("Unknown magic {:#x}.", magic);
                    return 0;
                },
                _ => {
                    console_log!("Unhandled file type.");
                    return 0;
                }
            }
        } else {
            console_log!("Failed parsing binary.");
            return 0;
        }
    }
}

fn clone_uint8_array_as_native(array: &js_sys::Uint8Array) -> Vec<u8> {
    let mut arr = vec![0; array.length() as usize];
    for i in 0..array.length() {
        arr[i as usize] = array.get_index(i);
    }
    return arr;
}