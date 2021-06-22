use std::path::PathBuf;

use app_dirs::{AppDataType, AppInfo};
use clap::{AppSettings, Clap};

use planner::*;

const APP_INFO: AppInfo = AppInfo {
    name: "planner-rs",
    author: "Dalton J. M. <maahs2017@gmail.com>",
};

#[derive(Clap)]
#[clap(version = "0.1", author = "Dalton J. M. <maahs2017@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "A CLI planner. Fun :)")]
struct Command {
    #[clap(long, about = "Specify the directory to your planners")]
    planner_directory: Option<String>,

    #[clap(long, default_value = "default_planner", about = "Specify the name of the planner you want to query")]
    planner_name: String,

    #[clap(short, long, about = "Show the directory where your planners is stored")]
    show_paths: bool,
    #[clap(long, about = "WARNING: This will permanently delete the planner")]
    delete_planner: bool,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    Set(SetCommand),
}

#[derive(Clap)]
#[clap(about = "Set a new event in your planner.\nUse `planner set --help` for more information")]
struct SetCommand {
    #[clap(short, long)]
    verbose: bool,
    event: String,
    date: String,
}

fn main() {
    let opts = Command::parse();

    let dir = opts
        .planner_directory
        .map(|pd| PathBuf::from(pd))
        .unwrap_or(app_dirs::app_root(AppDataType::UserData, &APP_INFO).unwrap());

    if opts.show_paths {
        println!("Planner directory: {}", dir.to_string_lossy());
        return;
    }

    let mut planner = Planner::load(&dir, &opts.planner_name);

    match opts.subcmd {
        Some(subcmd) => match subcmd {
            SubCommand::Set(set_opts) => {
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
            }
        },
        None => {}
    }

    planner.save(&dir, &opts.planner_name);

    if opts.delete_planner {
        std::fs::remove_file(dir.join(&opts.planner_name).with_extension(PLANNER_EXT))
            .expect("Failed to delete planner");
        println!("Deleted planner: {}", opts.planner_name);
        return;
    }
}
