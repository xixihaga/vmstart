use std::fs;
use std::collections::HashMap;
#[derive(Debug)]
pub struct VirtualMachine {
    pub name: String,
    pub path: String,
}
pub struct VMdir {
    pub basedir: String,
    pub dir:Vec<String>
}

impl VMdir {
    pub fn new(basedir: &str) -> VMdir{
        let basedir = String::from(basedir);
        let dir =VMdir::unwrap_dir(&basedir);
        VMdir{basedir, dir}
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
    
    pub fn get_vms(&self) -> HashMap<String, VirtualMachine> {
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
        vmlist   
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