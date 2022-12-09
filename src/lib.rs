use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{stdin, stdout, BufReader, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
extern crate directories;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualMachine {
    pub name: String,
    pub path: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct VMS {
    pub basedir: String,
    pub vms: HashMap<String, VirtualMachine>,
}

impl VMS {
    pub fn new(basedir: &str) -> VMS {
        let basedir = String::from(basedir);
        let dir = VMS::unwrap_dir(&basedir);
        let vms = VMS::init(dir).unwrap();
        VMS { basedir, vms }
    }

    fn unwrap_dir(basedir: &str) -> Vec<String> {
        let main_dir = fs::read_dir(&basedir).unwrap();
        let mut dir = Vec::new();
        for i in main_dir {
            let temp = i.unwrap();
            if temp.metadata().unwrap().is_dir() {
                dir.push(String::from(temp.path().to_str().unwrap()));
            }
        }
        dir
    }

    pub fn init(dir: Vec<String>) -> io::Result<HashMap<String, VirtualMachine>> {
        //初始化虚拟机映射表；
        let mut vmlist = HashMap::new();
        for vm_dir in dir {
            let mut vm_name = String::from(vm_dir.split("\\").last().unwrap());
            let mut path = String::new();
            let name = vm_name.clone();
            for i in fs::read_dir(&vm_dir).unwrap() {
                let file_path = String::from(i.unwrap().path().to_str().unwrap());
                if file_path.ends_with(".alias") {
                    //别名文件，用来更改虚拟机实例名，对应实例映射名将被修改。在需要重命名的虚拟机目录下创建一个扩展名为.alias的文件，文件名就是重命名后的虚机名称。
                    vm_name = String::from(
                        file_path
                            .split("\\")
                            .last()
                            .unwrap()
                            .split(".")
                            .next()
                            .unwrap(),
                    );
                } else if file_path.ends_with(".vmx") {
                    path = file_path;
                }
            }
            if !path.is_empty() {
                vmlist.insert(vm_name, VirtualMachine::new(name, path));
            }
        }
        return Ok(vmlist);
    }
}

impl VirtualMachine {
    pub fn new(name: String, path: String) -> VirtualMachine {
        VirtualMachine { name, path }
    }
    pub fn start(&self) {
        let command = format!("vmrun start \"{}\" nogui", self.path);
        println!("{}", command);
    }
}
#[derive(Debug)]

pub struct Profile(pub RefCell<fs::File>, pub Option<VMS>);

impl Profile {
    pub fn new() -> Profile {
        let file = RefCell::from(check_conf_file());
        let file = check_conf_file();
        match file {
            Ok(file) => {
                let mut content = String::new();
                file.borrow_mut().read_to_string(&mut content).unwrap();
                let result = parse_conf(content);
                return match result {
                    Ok((basedir, vms)) => {
                        let vms = VMS { basedir, vms };
                        Profile(file, Some(vms))
                    }
                    Err(_) => Profile(file, create_conf()),
                };
            }
            Err(e) => {
                println!("{}", e);
                panic!()
            }
        }
        //
    }
    pub fn save(&mut self) {
        if let Some(vms) = &self.1 {
            let content = serde_json::to_string(&vms);
            if let Ok(json) = content {
                self.0.borrow_mut().write(json.as_bytes()).unwrap();
            }
        }
    }
}
pub fn check_conf_file() -> Result<RefCell<fs::File>, String> {
    //TODO: 后期优化此函数
    let mut filepath = PathBuf::from(
        directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .to_str()
            .unwrap(),
    );
    filepath.push(".vmstart");
    filepath.push("vmstart.conf");
    let file = fs::File::create("vmstart.conf");
    if let Err(_) = file {
        return match fs::File::create(&filepath) {
            Ok(file) => Ok(RefCell::new(file)),
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    eprintln!("Opening configuration file ERROR: {}", e);
                    exit(1)
                }
                println!("The configuration file does not exist!");
                let mut line = String::new();
                stdout()
                    .write("Could you want initialize this program([y|yes]|other):".as_bytes())
                    .unwrap();
                stdout().flush().unwrap();
                stdin().read_line(&mut line).unwrap();
                let value = line.trim();
                if value == "y" || value == "yes" {
                    create_conf_dir();
                    let file = fs::File::create(&filepath).unwrap();
                    return Ok(RefCell::new(file));
                }
                return Err(e.to_string());
            }
        };
    };
}
pub fn create_conf_dir() {
    //初始化配置目录及配置文件
    let mut path = PathBuf::from(
        directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .to_str()
            .unwrap(),
    ); //解析用户目录
    path.push(".vmstart");
    if !path.exists() {
        if let Ok(_) = fs::create_dir(&path) {
            println!("Creating configure directory Success.")
        }
    } //用户目录下创建vmstart配置文件目录
    path.push("vmstart.conf");
    if !path.exists() {
        if let Ok(_) = fs::File::create(&path) {
            println!("Creating configure file Success.")
        };
    }
}
fn parse_conf(content: String) -> Result<(String, HashMap<String, VirtualMachine>), String> {
    // use serde_json::Value;
    let vm_json: Result<Value, serde_json::Error> = serde_json::from_str(&content);
    match vm_json {
        Ok(vm_json) => {
            let mut vm_map: HashMap<String, VirtualMachine> = HashMap::new();
            let mut basedir = String::new();
            if let Value::String(base) = &vm_json["basedir"] {
                basedir = String::from(base)
            };
            if let Value::Object(vms) = &vm_json["vms"] {
                for (k, v) in vms.iter() {
                    vm_map.insert(
                        k.to_string(),
                        VirtualMachine {
                            name: String::from(v["name"].as_str().unwrap()),
                            path: String::from(v["path"].as_str().unwrap()),
                        },
                    );
                }
            }
            Ok((basedir, vm_map))
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn create_conf() -> Option<VMS> {
    let mut line = String::new();
    stdout()
        .write("Please enter the VMs directory:  ".as_bytes())
        .unwrap();
    stdout().flush().unwrap();
    stdin().read_line(&mut line).unwrap();
    let path = Path::new(line.trim());
    Some(VMS::new(path.to_str().unwrap()))
}
