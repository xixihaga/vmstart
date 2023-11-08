use directories::UserDirs;
use serde::{Deserialize, Serialize};
use std::path::{PathBuf, Path};
use std::fs::{self, File};
use std::io::{stdin, stdout, Write, Read};
use std::process::exit;
use crate::vm::VMS;
use crate::scan::Scan;

#[derive(Debug)]
pub struct Profile(pub PathBuf);

impl Profile {
    pub fn new() -> Self{
        Profile(Self::get())
    }

    fn get() -> PathBuf {
        if Path::new("vmstart.conf").exists() {
            PathBuf::from("vmstart.conf")
        } else {
            let mut path = PathBuf::from(UserDirs::new().unwrap().home_dir());
            path.push(".vmstart\\vmstart.conf");
                if !path.exists() {
                    Profile::init();
                } 
            path
        }
        
    }

    fn init() {
        println!("The configuration file does not exist!");
        let mut line = String::new();
        stdout()
            .write("Could you want initialize this program([y|yes]|other):".as_bytes())
            .unwrap();
        stdout().flush().unwrap();
        stdin().read_line(&mut line).unwrap();
        if line.trim() == "y" || line.trim() == "yes" {
            let mut path = PathBuf::from(UserDirs::new().unwrap().home_dir());
            path.push(".vmstart");
            if !path.exists(){
                let _ = fs::create_dir(&path).map(|_|println!("Creating configure directory Success."));
            }
            path.push("vmstart.conf");
            let _ = fs::File::create(&path).map(|_|println!("Creating configure file Success."));
        } else {
            exit(1)
        }
    }
}

pub trait Configurate {
    fn write_config (&mut self, vms: Configuration) -> Result<usize, String>;
    fn read_config(&mut self) -> Option<Configuration>;
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Configuration{
    pub paths: Vec<String>,
    pub vms: VMS
}

impl Configurate for Profile {
    fn write_config (&mut self, vms: Configuration) -> Result<usize, String> {
        let content = serde_json::to_vec(&vms).unwrap();
        let mut cfile = File::create(&self.0).unwrap();
        let size = cfile.write(&content).map_err(|err|println!("{}",err)).unwrap();
        Ok(size)
        
    }

    fn read_config(&mut self) -> Option<Configuration> {
        let mut file = File::open(&self.0).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        if let Ok(configuration) = serde_json::from_str::<Configuration>(&buf){
            Some(configuration)
        } else {
            None
        }
    }

}

impl Configuration {
    pub fn new(paths: Vec<String>, vms: VMS) -> Self {
        Self {
            paths,
            vms
        }
    }
}
