#![feature(ptr_sub_ptr)]

use std::sync::OnceLock;

use mapunitcommand::{MapUnitCommandMenu, TradeMenuItem};
use unity::{ prelude::*, system::List };

use engage::{
    force::ForceType, gamedata::{unit::Unit, skill::SkillData}, titlebar::TitleBar, gamesound::GameSound, mapmind::MapMind, menu::*, proc::{desc::ProcDesc, Bindable, ProcInst}, sequence::{
        mapsequence::human::MapSequenceHuman,
        mapsequencetargetselect::{MapSequenceTargetSelect, MapTarget}
    }, util::{get_instance, get_singleton_proc_instance}
};

mod enume;
use enume::StealMapTargetEnumerator;

#[unity::class("App", "MapBattleInfoRoot")]
pub struct MapBattleInfoRoot {
    sup: [u8;0x10],
    command_root: &'static (),
    command_sub_root: &'static (),
    command_text: &'static (),
    command_sub_text: &'static (),
    info_left: &'static (),
    info_right: &'static (),
}


#[unity::class("App", "MapSituation")]
pub struct MapSituation {
    sup: [u8;0x10],
    status: &'static (),
    players: &'static (),
    groups: &'static (),
    current_force_type: i32,    
}

impl MapSituation{
    pub fn get_target_unit(&self,  forcetype: i32)  -> i32 {
        unsafe { mapsituation_get_player(self, forcetype, None) }
      }
    
}

#[unity::class("App", "MapCursor")]
pub struct MapCursor {
    sup: [u8;0x10],
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
}

#[repr(C)]
#[unity::class("App", "MapSequence")]
pub struct MapSequence {
    pub descs: &'static mut Il2CppArray<&'static mut ProcDesc>,
    pub desc_index: i32,
    pub name: Option<&'static Il2CppString>,
    /// Unique ID derived from the name of the ProcInst.
    pub hashcode: i32,
    /// The ProcInst this instance is attached to
    pub parent: &'static mut ProcInst,
    /// The next ProcInst to process. ProcInsts are processed from child to parent.
    pub child: *mut MapSequenceHuman2,
}

impl Bindable for MapSequence { }

#[repr(C)]
#[unity::class("App", "MapSequenceHuman")]
pub struct MapSequenceHuman2 {
    pub descs: &'static mut Il2CppArray<&'static mut ProcDesc>,
    pub desc_index: i32,
}

impl Bindable for MapSequenceHuman2 { }

/// A structure representing a call to a method that returns nothing.
#[repr(C)]
#[unity::class("App", "ProcVoidMethod")]
pub struct ProcVoidMethodMut<T: 'static + Bindable> {
    method_ptr: *const u8,
    invoke_impl: *const u8,
    // Usually the ProcInst
    target: Option<&'static mut T>,
    // MethodInfo
    method: *const MethodInfo,
    __: [u8; 0x38],
    delegates: *const u8,
    // ...
}

impl<T: Bindable> engage::proc::Delegate for ProcVoidMethodMut<T> { }

impl<T: Bindable> ProcVoidMethodMut<T> {
    /// Prepare a ProcVoidMethod using your target and method of choice.
    ///
    /// Do be aware that despite the target argument being immutable, the receiving method can, in fact, mutate the target.
    pub fn new(
        target: impl Into<Option<&'static mut T>>,
        method: extern "C" fn(&'static mut T, OptionalMethod),
    ) -> &'static mut ProcVoidMethodMut<T> {
        ProcVoidMethodMut::<T>::instantiate().map(|proc| {
            proc.method_ptr = method as _;
            proc.target = target.into();
            proc.method = Box::leak(Box::new(MethodInfo::new())) as *mut MethodInfo;
            proc
        }).unwrap()
    }
}

#[unity::class("App", "MapBattleInfoParamSetter")]
pub struct MapBattleInfoParamSetter { }

impl MapBattleInfoParamSetter {
    pub fn set_battle_info_for_trade(&self) {
        unsafe { mapbattleinfoparamsetter_setbattleinfofortrade(self, None) }
    }
    
    pub fn set_battle_info_for_no_param(&self, isweapon: bool, isgodname: bool) {
        unsafe { mapbattleinfoparamsetter_setbattleinfofornoparam(self, isweapon, isgodname, None) }
    }
}

