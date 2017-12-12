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
    let nodes = read_nodes()?;
    let connected_to_zero = find_connections_to(0, nodes);
    println!(
        "{} programs are in the group that contains id 0.",
        connected_to_zero.len()
    );

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
