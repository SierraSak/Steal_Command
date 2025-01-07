use unity::{
  prelude::*,
  system::List,
  il2cpp::class::Il2CppRGCTXData
};

use engage::{
  util::get_instance,
  gamedata::{
    PersonData,
    Gamedata,
    item::ItemDataFlag,
    unit::Unit,
    unit::UnitEdit,
    skill::SkillData,
  }
};

use std::ops::{Deref, DerefMut};



#[repr(C)]
#[unity::class("App", "MapImageCore`1")]
pub struct MapImageCore { }

#[unity::class("App", "TerrainData")]
pub struct TerrainData {
  sup: [u8;0x10],
  tid: &'static Il2CppString,
  name: &'static Il2CppString,
}

impl Gamedata for TerrainData { }

impl TerrainData {
    pub fn is_not_target(&self) -> bool {
        unsafe { terraindata_is_not_target(self, None) }
    }
}

#[unity::from_offset("App", "TerrainData", "IsNotTarget")]
extern "C" fn terraindata_is_not_target(this: &TerrainData, method_info: OptionalMethod) -> bool;

#[unity::class("App", "MapImage")]
pub struct MapImage {
  junk: [u8;0x10],
  name:  &'static Il2CppString,
  unit: &'static (),
  terrain: &'static MapImageTerrain,
  cost: &'static(),
  danger: &'static(),
  talk: &'static(),
  range: &'static(),
  history: &'static(),
  backup_terrains: &'static(),
  w: i32,
  h: i32,
  playarea_x: i32,
  playarea_z: i32,
  playarea_w: i32,
  playarea_h: i32,
  x1: i32,
  z1: i32,
  x2: i32,
  z2: i32,
  playarea_x1: i32,
  playarea_z1: i32,
  playarea_x2: i32,
  playarea_z2: i32,
}

impl MapImage {
  pub fn get_target_unit(&self, x: i32, y: i32) -> Option<&Unit> {
    unsafe { mapimage_get_target_unit(self, x, y, None) }
  }
}

#[unity::from_offset("App", "MapImage", "GetTargetUnit")]
extern "C" fn mapimage_get_target_unit(this: &MapImage, x: i32, y: i32, method_info: OptionalMethod) -> Option<&Unit>;

#[unity::class("App", "MapTarget")]
pub struct MapTarget {
  junk: [u8;0x10],
  pub unit: Option<&'static Unit>,
  pub x: i8,
  pub z: i8,
  pub m_mind: u32,
  pub m_action_mask: u32,
  pub m_action_temp: u32,
  pub m_dataset: Option<&'static mut MapTargetDataSet>,
  pub m_buffer_a: Option<&'static MapTargetDataSet>,
  pub m_buffer_b: Option<&'static MapTargetDataSet>,
  pub m_select_unit: Option<&'static Unit>,
  pub m_select_x: i8,
  pub m_select_z: i8,
  pub m_select_item_index: u32,
  pub m_command_skill: Option<&'static SkillData>,
  pub m_enumerate_attack_unit_items: &'static(),
  pub m_enumerate_attack_specified_item: &'static(),
  pub m_enumerate_rod_unit_items: &'static(),
  pub m_enumerate_rod_specified_item: &'static(),
}

#[unity::class("", "MapTarget.Data")]
pub struct MapTargetData {
  pub m_index: i8,
  pub m_unit: &'static Unit,
  pub m_x: i8,
  pub m_z: i8,
  pub m_x1: i8,
  pub m_z1: i8,
  pub m_x2: i8,
  pub m_z2: i8,
  pub m_item_mask: i32,
  pub m_select_item_index: i8,
}

impl MapTargetData {
  pub fn set(&self, unit: &Unit, x: i32, z: i32, item_mask: i32, select_item_mask: i32) -> &'static Self {
    unsafe { 
      maptargetdata_set(self, unit, x, z, item_mask, select_item_mask, None)
    }
  }
}

#[skyline::from_offset(0x1e41ba0)]
extern "C" fn maptargetdata_set(this: &MapTargetData, unit: &Unit, x: i32, z: i32, item_mask: i32, select_item_mask: i32, _method_info: OptionalMethod) -> &'static MapTargetData;

#[unity::class("", "MapTarget.DataSet")]
pub struct MapTargetDataSet {
  pub m_list: &'static mut List<MapTargetData>,
  pub m_stack: &'static mut Stack<MapTargetData>,
  pub _item_mask: i32,
}

impl MapTargetDataSet {
  pub fn clear(&self) {
    unsafe { 
      maptargetdataset_clear(self, None)
    }
  }
}

