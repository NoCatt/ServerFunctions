use rrplug::prelude::*;

#[derive(Debug)]
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn new(_plugin_data: &PluginData) -> Self {
        log::info!("yay logging :D");

        Self {}
    }

    fn main(&self) {}

    // omg some more functions in the trait
}

entry!(TemplatePlugin);
