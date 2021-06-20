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
// planner raw

use assert_cmd::Command;
use predicates::str::*;

#[test]
fn it_works() {
    Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .assert()
        .success()
        .stdout(contains("Hello, world!"));
}

#[test]
fn planner_list() {
    Command::cargo_bin(env!("CARGO_PKG_NAME") + "-d")
        .unwrap()
        .assert()
        .success()
        .stdout(contains(""))
}
