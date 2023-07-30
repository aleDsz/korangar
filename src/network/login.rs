use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

#[cfg(feature = "debug")]
use crate::debug::*;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct LoginSettings {
    pub username: String,
    pub password: String,
    pub remember_username: bool,
    pub remember_password: bool,
}

const FILENAME: &str = "client/login_settings.ron";

impl LoginSettings {
    pub fn new() -> Self {
        Self::load().unwrap_or_else(|| {
            #[cfg(feature = "debug")]
            print_debug!("failed to load login settings from {}{FILENAME}{}", MAGENTA, NONE);

            Default::default()
        })
    }

    pub fn load() -> Option<Self> {
        #[cfg(feature = "debug")]
        print_debug!("loading login settings from {}{FILENAME}{}", MAGENTA, NONE);

        std::fs::read_to_string(FILENAME).ok().and_then(|data| ron::from_str(&data).ok())
    }

    pub fn save(&self) {
        #[cfg(feature = "debug")]
        print_debug!("saving login settings to {}{FILENAME}{}", MAGENTA, NONE);

        let data = ron::ser::to_string_pretty(self, PrettyConfig::new()).unwrap();
        std::fs::write(FILENAME, data).expect("unable to write file");
    }
}

impl Drop for LoginSettings {
    fn drop(&mut self) {
        self.save();
    }
}
