#![feature(ptr_sub_ptr)]

use std::sync::OnceLock;

use mapunitcommand::{MapUnitCommandMenu, TradeMenuItem};
use unity::{ prelude::*, system::List };

use engage::{
    gamesound::GameSound, mapmind::MapMind, menu::*, proc::ProcInst, sequence::{
        mapsequence::human::MapSequenceHuman,
        mapsequencetargetselect::{MapSequenceTargetSelect, MapTarget}
    }, util::{get_instance, get_singleton_proc_instance}
};

mod enume;
use enume::StealMapTargetEnumerator;

static STEAL_CLASS: OnceLock<&'static mut Il2CppClass> = OnceLock::new();

#[unity::hook("App", "MapSequenceTargetSelect", "DecideNormal")]
pub fn mapsequencetargetselect_decide_normal(this: &mut MapSequenceTargetSelect, _method_info: OptionalMethod) {
    let maptarget_instance = get_instance::<MapTarget>();

    let cur_mind = maptarget_instance.m_mind;

    if cur_mind == 0x37 {
        // let cur_unit = maptarget_instance.unit;
        // let cur_skill = maptarget_instance.m_command_skill;
        let mapmind_instance = get_instance::<MapMind>();

        let can_select_check = this.can_select_target();
        let mut unit_index = 7;

        if (can_select_check) && (this.target_data.is_some()) {
            unit_index = this.target_data.unwrap().m_unit.index;
        }

        mapmind_instance.set_trade_unit_index(unit_index as _);
        
        let mapsequencehuman_instance = get_singleton_proc_instance::<MapSequenceHuman>().unwrap();
        
        ProcInst::jump(mapsequencehuman_instance, 0x1b);

        GameSound::post_event("Decide", None);
    }
    else {
        call_original!(this, _method_info)
    }
}

#[unity::hook("App", "MapTarget", "Enumerate")]
pub fn maptarget_enumerate(this: &mut MapTarget, mask: i32, _method_info: OptionalMethod) {
    if this.m_mind < 0x37 {
        call_original!(this, mask, _method_info);
    }
    else {
        this.m_action_mask = mask as u32;

        if this.unit.is_some() {
            if this.x < 0 {
                this.x = this.unit.unwrap().x as i8;
            }
            if this.z < 0 {
                this.z = this.unit.unwrap().z as i8;
            }
            if this.m_dataset.is_some() {
                this.m_dataset.as_mut().unwrap().clear();
            }
            this.enumerate_steal();
            
            this.m_dataset.as_mut().unwrap().m_list
                .iter_mut()
                .enumerate()
                .for_each(|(count_var, data_item)| {
                    data_item.m_index = count_var as i8;
                    
                });
        }
        return;
    }

}

#[unity::hook("App", "MapBasicMenu", ".ctor")]
pub fn mapbasicmenu_ctor(this: &(), menu_item_list: &mut List<TradeMenuItem>, menucontent: &BasicMenuContent, _method_info: OptionalMethod) {
    let steal = STEAL_CLASS.get_or_init(|| {
        let menu_class  = *MapUnitCommandMenu::class()
            .get_nested_types()
            .iter()
            .find(|class| class.get_name().contains("TradeMenuItem"))
            .unwrap();
        
        let new_class = menu_class.clone();

        new_class
            .get_virtual_method_mut("GetName")
            .map(|method| method.method_ptr = steal_get_name as _)
            .unwrap();

        new_class
            .get_virtual_method_mut("GetHelpText")
            .map(|method| method.method_ptr = steal_get_desc as _)
            .unwrap();

        new_class
            .get_virtual_method_mut("get_Mind")
            .map(|method| method.method_ptr = steal_get_mind as _)
            .unwrap();

        new_class
    });

    let instance = Il2CppObject::<TradeMenuItem>::from_class(steal).unwrap();

    menu_item_list.add(instance);

    call_original!(this, menu_item_list, menucontent, _method_info);
}

pub extern "C" fn steal_get_name(_this: &(), _method_info: OptionalMethod) -> &'static Il2CppString {
    "Steal".into()
}

pub extern "C" fn steal_get_desc(_this: &(), _method_info: OptionalMethod) -> &'static Il2CppString {
    "Take an item from an enemy.".into()
}

pub extern "C" fn steal_get_mind(_this: &(), _method_info: OptionalMethod) -> i32 {
    0x37
}


#[skyline::main(name = "Steal_Command")]
pub fn main() {
    // Install a panic handler for your plugin, allowing you to customize what to do if there's an issue in your code.
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        // Some magic thing to turn what was provided to the panic into a string. Don't mind it too much.
        // The message will be stored in the msg variable for you to use.
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        // This creates a new String with a message of your choice, writing the location of the panic and its message inside of it.
        // Note the \0 at the end. This is needed because show_error is a C function and expects a C string.
        // This is actually just a result of bad old code and shouldn't be necessary most of the time.
        let err_msg = format!(
            "StealCommand has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "StealCommand has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(
        mapbasicmenu_ctor,
        maptarget_enumerate,
        mapsequencetargetselect_decide_normal,
    );
}
