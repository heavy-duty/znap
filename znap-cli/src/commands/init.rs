use crate::utils::write_file;
use colored::Colorize;
use heck::ToKebabCase;
use std::fs::create_dir;

pub fn run(name: &String, dry_run: &bool) {
    std::process::Command::new("clear").status().unwrap();
    let cwd = std::env::current_dir().unwrap();
    let workspace_dir = cwd.join(name.to_kebab_case());

    println!("\n");

    let message =  r#"
    ____ 
   |\   \      ________  ________   ________  ________   
   \ \   \    |\_____  \|\   ___  \|\   __  \|\   __  \  
   _\_\   \    \|___/  /\ \  \\ \  \ \  \|\  \ \  \|\  \ 
  |\    ___\       /  / /\ \  \\ \  \ \   __  \ \   ____\
  \ \   \         /  /_/__\ \  \\ \  \ \  \ \  \ \  \___|
   \ \   \       |\________\ \__\\ \__\ \__\ \__\ \__\   
    \ \  /        \|_______|\|__| \|__|\|__|\|__|\|__|   
     \_\/

"#;
    println!("{}", message.bold().yellow());
    println!("");
    println!("Someone is about to get some action...");
    println!("No worries, we got you. {}", "BLINK BLINK".bold().italic());
    println!("");
    println!(
        "You are about to create a {} named: {}\n",
        "Znap workspace".bold(),
        &name.cyan()
    );

    if !dry_run {
        // Create a folder with the provided name.
        create_dir(&workspace_dir).unwrap();

        // Create a Cargo.toml file.
        write_file(
            workspace_dir.join("Cargo.toml").as_path(),
            &String::from("[workspace]\nmembers = [\"collections/*\", \".znap/*\"]"),
        );

        // Create a Znap.toml file.
        write_file(
            workspace_dir.join("Znap.toml").as_path(),
            &String::from("collections = []"),
        );

        // Create a .gitignore file.
        write_file(
            workspace_dir.join(".gitignore").as_path(),
            &String::from("/target\n.znap/.tmp*"),
        );

        // Create a collections folder.
        let collections_dir = &workspace_dir.join("collections");
        create_dir(&collections_dir).unwrap();

        // Create a .gitkeep in the collections folder.
        write_file(
            collections_dir.join(".gitkeep").as_path(),
            &String::from(""),
        );

        // Create a .znap folder.
        let znap_dir = &workspace_dir.join(".znap2");
        create_dir(&znap_dir).unwrap();

        // Create a .gitkeep in the .znap folder.
        write_file(znap_dir.join(".gitkeep").as_path(), &String::from(""));
    }

    println!("  Added:\n");
    println!("      {}", format!("+ {}/Cargo.toml", &name).on_bright_green());
    println!("      {}", format!("+ {}/Znap.toml", &name).on_bright_green());
    println!("      {}", format!("+ {}/.gitignore", &name).on_bright_green());
    println!(
        "      {}",
        format!("+ {}/.znap/.gitkeep", &name).on_bright_green()
    );
    println!(
        "      {}",
        format!("+ {}/collections/.gitkeep", &name).on_bright_green()
    );

    println!(
        "\nZnap workspace created at {}\n",
        format!("file://{}", &workspace_dir.to_str().unwrap())
            .italic()
            .bold()
    );
}
