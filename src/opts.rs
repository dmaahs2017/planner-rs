use std::convert::From;
use std::path::PathBuf;

use app_dirs::AppDataType;
use clap::{AppSettings, Clap};

use crate::constants::*;

#[derive(Clap)]
#[clap(version = VERSION, author = APP_INFO.author)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "A CLI planner. Fun :)")]
pub struct Opts {
    #[clap(short = 'd', long, about = "Specify the directory to your planners")]
    pub planner_directory: Option<String>,

    #[clap(
        short = 'n',
        long,
        default_value = "default_planner",
        about = "Specify the name of the planner you want to query"
    )]
    pub planner_name: String,

    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

pub struct Config {
    pub dir: PathBuf,
    pub name: String,
    pub subcmd: Option<SubCommand>,
}

impl From<Opts> for Config {
    fn from(opts: Opts) -> Self {
        Self {
            dir: match opts.planner_directory {
                Some(dir) => From::from(dir),
                None => app_dirs::app_root(AppDataType::UserData, &APP_INFO)
                    .expect("Could not find default app data dir"),
            },
            name: opts.planner_name,
            subcmd: opts.subcmd,
        }
    }
}

#[derive(Clap)]
pub enum SubCommand {
    Set(SetOpts),
    Delete(DeleteOpts),
    #[clap(about = "Show the directory where your planners is stored")]
    #[clap(setting = AppSettings::ColoredHelp)]
    Paths,
    #[clap(about = "Show existing planners")]
    #[clap(setting = AppSettings::ColoredHelp)]
    List,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(about = "Set a new event in your planner.\nUse `planner set --help` for more information")]
pub struct SetOpts {
    #[clap(short, long, about = "See full logging information")]
    pub verbose: bool,
    #[clap(about = "Name or summary of an event")]
    pub event: String,
    #[clap(about = "Date associated with the event")]
    pub date: String,
}

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(
    about = "WARNDING: This will permanently delete the current planner\nDelete the planner\nUse `planner delete --help` for more information"
)]
pub struct DeleteOpts {
    #[clap(about = "Planner to delete")]
    pub target: Option<String>,
}
