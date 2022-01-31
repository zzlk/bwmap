use crate::{
    chk::ChunkName, get_chk_from_mpq_filename, get_chk_from_mpq_in_memory, merge_raw_chunks,
    parse_chk, parse_merged_chunks,
};
use futures::FutureExt;
use rayon::prelude::*;
use std::fs::read;
use walkdir::{DirEntry, WalkDir};

fn for_all_test_maps<F: Fn(DirEntry) + Sync>(func: F) {
    let processed_maps = WalkDir::new(format!("{}/test_vectors", env!("CARGO_MANIFEST_DIR")))
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

    assert_eq!(processed_maps, 178);
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
            "filename: {}, chk_data.len(): {}, {:?}",
            e.file_name().to_string_lossy(),
            chk_data.len(),
            parsed_chunks
        );
    });
}

#[test]
#[ignore]
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

        let mut json: Vec<String> = rt.block_on(response.json()).unwrap();

        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let mut rng = thread_rng();
        json.shuffle(&mut rng);

        json
    };

    let do_map = |hash: String| {
        use anyhow::*;

        let hash = hash.clone();
        let client = client.clone();

        async move {
            let bytes = client
                .request(
                    reqwest::Method::GET,
                    format!("https://bounding.net/api/maps/{hash}"),
                )
                .header("cookie", "dontratelimitmebro=true")
                .send()
                .await?
                .bytes()
                .await?;

            tokio::task::spawn_blocking(move || {
                let chk_data = get_chk_from_mpq_in_memory(bytes.as_ref())?;
                let raw_chunks = parse_chk(&chk_data);
                let merged_chunks = merge_raw_chunks(&raw_chunks);
                let parsed_chunks = parse_merged_chunks(&merged_chunks)?;

                anyhow::ensure!(parsed_chunks.get(&ChunkName::VCOD).is_some());

                anyhow::Ok(())
            })
            .await?
            .context(hash.clone())?;

            anyhow::Ok(hash)
        }
    };

    rt.block_on(async {
        let iter = hashes.into_iter();

        process_iter_async_concurrent(
            iter,
            16,
            |count, last_obj| {
                if let Err(x) = last_obj {
                    println!("-------------------------------------------------------------------------");
                    println!("count: {count}");
                    println!("{x:?}");
                    println!("-------------------------------------------------------------------------\n\n\n");
                }
            },
            do_map,
        )
        .await;
    });

    rt.shutdown_background();
}

pub(crate) async fn process_iter_async_concurrent<I, T, F, J, R, F2>(
    mut iter: I,
    max_outstanding: usize,
    on_item_completed: F2,
    func: F,
) -> usize
where
    I: Iterator<Item = T>,
    F: Fn(T) -> R,
    R: futures::Future<Output = J> + Send,
    F2: Fn(usize, J),
{
    let mut futs = Vec::new();
    let mut counter = 0;
    loop {
        while futs.len() < max_outstanding {
            if let Some(entry) = iter.next() {
                futs.push(func(entry).boxed());
            } else {
                break;
            }
        }

        if futs.len() == 0 {
            break;
        }

        let (item, _, remaining_futures) = futures::future::select_all(futs).await;

        futs = remaining_futures;

        counter += 1;

        on_item_completed(counter, item);
    }

    counter
}

#[test]
fn test_get_chk_from_mpq_filename() {
    for_all_test_maps(|e| {
        assert!(
            get_chk_from_mpq_filename(e.path().to_string_lossy().to_string())
                .unwrap()
                .len()
                > 0
        );
    });
}

#[test]
fn test_get_chk_from_mpq_in_memory() {
    for_all_test_maps(|e| {
        assert_eq!(
            get_chk_from_mpq_in_memory(read(e.path()).unwrap().as_slice()).unwrap(),
            get_chk_from_mpq_filename(e.path().to_string_lossy().to_string()).unwrap()
        );
    });
}
