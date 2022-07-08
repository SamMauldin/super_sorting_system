use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::{
    config::Config,
    state::{
        operations::{OperationKind, OperationPriority, OperationStatus},
        State,
    },
    types::{Vec3, Location},
};

use super::service::Service;

struct TrackedNode {
    name: String,
    location: Location,
    portal_vec: Option<Vec3>,
    current_scan_operation_id: Option<Uuid>,
    last_scan: Option<DateTime<Utc>>,
    current_portal_scan_operation_id: Option<Uuid>,
    last_portal_scan: Option<DateTime<Utc>>,
}

pub struct NodeScannerService {
    tracked_nodes: Vec<TrackedNode>,
}

impl Service for NodeScannerService {
    fn new(_config: &Config) -> Self {
        NodeScannerService {
            tracked_nodes: Default::default(),
        }
    }

    fn tick(&mut self, state: &mut State) {
        let sign_config = state.sign_config.get_config();

        for (_name, node) in sign_config.nodes.iter() {
            if self
                .tracked_nodes
                .iter()
                .any(|e_node| e_node.name == node.name)
            {
                continue;
            }

            self.tracked_nodes.push(TrackedNode {
                name: node.name.clone(),
                location: node.location,
                portal_vec: node.portal.as_ref().map(|p| p.vec3),
                current_scan_operation_id: None,
                last_scan: None,
                current_portal_scan_operation_id: None,
                last_portal_scan: None
            })
        }

        // Scan node
        for node in self.tracked_nodes.iter_mut() {
            if let Some(op_id) = node.current_scan_operation_id {
                let op = state.operations.get(op_id);

                match op {
                    Some(op) => match op.status {
                        OperationStatus::Complete => {
                            node.current_scan_operation_id = None;
                            node.last_scan = Some(Utc::now());
                        }
                        OperationStatus::Aborted => node.current_scan_operation_id = None,
                        OperationStatus::InProgress | OperationStatus::Pending => continue,
                    },
                    None => node.current_scan_operation_id = None,
                }
            }

            let needs_rescan = match node.last_scan {
                Some(last_scan) => last_scan + Duration::hours(2) < Utc::now(),
                None => true,
            };

            if !needs_rescan {
                continue;
            }

            let priority = match node.last_scan {
                Some(_) => OperationPriority::Background,
                None => OperationPriority::UserInteractive,
            };

            let kind = OperationKind::ScanSigns {
                location: node.location,
                take_portal: None,
            };

            let op = state.operations.queue_operation(priority, kind);

            node.current_scan_operation_id = Some(op.id);
        }

        // Scan node portal
        for node in self.tracked_nodes.iter_mut() {
            if node.portal_vec.is_none() { continue }

            if let Some(op_id) = node.current_portal_scan_operation_id {
                let op = state.operations.get(op_id);

                match op {
                    Some(op) => match op.status {
                        OperationStatus::Complete => {
                            node.current_portal_scan_operation_id = None;
                            node.last_portal_scan = Some(Utc::now());
                        }
                        OperationStatus::Aborted => node.current_portal_scan_operation_id = None,
                        OperationStatus::InProgress | OperationStatus::Pending => continue,
                    },
                    None => node.current_portal_scan_operation_id = None,
                }
            }

            let needs_rescan = match node.last_portal_scan {
                Some(last_scan) => last_scan + Duration::hours(5) < Utc::now(),
                None => true,
            };

            if !needs_rescan {
                continue;
            }

            let priority = match node.last_portal_scan {
                Some(_) => OperationPriority::Background,
                None => OperationPriority::UserInteractive,
            };

            let kind = OperationKind::ScanSigns {
                location: node.location,
                take_portal: node.portal_vec,
            };

            let op = state.operations.queue_operation(priority, kind);

            node.current_scan_operation_id = Some(op.id);
        }
    }
}
