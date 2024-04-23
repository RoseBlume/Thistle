use std::env::set_current_dir;
use std::process::Command;
use core::count;
pub fn mesonbuild() {
    let output = Command::new("meson")
                          .arg("setup")
                          .arg("build")
                          .output()
                          .expect("failed to execute process meson_build()");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
pub fn mesoncompile() {
    let _ = set_current_dir("build");
    let counter = count();
    let _bar = counter.to_string();
    let output = Command::new("ninja")
                         .arg("-j{bar}")
                         .output()
                         .expect("failed to execute process meson_compile()");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
pub fn mesoninstall() {
    let output = Command::new("sudo")
                         .arg("ninja")
                         .arg("install")
                         .output()
                         .expect("failed to execute process meson_install()");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("Finished installing")
}
