#![allow(clippy::large_enum_variant)]

use crate::{models::DhcpEvent, EnforcerConfig};

/// This is the interface for communication between enforcer & controller
///
/// All types implementing `serde::Serializable` can be transferred,
/// however function calls can only be made by the enforcer.
#[tarpc::service]
pub trait NamibRpc {
    async fn heartbeat(version: Option<String>) -> Option<EnforcerConfig>;
    async fn dhcp_request(event: DhcpEvent);
    async fn send_logs(logs: Vec<String>);
}
