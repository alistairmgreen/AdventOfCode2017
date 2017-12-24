pub mod errors;
pub use errors::ParseComponentError;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Component {
    port1: usize,
    port2: usize,
}

impl FromStr for Component {
    type Err = ParseComponentError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('/')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<usize>, _>>()?;

        if parts.len() == 2 {
            Ok(Component::new(parts[0], parts[1]))
        } else {
            Err(ParseComponentError::missing_field())
        }
    }
}

impl Component {
    pub fn new(a: usize, b: usize) -> Component {
        Component { port1: a, port2: b }
    }

    pub fn compatible_with(&self, connector: usize) -> bool {
        self.port1 == connector || self.port2 == connector
    }

    pub fn strength(&self) -> usize {
        self.port1 + self.port2
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Bridge {
    components: Vec<Component>,
}

impl Bridge {
    pub fn new() -> Bridge {
        Bridge {
            components: Vec::new(),
        }
    }

    pub fn end_connector(&self) -> usize {
        let component_count = self.components.len();

        if component_count == 0 {
            0
        } else if component_count == 1 {
            if self.components[0].port1 == 0 {
                self.components[0].port2
            } else {
                self.components[0].port1
            }
        } else {
            let last = &self.components[component_count - 1];
            let previous = &self.components[component_count - 2];
            if previous.compatible_with(last.port1) {
                last.port2
            } else {
                last.port1
            }
        }
    }

    pub fn append(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn strength(&self) -> usize {
        self.components
            .iter()
            .map(|component| component.strength())
            .sum()
    }
}

pub fn build(existing_bridge: &Bridge, components: &[Component]) -> Bridge {
    let required_connector = existing_bridge.end_connector();
    let mut bridge = existing_bridge.clone();
    let mut highest_strength = existing_bridge.strength();

    for (index, _) in components
        .iter()
        .enumerate()
        .filter(|&(_, c)| c.compatible_with(required_connector))
    {
        let mut candidate_bridge = existing_bridge.clone();
        let mut remaining_components = components.to_owned();
        candidate_bridge.append(remaining_components.remove(index));

        if !remaining_components.is_empty() {
            candidate_bridge = build(&candidate_bridge, &remaining_components);
        }

        let candidate_strength = candidate_bridge.strength();
        if candidate_strength > highest_strength {
            highest_strength = candidate_strength;
            bridge = candidate_bridge;
        }
    }

    bridge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_has_correct_end_connector() {
        let mut bridge = Bridge::new();
        assert_eq!(bridge.end_connector(), 0);

        bridge.append(Component::new(0, 3));
        assert_eq!(bridge.end_connector(), 3);

        bridge.append(Component::new(3, 7));
        assert_eq!(bridge.end_connector(), 7);
    }

    #[test]
    fn build_finds_strongest_bridge() {
        let components = vec![
            Component::new(0, 2),
            Component::new(2, 2),
            Component::new(2, 3),
            Component::new(3, 4),
            Component::new(3, 5),
            Component::new(0, 1),
            Component::new(10, 1),
            Component::new(9, 10),
        ];

        let strongest_bridge = build(&Bridge::new(), &components);

        assert_eq!(
            strongest_bridge.components,
            vec![
                Component::new(0, 1),
                Component::new(10, 1),
                Component::new(9, 10),
            ]
        );

        assert_eq!(strongest_bridge.strength(), 31);
    }
}
