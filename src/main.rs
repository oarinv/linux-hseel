use std::process::Command;

fn main() {
    // init vps

    let package = ["ffmpeg", "bash-completion", "curl", "wget"];
    let mut install_text = "".to_string();

    for i in &package {
        install_text = format!("{} {}", install_text, i);
    }

    println!("{}", install_text);
    let _ = run_cmd(install_text);
}

fn run_cmd(install_text: String) {
    let cmd = format!("apt install {} -y", install_text);

    println!("{:?}", cmd);
    let output = Command::new("bash")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        let out = String::from_utf8(output.stdout).unwrap();
        println!("{}", out);
    } else {
        let err = String::from_utf8(output.stderr).unwrap();
        eprintln!("{}", err);
    }
}
// todo .....
