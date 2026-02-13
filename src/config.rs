use cosmic::cosmic_config::{Config, ConfigGet, ConfigSet, CosmicConfigEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VoiceHubConfig {
    pub language: String,
    pub auto_punctuation: bool,
    pub auto_inject: bool,
    pub save_history: bool,
}

impl Default for VoiceHubConfig {
    fn default() -> Self {
        Self {
            language: "pt-BR".to_string(),
            auto_punctuation: true,
            auto_inject: false,
            save_history: true,
        }
    }
}

impl CosmicConfigEntry for VoiceHubConfig {
    const VERSION: u64 = 1;

    fn write_entry(&self, config: &Config) -> Result<(), cosmic::cosmic_config::Error> {
        config.set("language", &self.language)?;
        config.set("auto_punctuation", self.auto_punctuation)?;
        config.set("auto_inject", self.auto_inject)?;
        config.set("save_history", self.save_history)?;
        Ok(())
    }

    fn get_entry(config: &Config) -> Result<Self, (Vec<cosmic::cosmic_config::Error>, Self)> {
        let mut errors = vec![];

        let language = match config.get::<String>("language") {
            Ok(v) => v,
            Err(e) => {
                errors.push(e);
                "pt-BR".to_string()
            }
        };

        let auto_punctuation = match config.get::<bool>("auto_punctuation") {
            Ok(v) => v,
            Err(e) => {
                errors.push(e);
                true
            }
        };

        let auto_inject = match config.get::<bool>("auto_inject") {
            Ok(v) => v,
            Err(e) => {
                errors.push(e);
                false
            }
        };

        let save_history = match config.get::<bool>("save_history") {
            Ok(v) => v,
            Err(e) => {
                errors.push(e);
                true
            }
        };

        let config = Self {
            language,
            auto_punctuation,
            auto_inject,
            save_history,
        };

        if errors.is_empty() {
            Ok(config)
        } else {
            Err((errors, config))
        }
    }

    fn update_keys<T: AsRef<str>>(
        &mut self,
        _config: &Config,
        _keys: &[T],
    ) -> (Vec<cosmic::cosmic_config::Error>, Vec<&'static str>) {
        // Por enquanto, não implementamos atualização parcial
        (vec![], vec![])
    }
}

impl VoiceHubConfig {
    pub fn load() -> Self {
        match Config::new("com.deivisan.voicehub", Self::VERSION) {
            Ok(config) => Self::get_entry(&config).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}