#[skyline::from_offset(0x1e42a60)]
extern "C" fn maptargetdataset_clear(this: &MapTargetDataSet, _method_info: OptionalMethod);

#[unity::class("App", "MapImageCoreByte")]
pub struct MapImageCoreByte {
  
}

#[unity::class("App", "MapImageTerrain")]
pub struct MapImageTerrain {
  m_original: &'static MapImageCoreByte,
  m_base: &'static MapImageCoreByte,
  m_result: &'static MapImageCoreByte,
  m_minimap_infos: &'static (),
  m_minimap_buffers: &'static (),
}

#[unity::class("", "ItemData.FlagField")]
pub struct ItemDataFlagField {
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct MapRange {
  x: i32,
  z: i32,
  range: i32,
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct RangeEnumerator {
  m_current: MapRange,
  m_pivot_x: i32,
  m_pivot_z: i32,
  m_min_x: i32,
  m_min_z: i32,
  m_max_x: i32,
  m_max_z: i32,
  m_near: i32,
  m_far: i32,
}

impl RangeEnumerator {
  pub fn get_enumerator(&self) -> RangeEnumerator {
    // The logic is that we basically initialize a RangeEnumerator to store the data into, have the function fill it, and then return it.
    // I think that's what happens on the C# side but abstractions make it look more convoluted than it is.
    // Truth is, this probably should just be reimplemented as a Rust iterator once the loop logic is understood.
    unsafe { rangeenumerator_getenumerator(self, None)}
  }
}


#[skyline::from_offset(0x24c54d0)]
extern "C" fn rangeenumerator_getenumerator(this: &RangeEnumerator, method_info: OptionalMethod) -> RangeEnumerator;

#[repr(C)]
#[unity::class("System.Collections.Generic", "Stack`1")]
pub struct Stack<T: 'static> {
    pub items: &'static mut Il2CppArray<&'static mut T>,
    pub size: u32,
    version: u32,
    sync_root: *const u8,
}

impl<T: 'static> Deref for StackFields<T> {
    type Target = [&'static mut T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.items.m_items.as_ptr(), self.size as usize) }
    }
}

impl<T: 'static> DerefMut for StackFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.items.m_items.as_mut_ptr(), self.size as usize) }

    }
}

impl<T> Stack<T> {
    pub fn pop(&mut self) -> Option<&'static mut T> {
        let method = self.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from("Pop")))
            .unwrap();
        
        let pop = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &MethodInfo) -> Option<&'static mut T>>(
                method.method_ptr,
            )
        };

        pop(self, method)
    }

    pub fn len(&self) -> usize {
      self.size as _
    }

    pub fn capacity(&self) -> usize {
        self.items.len() as _
    }
}


// What the fuck Sierra, these aren't hooks

// #[unity::hook("App", "unititemlist", "getitem")]
// pub fn UnitItemList_GetItem(this: &UnitItemList, index: i32, _method_info: OptionalMethod) -> &UnitItem{
//   call_original!(this, index, _method_info)
// }

// #[unity::hook("App", "MapEnum.RangeEnumerator", "GetEnumerator")]
// pub fn RangeEnumerator_GetEnumerator(ret_storage: &RangeEnumerator, this: &RangeEnumerator, _method_info: OptionalMethod) -> &'static RangeEnumerator{
//   call_original!(ret_storage, this, _method_info)
// }

// #[unity::hook("App", "MapImage", "GetTargetUnit")]
// pub fn MapImage_GetTargetUnit(this: &MapImage, x: i32, z: i32, _method_info: OptionalMethod) -> &Unit{
//   call_original!(this, x, z, _method_info)
// }

// Resume sane code here

impl MapTarget {
  pub fn enumerate_steal(&mut self) {
    // Moved down because GetEnumerator() returns it
    // let local_90 = RangeEnumerator::default();
    let mut local_c0 = RangeEnumerator::default();
    // Gone because it's instantiated in GetEnumerator();
    // let local_f0 = RangeEnumerator::default();
  
    if self.unit.is_none() {
      println!("self.unit = None");
      return;
    }
    else {
      println!("self.unit = {}", self.unit.unwrap().person.unit_icon_id.unwrap());
    }
    let cur_unit = self.unit.unwrap();
  
    if cur_unit.status.value & 0x10000 != 0{
      println!("self.unit's Status is funky");
      return;
    }
  
    if (cur_unit.extra_hp_stock_count + cur_unit.hp_stock_count == 0) && (cur_unit.hp_value == 0) {
      println!("self.unit's HP is funky");
      return;
    }
  
    let mut mapimage_instance = get_instance::<MapImage>();
  
    if ((mapimage_instance.playarea_z2 - cur_unit.z as i32) * (cur_unit.z as i32 - mapimage_instance.playarea_z1)) | ((mapimage_instance.playarea_x2 - cur_unit.x as i32) * (cur_unit.x as i32 - mapimage_instance.playarea_x1)) < 0 {
      println!("PlayArea for self.unit is funky");
      return;
    }
  
    let class = get_generic_class!(MapImageCore<u8>).unwrap();
      let rgctx = unsafe {
          &*(class.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 5])
      };
      let core_get = unsafe {
          std::mem::transmute::<_, extern "C" fn(&MapImageCoreByte, i64) -> u8>(
              rgctx[3].method_ptr,
          )
      };
  
