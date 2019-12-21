use libloading::{Library, Symbol};
use rp_log::*;
use rp_plugin::Plugin;
use std::ffi::OsStr;

type PluginCreate = unsafe fn() -> *mut dyn Plugin;

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> anyhow::Result<()> {
        let lib = Library::new(filename.as_ref())?;
        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")?;
        let plugin = Box::from_raw(constructor());

        plugin.on_load();
        debug!("Loaded plugin: {}", plugin.name());

        self.loaded_libraries.push(lib);
        self.plugins.push(plugin);

        Ok(())
    }

    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_unload for {}", plugin.name());
            plugin.on_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}
