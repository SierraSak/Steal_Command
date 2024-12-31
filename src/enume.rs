use unity::{
  prelude::*,
  system::List,
  il2cpp::class::Il2CppRGCTXData
};

use engage::{
  util::get_instance,
  gamedata::{
    Gamedata,
    unit::Unit,
    skill::SkillData,
    terrain::TerrainData,
  }
};

use std::ops::{Deref, DerefMut};


#[repr(C)]
#[unity::class("App", "MapImageCore`1")]
pub struct MapImageCore { }

#[unity::class("App", "MapImage")]
pub struct MapImage {
  junk: [u8;0x20],
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
  junk: [u8;0x20],
  unit: Option<&'static Unit>,
  x: i8,
  z: i8,
  m_mind: u32,
  m_action_mask: u32,
  m_action_temp: u32,
  m_dataset: &'static mut MapTargetDataSet,
  m_buffer_a: &'static MapTargetDataSet,
  m_buffer_b: &'static MapTargetDataSet,
  m_select_unit: Option<&'static Unit>,
  m_select_x: i8,
  m_select_z: i8,
  m_select_item_index: u32,
  m_command_skill: Option<&'static SkillData>,
  m_enumerate_attack_unit_items: &'static(),
  m_enumerate_attack_specified_item: &'static(),
  m_enumerate_rod_unit_items: &'static(),
  m_enumerate_rod_specified_item: &'static(),
}

#[unity::class("", "MapTarget.Data")]
pub struct MapTargetData {
  m_index: i8,
  m_unit: &'static Unit,
  m_x: i8,
  m_z: i8,
  m_x1: i8,
  m_z1: i8,
  m_x2: i8,
  m_z2: i8,
  m_item_mask: i32,
  m_select_item_index: i8,
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
  m_list: &'static mut List<MapTargetData>,
  m_stack: &'static mut Stack<MapTargetData>,
  _item_mask: i32,
}

#[unity::class("App", "MapImageCoreByte")]
pub struct MapImageCoreByte {
  
}

#[unity::class("App", "MapImageTerrain")]
pub struct MapImageTerrain {
  junk: [u8;0x10],
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
#[derive(Default)]
pub struct MapRange {
  x: i32,
  z: i32,
  range: i32,
}

#[repr(C)]
#[derive(Default)]
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
    let ret = Self::default();
    unsafe { rangeenumerator_getenumerator(&ret, self, None); }
    ret
  }
}


#[skyline::from_offset(0x24c54d0)]
extern "C" fn rangeenumerator_getenumerator(ret: &RangeEnumerator, this: &RangeEnumerator, method_info: OptionalMethod);

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

