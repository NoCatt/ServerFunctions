use rrplug::bindings::class_types::cplayer::CPlayer;
use rrplug::offset_functions;
use rrplug::prelude::*;
use rrplug::bindings::class_types::client::CClient; 
use core::ffi::c_char;

offset_functions! {
    ENGINE_FUNCTIONS + EngineFunctions for WhichDll::Engine => {
        client_array = *mut CClient where offset(0x12A53F90);
    }
}

#[derive(Debug)]
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    const PLUGIN_INFO: PluginInfo = PluginInfo::new(
        c"Neinguars small tag plugin",          // name
        c"NeinTagPl", // used for the label in the log should be 9 chars long to be consitent
        c"NeinguarTag",  // dependency string that mods can use
        PluginContext::all(), // context -> if it has only client it will not load on dedicated servers
    );
    fn new(_reloaded: bool) -> Self {
        log::info!("yay logging :D");

        register_sq_functions(set_player_tag);

        Self {}
    }

    // omg some more functions in the trait
}

#[inline]
pub(crate) unsafe fn set_c_char_array<const U: usize>(buf: &mut [c_char; U], new: &str) {
    *buf = [0;U]; // null everything
    buf.iter_mut()
        .zip(new.as_bytes())
        .for_each(|(buf_char, new)| *buf_char = *new as i8);
}

#[rrplug::sqfunction(VM = "SERVER", ExportName = "SetPlayerTag")]
fn set_player_tag(player: Option<&mut CPlayer>, tag: String)-> Result<(),String> {

    // shamelessly stolen from Catornot
    let player = player.ok_or_else(||"passed a non player entity".to_string())?;
    let client = unsafe{ ENGINE_FUNCTIONS.wait().client_array.add((**player.player_index - 1).try_into().unwrap()).as_mut().ok_or("Couldnt find player".to_string()) ?};

    unsafe {
        set_c_char_array(&mut client.clan_tag, &tag);
        set_c_char_array(&mut player.community_clan_tag, &tag);
    }

    Ok(()) // return ok
}


entry!(TemplatePlugin);
