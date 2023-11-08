use crate::vm::{VM, VMType, VMS};
// use std::fmt;
// use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;


#[derive(Debug)]
pub struct Scan;


impl Scan {
    pub fn get_vms(path: &Vec<&str>) -> VMS {
        let mut vms = HashMap::new();
        for i in path {
            let dir = Path::new(&i);
            if dir.is_dir() {
                let list = Scan::get_sub_path(dir);
                for i in list {
                    match Scan::get_vm(&i) {
                        Some((name, vm)) => vms.insert(name, vm),
                        _=> continue ,
                    };
                }
            }
        }
        VMS{vms}
    }

    pub fn get_sub_path(dir: &Path) -> Vec<PathBuf>{
        let mut list:Vec<PathBuf> = Vec::new();
        if dir.is_dir() {
            for entry in dir.read_dir().unwrap() {
                if let Ok(path) = entry {
                    if  path.path().is_dir() {
                        list.push(path.path())
                    }
                }
            }
        }
        list
    }

    pub fn get_vm (path: &PathBuf) -> Option<(String, VM)> {
        let mut name = String::new();
        let mut vm_path = PathBuf::new();
        for i in path.read_dir().unwrap() {
            if i.as_ref().unwrap().path().is_file() {
                if i.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".vmx") {
                    if name.len() == 0 {
                        name = i.as_ref().unwrap().file_name().to_str().unwrap().split(".").next().unwrap().to_string();
                    }
                    vm_path = i.as_ref().unwrap().path();
                }
                if i.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".alias") {
                    name = i.as_ref().unwrap().file_name().to_str().unwrap().split(".").next().unwrap().to_string();
                }
            }
        }
        if name.len() > 0 && vm_path.exists() {
            Some((name, VM::new(vm_path.to_str().unwrap().to_string(), Scan::get_type(&vm_path))))
        } else {
            None
        }
    }
    
    pub fn get_type(path: &PathBuf) -> VMType {
        if path.file_name().unwrap().to_str().unwrap().ends_with(".vmx") {
            VMType::VMware
        // } else if path.file_name().unwrap().to_str().unwrap().ends_with(".vbox") {
        }else{
            VMType::VirtualBox
        } 
    }
}



