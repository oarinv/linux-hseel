use std::process::Command;
extern crate ini;
use ini::Ini;

fn main() {
    // 读取配置
    let conf = Ini::load_from_file("config.ini").unwrap();
    let section = conf.section(Some("config")).unwrap();

    let url = section.get("url").unwrap();
    let local_dir = section.get("local_dir").unwrap();
    let remote_dir = section.get("remote_dir").unwrap();

    let cmd = format!("cd {local_dir} && rsync -av --delete ./ root@{url}:{remote_dir}");

    println!("{:?}", cmd);
    let output = Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to execute process");
    let out = String::from_utf8(output.stdout).unwrap();
    println!("{}", out);
}
