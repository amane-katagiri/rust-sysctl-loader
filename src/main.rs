use rust_sysctl_loader::schema;
use rust_sysctl_loader::sysctl;
use rust_sysctl_loader::validator::validate;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        panic!(
            "Usage: {} /path/to/sysctl.conf [/path/to/schema.txt]",
            args.get(0).unwrap()
        )
    }
    let sysctl_conf_file = args.get(1).unwrap();
    let schema_file = args.get(2);

    let file_content =
        fs::read_to_string(sysctl_conf_file).expect(&format!("cannot open '{}'", sysctl_conf_file));
    let sysctl_conf = sysctl::parse_str(&file_content).unwrap();

    if let Some(schema_file) = schema_file {
        let file_content =
            fs::read_to_string(schema_file).expect(&format!("cannot open '{}'", schema_file));
        let schema = schema::parse_str(&file_content).unwrap();
        match validate(sysctl_conf, schema) {
            Ok(()) => println!(
                "Validating '{}' with schema '{}': OK",
                sysctl_conf_file, schema_file
            ),
            Err(err) => println!(
                "Validating '{}' with schema '{}': NG\n{}",
                sysctl_conf_file, schema_file, err
            ),
        }
    } else {
        println!("{:?}", sysctl_conf);
    }
}
