use crate::util::CursorSlicer;
use std::cmp::min;

// Required for Hybrid (in Expansion mode) and Brood War. Not required for Melee.
// Validation: Must be size of 396 bytes.
// In Brood War scenarios this section replaces "TECS".

// This section is indentical to UNIS section except it uses the Brood War set of 44 technologies instead of the original 24.

#[derive(Debug)]
pub struct ChkTecx<'a> {
    pub technology_uses_default_settings: &'a [u8; 44],
    pub mineral_cost: &'a [u16; 44],
    pub gas_cost: &'a [u16; 44],
    pub time: &'a [u16; 44],
    pub energy_cost_to_cast: &'a [u16; 44],
}

pub(crate) fn parse_tecx(sec: &[u8]) -> Result<ChkTecx, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkTecx {
        technology_uses_default_settings: slicer.extract_ref()?,
        mineral_cost: slicer.extract_ref()?,
        gas_cost: slicer.extract_ref()?,
        time: slicer.extract_ref()?,
        energy_cost_to_cast: slicer.extract_ref()?,
    })
}
