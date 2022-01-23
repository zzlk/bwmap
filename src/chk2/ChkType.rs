use crate::util::CursorSlicer;
use serde::Serialize;
use std::cmp::min;

// Not required.

// This section specifies the type of scenario.

// u32: Specifies the type of scenario:
// 0x53574152 or RAWS - 1.04 StarCraft and above ("hybrid")
// 0x42574152 or RAWB - Brood War

#[derive(Debug, Serialize)]
pub struct ChkType<'a> {
    pub scenario_type: &'a u32,
}

pub(crate) fn parse_type(sec: &[u8]) -> Result<ChkType, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkType {
        scenario_type: slicer.extract_ref()?,
    })
}
