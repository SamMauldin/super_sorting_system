use std::{
    collections::HashMap,
    num::ParseIntError,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::types::{Vec2, Vec3};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sign {
    pub lines: [String; 4],
    pub location: Vec3,
    pub dimension: String,
}

// Sign Syntax

// Line 1: Marker & Offset for Location (to enable hiding signs)
// SSS[ (+/-)int,(+/-)int,(+/-)int]
// Examples: SSS +0,+2,+0; SSS -0,-2,-0; SSS
// Location offset is an absolute change in X, Y, and Z, not dependent on sign orientation

// Line 2: Sign Type
// Examples: path node, path connection

// Lines 3 and 4 are sign type specific

/*
 * Pathfinding Node Signs
 * These signs indicate a node that agents can travel to and from
 * Pathfinding connections are neccesary for these
 */

// Line 2: Sign type "path node"
// Line 3: Pathfinding Node Name
// Line 4: Unused

// Example:
// SSS +0,+2,+0
// path node
// Hallway A
// (4th line empty)

/*
 * Pathfinding Connection Signs
 * These signs indiciate that two pathfinding nodes have a clear path in between one another
 */

// Line 2: Sign type "path connection"
// Line 3: First node to connect
// Line 4: Second node to connect

// Example:
// SSS
// path connection
// Hallway A
// Hallway B

/*
 * Storage Complex Signs
 * These signs indicate an area of storage containers to be used by the network
 * This will also act as a pathfinding node of the given name
 * It is assumed that any pathfinding connections have a clear path to and from
 * anywhere one y-level above the containers
 */

// Line 2: Sign type "storage complex"
// Line 3: Offset similar to line 1 of complex end. This offset is specified from the effective location of the sign
// Line 4: Name of complex

#[derive(Debug)]
pub enum ParsedSign {
    PathfindingNode {
        effective_location: (Vec3, String),
        name: String,
    },
    PathfindingConnection {
        node_a_name: String,
        node_b_name: String,
    },
    StorageComplex {
        dimension: String,
        y_level: i32,
        bounds: (Vec2, Vec2),
        name: String,
    },
}

#[derive(Error, Debug, Serialize)]
pub enum SignParseError {
    #[error("Sign marker not found, this is probably not a SSS config sign")]
    NoMarker,
    #[error("Failed to parse offset")]
    OffsetParseFailed,
    #[error("Sign type unknown")]
    UnknownSignType,
    #[error("Name must not be empty")]
    NameEmpty,
}

#[derive(Error, Debug, Serialize)]
pub enum SignConfigValidationError {
    #[error("Duplicate pathfinding node with name {name}")]
    DuplicatePathfindingNode { name: String },
    #[error("Referenced node {name} is unknown")]
    UnknownNode { name: String },
    #[error("Connection between node {name_a} and {name_b} is invalid because they are in different dimensions")]
    InterdimentionalConnection { name_a: String, name_b: String },
}

fn parse_offset(offset: &str) -> Result<Vec3, SignParseError> {
    offset
        .split(",")
        .map(|coord| coord.parse())
        .collect::<Result<Vec<i32>, ParseIntError>>()
        .map_err(|_| SignParseError::OffsetParseFailed)
        .and_then(|coords| {
            if coords.len() != 3 {
                return Err(SignParseError::OffsetParseFailed);
            }

            Ok(Vec3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            })
        })
}

impl TryFrom<&Sign> for ParsedSign {
    type Error = SignParseError;

    fn try_from(s: &Sign) -> Result<Self, Self::Error> {
        if !s.lines[0].starts_with("SSS") {
            return Err(SignParseError::NoMarker);
        }

        let loc_offset = if let Some(offset) = s.lines[0].strip_prefix("SSS ") {
            parse_offset(offset)?
        } else {
            Vec3 { x: 0, y: 0, z: 0 }
        };

        let effective_location = (loc_offset + s.location, s.dimension.clone());

        match s.lines[1].as_str() {
            "path node" => {
                let name = s.lines[2].clone();

                if name.len() == 0 {
                    return Err(SignParseError::NameEmpty);
                }

                Ok(ParsedSign::PathfindingNode {
                    effective_location,
                    name,
                })
            }
            "path connection" => {
                let name_a = s.lines[2].clone();
                let name_b = s.lines[3].clone();

                if name_a.len() == 0 || name_b.len() == 0 {
                    return Err(SignParseError::NameEmpty);
                }

                Ok(ParsedSign::PathfindingConnection {
                    node_a_name: name_a,
                    node_b_name: name_b,
                })
            }
            "storage complex" => {
                let name = s.lines[3].clone();
                let second_offset = parse_offset(s.lines[2].as_str())?;

                Ok(ParsedSign::StorageComplex {
                    name,
                    dimension: effective_location.1,
                    y_level: effective_location.0.y,
                    bounds: (
                        effective_location.0.into(),
                        (effective_location.0 + second_offset).into(),
                    ),
                })
            }
            _ => Err(SignParseError::UnknownSignType),
        }
    }
}

pub struct PathfindingNode {
    location: (Vec3, String),
    name: String,
    connections: Vec<String>,
}

