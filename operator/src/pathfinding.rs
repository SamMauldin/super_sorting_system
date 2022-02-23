use pathfinding::directed::bfs::bfs;
use serde::Serialize;
use thiserror::Error;

use crate::{
    config::Config,
    types::{Vec2, Vec3},
};

#[derive(Error, Debug, Serialize)]
pub enum PathfindingError {
    #[error("No path available")]
    NoPath,
    #[error("Unknown starting location")]
    UnknownStartingLocation,
    #[error("Cross-dimension paths currently unsupported")]
    CrossDimensionUnsupported,
    #[error("Connection references non-existant node {0}")]
    NodeMissing(String),
    #[error("The pathfinding config must include a complex node")]
    ComplexMissing,
}

fn is_in_complex(dim: &str, loc: Vec3, config: &Config) -> bool {
    if dim != config.complex.dimension {
        return false;
    }

    if loc.y != config.complex.y_level + 1 {
        return false;
    }

    let (b1, b2) = config.complex.bounds;

    Vec2::from(loc).contained_by(b1, b2)
}

// Is a inbetween b and c?
fn is_in_between(a: Vec3, b: Vec3, c: Vec3) -> bool {
    let target_dist = b.dist(c);

    let actual_dist = a.dist(b) + a.dist(c);

    actual_dist - 5_f64 <= target_dist
}

fn find_aligned_node(start_loc: Vec3, start_dim: &str, config: &Config) -> Option<String> {
    if is_in_complex(start_dim, start_loc, &config) {
        return Some(String::from("complex"));
    }

    // Node within 3 blocks
    let nearby_node = config
        .pathfinding
        .nodes
        .iter()
        .filter(|(_name, node)| node.dimension == start_dim)
        .find(|(_name, node)| node.location.dist(start_loc) < 3_f64);

    if let Some((name, _node)) = nearby_node {
        return Some(name.to_owned());
    }

    // In between two nodes, first selected
    let in_between_node_res = config
        .pathfinding
        .nodes
        .iter()
        .find_map(|(_name, node)| {
            node.connections.iter().find(|other_name| {
                let other_node = config.pathfinding.nodes.get(*other_name).unwrap();

                is_in_between(start_loc, node.location, other_node.location)
            })
        })
        .map(|node| node.to_owned());

    in_between_node_res
}

pub fn find_path(
    start_loc: Vec3,
    start_dim: &str,
    end_loc: Vec3,
    end_dim: &str,
    config: &Config,
) -> Result<Vec<Vec3>, PathfindingError> {
    if start_dim != end_dim {
        return Err(PathfindingError::CrossDimensionUnsupported);
    }

    let starting_node = find_aligned_node(start_loc, start_dim, config)
        .ok_or(PathfindingError::UnknownStartingLocation)?;
    let ending_node = find_aligned_node(end_loc, end_dim, config)
        .ok_or(PathfindingError::UnknownStartingLocation)?;

    if starting_node == ending_node {
        return Ok(vec![end_loc]);
    }

    let path = bfs(
        &&starting_node,
        |node| {
            config
                .pathfinding
                .nodes
                .get(*node)
                .unwrap()
                .connections
                .iter()
        },
        |node| **node == ending_node,
    );

    path.map(|path| {
        path.iter()
            .map(|node| config.pathfinding.nodes.get(*node).unwrap().location)
            .collect::<Vec<Vec3>>()
    })
    .map(|mut path| {
        path.push(end_loc);

        path
    })
    .ok_or(PathfindingError::NoPath)
}

pub fn verify_pathfinding_config(config: &Config) -> Result<(), PathfindingError> {
    if let None = config.pathfinding.nodes.get("complex") {
        return Err(PathfindingError::ComplexMissing);
    }

    for (_node_id, node) in config.pathfinding.nodes.iter() {
        let failed_connection = node
            .connections
            .iter()
            .find(|connection| config.pathfinding.nodes.get(*connection).is_none());

        if let Some(connection) = failed_connection {
            return Err(PathfindingError::NodeMissing(connection.to_owned()));
        }
    }

    Ok(())
}
