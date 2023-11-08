use crate::profile::{Profile, Configurate, Configuration};
use crate::vm::{VMS, VM, Active, Goods};
use crate::scan::Scan;
use std::process::exit;
use std::io::{stdin, stdout, Write};
use std::cell::RefCell;


#[derive(Debug)]
pub struct Operation {
    pub profile: RefCell<Profile>,
    pub active: Action,
    pub vms: VMS,
}

#[derive(Debug)]
pub enum Action {
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


impl Operation {
    pub fn new() -> Self {
        let mut profile = RefCell::new(Profile::new());
        let mut config = profile.borrow_mut().read_config();
        let vms = match config {
            Some(conf) => conf.vms,
            None => {
                let conf = Operation::init();
                profile.borrow_mut().write_config(conf.clone());
                conf.vms
            },
        };
        let active = Self::get_act();
        Self {
            profile,
            active,
            vms,
        }
    }   

    fn get_act() -> Action {
        use std::env;
        use Action::*;
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
            "scan" => Scan,
            _ => {
                println!("Operate \"{}\" invalid", arg);
                exit(1)
            }
        }
    }

    fn init() -> Configuration{
        let mut line = String::new();
        stdout()
            .write("Please enter the VMs directory(use , split directory):  ".as_bytes())
            .unwrap();
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        let paths = line.trim_end().split(",").into_iter().collect::<Vec<&str>>();
        let vms = Scan::get_vms(&paths);
        let paths = paths.into_iter().map(|v| v.to_string()).collect::<Vec<String>>();
        Configuration {
            paths,
            vms
        }
    }
}

pub trait Operate {
    fn execute(&self);
    fn run(&self, name: &str) -> &VM;
}


impl Operate for Operation {
    fn execute(&self) {
        match &self.active {
            Action::Start(name) => self.run(name).start(),
            Action::Stop(name) => self.run(name).stop(),
            Action::Shutdown(name) => self.run(name).shutdown(),
            Action::Scan => {
                let mut conf = self.profile.borrow_mut().read_config().unwrap();
                conf.vms = Scan::get_vms(&conf.paths.iter().map(|s| s as &str).collect());
                self.profile.borrow_mut().write_config(conf);
            },
            Action::List => println!("{}",self.vms.list()),
            _ => todo!(),
        }
    }
    
    fn run(&self, name: &str) -> &VM{
        let vm = self.vms.get(name);
        vm
    }
}