use std::env::set_current_dir;
use std::process::Command;
use crate::count;
pub fn cmakebuild() {
    let output = Command::new("cmake")
                          .arg("-B")
                          .arg("build")
                          .arg("CMAKE_INSTALL_PREFIX=/usr")
                          .output()
                          .expect("failed to execute process cmake_build()");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
pub fn cmakecompile() {
    let _ = set_current_dir("build");
    let counter = count();
    let _bar = counter.to_string();
    let output = Command::new("make")
                         .arg("-j{bar}")
                         .output()
                         .expect("failed to execute process cmake_compile()");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
pub fn cmakeinstall() {
    let output = Command::new("sudo")
                         .arg("make")
                         .arg("install")
                         .output()
                         .expect("failed to execute process cmake_install()");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("Finished installing")
}
