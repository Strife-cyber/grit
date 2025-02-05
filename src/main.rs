use std::env;
use std::process;
use crate::systems::add::add;
use crate::systems::commits::commit::Commit;
use crate::systems::commits::log::log;
use crate::systems::init::{init_grit, update_branch};

mod systems;
mod structure;
mod algorithms;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: grit <command> [args]");
        process::exit(1);
    }

    match args[1].as_str() {
        "init" => {
            if let Err(e) = init_grit() {
                eprintln!("Error initializing repository: {}", e);
                process::exit(1);
            }
            println!("Initialized empty Grit repository");
        }
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: grit add <file>");
                process::exit(1);
            }

            let file_arg = if args[2] == "." { None } else { Some(args[2].as_str()) };

            if let Err(e) = add(file_arg) {
                eprintln!("Error adding file: {}", e);
                process::exit(1);
            }
            println!("Added: {}", args[2]);
        }
        "commit" => {
            if args.len() < 4 || args[2] != "-m" {
                eprintln!("Usage: grit commit -m \"message\"");
                process::exit(1);
            }

            let message = &args[3];
            match Commit::new(message, "Author") {
                Ok(Some(commit)) => {
                    println!("Committed: {}", commit.id);
                }
                Ok(None) => {
                    println!("No more changes to commit.");
                }
                Err(e) => {
                    eprintln!("Error committing: {}", e);
                    process::exit(1);
                }
            }
        }
        "branch" => {
            if args.len() < 4 || args[2] != "-M" {
                eprintln!("Usage: grit branch -M \"branch name\"");
                process::exit(1);
            }
            let branch = &args[3];
            update_branch(branch).expect("Failed to update branch");
        }
        "log" => {
            log().expect("An error occurred that's all we know");
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}
