use std::process::Command;
pub fn install(package: &String) {
    let output = Command::new("sudo")
                         .arg("pacman")
                         .arg("-S")
                         .arg("--noconfirm")
                         .arg(package)
                         .output()
                         .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
