use rrplug::engine_functions;
use rrplug::prelude::*;
use rrplug::bindings::entity::CBasePlayer;
use rrplug::bindings::entity::CBaseClient;
use core::ffi::c_char;

engine_functions! {
    ENGINE_FUNCTIONS + EngineFunctions for WhichDll::Engine => {
        client_array = *mut CBaseClient, at 0x12A53F90;
    }
}

#[derive(Debug)]
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn new(_plugin_data: &PluginData) -> Self {
        log::info!("yay logging :D");
        _plugin_data.register_sq_functions(set_player_tag);
        Self {}
    }

    fn on_dll_load(&self, _engine: &PluginLoadDLL, _dll_ptr: &DLLPointer) {
        unsafe { EngineFunctions::try_init(_dll_ptr, &ENGINE_FUNCTIONS) };
    }
    fn main(&self) {}

    // omg some more functions in the trait
}

#[inline]
pub(crate) unsafe fn set_c_char_array<const U: usize>(buf: &mut [c_char; U], new: &str) {
    *buf = [0;U]; // null everything
    buf.iter_mut()
        .zip(new.as_bytes())
        .for_each(|(buf_char, new)| *buf_char = *new as i8);
}

#[rrplug::sqfunction(VM = "Server", ExportName = "SetPlayerTag")]
fn set_player_tag(player: &mut CBasePlayer, tag: String) {

    // shamelessly stolen from Catornot
    
    let client = unsafe{ ENGINE_FUNCTIONS.wait().client_array.add((**player.player_index - 1).try_into().unwrap()).as_mut().ok_or("Couldnt find player".to_string()) ?};

    unsafe {
        set_c_char_array(&mut client.clan_tag, &tag);
        set_c_char_array(&mut player.community_clan_tag, &tag);
    }

    Ok(()) // return ok
}


entry!(TemplatePlugin);
