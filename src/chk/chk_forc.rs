use crate::{riff::RiffChunk, util::CursorSlicer};
use serde::Serialize;

// Required for all versions and all game types.
// Validation: Must be less than or equal to 20 bytes.

// This section specifies the forces and the information about them.

// u8[8]: 1 byte for each active player, specifying which of the 4 forces (0-based) that the player's on
// u16[4]: 1 integer for each force, string number of the name of the force
// u8[4]: 1 byte for each force specifying the properties:
// Bit 0 - Random start location
// Bit 1 - Allies
// Bit 2 - Allied victory
// Bit 3 - Shared vision
// Bit 4-7 - Unused
// Notes about FORC:

// If there is no string specified for a force name, it will default to a "Force #" string.
// If this section is less than 20 bytes, the remaining bytes are defaulted to 0.
// Players can be on a force greater than 4, however they will not appear in the game lobby.

#[derive(Debug, Serialize)]
pub struct ChkForc<'a> {
    pub player_forces: &'a [u8],
    pub force_name: &'a [u16],
    pub force_properties: &'a [u8],
}

pub(crate) fn parse_forc<'a>(chunks: &[RiffChunk<'a>]) -> Result<ChkForc<'a>, anyhow::Error> {
    anyhow::ensure!(chunks.len() > 0);

    let mut slicer = CursorSlicer::new(chunks[chunks.len() - 1].data);

    Ok(ChkForc {
        player_forces: slicer.extract_slice_lax(8)?,
        force_name: slicer.extract_slice_lax(4)?,
        force_properties: slicer.extract_slice_lax(4)?,
    })
}
