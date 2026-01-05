use clap::{Arg, ArgAction, Command};
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command as SysCommand;

fn main() {
    let matches = Command::new("Hephaestus")
        .version("3.2.0")
        .author("Hephaestus Team <gilles.infosec@gmail.com>")
        .about("Secure, cross-platform git helper CLI")
        .subcommand(
            Command::new("update")
                .about("git pull safely for a single repo or all child repos")
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .value_name("PATH")
                        .help("Path to a repo or a directory containing multiple repos")
                        .required(false)
                        .default_value("."),
                )
                .arg(
                    Arg::new("all")
                        .long("all")
                        .help("If set, iterate all first-level subdirectories and update each git repo")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("parallel")
                        .long("parallel")
                        .help("Execute updates in parallel (use with --all)")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("clone")
                .about("Clone repositories from a links file or HTML file")
                .arg(
                    Arg::new("links")
                        .short('l')
                        .long("links")
                        .value_name("FILE")
                        .help("Path to file containing repo URLs (one per line)")
                        .conflicts_with("input"),
                )
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("FILE")
                        .help("Input HTML file to parse for repo URLs")
                        .conflicts_with("links"),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("Optional output file to write extracted links (use with --input)")
                        .requires("input"),
                )
                .arg(
                    Arg::new("dest")
                        .short('d')
                        .long("dest")
                        .value_name("DIR")
                        .help("Destination directory for clones")
                        .required(false)
                        .default_value("."),
                )
                .arg(
                    Arg::new("parallel")
                        .long("parallel")
                        .help("Execute clones in parallel")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("init")
                .about("Initialize a new git repository with a README")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .value_name("NAME")
                        .help("Name of the new repository directory")
                        .required(true),
                )
                .arg(
                    Arg::new("readme")
                        .long("readme")
                        .value_name("TEXT")
                        .help("README.md initial content")
                        .default_value("# New project"),
                ),
        )
        .subcommand(Command::new("status").about("Show git status for current repo"))
        .subcommand(
            Command::new("rollback")
                .about("Reset current repo to a previous ref (destructive)")
                .arg(
                    Arg::new("to")
                        .short('t')
                        .long("to")
                        .value_name("REF")
                        .help("Ref to reset to (default HEAD~1)"),
                )
                .arg(
                    Arg::new("yes")
                        .short('y')
                        .long("yes")
                        .help("Skip confirmation prompt")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("push")
                .about("Stage all changes, commit with a message, and push to the current branch's upstream")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .value_name("MSG")
                        .help("Commit message")
                        .required(true),
                )
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .value_name("PATH")
                        .help("Path to the repository")
                        .default_value("."),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("update", sub_m)) => {
            let path = sub_m.get_one::<String>("path").unwrap();
            let all = sub_m.get_flag("all");
            let parallel = sub_m.get_flag("parallel");
            if all {
                update_all(Path::new(path), parallel);
            } else {
                git_update(Path::new(path));
            }
        }
        Some(("clone", sub_m)) => {
            let dest = sub_m.get_one::<String>("dest").unwrap();
            let parallel = sub_m.get_flag("parallel");

            // Handle HTML parsing or direct links file
            if let Some(input) = sub_m.get_one::<String>("input") {
                // Parse HTML file
                let output = sub_m.get_one::<String>("output");
                clone_from_html(input, output.map(|s| s.as_str()), Path::new(dest), parallel);
            } else if let Some(links) = sub_m.get_one::<String>("links") {
                // Clone from links file
                clone_from_links(Path::new(links), Path::new(dest), parallel);
            } else {
                eprintln!("Error: Either --links or --input must be provided");
            }
        }
        Some(("init", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let readme = sub_m.get_one::<String>("readme").unwrap();
            git_init(name, readme);
        }
        Some(("status", _)) => git_status("."),
        Some(("rollback", sub_m)) => {
            let to = sub_m
                .get_one::<String>("to")
                .map(|s| s.as_str())
                .unwrap_or("HEAD~1");
            let yes = sub_m.get_flag("yes");
            git_rollback(to, yes).ok();
        }
        Some(("push", sub_m)) => {
            let msg = sub_m.get_one::<String>("message").unwrap();
            let path = sub_m.get_one::<String>("path").unwrap();
            commit_and_push(Path::new(path), msg);
        }
        _ => {
            println!("Use --help to see available subcommands");
        }
    }
}

/// Run a git command, returning (stdout, stderr, success)
fn run_git(args: &[&str]) -> (String, String, bool) {
    let output = SysCommand::new("git").args(args).output();
    match output {
        Ok(o) => (
            String::from_utf8_lossy(&o.stdout).to_string(),
            String::from_utf8_lossy(&o.stderr).to_string(),
            o.status.success(),
        ),
        Err(e) => (String::new(), e.to_string(), false),
    }
}

