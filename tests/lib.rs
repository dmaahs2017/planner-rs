// planner set <due date/time> <string> [priority] [--repeats weekly/biweekly/monthly/weekdays/weekends/daily]
//
// planner [view] day [--verbose] [--priority <high/normal/low>]
// planner [view] week [--verbose] [--priority <high/normal/low>]
// planner [view] month [--verbose] [--priority <high/normal/low]
//
// planner rm [event_id / series_id]
//
// planner sync
// planner sync --set-upstream <upstream>
//
// planner export [--target <file>]
// planner import <planner data> # careful this command may be destructive

use assert_cmd::Command;
use predicates::str::*;
use tempfile::TempDir;

struct TestEnv {
    dir: TempDir,
    command: Command,
}

impl TestEnv {
    fn new() -> Self {
        Self {
            dir: tempfile::tempdir().unwrap(),
            command: Command::cargo_bin("planner").expect("binary was not found"),
        }
    }

    fn command(&mut self) -> &mut Command {
        self.command = Command::cargo_bin("planner").expect("binary was not found");
        self.command
            .arg("--planner-directory")
            .arg(self.dir.path().to_str().unwrap())
    }

    fn command_with_planner(&mut self, planner: &str) -> &mut Command {
        self.command().arg("--planner-name").arg(planner)
    }
}

#[test]
fn set_first_event() {
    TestEnv::new()
        .command()
        .arg("set")
        .arg("my event")
        .arg("2020-6-21")
        .assert()
        .success()
        .stdout(contains("Added to planner: my event [2020-06-21]"));
}

#[test]
fn set_first_event_verbose() {
    TestEnv::new()
        .command()
        .arg("set")
        .arg("-v")
        .arg("my event")
        .arg("2020-06-21")
        .assert()
        .success()
        .stdout(contains(
            "Added to planner: my event [id: 1, date: 2020-06-21]",
        ));
}

#[test]
fn show_paths() {
    let mut test_env = TestEnv::new();
    test_env
        .command()
        .arg("paths")
        .assert()
        .success()
        .stdout(contains(format!(
            "Planner directory: {}",
            test_env.dir.path().to_str().unwrap()
        )));
}

#[test]
fn delete_planner_that_dne() {
    let mut test_env = TestEnv::new();
    test_env
        .command()
        .arg("delete")
        .assert()
        .failure()
        .stderr(contains(format!(
            "Planner: default_planner, does not exist",
        )));
}

#[test]
fn delete_planner() {
    let mut test_env = TestEnv::new();
    let planner_path = test_env
        .dir
        .path()
        .join("default_planner")
        .with_extension("pln");
    test_env
        .command()
        .args(&["set", "test", "2020-01-01"])
        .assert()
        .success();

    assert!(planner_path.is_file());

    test_env
        .command()
        .arg("delete")
        .assert()
        .success()
        .stdout(contains("Deleted planner: default_planner"));

    assert!(!planner_path.is_file());
}

#[test]
fn delete_specified_planner() {
    let mut test_env = TestEnv::new();
    let planner_path = test_env.dir.path().join("my-planner").with_extension("pln");
    test_env
        .command_with_planner("my-planner")
        .args(&["set", "test", "2020-01-01"])
        .assert()
        .success();

    assert!(planner_path.is_file());

    test_env
        .command()
        .args(&["delete", "my-planner"])
        .assert()
        .success()
        .stdout(contains("Deleted planner: my-planner"));

    assert!(!planner_path.is_file());
}

#[test]
fn list_planners() {
    let mut test_env = TestEnv::new();
    test_env
        .command_with_planner("my-planner")
        .args(&["set", "test", "2020-01-01"])
        .assert()
        .success();

    test_env
        .command_with_planner("my-planner-2")
        .args(&["set", "test", "2020-01-01"])
        .assert()
        .success();

    test_env
        .command()
        .args(&["list"])
        .assert()
        .success()
        .stdout(contains("my-planner"))
        .stdout(contains("my-planner-2"));
}
