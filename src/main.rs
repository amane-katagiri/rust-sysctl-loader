use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: {} /path/to/sysctl.conf", args.get(0).unwrap())
    }
    let file = args.get(1).unwrap();
    let file_content = fs::read_to_string(file).expect(&format!("cannot open '{}'", file));
    let sysctl_conf = rust_sysctl_loader::sysctl::parse_str(&file_content).unwrap();
    println!("{:?}", sysctl_conf);
}
