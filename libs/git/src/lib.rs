use std::process::Command;
use std::env::set_current_dir;
//use cmake::{};
use meson::{mesonbuild, mesoncompile, mesoninstall};
use cmake::{cmakebuild, cmakecompile, cmakeinstall};
pub fn clone(buildsystem: &String, url: &String, name: &String) {
    let _ = set_current_dir("~/.local/state");
    let output = Command::new("git")
                         .arg("clone")
                         .arg(url)
                         .arg("package")
                         .output()
                         .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    let _ = set_current_dir(name);
    if buildsystem == "meson" {
        mesonbuild();
        mesoncompile();
        mesoninstall();
    }
    if buildsystem == "cmake" {
        cmakebuild();
        cmakecompile();
        cmakeinstall();
    }
}
