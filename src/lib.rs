use std::fs;
use std::collections::HashMap;
use std::path::{PathBuf};
#[derive(Debug)]
pub struct VirtualMachine {
    pub name: String,
    pub path: String,
}
#[derive(Debug)]
pub struct VMS {
    pub basedir: String,
    pub dir:Vec<String>,
    pub vms:Option<HashMap<String, VirtualMachine>>
}

impl VMS{
    pub fn new(basedir: &str) -> VMS{
        let basedir = String::from(basedir);
        let dir =VMS::unwrap_dir(&basedir);
        VMS{basedir, dir, vms:Option::None}
    }

    fn unwrap_dir(basedir: &str) -> Vec<String>{
        let main_dir =  fs::read_dir(&basedir).unwrap();
        let mut dir = Vec::new();
        for i in main_dir {
            let temp = i.unwrap();
            if temp.metadata().unwrap().is_dir(){
                dir.push(String::from(temp.path().to_str().unwrap()));
            }
        }
        dir
    }
    pub fn init(&mut self){
        let mut vmlist = HashMap::new();
        for vm_dir in &self.dir {
            let mut vm_name = String::from(vm_dir.split("\\").last().unwrap());
            let mut path = String::new();
            let name = vm_name.clone();
            for i in fs::read_dir(&vm_dir).unwrap() {
                let file_path = String::from(i.unwrap().path().to_str().unwrap());
                if file_path.ends_with(".alias") {
                    vm_name = String::from(file_path.split("\\").last().unwrap().split(".").next().unwrap());
                } else if file_path.ends_with(".vmx") {
                    path = file_path;
                }
            }
            if !path.is_empty() {
                vmlist.insert(vm_name, VirtualMachine::new(name, path));
            }
        }
        self.vms = Some(vmlist); 
    }
}

impl VirtualMachine {
    pub fn new(name:String, path:String) -> VirtualMachine {
        VirtualMachine { name, path }
    }
    pub fn start(&self){
        let command = format!("vmrun start \"{}\" nogui", self.path);
        println!("{}",command);
    }
}
#[derive(Debug)]

pub struct Profile(pub fs::File, pub Option<VMS>);

impl Profile {
    pub fn new() {
        
    }
    pub fn init() {
        extern crate directories;
        let mut path = PathBuf::from(directories::UserDirs::new().unwrap().home_dir().to_str().unwrap());
        path.push(".vmstart");
        if let Ok(_) = fs::create_dir(&path){
            println!("Creating configure directory Success.")
        }
        path.push("vmstart.conf");
        if let Ok(_) = fs::File::create(&path) {
            println!("Creating configure file Success.")
        };
    }
    pub fn write (&mut self) {
        use std::io::prelude::*;
        self.0.write_all(b"asldfj").unwrap()
    }

}