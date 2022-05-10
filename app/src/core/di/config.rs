use once_cell::sync::OnceCell;

use crate::domain::config::app_config::AppConfig;

use super::get::GetRef;

static INSTANCE: OnceCell<AppConfig> = OnceCell::new();

impl<'a> GetRef<'a> for AppConfig {
    fn get_ref() -> &'a Self {
        INSTANCE.get().unwrap()
    }
}

impl AppConfig {
    pub fn save(self) {
        INSTANCE.set(self).unwrap();
    }
}
