use core:Gateway;
use ethereu_coporce::*;

fn main() {
    let gateway = Gateway::new(ethereum_coprocessor::new());
    gateway.run();
}