#[unity::hook("App", "MapTarget", "EnumerateTrade")]
pub fn MapTarget_EnumerateTrade(this: &mut MapTarget, _method_info: OptionalMethod) {
  // Moved down because GetEnumerator() returns it
  // let local_90 = RangeEnumerator::default();
  let mut local_c0 = RangeEnumerator::default();
  // Gone because it's instantiated in GetEnumerator();
  // let local_f0 = RangeEnumerator::default();

  let cur_unit = this.unit.unwrap();

  if cur_unit.status.value & 0x10000 == 0{
    return;
  }

  if (cur_unit.extra_hp_stock_count + cur_unit.hp_stock_count != 0) && (cur_unit.hp_value == 0) {
    return;
  }

  let mut mapimage_instance = get_instance::<MapImage>();

  if ((mapimage_instance.playarea_z2 - cur_unit.z as i32) * (cur_unit.z as i32 - mapimage_instance.playarea_z1)) | ((mapimage_instance.playarea_x2 - cur_unit.x as i32) * (cur_unit.x as i32 - mapimage_instance.playarea_x1)) < 0 {
    return;
  }

  let class = get_generic_class!(MapImageCore<u8>).unwrap();
    let rgctx = unsafe {
        &*(class.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 5])
    };
    let core_get = unsafe {
        std::mem::transmute::<_, extern "C" fn(&MapImageCoreByte, i32) -> u8>(
            rgctx[3].method_ptr,
        )
    };


  let result = core_get(mapimage_instance.terrain.m_result, (cur_unit.x | cur_unit.z << 5).into());

  let ter_dat = TerrainData::try_index_get(result.into()).unwrap();

  if ter_dat.is_not_target() {
    return;
  }

  let mut force_type1 = 7;

  if cur_unit.force.unwrap().force_type != 0{
    force_type1 = cur_unit.force.unwrap().force_type & 0x1f;
  }

  if (1 << force_type1 & 0x19) != 0 {
    let mask_skill = cur_unit.mask_skill.unwrap();
    if (cur_unit.status.value & 0x600008000000 == 0) && (mask_skill.flags & 0x14 == 0) && (mask_skill.bad_States & 0x4d0 == 0){
      let mut x: i32 = this.x.into();
      let mut z: i32 = this.z.into();

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
                break;
              }
            }
            
            // Is the Unit we found not part of the Player Force
            if target_unit.force.unwrap().force_type != 0 {
              force_type2 = target_unit.force.unwrap().force_type & 0x1f;
            }

            force_type1 = 7;

            // Is the current Unit not part of the Player Force
            if cur_unit.force.unwrap().force_type != 0{
              force_type1 = target_unit.force.unwrap().force_type & 0x1f;
            }

            // If the ForceType of the target and user differ, dip
            // BUT THIS IS WRONG!!!!
            // What it does is *the loop repeats for as long as the forces DIFFER!
            if force_type1 != force_type2 {
              break;
            }
          }

          loop {
            let unit_item = cur_unit.item_list.get_item(item_index);
            if let Some(unit_item) = unit_item {
              if (unit_item.flags & 0x80) == 0 {
                if ((unit_item.flags & 0x200) == 0) && ((unit_item.index | 2) != 2) {
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
              if (unit_item.flags & 0x80) == 0 {
                if ((unit_item.flags & 0x200) == 0) && ((unit_item.index | 2) != 2) {
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
        if target_unit.status.value & 0x10000 == 0{
          return;
        }
    
        if (target_unit.extra_hp_stock_count + target_unit.hp_stock_count != 0) && (target_unit.hp_value != 0) {
          mapimage_instance = get_instance::<MapImage>();

          if ((mapimage_instance.playarea_z2 - target_unit.z as i32) * (target_unit.z as i32 - mapimage_instance.playarea_z1)) | ((mapimage_instance.playarea_x2 - target_unit.x as i32) * (target_unit.x as i32 - mapimage_instance.playarea_x1)) < 0 {
            break;
          }


          let result = core_get(mapimage_instance.terrain.m_result, (target_unit.x | target_unit.z << 5).into());

          let ter_dat = TerrainData::try_index_get(result.into()).unwrap();

          if ter_dat.is_not_target() {
            return;
          }

          let mut force_type = 7;

          if target_unit.force.unwrap().force_type != 0{
            force_type = target_unit.force.unwrap().force_type & 0x1f;
          }

          if (1 << force_type & 0x19) != 0 {
            let mask_skill = target_unit.mask_skill.unwrap();
            if (target_unit.status.value & 0x600008000000 == 0) && (mask_skill.flags & 0x14 == 0) && (mask_skill.bad_States & 0x4d0 == 0){
              if this.m_dataset.m_stack.capacity() > 0 {
                // We can unwrap here because the capacity is already checked so it cannot fail
                let entry = this.m_dataset.m_stack.pop().unwrap();
                entry.set(target_unit, x, z, 0, -1);
                this.m_dataset.m_list.add(entry);
              }
            }
          }
        }
      }
    }
    else{
      return;
    }
  }

  //call_original!(this, _method_info);
}
