mod internal;
mod spec;

use crate::internal::common::error_and_exit;
use clap::{Arg, Command};

fn main() {
    let cmd = Command::new("rund-rs")
        .about("Minimal OCI-compliant linux container runtime")
        .arg(
            Arg::new("root")
                .long("root")
                .num_args(1)
                .help("runtime root for container state"),
        )
        .subcommand(
            Command::new("create")
                .about("Create a new container")
                .arg(
                    Arg::new("bundle")
                        .long("bundle")
                        .short('b')
                        .num_args(1)
                        .required(true)
                        .help("bundle directory containing container configuration"),
                )
                .arg(Arg::new("id").required(true).help("ID of the container")),
        )
        .subcommand(
            Command::new("start")
                .about("Start the container process")
                .arg(Arg::new("id").required(true).help("ID of the container")),
        )
        .subcommand(
            Command::new("kill")
                .about("Send a signal to a container process")
                .arg(Arg::new("id").required(true).help("ID of the container"))
                .arg(
                    Arg::new("signal")
                        .required(true)
                        .help("Signal to send to the process (e.g. SIGTERM, SIGKILL, ...)"),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a container")
                .arg(Arg::new("id").required(true).help("ID of the container")),
        )
        .subcommand(
            Command::new("state")
                .about("Retrieve the state of a container")
                .arg(Arg::new("id").required(true).help("ID of the container")),
        )
        .get_matches();

    match cmd.subcommand() {
        Some(("create", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap();
            let bundle = sub_m.get_one::<String>("bundle").unwrap();
            println!("Creating container {} with bundle {}", id, bundle);
        }
        Some(("start", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap();
            println!("Starting container {}", id);
        }
        Some(("kill", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap();
            let signal = sub_m.get_one::<String>("signal").unwrap();
            println!("Sending signal {} to container {}", signal, id);
        }
        Some(("delete", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap();
            println!("Deleting container {}", id);
        }
        Some(("state", sub_m)) => {
            let id = sub_m.get_one::<String>("id").unwrap();
            println!("Fetching state of container {}", id);
        }
        _ => error_and_exit(1, "container command unknown"),
    }
}
