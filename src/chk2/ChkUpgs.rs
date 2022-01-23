use crate::util::CursorSlicer;
use std::cmp::min;

// Required for Vanilla and Hybrid (in Original mode). Not required for Melee.
// Validation: Must be size of 598 bytes.
// In Brood War scenarios, this section is replaced by "UPGx".

// This section contains upgrade settings:

// u8[46]: 1 byte per each upgrade, in order of upgrade id.
// 00 - Upgrade uses custom settings
// 01 - Upgrade uses default settings
// u16[46]: 1 integer per upgrade, base mineral cost for each upgrade, in order of upgrade id.
// u16[46]: 1 integer per upgrade, mineral cost factor for each upgrade, in order of upgrade id.
// u16[46]: 1 integer per upgrade, base gas cost for each upgrade, in order of upgrade id.
// u16[46]: 1 integer per upgrade, gas cost factor for each upgrade, in order of upgrade id.
// u16[46]: 1 integer per upgrade, base time for each upgrade, in order of upgrade id.
// u16[46]: 1 integer per upgrade, gas time factor for each upgrade, in order of upgrade id.
// See #List of Upgrade IDs for upgrade ids.

#[derive(Debug)]
pub struct ChkUpgs<'a> {
    pub upgrade_uses_default_setings: &'a [u8; 46],
    pub base_mineral_cost: &'a [u16; 46],
    pub mineral_cost_factor: &'a [u16; 46],
    pub base_gas_cost: &'a [u16; 46],
    pub gas_cost_factor: &'a [u16; 46],
    pub base_time: &'a [u16; 46],
    pub time_factor: &'a [u16; 46],
}

pub(crate) fn parse_upgs(sec: &[u8]) -> Result<ChkUpgs, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkUpgs {
        upgrade_uses_default_setings: slicer.extract_ref()?,
        base_mineral_cost: slicer.extract_ref()?,
        mineral_cost_factor: slicer.extract_ref()?,
        base_gas_cost: slicer.extract_ref()?,
        gas_cost_factor: slicer.extract_ref()?,
        base_time: slicer.extract_ref()?,
        time_factor: slicer.extract_ref()?,
    })
}
