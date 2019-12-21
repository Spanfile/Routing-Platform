pub mod log {
    pub use rp_log::*;
}

use std::any::Any;

pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &'static str;
    fn on_load(&self) {}
    fn on_unload(&self) {}
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut $crate::Plugin {
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
