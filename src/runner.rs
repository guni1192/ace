use std::env;
use std::process;
use std::process::exit;

// use nix::sched::{unshare, CloneFlags};
use nix::unistd::getuid;

use clap::ArgMatches;

// use super::bootstrap::pacstrap;
use super::container;
// use super::mounts;
use super::network::{Bridge, Network};
use super::options;

// TODO: deamonize option
pub fn run(sub_m: &ArgMatches) {
    let ace_container_path_env = "ACE_CONTAINER_PATH";
    let ace_path = "/var/lib/cromwell/containers";
    env::set_var(ace_container_path_env, ace_path);

    let command = match sub_m.value_of("exec_command") {
        Some(c) => c.to_string(),
        None => "/bin/sh".to_string(),
    };

    let container_name = sub_m
        .value_of("container_name")
        .expect("invalied arguments about container name");

    let pid = process::id();
    println!("pid: {}", pid);

    let uid = getuid();

    let container = container::Container::new(container_name.to_string(), command, uid);

    if sub_m.is_present("del") {
        container.delete().expect("Faild to remove container: ");
    }

    // TODO: pull rootfs docker image by DockerHub
    // bootstraping
    // if !Path::new(&container.path).exists() {
    //     println!(
    //         "Creating container bootstrap to {} ...",
    //         container.path.as_str()
    //     );
    //     fs::create_dir_all(container.path.as_str())
    //         .expect("Could not create directory to your path");
    //     pacstrap(container.path_str());
    // }

    container.prepare();

    container.run();
}

#[allow(dead_code)]
pub fn network(args: &[String]) {
    let args = args.to_vec();
    let matches = options::get_network_options(args).expect("Invalid arguments");

    let container_name = matches
        .opt_str("name")
        .expect("invalied arguments about container name");

    let network = Network::new(
        format!("{}-ns", &container_name),
        Bridge::new(),
        format!("{}_host", &container_name),
        format!("{}_guest", &container_name),
        "172.0.0.2".parse().unwrap(),
    );

    if matches.opt_present("create-bridge") {
        network
            .bridge
            .add_bridge_ace0()
            .expect("Could not create bridge");
        exit(0);
    }

    if matches.opt_present("delete-bridge") {
        network
            .bridge
            .del_bridge_ace0()
            .expect("Could not delete bridge");
        exit(0);
    }

    if matches.opt_present("clean") {
        network.clean().expect("Failed clean up network");
        exit(0);
    }
}
