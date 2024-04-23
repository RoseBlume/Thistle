use std::env::set_current_dir;
use std::process::Command;
pub fn tap(name: String) {
    let output = Command::new("debtap")
                         .arg("-p")
                         .arg(format!("{}.deb", name))
                         .output()
                         .expect("failed to execute process debtap()");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
pub fn install(name: String) {
    let output = Command::new("sudo")
                         .arg("pacman")
                         .arg("-U")
                         .arg("--noconfirm")
                         .arg(format!("{}-1.pkg.tar.zst", name))
                         .output()
                         .expect("failed to execute process install()");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}