use crate::util::CursorSlicer;
use serde::Serialize;

// Required for Hybrid (in Expansion mode) and Brood War. Not required for Melee.
// Validation: Must be size of 4168 bytes.
// In Brood War scenarios this section replaces "UNIS".

// This section is indentical to UNIS section except it uses the Brood War set of 130 weapons instead of the original 100.

#[derive(Debug, Serialize)]
pub struct ChkUnix<'a> {
    #[serde(skip_serializing)]
    pub config: &'a [u8; 228],
    #[serde(skip_serializing)]
    pub hit_points: &'a [u32; 228],
    #[serde(skip_serializing)]
    pub shield_points: &'a [u16; 228],
    #[serde(skip_serializing)]
    pub armor_points: &'a [u8; 228],
    #[serde(skip_serializing)]
    pub build_time: &'a [u16; 228],
    #[serde(skip_serializing)]
    pub mineral_cost: &'a [u16; 228],
    #[serde(skip_serializing)]
    pub gas_cost: &'a [u16; 228],
    #[serde(skip_serializing)]
    pub string_number: &'a [u16; 228],
    #[serde(skip_serializing)]
    pub base_weapon_damage: &'a [u16; 130],
    #[serde(skip_serializing)]
    pub upgrade_bonus_weapon_damage: &'a [u16; 130],
}

pub(crate) fn parse_unis(sec: &[u8]) -> Result<ChkUnix, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkUnix {
        config: slicer.extract_ref()?,
        hit_points: slicer.extract_ref()?,
        shield_points: slicer.extract_ref()?,
        armor_points: slicer.extract_ref()?,
        build_time: slicer.extract_ref()?,
        mineral_cost: slicer.extract_ref()?,
        gas_cost: slicer.extract_ref()?,
        string_number: slicer.extract_ref()?,
        base_weapon_damage: slicer.extract_ref()?,
        upgrade_bonus_weapon_damage: slicer.extract_ref()?,
    })
}
