use crate::{
    data::config::dtos::app_config_dto::AppConfigDto, domain::config::app_config::AppConfig,
};

impl From<AppConfigDto> for AppConfig {
    fn from(dto: AppConfigDto) -> Self {
        Self {
            near_network_id: dto.near_network_id.unwrap_or_else(|| "testnet".to_owned()),
            near_node_url: dto
                .near_node_url
                .unwrap_or_else(|| "https://rpc.testnet.near.org".to_owned()),
            near_wallet_url: dto
                .near_wallet_url
                .unwrap_or_else(|| "https://wallet.testnet.near.org".to_owned()),
            near_helper_url: dto
                .near_helper_url
                .unwrap_or_else(|| "https://helper.testnet.near.org".to_owned()),
            near_explorer_url: dto
                .near_explorer_url
                .unwrap_or_else(|| "https://explorer.testnet.near.org".to_owned()),
            bundlr_node_url: dto
                .bundlr_node_url
                .unwrap_or_else(|| "https://devnet.bundlr.network".to_owned()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_model_to_dto() {
        let dto = AppConfigDto {
            bundlr_node_url: Some("bundlr_node_url".to_owned()),
            near_explorer_url: Some("near_explorer_url".to_owned()),
            near_helper_url: Some("near_helper_url".to_owned()),
            near_network_id: Some("near_network_id".to_owned()),
            near_node_url: Some("near_node_url".to_owned()),
            near_wallet_url: Some("near_wallet_url".to_owned()),
        };

        let result = AppConfig::from(dto);

        assert_eq!(result.bundlr_node_url, "bundlr_node_url".to_owned());
        assert_eq!(result.near_explorer_url, "near_explorer_url".to_owned());
        assert_eq!(result.near_helper_url, "near_helper_url".to_owned());
        assert_eq!(result.near_network_id, "near_network_id".to_owned());
        assert_eq!(result.near_node_url, "near_node_url".to_owned());
        assert_eq!(result.near_wallet_url, "near_wallet_url".to_owned());
    }
}