    println!("cur_unit.x = {}", cur_unit.x);
    println!("cur_unit.z = {}", cur_unit.z);
    
    println!("(cur_unit.x + (cur_unit.z << 5)) = {}", ((cur_unit.x as i32) + ((cur_unit.z as i32) << 5)));
    let result = core_get(mapimage_instance.terrain.m_result, ((cur_unit.x as i32) + ((cur_unit.z as i32) << 5)).into());
  
    let ter_dat = TerrainData::try_index_get(result.into()).unwrap();
  
    println!("Terrain Data = {}", ter_dat.name);
    if ter_dat.is_not_target() {
      println!("Terrain Data is not a valid target");
      return;
    }
  
    let mut force_type1 = 7;
  
    if cur_unit.force.unwrap().force_type < 3{
      force_type1 = cur_unit.force.unwrap().force_type & 0x1f;
    }
  
    if force_type1 < 3 {
      println!("force is valid");
      let mask_skill = cur_unit.mask_skill.unwrap();
      if (cur_unit.status.value & 0x600008000000 == 0) && (mask_skill.flags & 0x14 == 0) && (mask_skill.bad_States & 0x4d0 == 0){
        let mut x: i32 = self.x.into();
        let mut z: i32 = self.z.into();
  
        let x_1 = (x - 1).clamp(mapimage_instance.playarea_x1, mapimage_instance.playarea_x2);
        let z_1 = (z - 1).clamp(mapimage_instance.playarea_z1, mapimage_instance.playarea_z2);
        let x_2 = (x + 1).clamp(mapimage_instance.playarea_x1, mapimage_instance.playarea_x2);
        let z_2 = (z + 1).clamp(mapimage_instance.playarea_z1, mapimage_instance.playarea_z2);
        local_c0.m_max_z = z_2;
        local_c0.m_current.z = local_c0.m_max_z;
        local_c0.m_current.x = x_1 - 1;
        local_c0.m_min_x = x_1;
        local_c0.m_pivot_z = z;
        local_c0.m_near = 1;
        local_c0.m_far = 1;
        // lol whatever i'm tired and don't wanna deal with this, pray the gods are benevolent
        // local_c0.m_current.range = x_1 << 0x20;
        //(local_c0.m_current.range, _) = x_1.overflowing_shl(0x20);
        (local_c0.m_current.range, _) = x_1.overflowing_shl(0x20);
        local_c0.m_max_x = x_2;
        local_c0.m_min_z = z_1;
  
        // local_f0 = local_c0.RangeEnumerator_GetEnumerator(&local_f0);
        // local_90 = local_f0;
  
        // ICYMI, C# enumerators are basically Rust iterators. This is also used to do Foreach.
        // Considering we have a huge loop that follows, you can probably tell where this is going.
        let mut local_90 = local_c0.get_enumerator();
  
        let mut force_type2 = 7;
        let mut item_index = 0;
  
        let mut target_unit: &Unit = Unit::instantiate().unwrap();
        loop {
          'outer: loop {
            loop {
              // Moved where it actually matters
  
              // Seems like the objective of this loop is walking through the range of coordinates until a Unit is found in the MapImage and break when it happens.
              loop {
                x = local_90.m_current.x;
                z = local_90.m_current.z;
  
                let mut piv_x;
                let mut piv_z;
  
                loop {
                  if x == local_90.m_max_x{
                    if z == local_90.m_min_z{
                      // Gone because the function is literally empty.
                      // local_90.RangeEnumerator_Dispose();
                      println!("No valid Targets in range");
                      return;
                    }
                    z = z - 1;
                    x = local_90.m_min_x;
                  }
                  else {
                    x = x + 1;
                  }
                  piv_x = (x - local_90.m_pivot_x).abs();
                  piv_z = (z - local_90.m_pivot_z).abs();
  
                  if ((piv_x + piv_z) < local_90.m_near) || ((piv_x + piv_z) > local_90.m_far) {
                    break;
                  }
                }
  
                local_90.m_current.x = x;
                local_90.m_current.z = z;
                local_90.m_current.range = piv_x + piv_z;
  
                if let Some(unit) = mapimage_instance.get_target_unit(x, z) {
                  target_unit = unit;
                  println!("Target unit = {}", target_unit.person.unit_icon_id.unwrap());
                  break;
                }
                else {
                  println!("Target unit = None");
                }
              }
              
              // Is the Unit we found not part of the Player Force
              if target_unit.force.is_some() {
                force_type2 = target_unit.force.unwrap().force_type & 0x1f;
              }
  
              force_type1 = 7;
  
              // Is the current Unit not part of the Player Force
              if cur_unit.force.is_some(){
                force_type1 = cur_unit.force.unwrap().force_type & 0x1f;
              }
  
              // If the ForceType of the target and user differ, dip
              // BUT THIS IS WRONG!!!!
              // What it does is *the loop repeats for as long as the forces DIFFER!
              if (force_type1 != force_type2) && ((target_unit.x == cur_unit.x) || (target_unit.z == cur_unit.z)) {
                println!("Target unit is different force and not diagonal");
                break;
              }
              println!("Target unit is same force or diagonal");
            }
  
            loop {
              let unit_item = cur_unit.item_list.get_item(item_index);
              if let Some(unit_item) = unit_item {
                if (unit_item.item.flag.value & 0x80) == 0 {
                  if ((unit_item.item.flag.value & 0x200) == 0) && ((unit_item.index | 2) != 2) {
                    break 'outer;
                  }
                }
              }
              item_index = item_index + 1;
              if item_index >= 8 {
                break;
              }
            }
  
            loop {
              let unit_item = target_unit.item_list.get_item(item_index);
              if let Some(unit_item) = unit_item {
                if (unit_item.item.flag.value & 0x80) == 0 {
                  if ((unit_item.item.flag.value & 0x200) == 0) && ((unit_item.index | 2) != 2) {
                    break 'outer;
                  }
                }
              }
              item_index = item_index + 1;
              if item_index >= 8 {
                break;
              }
            }
          }
  
          //ESCAPED
          if target_unit.status.value & 0x10000 != 0{
            println!("Target Unit has a bad status");
            return;
          }
      
          if (target_unit.extra_hp_stock_count + target_unit.hp_stock_count != 0) || (target_unit.hp_value != 0) {
            mapimage_instance = get_instance::<MapImage>();
  
            if ((mapimage_instance.playarea_z2 - target_unit.z as i32) * (target_unit.z as i32 - mapimage_instance.playarea_z1)) | ((mapimage_instance.playarea_x2 - target_unit.x as i32) * (target_unit.x as i32 - mapimage_instance.playarea_x1)) < 0 {
              break;
            }
  
            let result = core_get(mapimage_instance.terrain.m_result, ((target_unit.x as i32) + ((target_unit.z as i32) << 5)).into());
  
            let ter_dat = TerrainData::try_index_get(result.into()).unwrap();
  
            if ter_dat.is_not_target() {
              println!("Terrain Data is not a valid target 2");
              return;
            }
  
            let mut force_type = 7;
  
            if target_unit.force.is_some(){
              force_type = target_unit.force.unwrap().force_type & 0x1f;
            }
  
            if ((1 << force_type) & 0x6) != 0 {
              let mask_skill = target_unit.mask_skill.unwrap();
              if (target_unit.status.value & 0x600008000000 == 0) && (mask_skill.flags & 0x14 == 0) && (mask_skill.bad_States & 0x4d0 == 0){
                if self.m_dataset.as_mut().unwrap().m_stack.capacity() > 0 {
                  // We can unwrap here because the capacity is already checked so it cannot fail
                  println!("Added Entry");
                  let entry = self.m_dataset.as_mut().unwrap().m_stack.pop().unwrap();
                  entry.set(target_unit, x, z, 0, -1);
                  self.m_dataset.as_mut().unwrap().m_list.add(entry);
                }
                else {
                  println!("DataSet capacity 0 or less");
                }
              }
              else {
                println!("Target Unit's status/flag/bad states are funky");
              }
            }
            else {
              println!("Target Unit's Force Type & 0x19 == 0");
              println!("Target Unit's Force Type == {}", force_type);
            }
          }
          else {
            println!("Target Unit's HP is funky");
          }
        }
      }
      else{
        println!("General Return");
        return;
      }
    }
    println!("General Return 2");
    //call_original!(this, _method_info);
  }
}
