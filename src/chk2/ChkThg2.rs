use crate::util::CursorSlicer;
use std::cmp::min;

// Required for all versions and all game types.
// Validation: Must be a multiple of 10 bytes.

// The map editor only writes to this section. Starcraft uses this section.

// Sprites, usually on doodads, get the following structure.

// u16: Unit/Sprite number of the sprite
// u16: X coordinate of the doodad unit
// u16: Y coordinate of the doodad unit
// u8: Player number that owns the doodad
// u8: Unused
// u16: Flags
// Bit 0-11 - Unused
// Bit 12 - Draw as sprite (Determines if it is a sprite or a unit sprite)
// Bit 13 - Unused
// Bit 14 - Unused
// Bit 15 - Disabled (Only valid if Draw as sprite is unchecked, disables the unit)
// This section can be split. Additional THG2 sections will add more sprites.

#[derive(Debug)]
pub struct ChkThg2<'a> {
    pub sprite_number: &'a u16,
    pub x: &'a u16,
    pub y: &'a u16,
    pub owner: &'a u8,
    pub unused: &'a u8,
    pub flags: &'a u16,
}

pub(crate) fn parse_thg2(sec: &[u8]) -> Result<ChkThg2, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkThg2 {
        sprite_number: slicer.extract_ref()?,
        x: slicer.extract_ref()?,
        y: slicer.extract_ref()?,
        owner: slicer.extract_ref()?,
        unused: slicer.extract_ref()?,
        flags: slicer.extract_ref()?,
    })
}
