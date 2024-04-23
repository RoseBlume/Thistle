use std::process::Command;
use std::env::set_current_dir;
pub fn makepkg(name: &str) {
    let _output = Command::new("git")
                         .arg("clone")
                         .arg(format!("https://aur.archlinux.org/{name}.git"))
                         .output()
                         .expect("failed to clone aur repo");
    let _ = set_current_dir(name);
    let output = Command::new("makepkg")
                         .arg("-si")
                         .output()
                         .expect("failed to execute process makepkg");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
