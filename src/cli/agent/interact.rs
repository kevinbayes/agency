use std::path::{Path, PathBuf};
use std::{env, fs};
use std::io;
use std::process::Command;
use clap::ArgMatches;
use crate::config::config::read_json_config;

pub(crate) fn start_agent(command: &ArgMatches) -> io::Result<()> {

    let config = read_json_config("./.ai/config.json").unwrap();
    let name = config.project.unwrap().name;
    let project_root = Path::new(".ai/");

    fs::create_dir_all("./.ai/.docker-cache/")?;
    fs::create_dir_all("./.ai/desktop-data/")?;
    fs::create_dir_all("./.ai/sandbox/")?;

    Command::new("docker")
        .current_dir(&project_root)
        .args(&["build", "-f", "Dockerfile", "-t", name.as_str(), "."])
        .status()?;

    let path = env::current_dir()?;

    let sandbox_mount = format!("{}/.ai/sandbox:/opt/workspace", path.display());

    println!("{}", sandbox_mount);

    let mut args = vec!["run", "-d",
                    "--name", name.as_str(),
                    "-v", sandbox_mount.as_str(),
                    "-v", "apt-cache:/var/cache/apt",
                    "-v", "apt-lib:/var/lib/apt",
                    "--rm",
                    "--shm-size=512m"];

    if let Some(ports) = command.get_many::<String>("port") {
        for port in ports {
            args.extend_from_slice(&["-p", port]);
        }
    }

    args.push(name.as_str());

    Command::new("docker")
        .current_dir(&project_root)
        .args(args.as_slice())
        .status()?;

    Ok(())
}

pub(crate) fn interact_agent() -> io::Result<()> {

    let config = read_json_config("./.ai/config.json").unwrap();
    let name = config.project.unwrap().name;
    let project_root = Path::new("./");

    Command::new("docker")
        .current_dir(&project_root)
        .args(&["exec", "-it", name.as_str(), "bash"])
        .status()?;

    Ok(())
}

pub(crate) fn stop_agent() -> io::Result<()> {

    let config = read_json_config("./.ai/config.json").unwrap();
    let name = config.project.unwrap().name;
    let project_root = Path::new(".ai/");

    Command::new("docker")
        .current_dir(&project_root)
        .args(&["stop", name.as_str()])
        .status()?;

    Ok(())
}