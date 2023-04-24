use std::{collections::HashMap, ffi::OsStr, io, rc::Rc};

use libloading::Library;

use crate::backend::weather_forecast::WeatherForecastRS;
use crate::custom_backend;
use crate::custom_backend::{InvocationError, PluginDeclaration, WeatherForecastPlugin};
use crate::local::settings::Settings;

fn run(
    paths: Vec<String>,
    name: String,
    coordinates: Vec<String>,
    settings: Settings,
) -> WeatherForecastRS {
    let mut functions = ExternalBackends::new();

    unsafe {
        for path in paths {
            functions.load(path).expect("Function loading failed");
        }
    }

    // then call the function
    
    functions
        .call(name, coordinates, settings)
        .expect("Invocation failed")
}

/// A map of all externally provided functions.
#[derive(Default)]
pub struct ExternalBackends {
    functions: HashMap<String, BackendWrapper>,
    libraries: Vec<Rc<Library>>,
}

impl ExternalBackends {
    pub fn new() -> ExternalBackends {
        ExternalBackends::default()
    }

    pub fn call(
        &self,
        name: String,
        coordinates: Vec<String>,
        settings: Settings,
    ) -> Result<WeatherForecastRS, InvocationError> {
        self.functions
            .get(&*name)
            .ok_or_else(|| format!("\"{}\" not found", name))?
            .call(coordinates, settings)
    }

    /// Load a plugin library and add all contained functions to the internal
    /// function table.
    ///
    /// # Safety
    ///
    /// A plugin library **must** be implemented using the
    /// [`plugins_core::plugin_declaration!()`] macro. Trying manually implement
    /// a plugin without going through that macro will result in undefined
    /// behaviour.
    pub unsafe fn load<P: AsRef<OsStr>>(&mut self, library_path: P) -> io::Result<()> {
        // load the library into memory
        let library = Rc::new(Library::new(library_path).expect("Lib init failed"));

        // get a pointer to the plugin_declaration symbol.
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")
            .expect("plugin decl failed")
            .read();

        // version checks to prevent accidental ABI incompatibilities
        if decl.core_version != custom_backend::CORE_VERSION
        // TODO: Add rustc check
        {
            return Err(io::Error::new(io::ErrorKind::Other, "Version mismatch"));
        }

        let mut registrar = PluginRegistrar::new(Rc::clone(&library));

        (decl.register)(&mut registrar);

        // add all loaded plugins to the functions map
        self.functions.extend(registrar.functions);
        // and make sure ExternalFunctions keeps a reference to the library
        self.libraries.push(library);

        Ok(())
    }
}

struct PluginRegistrar {
    functions: HashMap<String, BackendWrapper>,
    lib: Rc<Library>,
}

impl PluginRegistrar {
    fn new(lib: Rc<Library>) -> PluginRegistrar {
        PluginRegistrar {
            lib,
            functions: HashMap::default(),
        }
    }
}

impl custom_backend::PluginRegistrar for PluginRegistrar {
    fn register_function(&mut self, name: &str, backend: Box<dyn WeatherForecastPlugin>) {
        let proxy = BackendWrapper {
            backend,
            _lib: Rc::clone(&self.lib),
        };
        self.functions.insert(name.to_string(), proxy);
    }
}

/// A proxy object which wraps a [`WeatherForecastPlugin`] and makes sure it can't outlive
/// the library it came from.
pub struct BackendWrapper {
    backend: Box<dyn WeatherForecastPlugin>,
    _lib: Rc<Library>,
}

impl WeatherForecastPlugin for BackendWrapper {
    fn call(
        &self,
        coordinates: Vec<String>,
        settings: Settings,
    ) -> Result<WeatherForecastRS, InvocationError> {
        self.backend.call(coordinates, settings)
    }

    fn help(&self) -> Option<&str> {
        self.backend.help()
    }
}
