use std::process::Command;
extern crate ini;
use ini::Ini;

fn main() {
    // 读取配置
    let conf = Ini::load_from_file("config.ini").unwrap();

    let section = conf.section(Some("config")).unwrap();
    let tommy = section.get("command").unwrap();

    println!("{:?}", tommy);
    let output = Command::new("bash")
        .arg("-c")
        .arg(tommy)
        .output()
        .expect("failed to execute process");
    let out = String::from_utf8(output.stdout).unwrap();

    // test
    println!("{}", out);
}