pub struct StorageComplex {
    dimension: String,
    y_level: i32,
    bounds: (Vec2, Vec2),
    name: String,
}

pub struct CompiledSignConfig {
    nodes: HashMap<String, PathfindingNode>,
    complexes: HashMap<String, StorageComplex>,

    sign_parse_errors: Vec<SignParseError>,
    validation_errors: Vec<SignConfigValidationError>,
}

pub struct SignConfigState {
    signs: Vec<Sign>,
    cached_config: Mutex<Option<Arc<CompiledSignConfig>>>,
}

impl Default for SignConfigState {
    fn default() -> Self {
        SignConfigState {
            signs: Default::default(),
            cached_config: Default::default(),
        }
    }
}

impl SignConfigState {
    pub fn clear_area(&mut self, dimension: &str, start: Vec2, end: Vec2) {
        self.signs.drain_filter(|sign| {
            sign.dimension == dimension && Vec2::from(sign.location).contained_by(start, end)
        });

        self.set_dirty();
    }

    pub fn add_sign(&mut self, sign: Sign) {
        self.signs.push(sign);

        self.set_dirty();
    }

    fn generate_config(&self) -> CompiledSignConfig {
        let (parsed_signs, parse_errors): (Vec<_>, Vec<_>) = self
            .signs
            .iter()
            .map(|sign| sign.try_into())
            .partition(Result::is_ok);

        let parsed_signs: Vec<ParsedSign> = parsed_signs.into_iter().map(Result::unwrap).collect();
        let sign_parse_errors: Vec<SignParseError> =
            parse_errors.into_iter().map(Result::unwrap_err).collect();

        let mut validation_errors = Vec::new();

        let mut nodes = HashMap::new();

        // Add all nodes to map first
        parsed_signs.iter().for_each(|sign| {
            match sign {
                ParsedSign::PathfindingNode {
                    effective_location,
                    name,
                } => {
                    let existing_node = nodes.insert(
                        name.clone(),
                        PathfindingNode {
                            name: name.clone(),
                            location: effective_location.clone(),
                            connections: Vec::new(),
                        },
                    );

                    if existing_node.is_some() {
                        validation_errors.push(
                            SignConfigValidationError::DuplicatePathfindingNode {
                                name: name.clone(),
                            },
                        )
                    }
                }
                ParsedSign::StorageComplex {
                    y_level,
                    bounds,
                    dimension,
                    name,
                } => {
                    let existing_node = nodes.insert(
                        name.clone(),
                        PathfindingNode {
                            name: name.clone(),
                            location: (
                                Vec3 {
                                    x: bounds.0.x,
                                    z: bounds.0.z,
                                    y: y_level + 1,
                                },
                                dimension.clone(),
                            ),
                            connections: Vec::new(),
                        },
                    );

                    if existing_node.is_some() {
                        validation_errors.push(
                            SignConfigValidationError::DuplicatePathfindingNode {
                                name: name.clone(),
                            },
                        )
                    }
                }
                _ => {}
            };
        });

        // Add all connections
        parsed_signs.iter().for_each(|sign| {
            if let ParsedSign::PathfindingConnection {
                node_a_name,
                node_b_name,
            } = sign
            {
                let node_a = nodes.get(node_a_name);
                let node_b = nodes.get(node_b_name);

                if node_a.is_none() {
                    validation_errors.push(SignConfigValidationError::UnknownNode {
                        name: node_a_name.clone(),
                    });
                    return;
                }

                if node_b.is_none() {
                    validation_errors.push(SignConfigValidationError::UnknownNode {
                        name: node_b_name.clone(),
                    });
                    return;
                }

                let node_a = node_a.unwrap();
                let node_b = node_b.unwrap();

                if node_a.location.1 != node_b.location.1 {
                    validation_errors.push(SignConfigValidationError::InterdimentionalConnection {
                        name_a: node_a_name.clone(),
                        name_b: node_b_name.clone(),
                    });
                    return;
                }

                let node_a = nodes.get_mut(node_a_name).unwrap();
                node_a.connections.push(node_b_name.clone());

                let node_b = nodes.get_mut(node_b_name).unwrap();
                node_b.connections.push(node_a_name.clone());
            }
        });

        let mut complexes = HashMap::new();

        parsed_signs.iter().for_each(|sign| {
            if let ParsedSign::StorageComplex {
                dimension,
                y_level,
                bounds,
                name,
            } = sign
            {
                complexes.insert(
                    name.clone(),
                    StorageComplex {
                        name: name.clone(),
                        dimension: dimension.clone(),
                        y_level: *y_level,
                        bounds: *bounds,
                    },
                );
            }
        });

        CompiledSignConfig {
            nodes,
            complexes,
            sign_parse_errors,
            validation_errors,
        }
    }

    fn set_dirty(&self) {
        let mut cached_config = self.cached_config.lock().unwrap();

        cached_config.take();
    }

    pub fn get_config(&self) -> Arc<CompiledSignConfig> {
        let mut cached_config = self.cached_config.lock().unwrap();

        if cached_config.is_none() {
            cached_config.replace(Arc::new(self.generate_config()));
        }

        cached_config.as_ref().unwrap().clone()
    }
}
