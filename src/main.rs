mod operation;
mod scan;
mod vm;
mod profile;

use crate::operation::{Operation, Operate};
use vm::Active;


fn main() {
    let opt = Operation::new();
    opt.execute()
}
