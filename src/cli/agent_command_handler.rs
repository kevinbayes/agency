use clap::ArgMatches;
use log::debug;
use crate::cli::agent::interact::{interact_agent, start_agent, stop_agent};
use crate::cli::agent::sandbox::{create_sandbox, sync_sandbox};

pub (crate) fn sandbox_command_handler(matches: ArgMatches, cli_command: &ArgMatches, sub_command: &ArgMatches) {

    match sub_command.clone().subcommand() {
        Some(("sync", sub_command)) => {
            debug!("enter: agent");

            let source = sub_command.get_one::<String>("source").unwrap();
            let target = sub_command.get_one::<String>("target").unwrap();
            let respect_gitignore = sub_command.get_one::<bool>("gitignore").unwrap();

            println!("Sync from {} on {} and include gitignore patterns {}", source, target, respect_gitignore);

            sync_sandbox(source.as_str(), target.as_str(), *respect_gitignore).unwrap();
        },
        _ => {
            println!("fallback: unknown command");
        }
    }

}