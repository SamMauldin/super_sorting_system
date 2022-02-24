use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::{
    config::Config,
    state::{
        operations::{OperationKind, OperationPriority, OperationStatus},
        State,
    },
    types::Location,
};

use super::service::Service;

struct TrackedNode {
    name: String,
    location: Location,
    current_scan_operation_id: Option<Uuid>,
    last_scan: Option<DateTime<Utc>>,
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
                current_scan_operation_id: None,
                last_scan: None,
            })
        }

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
            };

            let op = state.operations.queue_operation(priority, kind);

            node.current_scan_operation_id = Some(op.id);
        }
    }
}