#[unity::class("App", "SortieTradeItemMenuItem")]
pub struct SortieTradeItemMenuItem {
    sup: BasicMenuItemFields,
    unit: Option<&'static mut Unit>,
    receiver_unit: Option<&'static mut Unit>,
    item_index: i32,
    default_select: bool,
    selectable_blank: bool,
    enabled_to_select_blank: bool,
    disabled: bool,
}

#[unity::from_offset("App", "MapSituation", "GetPlayer")]
fn mapsituation_get_player(this: &MapSituation, forcetype: i32, method_info: OptionalMethod) -> i32;


#[unity::class("App", "InfoUtil")]
pub struct InfoUtil { }

impl InfoUtil {
    pub fn try_set_text(tmp: &(), string: impl Into<&'static Il2CppString>) {
        unsafe { infoutil_trysettext(tmp, string.into(), None) }
    }
}

#[unity::from_offset("App", "InfoUtil", "TrySetText")]
fn infoutil_trysettext(tmp: &(), str: &'static Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "MapBattleInfoParamSetter", "SetBattleInfoForTrade")]
fn mapbattleinfoparamsetter_setbattleinfofortrade(this: &MapBattleInfoParamSetter, method_info: OptionalMethod);

#[unity::from_offset("App", "MapBattleInfoParamSetter", "SetBattleInfoForNoParam")]
fn mapbattleinfoparamsetter_setbattleinfofornoparam(this: &MapBattleInfoParamSetter, isweapon: bool, isgodname: bool, method_info: OptionalMethod);

// Functions for the ProcDesc fuckening
#[unity::from_offset("App", "MapSequenceHuman", "UnitMenuPrepare")]
fn mapsequencehuman_unitmenuprepare(this: &MapSequenceHuman, method_info: OptionalMethod);

#[unity::from_offset("App", "MapSequenceHuman", "PreItemMenuTrade")]
fn mapsequencehuman_preitemmenutrade(this: &MapSequenceHuman, method_info: OptionalMethod);

#[unity::from_offset("App", "MapItemMenu", "CreateBindTrade")]
fn mapitemmenu_createbindtrade(sup: &ProcInst, method_info: OptionalMethod);

#[unity::from_offset("App", "MapSequenceHuman", "PostItemMenuTrade")]
fn mapsequencehuman_postitemmenutrade(this: &MapSequenceHuman, method_info: OptionalMethod);


static STEAL_CLASS: OnceLock<&'static mut Il2CppClass> = OnceLock::new();

// Change the header text.
// This function is for creating the Header bar ar the top of the screen while the
// Steal/Trade menus are open.  In truth, this particular hook is a bit of a cheat,
// as I am allowing the game to open and create the normal 'Trade' header, and then
// replacing it with the Steal text.  In testing, this hasn't caused any issues.
// But inform one of us if you spot any oddities here.
#[unity::hook("App", "SortieSequenceTrade", "Open")]
pub fn sortiesequencetrade_open(this: &(), _method_info: OptionalMethod) {
    call_original!(this, _method_info);

    if get_instance::<MapTarget>().m_mind == 0x38 {
        TitleBar::open_header("Steal", "Take items from an enemy", "");    
    }
    return
}

// Makes the game hide the damage forecast arrows.
// This function is primarily for setting the
// command name in between the two windows, and deciding whether to hide the damage arrows.
// Thankfully, the default behavior is almost exactly what we want, we just need to adjust it
// to return false, since that's what hides the damage arrows.
#[unity::hook("App", "MapBattleInfoRoot", "Setup")]
pub fn mapbattleinforoot_setup(this: &(), mindtype: i32, skill: &SkillData, info: &(), scene_list: &(), _method_info: OptionalMethod) -> bool {
  
    let mut result = call_original!(this, mindtype, skill, info, scene_list, _method_info);

    if mindtype == 0x38 {
        result = false;
    }

    result
}

// Make some weapons un-stealable
// This function builds the list of items in the trade menu, it runs on both units.
// We let the game build the list as normal, then run through all the items to check
// if their weight is greater or equal to the player's strength, and well as if
// the item is equipped.  If either of those two conditions are true, the menu item is
// disabled.
#[unity::hook("App", "SortieTradeItemMenu", "CreateMenuItemList")]
pub fn sortietradeitemmenuitem_createmenuitemlist(unit: &Unit, receiver_unit: &Unit, default_select: i32, _method_info: OptionalMethod) -> &'static mut List<SortieTradeItemMenuItem> {
    let item_list = call_original!(unit, receiver_unit, default_select, _method_info);
    if unit.force.unwrap().force_type != ForceType::Player as i32 {
        // Check if the command we're processing is Steal
        if get_instance::<MapTarget>().m_mind == 0x38 {
            item_list.iter_mut().zip(unit.item_list.fields.unit_items.iter()).for_each(|(menu_item, unit_item)| {
                menu_item.disabled = unit_item.as_ref()
                    .map(|current_item| current_item.is_equip() || current_item.item.weight > receiver_unit.get_capability(1, true) as u8)
                    .unwrap_or_default();
            });
        }
    }

    // No matter what, we give back the list of MenuItems
    item_list
}

