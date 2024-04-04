use std::path::PathBuf;

use log::{info, warn};

use backend::WeatherForecast;
use local::location::Coordinates;
use local::settings::Settings;
use weather_dirs::custom_backends_dir;
use weather_structs::WasmPluginInput;

pub fn is_valid_ext(f: &str) -> bool {
    let len = f.len();
    &f[len - 5..] == ".wasm"
}

pub struct WasmPlugin {
    // plugin: Plugin
}

impl WasmPlugin {
    pub fn new(path: &str) -> crate::Result<Self> {
        Self::load(custom_backends_dir()?.join(path))
    }

    pub fn load(path: PathBuf) -> crate::Result<Self> {
        // let file = Wasm::file(path);
        // let mut manifest = Manifest::new([file]);
        // manifest.with_memory_max(1 << 30); // max memory = 1GB TODO: fix
        // Ok(Self {
        //     plugin: Plugin::new(&manifest, [], true)?
        // })
        todo!("WasmPlugin::load")
    }

    pub fn name(&mut self) -> crate::Result<String> {
        // Ok(self.plugin.call::<(), String>("name", ())?)
        todo!("WasmPlugin::name")
    }

    pub fn version(&mut self) -> crate::Result<String> {
        // Ok(self.plugin.call::<(), String>("version", ())?)
        todo!("WasmPlugin::version")
    }

    pub fn about(&mut self) -> crate::Result<String> {
        // Ok(self.plugin.call::<(), String>("about", ())?)
        todo!("WasmPlugin::about")
    }

    pub fn run(&mut self, coordinates: Coordinates, settings: Settings) -> crate::Result<WeatherForecast> {
        let bytes = shared_deps::bincode::serialize(&WasmPluginInput {
            coordinates: coordinates,
            metric: settings.metric_default
        })?;
        // let res = self.plugin.call::<&[u8], &[u8]>("get_forecast", &bytes)?;
        // let forecast: WeatherForecast = shared_deps::bincode::deserialize(&res)?;
        // Ok(forecast)
        todo!("WasmPlugin::run")
    }
}

pub struct WasmLoader {
    pub plugins: Vec<WasmPlugin>
}

impl Default for WasmLoader {
    fn default() -> Self {
        Self {
            plugins: Vec::new()
        }
    }
}

impl WasmLoader {
    pub fn new() -> crate::Result<Self> {
        let mut plugins = Vec::new();
        for entry in std::fs::read_dir(custom_backends_dir()?)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && is_valid_ext(path.to_str().unwrap()) {
                let plugin_result = WasmPlugin::new(path.to_str().unwrap());
                if let Ok(plugin) = plugin_result {
                    info!("Loaded wasm plugin {}", path.to_str().unwrap());
                    plugins.push(plugin);
                } else if let Err(e) = plugin_result {
                    warn!("Failed to load wasm plugin {}: {:?}", path.to_str().unwrap(), e);
                }
            }
        }
        Ok(Self {
            plugins
        })
    }

    pub fn names(&mut self) -> crate::Result<Vec<String>> {
        let mut names = Vec::new();
        for plugin in &mut self.plugins {
            names.push(plugin.name()?);
        }
        Ok(names)
    }

    pub fn call(&mut self, name: &str, coordinates: Coordinates, settings: Settings) -> crate::Result<WeatherForecast> {
        for plugin in &mut self.plugins {
            if plugin.name()? == name {
                let forecast = plugin.run(coordinates, settings.clone())?;
                return Ok(forecast);
            }
        }
        Err("No such plugin found".to_string())? // TODO: string error bad
    }
}

mod tests {
    #[test]
    fn test_wasm_plugin() {
        use super::*;
        let mut plugin = WasmPlugin::new("test_wasm.wasm").unwrap(); // TODO: gen if needed
        assert_eq!(plugin.name().unwrap(), "wasm_test");
    }
}
