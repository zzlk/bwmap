use crate::util::CursorSlicer;
use serde::Serialize;

// Not Required.

// This section contains the doodad map of the level. There are several parts to doodads. The graphical portion is stored in the MTXM section. The second part of the doodad is stored in this section. This section is only used by the map editor.

// Each doodad in the map gets the following structure:

// u16: Number of the doodad. Size of the doodad is dependent on this. Doodads are different for each tileset.
// u16: X coordinate of the doodad unit
// u16: Y coordinate of the doodad unit
// u8: Player number that owns the doodad
// u8: Enabled flag
// 00 - Doodad is enabled (trap can attack, door is closed, etc)
// 01 - Doodad is disabled

#[derive(Debug, Serialize)]
pub struct ChkDd2<'a> {
    pub doodad_number: &'a u16,
    pub x: &'a u16,
    pub y: &'a u16,
    pub owner: &'a u8,
    pub disabled: &'a u8,
}

pub(crate) fn parse_dd2(sec: &[u8]) -> Result<ChkDd2, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkDd2 {
        doodad_number: slicer.extract_ref()?,
        x: slicer.extract_ref()?,
        y: slicer.extract_ref()?,
        owner: slicer.extract_ref()?,
        disabled: slicer.extract_ref()?,
    })
}
