use crate::config_firewall::ConfigFirewall;
use crate::models::DHCPRequestData;

#[tarpc::service]
pub trait RPC {
    async fn heartbeat(version: String) -> Option<Vec<ConfigFirewall>>;
    async fn dhcp_request(message: DHCPRequestData);
}
