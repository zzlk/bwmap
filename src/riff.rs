use crate::util::parse_slice;
use tracing::instrument;

pub struct RiffChunk<'a> {
    fourcc: [u8; 4],
    size: u32,
    offset: usize,
    data: &'a [u8],
}

#[instrument(level = "trace", skip_all)]
pub fn parse_riff(chk: &[u8]) -> Vec<RiffChunk> {
    let mut offset = 0;
    let mut ret = Vec::new();

    while offset + 8 < chk.len() {
        let fourcc: [u8; 4] = parse_slice(&chk[offset..offset + 4]);
        let size: u32 = u32::from_le_bytes(parse_slice(&chk[offset + 4..offset + 8]));

        let chunk_data_start_offset = offset + 8;
        let chunk_data_end_offset = (chunk_data_start_offset + size as usize) % u32::MAX as usize;
        offset = chunk_data_end_offset;

        if chunk_data_end_offset < chunk_data_start_offset {
            // negative sized chunks are ignored.
            continue;
        }

        if chunk_data_end_offset <= chk.len() {
            let data = &chk[chunk_data_start_offset..chunk_data_end_offset];
            ret.push(RiffChunk {
                fourcc,
                size,
                offset,
                data,
            });
        }
    }

    ret
}
