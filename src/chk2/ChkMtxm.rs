use crate::util::CursorSlicer;
use serde::Serialize;
use std::cmp::min;

// Required for all versions and all game types.
// Validation: Must be less than or equal to 131072 (0x20000) bytes (256 * 256 * 2).

// Terrain section that contains a map of the level's appearance. StarEdit disregards this section and instead uses TILE; it is only used in Starcraft.

// u16[map width * height]: one integer for each tile.
// Notes about MTXM:

// The Width/Height of the map is measured in the number of square 32x32p tiles.
// Tiles in this section are listed from left to right, top to bottom.
// The values for each integer are their respective "MegaTile" values in the scenario's tileset. If the size of this section is greater than width*height*2, the data following is ignored. If the size of this section is less, the resulting tiles that have not been defined will be null tiles.
// This section includes doodads as terrain; TILE, which is otherwise identical, doesn't. Out of the terrain sections (TILE, ISOM, and MTXM), SC only reads MTXM for the sake of not having to generate this data on-the-fly: it contains the exact representation of the level's appearance, including doodads. TILE, on the other hand, is directly tied via a tile lookup function to ISOM, and exists for the sake of not having to generate tiles from ISOM on-the-fly in StarEdit.
// If MTXM section is smaller than (map width*height), then the remaining tiles will be filled with null tiles or tiles specified by previous MTXM sections.

#[derive(Debug, Serialize)]
pub struct ChkMtxm<'a> {
    pub data: &'a [u16],
}

pub(crate) fn parse_mtxm(sec: &[u8]) -> Result<ChkMtxm, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkMtxm {
        data: slicer.extract_rest_as_slice()?,
    })
}
