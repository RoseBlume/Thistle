mod cores;
mod package;
mod git;
use git::clone;
use cores::count;
use package::install;
mod aur;
use aur::makepkg;
use std::env;
mod meson;
use meson::{mesonbuild, mesoncompile, mesoninstall};
mod cmake;
use cmake::{cmakebuild, cmakecompile, cmakeinstall};
mod help;
use help::help;

fn main() {
    // Collect command-line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Iterate over command-line arguments
    for (i, arg) in args.iter().enumerate() {
        println!("Argument {}: {}", i, arg);
    }
    // Access specific arguments by index
    //let first_arg = &args[1];
    //let second_arg = &args[2];
    //let third_arg = &args[3];
    //println!("First argument: {}", first_arg);
    //println!("Second argument: {}", second_arg);
    //println!("Third argument: {}", third_arg);
    if args[1] == "git" {
        clone(&args[2], &args[3], &args[4]);
    }
    if args[1] == "pacman" {
        install(&args[2])
    }
    if args[1] == "aur" {
        makepkg(&args[2]);
    }
    if args[1] == "help" {
        help();
    }
}
