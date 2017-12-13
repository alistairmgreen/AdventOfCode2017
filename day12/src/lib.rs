#[macro_use]
extern crate failure;

use std::collections::HashMap;
use failure::Error;
use std::num::ParseIntError;
use std::str::FromStr;

pub type NodeId = u32;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Node {
    id: NodeId,
    connections: Vec<NodeId>,
}

impl Node {
    pub fn new(id: NodeId, connections: Vec<NodeId>) -> Node {
        Node {
            id: id,
            connections: connections,
        }
    }

    pub fn get_id(&self) -> NodeId {
        self.id
    }
}

impl FromStr for Node {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("<->").collect::<Vec<&str>>();
        if parts.len() != 2 {
            bail!("Cannot parse '{}' as a node.", s);
        }

        let id = parts[0].trim().parse::<NodeId>()?;
        let connections = parts[1]
            .split(',')
            .map(|number| number.trim().parse::<NodeId>())
            .collect::<Result<Vec<NodeId>, ParseIntError>>()?;

        Ok(Node {
            id: id,
            connections: connections,
        })
    }
}

pub fn find_connections_to(
    target: NodeId,
    nodes: &mut HashMap<NodeId, Node>,
) -> HashMap<NodeId, Node> {
    let mut connected: HashMap<NodeId, Node> = HashMap::with_capacity(nodes.len());

    let target_node = nodes
        .remove(&target)
        .expect(&format!("No node exists with the target id {}.", target));

    for id in target_node.connections.iter().filter(|&&connected_id| connected_id != target_node.id) {
        let node = nodes.remove(id).unwrap();
        connected.insert(*id, node);
    }

    connected.insert(target, target_node);

    loop {
        let directly_connected: Vec<NodeId> = nodes
            .values()
            .filter(|&node| {
                node.connections.iter().any(|id| connected.contains_key(id))
            })
            .map(|ref node| node.id)
            .collect();

        if directly_connected.is_empty() {
            break;
        }

        for id in directly_connected.into_iter() {
            let node = nodes.remove(&id).unwrap();
            connected.insert(id, node);
        }
    }

    connected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_connected_to_zero() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::with_capacity(7);
        nodes.insert(0, Node::new(0, vec![2]));
        nodes.insert(1, Node::new(1, vec![1]));
        nodes.insert(2, Node::new(2, vec![0, 3, 4]));
        nodes.insert(3, Node::new(3, vec![2, 4]));
        nodes.insert(4, Node::new(4, vec![2, 3, 6]));
        nodes.insert(5, Node::new(5, vec![6]));
        nodes.insert(6, Node::new(6, vec![4, 5]));

        let connected_to_zero = find_connections_to(0, &mut nodes);
        assert_eq!(connected_to_zero.len(), 6);
        assert!(!connected_to_zero.contains_key(&1));
    }

    #[test]
    fn parse_node_from_puzzle_input() {
        let node = "8 <-> 599, 1068, 1850".parse::<Node>().unwrap();
        assert_eq!(node, Node::new(8, vec![599, 1068, 1850]));
    }
}
