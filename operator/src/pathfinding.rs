use pathfinding::directed::bfs::bfs;
use serde::Serialize;
use thiserror::Error;

use crate::{
    state::{
        sign_config::{CompiledSignConfig, StorageComplex},
        State,
    },
    types::{Dimension, Location, Vec2, Vec3},
};

#[derive(Error, Debug, Serialize)]
pub enum PathfindingError {
    #[error("No path available")]
    NoPath,
    #[error("Unknown starting location")]
    UnknownStartingLocation,
}

// Is the given point inside a complex? If so, which one?
fn is_in_complex(loc: Location, sign_config: &CompiledSignConfig) -> Option<String> {
    for (_name, complex) in sign_config.complexes.iter() {
        match complex {
            StorageComplex::Tower {
                dimension,
                name,
                origin,
                height,
            } => {
                if loc.dim != *dimension {
                    continue;
                }

                if loc.vec3.y < origin.y || loc.vec3.y > (origin.y + (*height as i32)) {
                    continue;
                }

                if Vec2::from(*origin) == Vec2::from(loc.vec3) {
                    return Some(name.clone());
                }
            }
            StorageComplex::FlatFloor {
                dimension,
                name,
                y_level,
                bounds,
            } => {
                if loc.dim != *dimension {
                    continue;
                }

                if loc.vec3.y != y_level + 1 {
                    continue;
                }

                let (b1, b2) = bounds;

                if Vec2::from(loc.vec3).contained_by(*b1, *b2, 1) {
                    return Some(name.clone());
                }
            }
        }
    }

    None
}

fn is_exactly_in_between(a: Vec3, b: Vec3, c: Vec3) -> bool {
    let mut equality_count = 0;
    if a.x == b.x && b.x == c.x {
        equality_count += 1;
    }
    if a.y == b.y && b.y == c.y {
        equality_count += 1;
    }
    if a.z == b.z && b.z == c.z {
        equality_count += 1;
    }

    return equality_count >= 2;
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

    // Portal within 3 blocks
    let nearby_portal = sign_config
        .nodes
        .iter()
        .filter(|(_name, node)| node.location.dim == start_loc.dim)
        .find(|(_name, node)| {
            if let Some(portal) = &node.portal {
                portal.vec3.dist(start_loc.vec3) < 3_f64
            } else {
                false
            }
        });

    if let Some((name, _node)) = nearby_portal {
        return Some(name.to_owned());
    }

    // Fallback to nearest node in same dimension
    let mut nodes_in_dim = sign_config
        .nodes
        .iter()
        .filter(|(_name, node)| node.location.dim == start_loc.dim)
        .collect::<Vec<_>>();

    nodes_in_dim.sort_by(|a, b| {
        let a_dist = start_loc.vec3.dist(a.1.location.vec3);
        let b_dist = start_loc.vec3.dist(b.1.location.vec3);
        a_dist.total_cmp(&b_dist)
    });

    nodes_in_dim
        .first()
        .map(|(name, _node)| name.to_owned())
        .cloned()
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
    let starting_config_node = sign_config.nodes.get(&starting_node).unwrap();
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
                let mut connected_nodes: Vec<PfNode> = sign_config
                    .nodes
                    .iter()
                    .filter(|(name, sign_node)| {
                        if node == *name {
                            return false;
                        }

                        if sign_node.location.dim != config_node.location.dim {
                            return false;
                        }

                        return true;
                    })
                    .map(|(name, _sign_node)| PfNode::Normal {
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
                let portal = config_node.portal.as_ref().unwrap();
                let destination_node = sign_config.nodes.get(&portal.destination_node_name);

                if destination_node.is_some() {
                    vec![PfNode::Normal {
                        node: config_node
                            .portal
                            .as_ref()
                            .unwrap()
                            .destination_node_name
                            .clone(),
                    }]
                } else {
                    vec![]
                }
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
        path.insert(0, PfResultNode::Vec(starting_config_node.location.vec3));
        path.push(PfResultNode::Vec(end_loc.vec3));

        path
    })
    .map(|mut path| {
        let mut keep = vec![];
        for (i, node) in path.iter().enumerate() {
            let prev_node = path.get(i - 1);
            let next_node = path.get(i + 1);

            if let PfResultNode::Vec(curr_vec) = node {
                if let Some(PfResultNode::Vec(prev_vec)) = prev_node {
                    if let Some(PfResultNode::Vec(next_vec)) = next_node {
                        keep.push(!is_exactly_in_between(*prev_vec, *curr_vec, *next_vec))
                    } else {
                        keep.push(true)
                    }
                } else {
                    keep.push(true)
                }
            } else {
                keep.push(true)
            }
        }

        let mut iter = keep.iter();
        path.retain(|_| *iter.next().unwrap());
        path
    })
    .ok_or(PathfindingError::NoPath)
}
