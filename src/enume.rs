#![feature(lazy_cell, ptr_sub_ptr)]
use std::cmp::Ordering;

use engage::gamedata::Gamedata;

use engage::gamedata::unit::Unit;

use engage::gamedata::unit::UnitStatus;

use engage::gamedata::skill::SkillData;

use engage::gamedata::skill::SkillArray;

use engage::gamedata::terrain::TerrainData;

use engage::gamedata::item::UnitItem;

use engage::gamedata::item::UnitItemList;

use engage::force::Force;

use engage::menu::*;

use engage::util::get_instance;

use std::sync::OnceLock;

use std::{ops::{Deref, DerefMut}};

use unity::{prelude::*, system::List, il2cpp::class::Il2CppRGCTXData};

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
  w: u32,
  h: u32,
  playarea_x: u32,
  playarea_z: u32,
  playarea_w: u32,
  playarea_h: u32,
  x1: u32,
  z1: u32,
  x2: u32,
  z2: u32,
  playarea_x1: u32,
  playarea_z1: u32,
  playarea_x2: u32,
  playarea_z2: u32,
}

#[unity::class("App", "MapTarget")]
pub struct MapTarget {
  junk: [u8;0x20],
  unit: Option<&'static Unit>,
  x: u8,
  z: u8,
  m_mind: u32,
  m_action_mask: u32,
  m_action_temp: u32,
  m_dataset: &'static MapTargetDataSet,
  m_buffer_a: &'static MapTargetDataSet,
  m_buffer_b: &'static MapTargetDataSet,
  m_select_unit: Option<&'static Unit>,
  m_select_x: u8,
  m_select_z: u8,
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

#[unity::class("", "MapTarget.DataSet")]
pub struct MapTargetDataSet {
  m_list: &'static List<MapTargetData>,
  m_stack: &'static Stack<MapTargetData>,
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
pub struct MapRange {
  x: i32,
  z: i32,
  range: i32,
}

#[repr(C)]
pub struct RangeEnumerator {
  m_current: &'static MapRange,
  m_pivot_x: i32,
  m_pivot_z: i32,
  m_min_x: i32,
  m_min_z: i32,
  m_max_x: i32,
  m_max_z: i32,
  m_near: i32,
  m_far: i32,
}


#[repr(C)]
#[crate::class("System.Collections.Generic", "Stack`1")]
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
    pub fn pop(&mut self) {
        let method = self.get_class()
            .get_methods()
            .iter()
            .find(|method| method.get_name() == Some(String::from("Pop")))
            .unwrap();
        
        let pop = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &MethodInfo) -> &'static MapTargetData>(
                method.method_ptr,
            )
        };

        pop(self, method);
    }

    pub fn len(&self) -> usize {
      self.size as _
    }

    pub fn capacity(&self) -> usize {
        self.items.len() as _
    }
}


#[unity::hook("App", "unititemlist", "getitem")]
pub fn UnitItemList_GetItem(this: &UnitItemList, index: i32, _method_info: OptionalMethod) -> &UnitItem{
  call_original!(this, index, _method_info)
}

#[unity::hook("App", "TerrainData", "IsNotTarget")]
pub fn TerrainData_IsNotTarget(this: &(), _method_info: OptionalMethod) -> bool{
  call_original!(this, _method_info)
}

#[unity::hook("App", "MapEnum.RangeEnumerator", "Dispose")]
pub fn RangeEnumerator_Dispose(this: &RangeEnumerator, _method_info: OptionalMethod){
  call_original!(this, _method_info)
}

#[unity::hook("App", "MapEnum.RangeEnumerator", "GetEnumerator")]
pub fn RangeEnumerator_GetEnumerator(ret_storage: &RangeEnumerator, this: &RangeEnumerator, _method_info: OptionalMethod) -> &'static RangeEnumerator{
  call_original!(ret_storage, this, _method_info)
}

#[unity::hook("App", "MapImage", "GetTargetUnit")]
pub fn MapImage_GetTargetUnit(this: &MapImage, x: i32, z: i32, _method_info: OptionalMethod) -> &Unit{
  call_original!(this, x, z, _method_info)
}

#[unity::hook("App", "MapTarget.Data", "Set")]
pub fn MapTargetData_Set(this: &MapTargetData, unit: &Unit, x: i8, z: i8, item_mask: i32, select_item_mask: i32, _method_info: OptionalMethod) -> &'static MapTargetData{
  call_original!(this, unit, x, z, item_mask, select_item_mask, _method_info)
}

