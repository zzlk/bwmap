use crate::util::CursorSlicer;
use std::cmp::min;

// Not Required.

// This section will only be different from the MTXM section in tiles where doodads are present.

// u16[ map width * height ]: 1 integer for each map tile. Moves horizontally across the map.
// The values in TILE are normally directly generated from the ISOM section (see "'ISOM' section" above), and thus do not match that of MTXM on doodad tiles.

#[derive(Debug)]
pub struct ChkTile<'a> {
    pub data: &'a [u16],
}

pub(crate) fn parse_tile(sec: &[u8]) -> Result<ChkTile, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkTile {
        data: slicer.extract_rest_as_slice()?,
    })
}
