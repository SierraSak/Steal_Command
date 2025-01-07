#![feature(ptr_sub_ptr)]
use engage::{
    util::get_instance,
    proc::ProcInst,
    proc::Bindable,
    menu::*,
    gamedata::skill::SkillArray,
    gamedata::unit::Unit,
};

use std::sync::OnceLock;

use unity::{prelude::*, system::List};

mod enume;

#[unity::class("App", "MapSequenceTargetSelect")]
pub struct MapSequenceTargetSelect {
    sup: [u8;0x68],
    target_data: Option<&'static enume::MapTargetData>,
    item_index: i32,
    battle_info: &'static (),
    battle_calc: &'static (),
    engage_link_info: &'static (),
    mask_skill: &'static SkillArray,
}

impl MapSequenceTargetSelect {
    pub fn can_select_target(&self) -> bool {
        unsafe { mapSequencetargetselect_canselecttarget(self, None) }
    }
}

#[unity::class("App", "MapSequenceHuman")]
pub struct MapSequenceHuman {
    sup: [u8;0x10],
    job_intro_unit: Option<&'static Unit>,
    job_intro_keyhelp_type: i32,
    return_label: i32,
    old_unit_x: i32,
    old_unit_z: i32,
    old_cursor_x: i32,
    old_cursor_z: i32,
    old_pickup_x: i32,
    old_pickup_z: i32,
    engage_x: i32,
    engage_z: i32,
    enter_x: i32,
    enter_z: i32,
    is_enemy_attack_range: bool,
    is_update_support_skill: bool,
    update_support_skill_unit: Option<&'static Unit>,
    operate_mode: i32,
}

impl Bindable for MapSequenceHuman { }

#[repr(C)]
#[unity::class("App", "MapMind")]
pub struct MapMind {
    sup: [u8;0x10],
    unit_index: u8,
    first_unit_index: u8,
    first_x: i8,
    first_z: i8,
    unit_show_x: i8,
    unit_show_z: i8,
    x: i8,
    z: i8,
    mind: i32,
    attack_x: i8,
    attack_z: i8,
    item_index: i8,
    target_unit_index: u8,
    target_x: i8,
    target_z: i8,
    focus_x: i8,
    focus_z: i8,
    target_argument: i16,
    trade_unit_index: u8,
    event_unit_index: u8,
}

// App.MapMind$$get_Unit	7101dee2b0	App_Unit_o * App.MapMind$$get_Unit(App_MapMind_o * __this, MethodInfo * method)	12
#[unity::from_offset("App", "MapMind", "get_Unit")]
fn get_unit(this: &MapMind, method_info: OptionalMethod) -> &mut Unit;

impl MapMind {
    pub fn get_instance() -> &'static mut MapMind {
        get_instance::<MapMind>()
    }

    /// Seems to get the current unit that is selected by the player. Needs more experimentation.
    pub fn get_unit() -> &'static mut Unit {
        let instance = Self::get_instance();
        unsafe { get_unit(instance, None) }
    }

    pub fn set_tradeunitindex(&mut self, value: i32,) {
        self.focus_x = value as i8;
        return;
    }
}


#[unity::class("App", "MapUnitCommandMenu")]
pub struct MapUnitCommandMenu {
  
}


#[unity::class("", "MapUnitCommandMenu.TradeMenuItem")]
pub struct TradeMenuItem {
    base: BasicMenuItemFields
}

#[skyline::from_offset(0x1f372e0)]
extern "C" fn mapSequencetargetselect_canselecttarget(this: &MapSequenceTargetSelect, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2272fd0)]
extern "C" fn gamesound_postevent(eventname: &'static Il2CppString, character: Option<&()>, method_info: OptionalMethod) -> bool;


static STEAL_CLASS: OnceLock<&'static mut Il2CppClass> = OnceLock::new();



#[unity::hook("App", "MapSequenceTargetSelect", "DecideNormal")]
pub fn MapSequenceTargetSelect_DecideNormal(this: &mut MapSequenceTargetSelect, _method_info: OptionalMethod) {
    let mut maptarget_instance = get_instance::<enume::MapTarget>();
    let cur_mind = maptarget_instance.m_mind;
    if cur_mind == 0x37 {
        let cur_unit = maptarget_instance.unit;
        let cur_skill = maptarget_instance.m_command_skill;
        let mapmind_instance = get_instance::<MapMind>();
        let can_select_check = this.can_select_target();
        let mut unit_index = 7;
        if (can_select_check) && (this.target_data.is_some()) && (this.target_data.unwrap().unit.is_some()) {
            unit_index = this.target_data.unwrap().unit.unwrap().index;
            
        }

        mapmind_instance.focus_x = unit_index as i8;
        //panic!("{}", mapmind_instance.focus_x);
        
        let mapsequencehuman_instance = get_instance::<MapSequenceHuman>();
        
        unsafe{engage::proc::procinst_jump(mapsequencehuman_instance, 0x1b, None)};

        unsafe{gamesound_postevent("Decide".into(), None, None);}
        return;

    }
    else {
        call_original!(this, _method_info);
    }
    return;
}

#[unity::hook("App", "MapTarget", "Enumerate")]
pub fn MapTarget_Enumerate(this: &mut enume::MapTarget, mask: i32, _method_info: OptionalMethod) {
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
            let mut countVar = 0;
            
            this.m_dataset.as_mut().unwrap().m_list
                .iter_mut()
                .for_each(|data_item| {
                    data_item.m_index = countVar;
                    countVar = countVar + 1;
                    
                });
        }
        return;
    }

}

#[unity::hook("App", "MapBasicMenu", ".ctor")]
pub fn MapBasicMenu_ctor(this: &(), menu_item_list: &mut List<TradeMenuItem>, menucontent: &BasicMenuContent, _method_info: OptionalMethod) {
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
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        // We call the native Error dialog of the Nintendo Switch with this convenient method.
        // The error code is set to 69 because we do need a value, while the first message displays in the popup and the second shows up when pressing Details.
        skyline::error::show_error(
            69,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(
        MapBasicMenu_ctor,
        MapTarget_Enumerate,
        MapSequenceTargetSelect_DecideNormal,
    );
}
