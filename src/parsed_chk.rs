use crate::{
    chk2::{
        chk_colr::{parse_colr2, ChkColr},
        chk_crgb::{parse_crgb2, ChkCrgb},
        chk_dd2::{parse_dd22, ChkDd2},
    },
    riff::{parse_riff, validate_and_group_riff_chunks},
    ChunkName,
};
use anyhow::Result;
use tracing::instrument;

pub struct ParsedChk<'a> {
    pub colr: Result<ChkColr<'a>>,
    pub crgb: Result<ChkCrgb<'a>>,
    pub dd2: Result<ChkDd2>,
}

#[instrument(level = "trace", skip_all)]
pub fn parse_chk_full<'a>(chk: &'a [u8]) -> ParsedChk<'a> {
    let riff_chunks = parse_riff(chk);
    let riff_chunks = validate_and_group_riff_chunks(riff_chunks.as_slice());

    #[rustfmt::skip]
    let ret = ParsedChk {
        colr: riff_chunks.get(&ChunkName::COLR).map(|x| parse_colr2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        crgb: riff_chunks.get(&ChunkName::CRGB).map(|x| parse_crgb2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        dd2:  riff_chunks.get(&ChunkName::DD2 ).map(|x| parse_dd22 (x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
    };

    ret
}
