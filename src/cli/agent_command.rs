use clap::{arg, Arg, ArgAction, ArgMatches, Command};
use log::debug;

pub(crate) fn create_agents_command() -> Command {
    Command::new("agents")
        .about("Run agent agent")
        .subcommand_required(true)
        .subcommand(create_init_command())
        .subcommand(create_sandbox_command())
        .subcommand(
            Command::new("start")
                .about("Start agent").arg(arg!([NAME]))
        )
        .subcommand(
            Command::new("interact")
                .about("Interact with agent").arg(arg!([NAME]))
        )
        .subcommand(
            Command::new("stop")
                .about("Stop agent").arg(arg!([NAME]))
        )
}


fn create_init_command() -> Command {
    Command::new("init")
        .about("Create code ai sandbox")
}


fn create_sandbox_command() -> Command {
    Command::new("sandbox")
        .about("Create code ai sandbox")
        .subcommand(
            Command::new("sync")
                .about("Sync your agent sandbox").arg(arg!([NAME]))
                .arg(Arg::new("target").help("target directory").long("target").default_value(".ai/sandbox/"))
                .arg(Arg::new("source").help("source directory").long("source").default_value("./"))
                .arg(Arg::new("gitignore")
                         .long("gitignore")
                         .help("Respect gitignore flag")
                         .action(ArgAction::SetTrue))
        )
}


