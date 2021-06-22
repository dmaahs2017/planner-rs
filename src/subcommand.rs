use crate::{constants::*, opts::*, *};

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
            println!("{}", path.to_string_lossy())
        }
    }
}