// This function is... interesting.  It essentially builds a BIG list of labels and functions to run.
// The labels are a way for the game to jump around the list and then run a series of functions in a row.
// This is essentially how the ENTIRE game functions to some degree.
// What we're doing here is adding a new section of entries to the list specifically for the Steal command.
// We insert the new function calls and labels in reverse order because adding something to an existing index
// pushes whatever was already there forward, and also makes later additions simpler.
// Ray: I do not see.
#[skyline::hook(offset = 0x2677780)]
pub fn mapsequencehuman_createbind(sup: &mut MapSequence, is_resume: bool, _method_info: OptionalMethod) {
    call_original!(sup, is_resume, _method_info);

    let mut vec = unsafe { (*(sup.child)).descs.to_vec() };

    let desc = engage::proc::desc::ProcDesc::jump(0x10);
    vec.insert(0x9a, desc);

    let method = mapsequencehuman_postitemmenutrade::get_ref();
    let method = unsafe { std::mem::transmute(method.method_ptr) };
    let desc = unsafe { ProcDesc::call(ProcVoidMethodMut::new(&mut (*sup.child), method)) };
    vec.insert(0x9a, desc);

    let method = mapitemmenu_createbindtrade::get_ref();
    let method = unsafe { std::mem::transmute(method.method_ptr) };
    let desc = unsafe { ProcDesc::call(ProcVoidMethodMut::new(&mut (*sup.child), method)) };
    vec.insert(0x9a, desc);

    let method = mapsequencehuman_preitemmenutrade::get_ref();
    let method = unsafe { std::mem::transmute(method.method_ptr) };
    let desc = unsafe { ProcDesc::call(ProcVoidMethodMut::new(&mut (*sup.child), method)) };
    vec.insert(0x9a, desc);

    let method = mapsequencehuman_unitmenuprepare::get_ref();
    let method = unsafe { std::mem::transmute(method.method_ptr) };
    let desc = unsafe { ProcDesc::call(ProcVoidMethodMut::new(&mut (*sup.child), method)) };
    vec.insert(0x9a, desc);

    let steal_label = ProcDesc::label(53);
    vec.insert(0x9a, steal_label);

    let new_descs = Il2CppArray::from_slice(vec).unwrap();
    unsafe { (*sup.child).descs = new_descs };
}

 
// Make the Trade preview window show up when highlighting an enemy with Steal, instead of the battle preview.
// This function is responsible for the windows that pop up when you highlight a target.
// The default behavior without this hook makes the battle forecast appear.  So weapons, hp, etc.
#[unity::hook("App", "MapBattleInfoParamSetter", "SetBattleInfo")]
pub fn mapbattleinfoparamsetter_setbattleinfo(this: &mut MapBattleInfoParamSetter, side_type: i32, show_window: bool, battle_info: &(), scene_list: &(), _method_info: OptionalMethod) {
    call_original!(this, side_type, show_window, battle_info, scene_list, _method_info);

    let maptarget_instance = get_instance::<MapTarget>();

    let cur_mind = maptarget_instance.m_mind;

    if cur_mind == 0x38 {
        this.set_battle_info_for_trade();
    }
}

// Make "Steal" appear on the preview when highlighting an enemy to steal from.
// This function is what sets the text that appears in between the two windows
// when highlighting an enemy.
#[unity::hook("App", "MapBattleInfoRoot", "SetCommandText")]
pub fn mapbattleinforoot_setcommandtext(this: &mut MapBattleInfoRoot, mind_type: i32, _method_info: OptionalMethod) {
    if mind_type != 0x38 {
        call_original!(this, mind_type, _method_info);
    } else {
        InfoUtil::try_set_text(&this.command_text, "Steal");
    }
}

