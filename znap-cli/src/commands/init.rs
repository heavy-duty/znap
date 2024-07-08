use crate::utils::write_file;
use colored::Colorize;
use console::Emoji;
use heck::ToKebabCase;
use std::fs::create_dir;

pub fn run(name: &String, dry_run: &bool) {
    std::process::Command::new("clear").status().unwrap();
    println!("\n");
    let message = r#"
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
    println!(
        "No worries, we got you. {}{}{}",
        Emoji("✨", ""),
        "BLINK BLINK".bold().italic(),
        Emoji("✨", "")
    );
    println!("");
    println!(
        "You are about to create a {} named: {}\n",
        "Znap workspace".bold(),
        &name.cyan()
    );

    let cwd = std::env::current_dir().unwrap();
    let workspace_dir = cwd.join(name.to_kebab_case());
    let collections_dir = &workspace_dir.join("collections");
    let tests_dir = &workspace_dir.join("tests");
    let znap_dir = &workspace_dir.join(".znap");

    if !dry_run {
        // Create a folder with the provided name.
        create_dir(&workspace_dir).unwrap();

        // Create a Cargo.toml file.
        write_file(
            workspace_dir.join("Cargo.toml").as_path(),
            &String::from(
                "[workspace]\nmembers = [\"collections/*\"]\nresolver = \"2\"\n\n[patch.crates-io]\ncurve25519-dalek = { git = \"https://github.com/dalek-cryptography/curve25519-dalek\", rev = \"8274d5cbb6fc3f38cdc742b4798173895cd2a290\" }",
            ),
        );

        // Create a Znap.toml file.

        write_file(
            workspace_dir.join("Znap.toml").as_path(),
            &format!("identity = \"~/.config/solana/id.json\""),
        );

        // Create a default actions.json file
        write_file(
            workspace_dir.join("actions.json").as_path(),
            &String::from("{\"rules\":[{\"pathPattern\":\"/**\",\"apiPath\":\"/api/**\"}]}"),
        );

        // Create a .gitignore file.
        write_file(
            workspace_dir.join(".gitignore").as_path(),
            &String::from("/target\n.znap\nnode_modules"),
        );

        // Create a package.json file.
        write_file(
            workspace_dir.join("package.json").as_path(),
            &String::from(
                r#"
{
    "scripts": {
        "lint:fix": "prettier */*.js \"*/**/*{.js,.ts}\" -w",
        "lint": "prettier */*.js \"*/**/*{.js,.ts}\" --check",
        "test": "ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
    },
    "dependencies": {
        "@solana/web3.js": "^1.93.4"
    },
    "devDependencies": {
        "@types/chai": "^4.3.0",
        "@types/mocha": "^9.0.0",
        "@types/node": "^20.14.9",
        "chai": "^4.3.4",
        "mocha": "^9.0.3",
        "prettier": "^2.6.2",
        "ts-mocha": "^10.0.0",
        "typescript": "^4.3.5"
    }
}
"#,
            ),
        );

        // Create a tsconfig.json file.
        write_file(
            workspace_dir.join("tsconfig.json").as_path(),
            &String::from(
                r#"
{
  "compilerOptions": {
    "types": ["mocha", "chai", "node"],
    "typeRoots": ["./node_modules/@types"],
    "lib": ["es2015"],
    "module": "commonjs",
    "target": "es6",
    "esModuleInterop": true
  }
}
"#,
            ),
        );

        // Create a collections folder.
        create_dir(&collections_dir).unwrap();

        // Create a .gitkeep in the collections folder.
        write_file(
            collections_dir.join(".gitkeep").as_path(),
            &String::from(""),
        );

        // Create a tests folder.
        create_dir(&tests_dir).unwrap();

        // Create a tests/utils.ts file.
        write_file(
            tests_dir.join("utils.ts").as_path(),
            &String::from(
                r#"
export interface Action {
  label: string;
  href: string;
  parameters: {
    label: string;
    name: string;
  }[];
}

export interface Metadata {
  icon: string;
  title: string;
  description: string;
  label: string;
  disabled: boolean;
  error: null;
  links: { actions: Action[] } | null;
}

export function createActionClient(actionUrl: string) {
  return {
    async getMetadata() {
      const url = new URL(actionUrl);
      const response = await fetch(url.toString(), {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
      });
      const responseJson = (await response.json()) as Metadata;

      return responseJson;
    },
    async getTransaction<T extends {}>(account: string, query: T) {
      const url = new URL(actionUrl);

      Object.keys(query).forEach((name) =>
        url.searchParams.set(name, query[name])
      );

      const response = await fetch(url.toString(), {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ account }),
      });
      const responseJson = (await response.json()) as {
        transaction: string;
        message: string;
      };

      return responseJson;
    },
  };
}
"#,
            ),
        );

        // Create a tests/e2e.ts file.
        write_file(
            tests_dir.join("e2e.ts").as_path(),
            &String::from(
                r#"
import { assert } from "chai";

describe("My tests", () => {
  it("should hello world", async () => {
    const hello = "world";

    assert.equal(hello, "world");
  });
});
"#,
            ),
        );

        // Create a .znap folder.
        create_dir(&znap_dir).unwrap();

        // Create a .gitkeep in the .znap folder.
        write_file(znap_dir.join(".gitkeep").as_path(), &String::from(""));
    }

    println!("  Added:\n");
    println!("      {}", format!("+ {}/Cargo.toml", &name).green());
    println!("      {}", format!("+ {}/Znap.toml", &name).green());
    println!("      {}", format!("+ {}/package.json", &name).green());
    println!("      {}", format!("+ {}/tsconfig.json", &name).green());
    println!("      {}", format!("+ {}/actions.json", &name).green());
    println!("      {}", format!("+ {}/.gitignore", &name).green());
    println!("      {}", format!("+ {}/.znap/.gitkeep", &name).green());
    println!(
        "      {}",
        format!("+ {}/collections/.gitkeep", &name).green()
    );
    println!("      {}", format!("+ {}/tests/utils.ts", &name).green());
    println!("      {}", format!("+ {}/tests/e2e.ts", &name).green());

    println!(
        "\nZnap workspace created at {}\n",
        format!("file://{}", &workspace_dir.to_str().unwrap())
            .italic()
            .bold()
    );
}
