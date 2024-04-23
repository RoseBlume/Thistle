pub fn help() {
    println!("Usage: \"Thistle [subcommand]\"\nSubcommands:\n\tinit -This gets all sources and initializes debtap\n\taur \"name\" - This installs an application from the aur\n\tgit [buildsystem] \"url\" \"name\"- Installs from source using the buildsystem provided by the user\n\tpac \"name\" - Installs an application using pacman")
}