// This is the function that usually runs when you press A while highlighting a target and the
// forecast windows are up.
#[unity::hook("App", "MapSequenceTargetSelect", "DecideNormal")]
pub fn mapsequencetargetselect_decide_normal(this: &mut MapSequenceTargetSelect, _method_info: OptionalMethod) {
    let maptarget_instance = get_instance::<MapTarget>();

    let cur_mind = maptarget_instance.m_mind;

    if cur_mind == 0x38 {
        let mapmind_instance = get_instance::<MapMind>();

        let mut unit_index = 7;

        if this.can_select_target() && this.target_data.is_some() {
            unit_index = this.target_data.unwrap().m_unit.index;
        }

        mapmind_instance.set_trade_unit_index(unit_index as _);
        
        let mapsequencehuman_instance = get_singleton_proc_instance::<MapSequenceHuman>().unwrap();
        
        // This is using the new label added in the mapsequencehuman_createbind.
        ProcInst::jump(mapsequencehuman_instance, 0x35);

        GameSound::post_event("Decide", None);
    } else {
        call_original!(this, _method_info)
    }
}


// This is a generic function that essentially checks the Mind value, and then calls
// a more specialized Enumerate function based on the result.
// Enumerate functions are used for checking if there is a valid target in range,
// and making a list of them.
#[unity::hook("App", "MapTarget", "Enumerate")]
pub fn maptarget_enumerate(this: &mut MapTarget, mask: i32, _method_info: OptionalMethod) {
    
    if this.m_mind < 0x38 {
        call_original!(this, mask, _method_info);
    } else {
        this.m_action_mask = mask as u32;

        if let Some(unit) = this.unit {
            if this.x < 0 {
                this.x = unit.x as i8;
            }

            if this.z < 0 {
                this.z = unit.z as i8;
            }
        }

        if let Some(dataset) = this.m_dataset.as_mut() {
            dataset.clear();
        }

        this.enumerate_steal();

        if let Some(dataset) = this.m_dataset.as_mut() {
            dataset.m_list
                .iter_mut()
                .enumerate()
                .for_each(|(count_var, data_item)| {
                    data_item.m_index = count_var as i8;    
                });
        }
    }

}

// Create our new menu command for Steal.
#[unity::hook("App", "MapUnitCommandMenu", "CreateBind")]
pub fn mapunitcommandmenu_createbind(sup: &mut ProcInst, _method_info: OptionalMethod) {
    let maptarget_instance = get_instance::<MapTarget>();
    let cur_mind = maptarget_instance.m_mind;

    // Create a new class using TradeMenuItem as reference so that we do not wreck the original command for ours.
    let steal = STEAL_CLASS.get_or_init(|| {
        // TradeMenuItem is a nested class inside of MapUnitCommandMenu, so we need to dig for it.
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
            .get_virtual_method_mut("GetCommandHelp")
            .map(|method| method.method_ptr = steal_get_desc as _)
            .unwrap();

        new_class
            .get_virtual_method_mut("get_Mind")
            .map(|method| method.method_ptr = steal_get_mind as _)
            .unwrap();

        new_class
            .get_virtual_method_mut("get_FlagID")
            .map(|method| method.method_ptr = steal_get_flagid as _)
            .unwrap();

        new_class
    });

    call_original!(sup, _method_info);

    // Instantiate our custom class as if it was TradeMenuItem
    let instance = Il2CppObject::<TradeMenuItem>::from_class(steal).unwrap();

    let menu_item_list = &mut sup.child.as_mut().unwrap().cast_mut::<BasicMenu<TradeMenuItem>>().full_menu_item_list;
    menu_item_list.insert((menu_item_list.len() - 1) as i32, instance);
    

}

pub extern "C" fn steal_get_name(_this: &(), _method_info: OptionalMethod) -> &'static Il2CppString {
    "Steal".into()
}

pub extern "C" fn steal_get_desc(_this: &(), _method_info: OptionalMethod) -> &'static Il2CppString {
    "Take items from an enemy.".into()
}

pub extern "C" fn steal_get_mind(_this: &(), _method_info: OptionalMethod) -> i32 {
    0x38
}

pub extern "C" fn steal_get_flagid(_this: &(), _method_info: OptionalMethod) -> &'static Il2CppString {
    "Steal".into()
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
        mapunitcommandmenu_createbind,
        maptarget_enumerate,
        mapsequencetargetselect_decide_normal,
        mapbattleinforoot_setcommandtext,
        mapbattleinfoparamsetter_setbattleinfo,
        mapsequencehuman_createbind,
        sortietradeitemmenuitem_createmenuitemlist,
        mapbattleinforoot_setup,
        sortiesequencetrade_open,
    );
}
