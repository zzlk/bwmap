use crate::util::parse_slice;
use tracing::instrument;

#[derive(Debug)]
pub struct RiffChunk<'a> {
    pub fourcc: [u8; 4],
    pub size: u32,
    pub offset: usize,
    pub data: &'a [u8],
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

#[cfg(test)]
mod test {
    use crate::riff::parse_riff;
    use anyhow::Result;
    use walkdir::{DirEntry, WalkDir};

    fn for_all_test_maps<F: Fn(DirEntry)>(func: F) {
        let processed_maps = WalkDir::new(format!("{}/test_vectors", env!("CARGO_MANIFEST_DIR")))
            .into_iter()
            .filter_map(Result::ok)
            .filter(
                |e| match e.file_name().to_string_lossy().to_string().as_str() {
                    "[EUD]컴디 파이널.scx" => false,
                    "마인의 폭피 1.scm" => false,
                    _ => {
                        !e.file_type().is_dir()
                            && (e.file_name().to_string_lossy().ends_with(".scx")
                                || e.file_name().to_string_lossy().ends_with(".scm"))
                    }
                },
            )
            .map(|e| {
                func(e);
            })
            .count();

        assert_eq!(processed_maps, 185);
    }

    #[test]
    fn test_parse_riff() {
        for_all_test_maps(|e| {
            println!("file: {}", e.file_name().to_string_lossy());
            let chk_data =
                crate::get_chk_from_mpq_filename(e.path().to_string_lossy().to_string()).unwrap();

            let riff_chunks = parse_riff(&chk_data);

            assert!(riff_chunks.len() > 0);
            assert!(riff_chunks
                .iter()
                .position(|x| x.fourcc == *b"VER ")
                .is_some());
        });
    }
}
