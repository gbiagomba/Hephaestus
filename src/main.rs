use clap::{Arg, ArgAction, Command};
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command as SysCommand, Stdio};
use std::time::{Duration, Instant};
use wait_timeout::ChildExt;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

fn main() {
    let matches = Command::new("Hephaestus")
        .version("3.2.2")
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
                )
                .arg(
                    Arg::new("timeout")
                        .short('t')
                        .long("timeout")
                        .value_name("SECONDS")
                        .help("Timeout for git operations in seconds (default: 300)")
                        .default_value("300"),
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
                )
                .arg(
                    Arg::new("timeout")
                        .short('t')
                        .long("timeout")
                        .value_name("SECONDS")
                        .help("Timeout for git clone operations in seconds (default: 600)")
                        .default_value("600"),
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
            let timeout: u64 = sub_m
                .get_one::<String>("timeout")
                .unwrap()
                .parse()
                .unwrap_or(300);
            if all {
                update_all(Path::new(path), parallel, Some(timeout));
            } else {
                git_update(Path::new(path), Some(timeout));
            }
        }
        Some(("clone", sub_m)) => {
            let dest = sub_m.get_one::<String>("dest").unwrap();
            let parallel = sub_m.get_flag("parallel");
            let timeout: u64 = sub_m
                .get_one::<String>("timeout")
                .unwrap()
                .parse()
                .unwrap_or(600);

            // Handle HTML parsing or direct links file
            if let Some(input) = sub_m.get_one::<String>("input") {
                // Parse HTML file
                let output = sub_m.get_one::<String>("output");
                clone_from_html(
                    input,
                    output.map(|s| s.as_str()),
                    Path::new(dest),
                    parallel,
                    Some(timeout),
                );
            } else if let Some(links) = sub_m.get_one::<String>("links") {
                // Clone from links file
                clone_from_links(Path::new(links), Path::new(dest), parallel, Some(timeout));
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

#[derive(Debug)]
struct GitRunResult {
    stdout: String,
    stderr: String,
    success: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BudgetState {
    Unlimited,
    Remaining(Duration),
    Expired,
}

#[derive(Clone, Copy, Debug)]
struct RepoTimeoutBudget {
    total: Option<Duration>,
    deadline: Option<Instant>,
}

impl RepoTimeoutBudget {
    fn new(timeout_secs: Option<u64>) -> Self {
        let total = timeout_secs.map(Duration::from_secs);
        Self {
            total,
            deadline: total.map(|duration| Instant::now() + duration),
        }
    }

    #[cfg(test)]
    fn with_deadline(total: Duration, deadline: Instant) -> Self {
        Self {
            total: Some(total),
            deadline: Some(deadline),
        }
    }

    fn state(&self) -> BudgetState {
        match self.deadline {
            None => BudgetState::Unlimited,
            Some(deadline) => deadline
                .checked_duration_since(Instant::now())
                .filter(|remaining| !remaining.is_zero())
                .map(BudgetState::Remaining)
                .unwrap_or(BudgetState::Expired),
        }
    }
}

/// Run a git command with optional timeout, returning (stdout, stderr, success).
/// Default timeout is None (no timeout). Timeout is in seconds.
fn run_git(args: &[&str], timeout_secs: Option<u64>) -> (String, String, bool) {
    let result = run_git_with_limit(
        args,
        timeout_secs.map(Duration::from_secs),
        None,
        infer_git_operation(args),
        None,
        false,
    );
    (result.stdout, result.stderr, result.success)
}

fn run_git_with_budget(
    args: &[&str],
    budget: &RepoTimeoutBudget,
    repo_path: &Path,
    operation: &str,
) -> GitRunResult {
    match budget.state() {
        BudgetState::Unlimited => {
            run_git_with_limit(args, None, Some(repo_path), operation, None, false)
        }
        BudgetState::Remaining(remaining) => run_git_with_limit(
            args,
            Some(remaining),
            Some(repo_path),
            operation,
            budget.total,
            false,
        ),
        BudgetState::Expired => GitRunResult {
            stdout: String::new(),
            stderr: timeout_message(
                Some(repo_path),
                operation,
                Duration::ZERO,
                budget.total,
                true,
            ),
            success: false,
        },
    }
}

fn run_git_with_limit(
    args: &[&str],
    timeout: Option<Duration>,
    repo_path: Option<&Path>,
    operation: &str,
    repo_budget: Option<Duration>,
    budget_expired: bool,
) -> GitRunResult {
    let mut command = SysCommand::new("git");
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("GIT_TERMINAL_PROMPT", "0");

    #[cfg(unix)]
    unsafe {
        command.pre_exec(|| {
            if libc::setpgid(0, 0) == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        });
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            return GitRunResult {
                stdout: String::new(),
                stderr: format!("Failed to spawn git: {}", e),
                success: false,
            };
        }
    };

    if let Some(timeout) = timeout {
        match child.wait_timeout(timeout) {
            Ok(Some(_status)) => read_git_output(child),
            Ok(None) => {
                kill_git_process_tree(&mut child);
                let _ = child.wait_with_output();
                GitRunResult {
                    stdout: String::new(),
                    stderr: timeout_message(
                        repo_path,
                        operation,
                        timeout,
                        repo_budget,
                        budget_expired,
                    ),
                    success: false,
                }
            }
            Err(e) => GitRunResult {
                stdout: String::new(),
                stderr: format!("Error waiting for git: {}", e),
                success: false,
            },
        }
    } else {
        read_git_output(child)
    }
}

fn read_git_output(child: std::process::Child) -> GitRunResult {
    match child.wait_with_output() {
        Ok(output) => GitRunResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
        },
        Err(e) => GitRunResult {
            stdout: String::new(),
            stderr: format!("Error reading git output: {}", e),
            success: false,
        },
    }
}

fn kill_git_process_tree(child: &mut std::process::Child) {
    #[cfg(unix)]
    unsafe {
        let process_group = -(child.id() as libc::pid_t);
        if libc::kill(process_group, libc::SIGKILL) == 0 {
            return;
        }
    }

    let _ = child.kill();
}

fn timeout_message(
    repo_path: Option<&Path>,
    operation: &str,
    timeout: Duration,
    repo_budget: Option<Duration>,
    budget_expired: bool,
) -> String {
    let repo = repo_path
        .map(|path| format!(" for {}", path.display()))
        .unwrap_or_default();

    if let Some(total) = repo_budget {
        if budget_expired {
            format!(
                "git {}{} timed out: per-repo timeout budget expired after {} seconds",
                operation,
                repo,
                total.as_secs()
            )
        } else {
            format!(
                "git {}{} timed out after {} seconds; per-repo timeout budget is {} seconds",
                operation,
                repo,
                seconds_for_message(timeout),
                total.as_secs()
            )
        }
    } else {
        format!(
            "git {}{} timed out after {} seconds",
            operation,
            repo,
            seconds_for_message(timeout)
        )
    }
}

fn seconds_for_message(duration: Duration) -> u64 {
    duration.as_secs().max(1)
}

fn infer_git_operation<'a>(args: &'a [&'a str]) -> &'a str {
    let mut skip_next = false;
    for arg in args {
        if skip_next {
            skip_next = false;
            continue;
        }
        if *arg == "-C" {
            skip_next = true;
            continue;
        }
        if !arg.starts_with('-') {
            return arg;
        }
    }
    "command"
}

/// Updates a single repository safely (fetch + ff-only merge)
fn git_update(path: &Path, timeout: Option<u64>) {
    println!("Updating repo at {}", path.display());
    if !path.join(".git").exists() {
        eprintln!("Skipping: not a git repo");
        return;
    }
    let budget = RepoTimeoutBudget::new(timeout);
    let fetch = run_git_with_budget(
        &[
            "-C",
            path.to_str().unwrap(),
            "fetch",
            "--all",
            "--tags",
            "--prune",
        ],
        &budget,
        path,
        "fetch",
    );
    if !fetch.success {
        eprintln!("fetch failed: {}", fetch.stderr);
        return;
    }
    let branch_result = run_git_with_budget(
        &[
            "-C",
            path.to_str().unwrap(),
            "rev-parse",
            "--abbrev-ref",
            "@",
        ],
        &budget,
        path,
        "branch detection",
    );
    if !branch_result.success {
        eprintln!("cannot detect current branch: {}", branch_result.stderr);
        return;
    }
    let branch = branch_result.stdout.trim();
    let upstream = format!("{}@{{u}}", branch);
    let merge = run_git_with_budget(
        &[
            "-C",
            path.to_str().unwrap(),
            "merge",
            "--ff-only",
            &upstream,
        ],
        &budget,
        path,
        "fast-forward merge",
    );
    if !merge.success {
        eprintln!(
            "fast-forward merge failed (maybe no upstream?): {}",
            merge.stderr
        );
    } else {
        println!("Updated {}", branch);
    }
}

/// Updates all first-level child directories that are git repos
fn update_all(root: &Path, parallel: bool, timeout: Option<u64>) {
    if let Ok(entries) = fs::read_dir(root) {
        let paths: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir() && p.join(".git").exists())
            .collect();

        if parallel {
            paths.par_iter().for_each(|p| {
                git_update(p, timeout);
            });
        } else {
            for p in paths {
                git_update(&p, timeout);
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
    let (o, e, ok) = run_git(&["init", dir.to_str().unwrap()], None);
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
    run_git(&["-C", repo, "add", "."], None);
    let (_o2, e2, ok2) = run_git(&["-C", repo, "commit", "-m", "Initial commit"], None);
    if !ok2 {
        eprintln!("commit failed: {}", e2);
        return;
    }
    println!("Initialized repo at {}", repo);
}

fn git_status(path: &str) {
    let (o, e, ok) = run_git(&["-C", path, "status"], None);
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
    let (_o, e, ok) = run_git(&["reset", "--hard", to], None);
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
    let (_o1, e1, ok1) = run_git(&["-C", repo, "add", "-A"], None);
    if !ok1 {
        eprintln!("git add failed: {}", e1);
        return;
    }
    let (_o2, e2, ok2) = run_git(&["-C", repo, "commit", "-m", message], None);
    if !ok2 {
        eprintln!("git commit failed (maybe no changes?): {}", e2);
        // continue to push anyway, it's harmless if nothing to push
    }
    let (branch, _e3, ok3) = run_git(&["-C", repo, "rev-parse", "--abbrev-ref", "@"], None);
    if !ok3 {
        eprintln!("cannot determine current branch");
        return;
    }
    let branch = branch.trim();
    // Check if upstream is set
    let (_o4, _e4, ok4) = run_git(
        &[
            "-C",
            repo,
            "rev-parse",
            "--abbrev-ref",
            "--symbolic-full-name",
            "@{u}",
        ],
        None,
    );
    let push_args: Vec<&str> = if ok4 {
        vec!["-C", repo, "push"]
    } else {
        vec!["-C", repo, "push", "-u", "origin", branch]
    };
    let (_o5, e5, ok5) = run_git(&push_args, Some(300)); // 5 minute timeout for push
    if !ok5 {
        eprintln!("git push failed: {}", e5);
    } else {
        println!("Pushed {}", branch);
    }
}

/// Clone repositories from an HTML file (parse then clone)
fn clone_from_html(
    input: &str,
    output: Option<&str>,
    dest: &Path,
    parallel: bool,
    timeout: Option<u64>,
) {
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
    clone_repos(&links, dest, parallel, timeout);
}

/// Clone all repos listed in a file to a destination directory
fn clone_from_links(links_file: &Path, dest: &Path, parallel: bool, timeout: Option<u64>) {
    let content = match fs::read_to_string(links_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {}", links_file.display(), e);
            return;
        }
    };

    let urls = parse_links_from_content(&content);
    clone_repos(&urls, dest, parallel, timeout);
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
fn clone_repos(urls: &[String], dest: &Path, parallel: bool, timeout: Option<u64>) {
    if parallel {
        urls.par_iter().for_each(|url| {
            clone_single_repo(url, dest, timeout);
        });
    } else {
        for url in urls {
            clone_single_repo(url, dest, timeout);
        }
    }
}

/// Clone a single repository
fn clone_single_repo(url: &str, dest: &Path, timeout: Option<u64>) {
    let name = repo_name_from_url(url);
    let target = dest.join(&name);
    if target.exists() {
        println!("Skipping existing: {}", target.display());
        return;
    }
    println!("Cloning {} -> {}", url, target.display());
    let (_o, e, ok) = run_git(&["clone", url, target.to_str().unwrap()], timeout);
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
    use super::{extract_repo_links, BudgetState, RepoTimeoutBudget};
    use std::time::{Duration, Instant};

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

    #[test]
    fn repo_timeout_budget_is_unlimited_without_timeout() {
        let budget = RepoTimeoutBudget::new(None);
        assert_eq!(BudgetState::Unlimited, budget.state());
    }

    #[test]
    fn repo_timeout_budget_reports_remaining_time() {
        let total = Duration::from_secs(5);
        let budget = RepoTimeoutBudget::with_deadline(total, Instant::now() + total);

        match budget.state() {
            BudgetState::Remaining(remaining) => {
                assert!(remaining <= total);
                assert!(remaining > Duration::ZERO);
            }
            state => panic!("expected remaining budget, got {:?}", state),
        }
    }

    #[test]
    fn repo_timeout_budget_expires_once_deadline_passes() {
        let total = Duration::from_secs(5);
        let budget =
            RepoTimeoutBudget::with_deadline(total, Instant::now() - Duration::from_secs(1));

        assert_eq!(BudgetState::Expired, budget.state());
    }
}
