use crate::util::CursorSlicer;
use serde::Serialize;

// Not Required.

// This section is required to place isometric terrain on the map. It provides data about the nature of the isometrical "diamonds" (the editing mode of StarEdit).

// u16[ (width / 2 + 1) * (height + 1) * 4 ]: 4 integers for each map rectangle tile (plus one extra tile to the right and bottom of the map), as per the RECT structure. Each rectangle border (left, top, right, bottom) is assigned an "ISOM value." These form the "diamond" pattern of the map, and each rectangle thus gets two ISOM values associated with it. StarEdit somehow hashes these (the exact algorithm is unknown) and thus produces the tile index of the rectangle (which it stores in TILE for reasons of efficiency). The index of the rectangle's right tile is that of its left tile + 16.
// This section is the only truly unknown section of the .chk format. If you're an ex-Blizzard employee or SI, please edit this section. If you have additional research, post on the forums about it and/or edit this section.

#[derive(Debug, Serialize)]
pub struct ChkIsom<'a> {
    pub data: &'a [u16],
}

pub(crate) fn parse_isom(sec: &[u8]) -> Result<ChkIsom, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkIsom {
        data: slicer.extract_rest_as_slice()?,
    })
}
