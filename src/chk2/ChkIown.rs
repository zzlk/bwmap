use crate::util::CursorSlicer;
use std::cmp::min;

// Not Required.

// This section specifies the owner of each player.

// u8[12]: One byte for each player, specifies the owner of the player:
// 00 - Inactive
// 01 - Computer (game)
// 02 - Occupied by Human Player
// 03 - Rescue Passive
// 04 - Unused
// 05 - Computer
// 06 - Human (Open Slot)
// 07 - Neutral
// 08 - Closed slot
// This section is separate from OWNR as a staredit value. Staredit does not display "inactive" as a valid option. Italicized settings denote invalid map options, which may involve a buffer overflow.

#[derive(Debug)]
pub struct ChkIown<'a> {
    pub player_owner: &'a [u8; 12],
}

pub(crate) fn parse_iown(sec: &[u8]) -> Result<ChkIown, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkIown {
        player_owner: slicer.extract_ref()?,
    })
}
