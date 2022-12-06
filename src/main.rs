use vmstart::VMdir;
const DEFAULT_DIR: &str = "d:\\VMware";

fn main() {
    let vmdir = VMdir::new(&DEFAULT_DIR);
    let vm = vmdir.get_vms();
    for (_,instanse)in vm.iter(){
        instanse.start();
    }
}
