use crate::util::CursorSlicer;
use serde::Serialize;

// Not required.

// This section "additionally identifies" the map version.

// u16: "Additional file format version:"
// 9 - beta/obsolete versions
// 10 - current versions
// When saving as expansion this section gets removed entirely. Contrary to popular belief, it doesn't get "replaced" by IVE2 in hybrid/Brood War scenarios: both seem to be written by StarEdit but not read by StarCraft.

#[derive(Debug, Serialize)]
pub struct ChkIver<'a> {
    pub additional_file_format_version: Option<&'a u16>,
}

pub(crate) fn parse_iver(sec: &[u8]) -> Result<ChkIver, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkIver {
        additional_file_format_version: slicer.extract_ref_lax()?,
    })
}
