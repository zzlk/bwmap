use crate::{
    chk::ChunkName, get_chk_from_mpq_in_memory, merge_raw_chunks, parse_chk, parse_merged_chunks,
};
use rayon::prelude::*;

fn for_all_test_maps<F: Fn(walkdir::DirEntry) + Sync>(func: F) {
    let processed_maps =
        walkdir::WalkDir::new(format!("{}/test_vectors", env!("CARGO_MANIFEST_DIR")))
            .into_iter()
            .par_bridge()
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

    assert_eq!(processed_maps, 176);
}

#[test]
fn test_parse_merged_chunks() {
    for_all_test_maps(|e| {
        let chk_data =
            crate::get_chk_from_mpq_filename(e.path().to_string_lossy().to_string()).unwrap();

        let raw_chunks = parse_chk(&chk_data);
        let merged_chunks = merge_raw_chunks(&raw_chunks);
        let parsed_chunks = parse_merged_chunks(&merged_chunks);

        assert!(
            parsed_chunks.is_ok(),
            "filename: {}, {:?}",
            e.file_name().to_string_lossy(),
            parsed_chunks
        );

        let parsed_chunks = parsed_chunks.unwrap();

        // println!("{:?}", parsed_chunks);

        assert!(
            parsed_chunks.get(&ChunkName::VCOD).is_some(),
            "filename: {}, {:?}",
            e.file_name().to_string_lossy(),
            parsed_chunks
        );
    });
}

#[test]
fn test_a_lot_of_maps_from_bounding_dot_net() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let client = reqwest::Client::builder()
        .user_agent("norecord")
        .build()
        .unwrap();

    let hashes = {
        let response = rt
            .block_on(
                client
                    .request(
                        reqwest::Method::GET,
                        "https://bounding.net/api/tests/all_maps",
                    )
                    .header("cookie", "dontratelimitmebro=true")
                    .send(),
            )
            .unwrap();

        let json: Vec<String> = rt.block_on(response.json()).unwrap();

        json
    };

    let ret: Vec<_> = hashes
        .into_par_iter()
        .map(|hash| (hash, client.clone()))
        .map(|(hash, client)| async move {
            client
                .request(
                    reqwest::Method::GET,
                    format!("https://bounding.net/api/maps/{hash}"),
                )
                .header("cookie", "dontratelimitmebro=true")
                .send()
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap()
        })
        .map(|mpq| async {
            let chk_data = get_chk_from_mpq_in_memory(mpq.await.as_ref()).unwrap();
            let raw_chunks = parse_chk(&chk_data);
            let merged_chunks = merge_raw_chunks(&raw_chunks);
            let parsed_chunks = parse_merged_chunks(&merged_chunks);

            assert!(parsed_chunks.is_ok(), "{:?}", parsed_chunks);

            let parsed_chunks = parsed_chunks.unwrap();

            assert!(
                parsed_chunks.get(&ChunkName::VCOD).is_some(),
                "{:?}",
                parsed_chunks
            );
        })
        .collect();

    rt.block_on(futures::future::join_all(ret));

    rt.shutdown_background();
}
