use crate::{
    data::wallet::dtos::near_config_dto::NearConfigDto, domain::config::app_config::AppConfig,
};

impl From<Box<AppConfig>> for NearConfigDto {
    fn from(config: Box<AppConfig>) -> Self {
        NearConfigDto {
            network_id: config.near_network_id,
            node_url: config.near_node_url,
            explorer_url: config.near_explorer_url,
            helper_url: config.near_helper_url,
            wallet_url: config.near_wallet_url,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_config_to_near_config() {
        let config = Box::new(AppConfig {
            near_network_id: "near_network_id".to_owned(),
            near_node_url: "near_node_url".to_owned(),
            near_wallet_url: "near_wallet_url".to_owned(),
            near_helper_url: "near_helper_url".to_owned(),
            near_explorer_url: "near_explorer_url".to_owned(),
            ..Default::default()
        });

        let result = NearConfigDto::from(config.clone());

        assert_eq!(result.network_id, config.near_network_id);
        assert_eq!(result.node_url, config.near_node_url);
        assert_eq!(result.wallet_url, config.near_wallet_url);
        assert_eq!(result.helper_url, config.near_helper_url);
        assert_eq!(result.explorer_url, config.near_explorer_url);
    }
}
