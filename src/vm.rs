use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum VMType {
    VMware,
    VirtualBox,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct VM {
    pub path: String,
    pub vmtype: VMType,
}


impl VM {
    pub fn new(path: String, vmtype: VMType) -> Self {
        Self{path, vmtype}
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct VMS {
    pub vms: HashMap<String, VM>
}

impl VMS {
    pub fn new() -> Self {
        Self {
            vms: HashMap::new(),
        }
    }
}


pub trait Active {
    fn start(&self);
    fn stop(&self);
    fn shutdown(&self);
    fn restart(&self);
    fn pause(&self);
    fn unpause(&self);
    fn suspend(&self);
    fn run_command(&self, args: Vec<&str>) -> std::io::Result<String> {
        use std::process::Command;
        let output = Command::new("vmrun").args(args).output().unwrap();
        let content = String::from_utf8(output.stdout);
        let msg = match content{
            Ok(msg) => msg,
            Err(_) => "Exec Error!".to_string(),
        };
        Ok(msg)
    }
}

impl Active for VM {

    fn start(&self) {
        let args = vec!["start", &self.path, "nogui"];
        self.run_command(args).unwrap();
    }
    fn stop(&self) {
        let args = vec!["stop", &self.path, "hard"];
        self.run_command(args).unwrap();
    }
    fn shutdown(&self) {
        let args = vec!["stop", &self.path, "soft"];
        self.run_command(args).unwrap();
    }
    fn restart(&self) {
        let args = vec!["reset", &self.path];
        self.run_command(args).unwrap();
    }
    fn pause(&self) {
        let args = vec!["pause", &self.path];
        self.run_command(args).unwrap();
    }
    fn unpause(&self) {
        let args = vec!["unpause", &self.path];
        self.run_command(args).unwrap();
    }
    fn suspend(&self) {
        let args = vec!["suspend", &self.path];
        self.run_command(args).unwrap();
    }
}


pub trait Goods {
    fn get(&self, name: &str) -> &VM;
    fn list(&self) -> String;
}


impl Goods for VMS {
    fn get(&self, name: &str) -> &VM {
        self.vms.get(name).unwrap()
    }

    fn list(&self) -> String {
        let mut list = String::new();
        for (name, path) in self.vms.iter() {
            list.push_str(&format!("{}\t{}\n", name, path.path))
        }
        list
    }
}