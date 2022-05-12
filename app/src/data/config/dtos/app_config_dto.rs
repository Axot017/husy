use serde::Deserialize;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct AppConfigDto {
    #[serde(rename = "NEAR_NETWORK_ID")]
    pub near_network_id: Option<String>,

    #[serde(rename = "NEAR_NODE_URL")]
    pub near_node_url: Option<String>,

    #[serde(rename = "NEAR_WALLET_URL")]
    pub near_wallet_url: Option<String>,

    #[serde(rename = "NEAR_HELPER_URL")]
    pub near_helper_url: Option<String>,

    #[serde(rename = "NEAR_EXPLORER_URL")]
    pub near_explorer_url: Option<String>,

    #[serde(rename = "BUNDLR_NODE_URL")]
    pub bundlr_node_url: Option<String>,
}
