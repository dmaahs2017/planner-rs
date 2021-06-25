use clap::Clap;

use planner::{opts::*, *};

fn main() {
    let config: Config = Opts::parse().into();
    if let Some(subcmd) = &config.subcmd {
        match subcmd {
            SubCommand::Set(set_opts) => subcommand::set(&config, set_opts),
            SubCommand::Delete(delete_opts) => subcommand::delete(&config, delete_opts),
            SubCommand::Paths => subcommand::show(&config),
            SubCommand::List => subcommand::list(&config),
            SubCommand::View(view_opts) => subcommand::view(&config, view_opts),
            SubCommand::Rm(opts) => subcommand::rm(&config, opts),
        }
    }
}
