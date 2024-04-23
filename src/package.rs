use std::process::Command;
pub fn install(package: &String) {
    let output = Command::new("sudo")
                         .arg("pacman")
                         .arg("-S")
                         .arg(package)
                         .output()
                         .expect("failed to execute process");
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
}
