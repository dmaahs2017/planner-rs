use std::fs;

use calendarize;
use chrono::{offset::Local, Datelike};
use comfy_table::Table;

use crate::{constants::*, opts::*, *};

pub fn set(config: &Config, set_opts: &SetOpts) {
    let mut planner = Planner::load(&config.dir, &config.name);
    let event = planner
        .add_event(&set_opts.event, &set_opts.date)
        .expect("Event failed");

    if set_opts.verbose {
        println!(
            "Added to planner: {} [id: {}, date: {}]",
            event.name, event.id, event.date
        );
    } else {
        println!("Added to planner: {} [{}]", event.name, event.date);
    }
    planner.events.sort_by_key(|e| e.date);
    planner.save(&config.dir, &config.name);
}

pub fn delete(config: &Config, delete_opts: &DeleteOpts) {
    let target = if let Some(target) = &delete_opts.target {
        target
    } else {
        &config.name
    };

    let target_path = config.dir.join(&target).with_extension(PLANNER_EXT);

    match std::fs::remove_file(target_path) {
        Ok(_) => println!("Deleted planner: {}", target),
        Err(_) => {
            eprintln!("Planner: {}, does not exist", target);
            std::process::exit(1)
        }
    }
}

pub fn show(config: &Config) {
    println!("Planner directory: {}", config.dir.to_string_lossy());
}

pub fn list(config: &Config) {
    for entry in fs::read_dir(&config.dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "pln" {
            // unwrap safe since extension was already checked
            println!(
                "{}",
                path.with_extension("")
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
            )
        }
    }
}

pub fn view(config: &Config, opts: &ViewOpts) {
    let planner = Planner::load(&config.dir, &config.name);
    let today = Local::today().naive_local();

    if opts.list {
        let mut past_due = planner.events.iter().filter(|e| e.date < today).peekable();
        let mut upcoming = planner.events.iter().filter(|e| e.date >= today).peekable();
        if past_due.peek().is_some() {
            println!("Past Due:");
            for e in past_due {
                println!("\t{} - {}", e.name, e.date.format("%A, %-d %B, %C%y"));
            }
            println!();
        }
        if upcoming.peek().is_some() {
            println!("Upcoming Dates:");
            for e in upcoming {
                println!("\t{} - {}", e.name, e.date.format("%A, %-d %B, %C%y"));
            }
            println!();
        }
        return;
    }

    let events = planner
        .events
        .group_by(|a, b| a.date.month() == b.date.month());

    for events in events {
        let days = calendarize::calendarize(events[0].date);
        let mut table = Table::new();
        table.set_header(&[
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
        ]);

        let events_of_the_month = days.iter().map(|week| {
            week.iter()
                .map(|&day| {
                    if day == 0 {
                        String::new()
                    } else {
                        let event_string = events
                            .iter()
                            .filter(|e| e.date.day() == day)
                            .map(|e| e.name.clone())
                            .reduce(|mut a, b| {
                                a.push('\n');
                                a.push_str(&b);
                                a
                            })
                            .unwrap_or_default();
                        format!("{}.\n{}", day, event_string)
                    }
                })
                .collect::<Vec<_>>()
        });

        for week in events_of_the_month {
            table.add_row(week);
        }

        println!("\n{}", events[0].date.format("%B, %C%y"));
        println!("{}", table);
    }
}

pub fn rm(config: &Config, opts: &RmOpts) {
    let mut planner = Planner::load(&config.dir, &config.name);
    let e = planner.events.iter().find(|e| e.id == opts.event_id).expect("Could not find event").clone();
    planner.remove_event_by_id(opts.event_id).expect("Failed to remove event from planner");
    println!("Removed event: {{id: {}, name: {}, date: {}}}", e.id, e.name, e.date);
    planner.save(&config.dir, &config.name);
}
