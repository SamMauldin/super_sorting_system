use pathfinding::directed::bfs::bfs;
use serde::Serialize;
use thiserror::Error;

use crate::{
    state::{sign_config::CompiledSignConfig, State},
    types::{Dimension, Location, Vec2, Vec3},
};

#[derive(Error, Debug, Serialize)]
pub enum PathfindingError {
    #[error("No path available")]
    NoPath,
    #[error("Unknown starting location")]
    UnknownStartingLocation,
    #[error("Cross-dimension paths currently unsupported")]
    CrossDimensionUnsupported,
}

// Is the given point inside a complex? If so, which one?
fn is_in_complex(loc: Location, sign_config: &CompiledSignConfig) -> Option<String> {
    for (name, complex) in sign_config.complexes.iter() {
        if loc.dim != complex.dimension {
            continue;
        }

        if loc.vec3.y != complex.y_level + 1 {
            continue;
        }

        let (b1, b2) = complex.bounds;

        if Vec2::from(loc.vec3).contained_by(b1, b2, 1) {
            return Some(name.clone());
        }
    }

    None
}

// Is a inbetween b and c?
fn is_in_between(a: Vec3, b: Vec3, c: Vec3) -> bool {
    let target_dist = b.dist(c);

    let actual_dist = a.dist(b) + a.dist(c);

    actual_dist - 5_f64 <= target_dist
}

fn find_aligned_node(start_loc: Location, sign_config: &CompiledSignConfig) -> Option<String> {
    if let Some(complex) = is_in_complex(start_loc, &sign_config) {
        return Some(complex);
    }

    // Node within 3 blocks
    let nearby_node = sign_config
        .nodes
        .iter()
        .filter(|(_name, node)| node.location.dim == start_loc.dim)
        .find(|(_name, node)| node.location.vec3.dist(start_loc.vec3) < 3_f64);

    if let Some((name, _node)) = nearby_node {
        return Some(name.to_owned());
    }

    // In between two nodes, first selected
    let in_between_node_res = sign_config
        .nodes
        .iter()
        .find_map(|(_name, node)| {
            node.connections.iter().find(|other_name| {
                let other_node = sign_config.nodes.get(*other_name).unwrap();

                is_in_between(start_loc.vec3, node.location.vec3, other_node.location.vec3)
            })
        })
        .map(|node| node.to_owned());

    in_between_node_res
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum PfNode {
    Portal { source_node: String },
    Normal { node: String },
}

#[derive(Serialize)]
pub enum PfResultNode {
    Vec(Vec3),
    Portal {
        vec: Vec3,
        destination_dim: Dimension,
    },
}

pub fn find_path(
    start_loc: Location,
    end_loc: Location,
    state: &State,
) -> Result<Vec<PfResultNode>, PathfindingError> {
    let sign_config = state.sign_config.get_config();

    let starting_node = find_aligned_node(start_loc, &sign_config)
        .ok_or(PathfindingError::UnknownStartingLocation)?;
    let ending_node = find_aligned_node(end_loc, &sign_config)
        .ok_or(PathfindingError::UnknownStartingLocation)?;

    if starting_node == ending_node {
        return Ok(vec![PfResultNode::Vec(end_loc.vec3)]);
    }

    let path = bfs(
        &PfNode::Normal {
            node: starting_node,
        },
        |node| match &node {
            PfNode::Normal { node } => {
                let config_node = sign_config.nodes.get(node).unwrap();
                let mut connected_nodes: Vec<PfNode> = config_node
                    .connections
                    .iter()
                    .map(|name| PfNode::Normal {
                        node: name.to_owned(),
                    })
                    .collect();

                if config_node.portal.is_some() {
                    connected_nodes.push(PfNode::Portal {
                        source_node: node.clone(),
                    });
                }

                connected_nodes
            }
            PfNode::Portal { source_node } => {
                let config_node = sign_config.nodes.get(source_node).unwrap();

                vec![PfNode::Normal {
                    node: config_node
                        .portal
                        .as_ref()
                        .unwrap()
                        .destination_node_name
                        .clone(),
                }]
            }
        },
        |node| match &node {
            PfNode::Normal { node } => *node == ending_node,
            _ => false,
        },
    );

    path.map(|path| {
        path.iter()
            .map(|node| match node {
                PfNode::Normal { node } => {
                    let config_node = sign_config.nodes.get(node).unwrap();

                    PfResultNode::Vec(config_node.location.vec3)
                }
                PfNode::Portal { source_node } => {
                    let config_node = sign_config.nodes.get(source_node).unwrap();
                    let portal = config_node.portal.as_ref().unwrap();
                    let destination_node = sign_config
                        .nodes
                        .get(&portal.destination_node_name)
                        .unwrap();

                    PfResultNode::Portal {
                        vec: portal.vec3,
                        destination_dim: destination_node.location.dim,
                    }
                }
            })
            .collect::<Vec<PfResultNode>>()
    })
    .map(|mut path| {
        path.push(PfResultNode::Vec(end_loc.vec3));

        path
    })
    .ok_or(PathfindingError::NoPath)
}
