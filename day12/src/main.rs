extern crate digital_plumber;
extern crate failure;

use digital_plumber::{find_connections_to, Node, NodeId};
use failure::Error;
use std::process::exit;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let mut nodes = read_nodes()?;
    let connected_to_zero = find_connections_to(0, &mut nodes);
    println!(
        "{:3} programs are in the group that contains id    0.",
        connected_to_zero.len()
    );

    let mut groups = 1;

    while !nodes.is_empty() {
        let group_key = *nodes.keys().nth(0).unwrap();
        let group = find_connections_to(group_key, &mut nodes);
        println!("{:3} programs are in the group that contains id {:4}.", group.len(), group_key);
        groups += 1;
    }

    println!("\nThere are {:3} groups in total.", groups);

    Ok(())
}

fn read_nodes() -> Result<HashMap<NodeId, Node>, Error> {
    let input = include_str!("puzzle_input.txt");
    let mut nodes: HashMap<NodeId, Node> = HashMap::with_capacity(2000);

    for line in input.lines() {
        let node = Node::from_str(line)?;
        nodes.insert(node.get_id(), node);
    }

    Ok(nodes)
}
