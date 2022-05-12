use serde::Serialize;

#[derive(Serialize, PartialEq, Clone, Debug)]
pub struct NearConfigDto {
    #[serde(rename = "networkId")]
    pub network_id: String,

    #[serde(rename = "nodeUrl")]
    pub node_url: String,

    #[serde(rename = "walletUrl")]
    pub wallet_url: String,

    #[serde(rename = "helperUrl")]
    pub helper_url: String,

    #[serde(rename = "explorerUrl")]
    pub explorer_url: String,
}
