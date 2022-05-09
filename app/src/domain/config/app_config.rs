#[derive(PartialEq, Clone, Debug)]
pub struct AppConfig {
    pub near_network_id: String,
    pub near_node_url: String,
    pub near_wallet_url: String,
    pub near_helper_url: String,
    pub near_explorer_url: String,
    pub bundlr_node_url: String,
}
