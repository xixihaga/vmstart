use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use vmstart::{check_conf_file, Profile, VirtualMachine, VMS, create_conf};
extern crate json;
extern crate serde_json;
fn main() {
    let file = fs::File::create("test").unwrap();
    let a = std::cell::RefCell::from(file);
    a.borrow_mut().write("alsdjfalsjdfkljasdkljf".as_bytes());
}
