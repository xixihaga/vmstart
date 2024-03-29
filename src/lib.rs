use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
extern crate directories;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;

pub fn run_command(args: Vec<&str>) -> std::io::Result<String> {
    let output = Command::new("vmrun").args(args).output().unwrap();
    let content = String::from_utf8(output.stdout);
    let msg = match content{
        Ok(msg) => msg,
        Err(_) => "Exec Error!".to_string(),
    };
    Ok(msg)
}

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
#[derive(Debug)]
pub enum Operate {
    Start(String),
    Stop(String),
    Shutdown(String),
    Restart(String),
    Poweroff(String),
    Reboot(String),
    HangUp(String),
    List,
    Scan,
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
        let args = vec!["start", &self.path, "nogui"];
        run_command(args).unwrap();
    }
    pub fn stop(&self) {
        let args = vec!["stop", &self.path];
        run_command(args).unwrap();
    }
    pub fn shutdown(&self) {
        let command = format!("vmrun shutdown \"{}\" nogui", self.path);
        println!("{}", command);
    }
    pub fn restart(&self) {
        let args = vec!["reset", &self.path];
        run_command(args).unwrap();
    }
    pub fn reboot(&self) {
        let command = format!("vmrun reboot \"{}\" nogui", self.path);
        println!("{}", command);
    }
    pub fn poweroff(&self) {
        let command = format!("vmrun poweroff \"{}\" nogui", self.path);
        println!("{}", command);
    }
    pub fn hangup(&self) {
        let command = format!("vmrun hangup \"{}\" nogui", self.path);
        println!("{}", command);
    }
}
#[derive(Debug)]

pub struct Profile(pub std::path::PathBuf, pub Option<VMS>);

impl Profile {
    pub fn new() -> Profile {
        let path = check_conf_file();
        match path {
            Ok(path) => {
                let file = fs::File::open(&path);
                match file {
                    Ok(mut file) => {
                        let mut content = String::new();
                        file.read_to_string(&mut content).unwrap();
                        let result = parse_conf(content);
                        match result {
                            Ok((basedir, vms)) => {
                                let vms = VMS { basedir, vms };
                                Profile(path, Some(vms))
                            }
                            Err(_) => {
                                let vms = create_conf();
                                Profile(path, vms)
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                        exit(1)
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                panic!()
                // Profile(file, None)
            }
        }
    }
    pub fn save(&mut self) {
        if let Some(vms) = &self.1 {
            let content = serde_json::to_string(&vms);
            if let Ok(json) = content {
                let result = fs::File::create(&self.0);
                match result {
                    Ok(mut file) => {
                        file.write(json.as_bytes()).unwrap();
                    }
                    Err(e) => eprintln!("{}", e.to_string()),
                }
            }
        }
    }
    pub fn get(&self) -> &VMS {
        match &self.1 {
            Some(vms) => &vms,
            None => exit(1),
        }
    }
    pub fn run(&mut self) {
        let oper = parse_command();
        println!("{:?}", oper);
        match oper {
            Operate::Start(vm_name) => {
                let vm = self.1.as_ref().unwrap().vms.get(&vm_name);
                match vm {
                    Some(vm) => vm.start(),
                    None => eprintln!("The VM {} does not exist", vm_name),
                }
            }
            Operate::Stop(vm_name) => {
                let vm = self.1.as_ref().unwrap().vms.get(&vm_name);
                match vm {
                    Some(vm) => vm.stop(),
                    None => eprintln!("The VM {} does not exist", vm_name),
                }
            }
            Operate::Shutdown(vm_name) => {
                let vm = self.1.as_ref().unwrap().vms.get(&vm_name);
                match vm {
                    Some(vm) => vm.shutdown(),
                    None => eprintln!("The VM {} does not exist", vm_name),
                }
            }
            Operate::Restart(vm_name) => {
                let vm = self.1.as_ref().unwrap().vms.get(&vm_name);
                match vm {
                    Some(vm) => vm.restart(),
                    None => eprintln!("The VM {} does not exist", vm_name),
                }
            }
            Operate::Poweroff(vm_name) => {
                let vm = self.1.as_ref().unwrap().vms.get(&vm_name);
                match vm {
                    Some(vm) => vm.poweroff(),
                    None => eprintln!("The VM {} does not exist", vm_name),
                }
            }
            Operate::Reboot(vm_name) => {
                let vm = self.1.as_ref().unwrap().vms.get(&vm_name);
                match vm {
                    Some(vm) => vm.reboot(),
                    None => eprintln!("The VM {} does not exist", vm_name),
                }
            }
            Operate::HangUp(vm_name) => {
                let vm = self.1.as_ref().unwrap().vms.get(&vm_name);
                match vm {
                    Some(vm) => vm.hangup(),
                    None => eprintln!("The VM {} does not exist", vm_name),
                }
            }
            Operate::List => {
                for (k, v) in self.1.as_ref().unwrap().vms.iter() {
                    println!("{}\t{}",k, v.path )
                }
            }
            Operate::Scan => {
				create_conf_dir()
            }
        }
    }
}
pub fn check_conf_file() -> io::Result<std::path::PathBuf> {
    let mut filepath = PathBuf::from("vmstart.conf");
    if !filepath.exists() {
        filepath = PathBuf::from(
            directories::UserDirs::new()
                .unwrap()
                .home_dir()
                .to_str()
                .unwrap(),
        );
        filepath.push(".vmstart\\vmstart.conf")
    }
    if !filepath.exists() {
        println!("The configuration file does not exist!");
        let mut line = String::new();
        stdout()
            .write("Could you want initialize this program([y|yes]|other):".as_bytes())
            .unwrap();
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        if line.trim() == "y" || line.trim() == "yes" {
            create_conf_dir()
        } else {
            exit(1)
        }
    }
    Ok(filepath)
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

pub fn daemon() {
    use std::env;
    let args = env::args();
    if args.len() > 2 {
        for comm in args {
            println!("{}", comm)
        }
    }
}

pub fn parse_command() -> Operate {
    use std::env;
    use Operate::*;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Use the following command format\n\tvmstart <Operate> <VMname>");
        exit(1)
    }
    let arg = args[1].as_str();
    match arg {
        "start" => Start(args[2].to_string()),
        "stop" => Stop(args[2].to_string()),
        "shutdown" => Shutdown(args[2].to_string()),
        "restart" => Restart(args[2].to_string()),
        "poweroff" => Poweroff(args[2].to_string()),
        "reboot" => Reboot(args[2].to_string()),
        "hangup" => HangUp(args[2].to_string()),
        "list" => List,
        "Scan" => Scan,
        _ => {
            println!("Operate \"{}\" invalid", arg);
            exit(1)
        }
    }
}