/// Updates a single repository safely (fetch + ff-only merge)
fn git_update(path: &Path) {
    println!("Updating repo at {}", path.display());
    if !path.join(".git").exists() {
        eprintln!("Skipping: not a git repo");
        return;
    }
    let (_o, e1, ok1) = run_git(&[
        "-C",
        path.to_str().unwrap(),
        "fetch",
        "--all",
        "--tags",
        "--prune",
    ]);
    if !ok1 {
        eprintln!("fetch failed: {}", e1);
        return;
    }
    let (branch, _e, okb) = run_git(&[
        "-C",
        path.to_str().unwrap(),
        "rev-parse",
        "--abbrev-ref",
        "@",
    ]);
    if !okb {
        eprintln!("cannot detect current branch");
        return;
    }
    let branch = branch.trim();
    let upstream = format!("{}@{{u}}", branch);
    let (_o2, e2, ok2) = run_git(&[
        "-C",
        path.to_str().unwrap(),
        "merge",
        "--ff-only",
        &upstream,
    ]);
    if !ok2 {
        eprintln!("fast-forward merge failed (maybe no upstream?): {}", e2);
    } else {
        println!("Updated {}", branch);
    }
}

/// Updates all first-level child directories that are git repos
fn update_all(root: &Path, parallel: bool) {
    if let Ok(entries) = fs::read_dir(root) {
        let paths: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir() && p.join(".git").exists())
            .collect();

        if parallel {
            paths.par_iter().for_each(|p| {
                git_update(p);
            });
        } else {
            for p in paths {
                git_update(&p);
            }
        }
    }
}

/// Initializes a new Git repository with a README and initial commit
fn git_init(name: &str, readme: &str) {
    let dir = PathBuf::from(name);
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Failed to create {}: {}", dir.display(), e);
        return;
    }
    let (o, e, ok) = run_git(&["init", dir.to_str().unwrap()]);
    if !ok {
        eprintln!("git init failed: {} {}", o, e);
        return;
    }
    let readme_path = dir.join("README.md");
    if let Err(e) = fs::write(&readme_path, format!("{}\n", readme)) {
        eprintln!("Failed to write README: {}", e);
        return;
    }
    let repo = dir.to_str().unwrap();
    run_git(&["-C", repo, "add", "."]);
    let (_o2, e2, ok2) = run_git(&["-C", repo, "commit", "-m", "Initial commit"]);
    if !ok2 {
        eprintln!("commit failed: {}", e2);
        return;
    }
    println!("Initialized repo at {}", repo);
}

fn git_status(path: &str) {
    let (o, e, ok) = run_git(&["-C", path, "status"]);
    if ok {
        println!("{}", o);
    } else {
        eprintln!("{}", e);
    }
}

/// Destructive reset with confirmation
fn git_rollback(to: &str, yes: bool) -> io::Result<()> {
    if !yes {
        print!(
            "This will run 'git reset --hard {}' and discard changes. Continue? [y/N]: ",
            to
        );
        io::stdout().flush()?;
        let mut ans = String::new();
        io::stdin().read_line(&mut ans)?;
        if ans.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return Ok(());
        }
    }
    let (_o, e, ok) = run_git(&["reset", "--hard", to]);
    if ok {
        println!("Reset to {}", to);
    } else {
        eprintln!("reset failed: {}", e);
    }
    Ok(())
}

/// Stage all changes, commit, and push to current branch's upstream
fn commit_and_push(path: &Path, message: &str) {
    if !path.join(".git").exists() {
        eprintln!("{} is not a git repo", path.display());
        return;
    }
    let repo = path.to_str().unwrap();
    let (_o1, e1, ok1) = run_git(&["-C", repo, "add", "-A"]);
    if !ok1 {
        eprintln!("git add failed: {}", e1);
        return;
    }
    let (_o2, e2, ok2) = run_git(&["-C", repo, "commit", "-m", message]);
    if !ok2 {
        eprintln!("git commit failed (maybe no changes?): {}", e2);
        // continue to push anyway, it's harmless if nothing to push
    }
    let (branch, _e3, ok3) = run_git(&["-C", repo, "rev-parse", "--abbrev-ref", "@"]);
    if !ok3 {
        eprintln!("cannot determine current branch");
        return;
    }
    let branch = branch.trim();
    // Check if upstream is set
    let (_o4, _e4, ok4) = run_git(&[
        "-C",
        repo,
        "rev-parse",
        "--abbrev-ref",
        "--symbolic-full-name",
        "@{u}",
    ]);
    let push_args: Vec<&str> = if ok4 {
        vec!["-C", repo, "push"]
    } else {
        vec!["-C", repo, "push", "-u", "origin", branch]
    };
    let (_o5, e5, ok5) = run_git(&push_args);
    if !ok5 {
        eprintln!("git push failed: {}", e5);
    } else {
        println!("Pushed {}", branch);
    }
}

