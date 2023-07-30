use std::collections::HashMap;

use cgmath::Vector2;
use derive_new::new;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

#[cfg(feature = "debug")]
use crate::debug::*;
use crate::interface::{Position, Size};

#[derive(Serialize, Deserialize, new)]
pub struct WindowState {
    pub position: Position,
    pub size: Size,
}

#[derive(Default, Serialize, Deserialize)]
pub struct WindowCache {
    entries: HashMap<String, WindowState>,
}

const FILENAME: &str = "client/window_cache.ron";

impl WindowCache {
    pub fn new() -> Self {
        Self::load().unwrap_or_else(|| {
            #[cfg(feature = "debug")]
            print_debug!(
                "failed to load window cache from {}{FILENAME}{}. creating empty cache",
                MAGENTA,
                NONE
            );

            Default::default()
        })
    }

    pub fn load() -> Option<Self> {
        #[cfg(feature = "debug")]
        print_debug!("loading window cache from {}{FILENAME}{}", MAGENTA, NONE);

        std::fs::read_to_string(FILENAME)
            .ok()
            .and_then(|data| ron::from_str(&data).ok())
            .map(|entries| Self { entries })
    }

    pub fn save(&self) {
        #[cfg(feature = "debug")]
        print_debug!("saving window cache to {}{FILENAME}{}", MAGENTA, NONE);

        let data = ron::ser::to_string_pretty(&self.entries, PrettyConfig::new()).unwrap();
        std::fs::write(FILENAME, data).expect("unable to write file");
    }

    pub fn register_window(&mut self, identifier: &str, position: Position, size: Size) {
        if let Some(entry) = self.entries.get_mut(identifier) {
            entry.position = position;
            entry.size = size;
        } else {
            let entry = WindowState::new(position, size);
            self.entries.insert(identifier.to_string(), entry);
        }
    }

    pub fn update_position(&mut self, identifier: &str, position: Vector2<f32>) {
        if let Some(entry) = self.entries.get_mut(identifier) {
            entry.position = position;
        }
    }

    pub fn update_size(&mut self, identifier: &str, size: Size) {
        if let Some(entry) = self.entries.get_mut(identifier) {
            entry.size = size;
        }
    }

    pub fn get_window_state(&self, identifier: &str) -> Option<(Position, Size)> {
        self.entries.get(identifier).map(|entry| (entry.position, entry.size))
    }
}

impl Drop for WindowCache {
    fn drop(&mut self) {
        self.save();
    }
}
