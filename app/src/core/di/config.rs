use once_cell::sync::OnceCell;

use crate::domain::config::app_config::AppConfig;

use super::get::Get;

static INSTANCE: OnceCell<Box<AppConfig>> = OnceCell::new();

impl Get for Box<AppConfig> {
    fn get() -> Self {
        INSTANCE.get().unwrap().to_owned()
    }
}

impl AppConfig {
    pub fn save(self) {
        INSTANCE.set(Box::new(self)).unwrap();
    }
}
