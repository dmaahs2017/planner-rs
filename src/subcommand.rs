use crate::{constants::*, opts::*, *};

use chrono::offset::Local;

use std::fs;

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

pub fn view(config: &Config) {
    let planner = Planner::load(&config.dir, &config.name);

    let today = Local::today().naive_local();

    let i =
        planner.events.iter().enumerate().find_map(
            |(i, e)| {
                if e.date >= today {
                    Some(i)
                } else {
                    None
                }
            },
        );

    //TODO: test for all past due
    //TODO: test for all upcoming
    let (past_due, upcoming) = if let Some(i) = i {
        planner.events.split_at(i)
    } else {
        (&planner.events[..], &planner.events[planner.events.len()..])
    };

    if past_due.len() > 0 {
        println!("Past Due:");
        for e in past_due {
            println!("\t{} - {}", e.name, e.date.format("%A, %-d %B, %C%y"));
        }
        println!();
    }
    if upcoming.len() > 0 {
        println!("Upcoming Dates:");
        for e in upcoming {
            println!("\t{} - {}", e.name, e.date.format("%A, %-d %B, %C%y"));
        }
        println!();
    }
}
