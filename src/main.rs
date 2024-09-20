use clap::{Arg, Command};
use std::process::Command as SysCommand;
use std::fs;

fn main() {
    let matches = Command::new("Hephaestus")
        .version("0.1.0")
        .author("Your Name <youremail@example.com>")
        .about("A Rust CLI tool for git-related management tasks")
        .subcommand(Command::new("git-update")
            .about("Updates the git repository")
            .arg(Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Path to the git repository")
                .required(false)))
        .subcommand(Command::new("git-parse")
            .about("Parses the git HTML files")
            .arg(Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input HTML file")
                .required(true)))
        .subcommand(Command::new("git-init")
            .about("Initializes a git repository")
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .value_name("NAME")
                .help("Name of the git repository")
                .required(true)))
        .subcommand(Command::new("git-manage")
            .about("Manages git tasks"))
        .subcommand(Command::new("git-rollback")
            .about("Performs a rollback in git"))
        .get_matches();

    match matches.subcommand() {
        Some(("git-update", sub_m)) => {
            let path = sub_m.get_one::<String>("path").map_or(".", String::as_str);
            git_update(path);
        }
        Some(("git-parse", sub_m)) => {
            let input = sub_m.get_one::<String>("input").unwrap();
            git_parse(input);
        }
        Some(("git-init", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            git_init(name);
        }
        Some(("git-manage", _)) => git_manage(),
        Some(("git-rollback", _)) => git_rollback(),
        _ => println!("Invalid subcommand! Use --help for guidance."),
    }
}

/// Updates the git repository by pulling the latest changes
fn git_update(path: &str) {
    println!("Updating repository at: {}", path);
    let output = SysCommand::new("git")
        .args(&["-C", path, "pull"])
        .output()
        .expect("Failed to execute git pull");

    if output.status.success() {
        println!("Repository updated successfully");
    } else {
        eprintln!("Failed to update repository");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Parses HTML file and extracts Git-related data
fn git_parse(input: &str) {
    println!("Parsing HTML file: {}", input);
    
    let contents = fs::read_to_string(input)
        .expect("Failed to read HTML file");

    // This is a mock parser. You can replace it with actual HTML parsing logic.
    let git_data = extract_git_info(&contents);
    println!("Extracted Git-related information:\n{}", git_data);
}

/// A mock function to simulate extracting git-related information from HTML
fn extract_git_info(html: &str) -> String {
    // This simulates finding and returning some git-related data from HTML
    html.lines()
        .filter(|line| line.contains("git"))
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Initializes a new Git repository with the given name
fn git_init(name: &str) {
    println!("Initializing git repository: {}", name);

    let output = SysCommand::new("git")
        .arg("init")
        .arg(name)
        .output()
        .expect("Failed to execute git init");

    if output.status.success() {
        println!("Repository initialized successfully");
    } else {
        eprintln!("Failed to initialize repository");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Manages various git tasks, such as checking status, branches, etc.
fn git_manage() {
    println!("Managing git repository");

    // Example: Running `git status`
    let output = SysCommand::new("git")
        .arg("status")
        .output()
        .expect("Failed to execute git status");

    if output.status.success() {
        println!("Git Status:\n{}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Failed to check git status");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // Add more git management tasks as necessary, such as branch listing, commit history, etc.
}

/// Rolls back the git repository to the previous commit
fn git_rollback() {
    println!("Rolling back repository");

    let output = SysCommand::new("git")
        .args(&["reset", "--hard", "HEAD~1"])
        .output()
        .expect("Failed to execute git rollback");

    if output.status.success() {
        println!("Repository rolled back to the previous commit");
    } else {
        eprintln!("Failed to rollback repository");
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    }
}