/// Clone repositories from an HTML file (parse then clone)
fn clone_from_html(input: &str, output: Option<&str>, dest: &Path, parallel: bool) {
    let contents = match fs::read_to_string(input) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {}", input, e);
            return;
        }
    };
    let links = extract_repo_links(&contents);

    if links.is_empty() {
        println!("No repository links found in HTML file");
        return;
    }

    // Optionally write links to output file
    if let Some(out) = output {
        if let Err(e) = fs::write(out, links.join("\n") + "\n") {
            eprintln!("Failed to write {}: {}", out, e);
            return;
        }
        println!("Written {} links to {}", links.len(), out);
    }

    println!("Found {} repositories to clone", links.len());
    clone_repos(&links, dest, parallel);
}

/// Clone all repos listed in a file to a destination directory
fn clone_from_links(links_file: &Path, dest: &Path, parallel: bool) {
    let content = match fs::read_to_string(links_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {}", links_file.display(), e);
            return;
        }
    };

    let urls = parse_links_from_content(&content);
    clone_repos(&urls, dest, parallel);
}

/// Helper function to parse and validate URLs from content
fn parse_links_from_content(content: &str) -> Vec<String> {
    let re = Regex::new(r#"^(https://|git@)[^\s]+$"#).unwrap();
    content
        .lines()
        .map(|line| line.trim())
        .filter(|url| !url.is_empty() && !url.starts_with('#'))
        .filter(|url| {
            if re.is_match(url) {
                true
            } else {
                eprintln!("Skipping invalid URL: {}", url);
                false
            }
        })
        .map(|s| s.to_string())
        .collect()
}

/// Helper function to clone repositories sequentially or in parallel
fn clone_repos(urls: &[String], dest: &Path, parallel: bool) {
    if parallel {
        urls.par_iter().for_each(|url| {
            clone_single_repo(url, dest);
        });
    } else {
        for url in urls {
            clone_single_repo(url, dest);
        }
    }
}

/// Clone a single repository
fn clone_single_repo(url: &str, dest: &Path) {
    let name = repo_name_from_url(url);
    let target = dest.join(&name);
    if target.exists() {
        println!("Skipping existing: {}", target.display());
        return;
    }
    println!("Cloning {} -> {}", url, target.display());
    let (_o, e, ok) = run_git(&["clone", url, target.to_str().unwrap()]);
    if !ok {
        eprintln!("clone failed: {}", e);
    }
}

fn repo_name_from_url(url: &str) -> String {
    let trimmed = url.trim_end_matches(".git");
    Path::new(trimmed)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("repo")
        .to_string()
}

/// Extract repository links from HTML/text, filter to common git forges, sort+dedup.
fn extract_repo_links(contents: &str) -> Vec<String> {
    // A simple, pragmatic URL regex for http(s) URLs; avoids backtracking bombs.
    let re = Regex::new(r#"https?://[A-Za-z0-9._~:/?#\[\]@!$&'()*+,;=%-]+"#).unwrap();
    let mut links: Vec<String> = re
        .find_iter(contents)
        .map(|m| {
            m.as_str()
                .trim_matches(|c| c == '"' || c == '\'')
                .to_string()
        })
        .filter(|u| {
            // Only include common git hosting providers by default
            u.contains("github.com") || u.contains("gitlab.com") || u.contains("bitbucket.org")
        })
        .collect();
    links.sort();
    links.dedup();
    links
}

#[cfg(test)]
mod tests {
    use super::extract_repo_links;

    #[test]
    fn extracts_and_dedups_links() {
        let html = r#"
            <a href="https://github.com/user/repo">repo</a>
            <a href='https://gitlab.com/group/project'>project</a>
            <a href="https://bitbucket.org/team/repo">bb</a>
            <a href="https://github.com/user/repo">duplicate</a>
            <a href="https://example.com/notgit">ignore</a>
        "#;
        let mut links = extract_repo_links(html);
        links.sort();
        assert_eq!(
            links,
            vec![
                "https://bitbucket.org/team/repo".to_string(),
                "https://github.com/user/repo".to_string(),
                "https://gitlab.com/group/project".to_string()
            ]
        );
    }
}
