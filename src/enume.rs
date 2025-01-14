use unity::{
  prelude::*,
  il2cpp::class::Il2CppRGCTXData
};

use engage::{
  gamedata::{ terrain::TerrainData, unit::Unit, Gamedata },
  map::{
    r#enum::RangeEnumerator,
    image::{MapImage, MapImageCore, MapImageCoreByte}
  },
  sequence::mapsequencetargetselect::MapTarget,
  util::get_instance
};

// Define our new method as a trait so that we can extend the MapTarget structure without adding the function in the Engage crate
pub trait StealMapTargetEnumerator {
  fn enumerate_steal(&mut self);
}

impl StealMapTargetEnumerator for MapTarget {
  fn enumerate_steal(&mut self) {
    let mut local_c0 = RangeEnumerator::default();
  
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
  
    if cur_unit.force.unwrap().force_type < 3 {
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
        local_c0.max_z = z_2;
        local_c0.current.z = z_2;
        local_c0.current.x = x_1 - 1;
        local_c0.min_x = x_1;
        local_c0.pivot_z = z;
        local_c0.pivot_x = x_1 + 1;
        local_c0.near = 1;
        local_c0.far = 1;
        // lol whatever i'm tired and don't wanna deal with this, pray the gods are benevolent
        // local_c0.m_current.range = x_1 << 0x20;
        (local_c0.current.range, _) = x_1.overflowing_shl(0x20);
        local_c0.max_x = x_2;
        local_c0.min_z = z_1;
  
        // local_f0 = local_c0.RangeEnumerator_GetEnumerator(&local_f0);
        // local_90 = local_f0;
  
        // ICYMI, C# enumerators are basically Rust iterators. This is also used to do Foreach.
        // Considering we have a huge loop that follows, you can probably tell where this is going.
        let mut local_90 = local_c0.get_enumerator();
  
        let mut force_type2 = 7;
        let mut item_index = 0;
  
        let mut target_unit: &Unit;

        loop {
          'outer: loop {
            loop {
              // Moved where it actually matters

              // local_90.flat_map(|(x, z)| {
              //   println!("Iterating through ({x}, {z})");
              //   mapimage_instance.get_target_unit(x, z)
              // }).for_each(|unit| {
              //   println!("Unit found: {}", engage::mess::Mess::get(unit.person.get_name().unwrap()))
              // });
              
              // Seems like the objective of this loop is walking through the range of coordinates until a Unit is found in the MapImage and break when it happens.
              loop {
                x = local_90.current.x;
                z = local_90.current.z;
  
                let mut piv_x;
                let mut piv_z;

                // Repeat while piv isn't higher than near or far
                loop {
                  //
                  if x == local_90.max_x{
                    // If both max coords are reached, we found nothing and stop here
                    if z == local_90.min_z {
                      // Gone because the function is literally empty.
                      // local_90.RangeEnumerator_Dispose();
                      println!("No valid Targets in range");
                      return;
                    }
                    // We haven't gone through every z position yet, restart the loop from the leftmost X position and check again
                    z = z - 1;
                    x = local_90.min_x;
                  }
                  else {
                    x = x + 1;
                  }

                  piv_x = (x - local_90.pivot_x).abs();
                  piv_z = (z - local_90.pivot_z).abs();
  
                  if ((piv_x + piv_z) < local_90.near) || ((piv_x + piv_z) > local_90.far) {
                    break;
                  }
                }

                // Write the current coords for the next run of the loop
                local_90.current.x = x;
                local_90.current.z = z;
                local_90.current.range = piv_x + piv_z;

                // This'd be the content of the iterator closure
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
