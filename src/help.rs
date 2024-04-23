pub fn help() {
    println!("Usage: \"Thistle [subcommand]\"\nSubcommands:\n\taur \"name\" - This installs an application from the aur\n\tgit [buildsystem] \"url\" \"name\"- Installs from source using the buildsystem provided by the user\n\tpacman \"name\" - Installs an application using pacman")
}
