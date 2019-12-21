use rp_plugin::{declare_plugin, log::*, Plugin};

pub mod dhcp;
pub mod dns;
pub mod frr;
pub mod net;
pub mod nft;

#[derive(Debug, Default)]
pub struct System;

impl Plugin for System {
    fn name(&self) -> &'static str {
        "system"
    }

    fn on_load(&self) {
        debug!("Loading system plugin");
    }

    fn on_unload(&self) {
        debug!("Unloading system plugin");
    }
}

declare_plugin!(System, System::default);
