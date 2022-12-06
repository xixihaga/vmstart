use std::fs;
use std::path::Path;
use std::collections::HashMap;
#[derive(Debug)]
pub struct VirtualMachine {
    name: String,
    path: String,
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
        for indir in &self.dir {
            let vm_dir = fs::read_dir(indir).unwrap();
            for i in vm_dir{
                let temp = String::from(i.unwrap().path().to_str().unwrap());
                let mut key_name = String::new();

                if temp.ends_with(".vmx") {
                    let name = Path::new(&temp);
                    let vm = VirtualMachine {
                        name: String::from(name.file_stem().unwrap().to_str().unwrap()),
                        path: temp,
                    };
                    vmlist.insert(String::from(&vm.name),vm);
                }
            }
        }
        vmlist
    }
}

impl VirtualMachine {
    pub fn start(&self){
        let command = format!("vmrun start \"{}\" nogui", self.path);
        println!("{}",command);
    }
}