#[unity::hook("App", "MapTarget", "EnumerateTrade")]
pub fn MapTarget_EnumerateTrade(this: &MapTarget, _method_info: OptionalMethod) {
  let local_90 = RangeEnumerator{m_current: &MapRange{x: 0, z: 0, range: 0}, m_pivot_x: 0, m_pivot_z: 0, m_min_x: 0, m_min_z: 0, m_max_x: 0, m_max_z: 0, m_near: 0, m_far: 0};
  let local_c0 = RangeEnumerator{m_current: &MapRange{x: 0, z: 0, range: 0}, m_pivot_x: 0, m_pivot_z: 0, m_min_x: 0, m_min_z: 0, m_max_x: 0, m_max_z: 0, m_near: 0, m_far: 0};
  let local_f0 = RangeEnumerator{m_current: &MapRange{x: 0, z: 0, range: 0}, m_pivot_x: 0, m_pivot_z: 0, m_min_x: 0, m_min_z: 0, m_max_x: 0, m_max_z: 0, m_near: 0, m_far: 0};

  let curUnit = this.unit.unwrap();

  if curUnit.status.value & 0x10000 == 0{
    return;
  }

  if (curUnit.extra_hp_stock_count + curUnit.hp_stock_count != 0) && (curUnit.hp_value == 0) {
    return;
  }

  let mapImage_instance = get_instance::<MapImage>();

  if (((mapImage_instance.playarea_z2 - curUnit.z.into()) * (curUnit.z.into() - mapImage_instance.playarea_z1)) | ((mapImage_instance.playarea_x2 - curUnit.x.into()) * (curUnit.x.into() - mapImage_instance.playarea_x1)) < 0 ) {
    return;
  }

  let class = get_generic_class!(MapImageCore<u8>).unwrap();
  let virt_method = class.get_vtable().iter().find(|method| method.get_name().unwrap() == "Get" && method.method_info.parameters_count == 2).unwrap();
  

  let class = get_generic_class!(MapImageCore<u8>).unwrap();
    let rgctx = unsafe {
        &*(class.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 5])
    };
    let core_get = unsafe {
        std::mem::transmute::<_, extern "C" fn(&MapImageCoreByte, i32) -> u8>(
            rgctx[3].method_ptr,
        )
    };


  let result = core_get(mapImage_instance.terrain.m_result, (curUnit.x | curUnit.z << 5).into());

  let ter_dat = TerrainData::try_index_get(result.into()).unwrap();

  if ter_dat.TerrainData_IsNotTarget() {
    return;
  }

  let force_type1 = 7;
  if curUnit.force.unwrap().force_type != 0{
    force_type1 = curUnit.force.unwrap().force_type & 0x1f;
  }

  if (1 << force_type1 & 0x19) != 0 {
    let mask_skill = curUnit.mask_skill.unwrap();
    if (curUnit.status.value & 0x600008000000 == 0) && (mask_skill.flags & 0x14 == 0) && (mask_skill.bad_States & 0x4d0 == 0){
      let x: i32 = this.x.into();
      let z: i32 = this.z.into();

      let x_1 = (x - 1).clamp(mapImage_instance.playarea_x1, mapImage_instance.playarea_x2);
      let z_1 = (z - 1).clamp(mapImage_instance.playarea_z1, mapImage_instance.playarea_z2);
      let x_2 = (x + 1).clamp(mapImage_instance.playarea_x1, mapImage_instance.playarea_x2);
      let z_2 = (z + 1).clamp(mapImage_instance.playarea_z1, mapImage_instance.playarea_z2);
      local_c0.m_max_z = z_2;
      local_c0.m_current.z = local_c0.m_max_z;
      local_c0.m_current.x = x_1 - 1;
      local_c0.m_min_x = x_1;
      local_c0.m_pivot_z = z;
      local_c0.m_near = 1;
      local_c0.m_far = 1;
      local_c0.m_current.range = x_1 << 0x20;
      local_c0.m_max_x = x_2;
      local_c0.m_min_z = z_1;

      local_f0 = local_c0.RangeEnumerator_GetEnumerator(&local_f0);
      local_90 = local_f0;

      let piv_x = 0;
      let piv_z = 0;
      let target_unit: &Unit = 0;
      let force_type2 = 7;
      let unit_item: &UnitItem = 0;
      let item_index = 0;

      loop {
        'outer: loop {
          loop {
            loop {
              x = local_90.m_current.x;
              z = local_90.m_current.z;
              loop {
                if x == local_90.m_max_x{
                  if z == local_90.m_min_z{
                    local_90.RangeEnumerator_Dispose();
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
              local_90.m_current.range = (piv_x + piv_z);
              target_unit = mapImage_instance.MapImage_GetTargetUnit(x, z);
              if (target_unit != 0) {
                break;
              }
            }
            
            if target_unit.force.unwrap().force_type != 0{
              force_type2 = target_unit.force.unwrap().force_type & 0x1f;
            }

            force_type1 = 7;
            if curUnit.force.unwrap().force_type != 0{
              force_type1 = target_unit.force.unwrap().force_type & 0x1f;
            }
            if force_type1 != force_type2 {
              break;
            }
          }
          loop {
            unit_item = curUnit.item_list.UnitItemList_GetItem(item_index);
            if unit_item != 0{
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
            unit_item = target_unit.item_list.UnitItemList_GetItem(item_index);
            if unit_item != 0{
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
        mapImage_instance = get_instance::<MapImage>();

        if (((mapImage_instance.playarea_z2 - target_unit.z.into()) * (target_unit.z.into() - mapImage_instance.playarea_z1)) | ((mapImage_instance.playarea_x2 - target_unit.x.into()) * (target_unit.x.into() - mapImage_instance.playarea_x1)) < 0 ) {
          break;
        }


        let result = core_get(mapImage_instance.terrain.m_result, (target_unit.x | target_unit.z << 5).into());

        let ter_dat = TerrainData::try_index_get(result.into()).unwrap();

        if ter_dat.TerrainData_IsNotTarget() {
          return;
        }

        let force_type = 7;
        if target_unit.force.unwrap().force_type != 0{
          force_type = target_unit.force.unwrap().force_type & 0x1f;
        }

        if (1 << force_type & 0x19) != 0 {
          let mask_skill = target_unit.mask_skill.unwrap();
          if (target_unit.status.value & 0x600008000000 == 0) && (mask_skill.flags & 0x14 == 0) && (mask_skill.bad_States & 0x4d0 == 0){
            if this.m_dataset.m_stack.capacity() > 0 {
              let entry = this.m_dataset.m_stack.pop();
              this.m_dataset.m_list.add(entry);
              if entry != 0 {
                entry.MapTargetData_Set(target_unit, x, z, 0, -1);
              }
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
