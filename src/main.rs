#[allow(dead_code, unused_variables, unused_imports)]
// use std::collections::HashMap;
// use std::fs;
// use std::io::{Read, Write};
// use vmstart::{check_conf_file, Profile, VirtualMachine, VMS, create_conf};
// extern crate json;
// extern crate serde_json;
use std::env;

// use vmstart::parse_command;
fn main() {
    let profile = vmstart::Profile::new();
    profile.run();
}
