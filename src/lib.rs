use std::collections::HashMap;
use std::fs;
use std::io::{stdin, stdout, ErrorKind, Write};
use std::path::PathBuf;
use std::process::exit;
extern crate directories;
#[derive(Debug)]
pub struct VirtualMachine {
    pub name: String,
    pub path: String,
}
#[derive(Debug)]
pub struct VMS {
    pub basedir: String,
    pub dir: Vec<String>,
    pub vms: Option<HashMap<String, VirtualMachine>>,
}

impl VMS {
    pub fn new(basedir: &str) -> VMS {
        let basedir = String::from(basedir);
        let dir = VMS::unwrap_dir(&basedir);
        VMS {
            basedir,
            dir,
            vms: Option::None,
        }
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

    pub fn init(&mut self) {
        //初始化虚拟机映射表；
        let mut vmlist = HashMap::new();
        for vm_dir in &self.dir {
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
        self.vms = Some(vmlist);
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

pub struct Profile(pub fs::File, pub Option<VMS>);

impl Profile {
    pub fn new() -> Profile {
        let conf = check_conf_file().unwrap();

        Profile(conf, None)
    }
}
fn check_conf_file() -> Result<fs::File, &'static str> {
    let mut filepath = PathBuf::from(
        directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .to_str()
            .unwrap(),
    );
    filepath.push(".vmstart");
    filepath.push("vmstart.conf");
    println!("{:?}", filepath);
    let file = fs::File::open("vmstart.conf");
    if let Err(_) = file {
        return match fs::File::open(&filepath) {
            Ok(file) => Ok(file),
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
                    let file = fs::File::open(&filepath).unwrap();
                    return Ok(file);
                }
                return Err("");
            }
        };
    };
    Err("")
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
