use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use vmstart::{check_conf_file, Profile, VirtualMachine, VMS, create_conf};
extern crate json;
extern crate serde_json;
fn main() {
    let mut profile = Profile::new();
    // let vms = VMS::new("d:/VMWare");
    // create_conf()
    // println!("{:?}", profile.0.path());
    profile.save()

}
