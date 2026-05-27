use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[test]
fn update_all_timeout_uses_one_repo_budget() {
    let sandbox = TestSandbox::new("update-all-timeout").unwrap();
    let repo = sandbox.path.join("repo-one");
    fs::create_dir_all(repo.join(".git")).unwrap();
    sandbox.write_fake_git(
        r#"#!/bin/sh
case "$*" in
  *" rev-parse --abbrev-ref @"*)
    echo main
    exit 0
    ;;
  *" fetch "*|*" fetch")
    sleep 5
    exit 0
    ;;
  *)
    exit 0
    ;;
esac
"#,
    );

    let started = Instant::now();
    let output = sandbox
        .hephaestus()
        .args([
            "update",
            "--all",
            "--path",
            sandbox.path.to_str().unwrap(),
            "--timeout",
            "1",
        ])
        .output()
        .unwrap();

    assert!(
        started.elapsed() < Duration::from_secs(4),
        "update should stop promptly after the per-repo timeout"
    );
    assert!(output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("fetch failed"), "stderr was: {stderr}");
    assert!(
        stderr.contains("per-repo timeout budget"),
        "stderr was: {stderr}"
    );
    assert!(stderr.contains("repo-one"), "stderr was: {stderr}");
}

#[test]
fn update_steps_share_one_repo_timeout_budget() {
    let sandbox = TestSandbox::new("update-shared-budget").unwrap();
    let repo = sandbox.path.join("repo-one");
    fs::create_dir_all(repo.join(".git")).unwrap();
    sandbox.write_fake_git(
        r#"#!/bin/sh
case "$*" in
  *" fetch "*|*" fetch")
    sleep 1
    exit 0
    ;;
  *" rev-parse --abbrev-ref @"*)
    sleep 5
    echo main
    exit 0
    ;;
  *)
    exit 0
    ;;
esac
"#,
    );

    let started = Instant::now();
    let output = sandbox
        .hephaestus()
        .args([
            "update",
            "--all",
            "--path",
            sandbox.path.to_str().unwrap(),
            "--timeout",
            "2",
        ])
        .output()
        .unwrap();

    assert!(
        started.elapsed() < Duration::from_secs(4),
        "branch detection should inherit only the remaining per-repo budget"
    );
    assert!(output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cannot detect current branch"),
        "stderr was: {stderr}"
    );
    assert!(stderr.contains("branch detection"), "stderr was: {stderr}");
    assert!(
        stderr.contains("per-repo timeout budget"),
        "stderr was: {stderr}"
    );
}

#[cfg(unix)]
#[test]
fn update_timeout_kills_spawned_descendants() {
    let sandbox = TestSandbox::new("update-kills-descendants").unwrap();
    let repo = sandbox.path.join("repo-one");
    let child_pid_file = sandbox.path.join("child.pid");
    fs::create_dir_all(repo.join(".git")).unwrap();
    sandbox.write_fake_git(&format!(
        r#"#!/bin/sh
case "$*" in
  *" fetch "*|*" fetch")
    sh -c 'echo $$ > "{}"; sleep 30' &
    sleep 30
    exit 0
    ;;
  *" rev-parse --abbrev-ref @"*)
    echo main
    exit 0
    ;;
  *)
    exit 0
    ;;
esac
"#,
        child_pid_file.display()
    ));

    let output = sandbox
        .hephaestus()
        .args([
            "update",
            "--all",
            "--path",
            sandbox.path.to_str().unwrap(),
            "--timeout",
            "1",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    std::thread::sleep(Duration::from_millis(250));

    let child_pid: libc::pid_t = fs::read_to_string(&child_pid_file)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    assert!(
        !process_exists(child_pid),
        "descendant process {child_pid} survived git timeout cleanup"
    );
}

#[cfg(unix)]
fn process_exists(pid: libc::pid_t) -> bool {
    unsafe { libc::kill(pid, 0) == 0 }
}

struct TestSandbox {
    path: PathBuf,
    bin_dir: PathBuf,
}

impl TestSandbox {
    fn new(name: &str) -> io::Result<Self> {
        let mut path = env::temp_dir();
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("hephaestus-{name}-{}-{unique}", std::process::id()));
        let bin_dir = path.join("bin");
        fs::create_dir_all(&bin_dir)?;
        Ok(Self { path, bin_dir })
    }

    fn write_fake_git(&self, script: &str) {
        let git_path = self.bin_dir.join("git");
        fs::write(&git_path, script).unwrap();
        make_executable(&git_path).unwrap();
    }

    fn hephaestus(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_hephaestus"));
        let existing_path = env::var_os("PATH").unwrap_or_default();
        let mut path_entries = vec![self.bin_dir.clone()];
        path_entries.extend(env::split_paths(&existing_path));
        command.env("PATH", env::join_paths(path_entries).unwrap());
        command
    }
}

impl Drop for TestSandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(unix)]
fn make_executable(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> io::Result<()> {
    Ok(())
}
