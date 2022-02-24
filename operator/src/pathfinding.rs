use pathfinding::directed::bfs::bfs;
use serde::Serialize;
use thiserror::Error;

use crate::{
    state::{sign_config::CompiledSignConfig, State},
    types::{Location, Vec2, Vec3},
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

        if Vec2::from(loc.vec3).contained_by(b1, b2) {
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

pub fn find_path(
    start_loc: Location,
    end_loc: Location,
    state: &State,
) -> Result<Vec<Vec3>, PathfindingError> {
    if start_loc.dim != end_loc.dim {
        return Err(PathfindingError::CrossDimensionUnsupported);
    }

    let sign_config = state.sign_config.get_config();

    let starting_node = find_aligned_node(start_loc, &sign_config)
        .ok_or(PathfindingError::UnknownStartingLocation)?;
    let ending_node = find_aligned_node(end_loc, &sign_config)
        .ok_or(PathfindingError::UnknownStartingLocation)?;

    if starting_node == ending_node {
        return Ok(vec![end_loc.vec3]);
    }

    let path = bfs(
        &&starting_node,
        |node| sign_config.nodes.get(*node).unwrap().connections.iter(),
        |node| **node == ending_node,
    );

    path.map(|path| {
        path.iter()
            .map(|node| sign_config.nodes.get(*node).unwrap().location.vec3)
            .collect::<Vec<Vec3>>()
    })
    .map(|mut path| {
        path.push(end_loc.vec3);

        path
    })
    .ok_or(PathfindingError::NoPath)
}
