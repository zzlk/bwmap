use crate::{riff::RiffChunk, util::CursorSlicer};
use serde::Serialize;

// Required for all versions and all game types.
// Validation: Must be size of 12 bytes.

// This section contains the species/race of each player.

// u8[12]: 1 byte per player the species of that player:
// 00 - Zerg
// 01 - Terran
// 02 - Protoss
// 03 - Invalid (Independent), shown as "Unknown" in SC:R lobby
// 04 - Invalid (Neutral), shown as "Unknown"
// 05 - User Select
// 06 - Random (Forced; Acts as a selected race)
// 07 - Inactive, shown as locked "Select Race" option in SC:R lobby
// Italicized settings denote invalid map options. Note Players 9-11 are defaultly Inactive and Player 12 is defaultly Neutral.

#[derive(Debug, Serialize)]
pub struct ChkSide<'a> {
    pub player_side: &'a [u8; 12],
}

pub(crate) fn parse_side(sec: &[u8]) -> Result<ChkSide, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkSide {
        player_side: slicer.extract_ref()?,
    })
}

pub(crate) fn parse_side2<'a>(chunks: &[RiffChunk<'a>]) -> Result<ChkSide<'a>, anyhow::Error> {
    anyhow::ensure!(chunks.len() > 0);

    let mut slicer = CursorSlicer::new(chunks[chunks.len() - 1].data);

    Ok(ChkSide {
        player_side: slicer.extract_ref()?,
    })
}
