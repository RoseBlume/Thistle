use git::clone;
use package::install;
use aur::makepkg;
use help::help;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = &args[1];
    match mode.as_str(){
        "git" => clone(&args[2], &args[3], &args[4]),
        "pac" => install(&args[2]),
        "aur" => makepkg(&args[2]),
        "help" => help(),
        _=>println!("Error: {mode} is not a valid subcommand"),
    }
}