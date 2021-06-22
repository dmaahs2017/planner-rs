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
        .arg("--show-paths")
        .assert()
        .success()
        .stdout(contains(format!(
            "Planner directory: {}",
            test_env.dir.path().to_str().unwrap()
        )));
}

// TODO: Delete planner test where planner is already deleted
#[test]
fn delete_planner() {
    let mut test_env = TestEnv::new();
    let planner_path = test_env.dir.path().join("default_planner.pln");
    test_env
        .command()
        //.args(&["set", "test", "2020-01-01"])
        .arg("set")
        .arg("test")
        .arg("2020-01-01")
        .assert()
        .success();

    assert!(planner_path.is_file());

    test_env
        .command()
        .arg("--delete-planner")
        .assert()
        .success()
        .stdout(contains(format!(
            "Deleted planner: {}",
            planner_path
                .with_extension("")
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        )));

    assert!(!planner_path.is_file());
}
