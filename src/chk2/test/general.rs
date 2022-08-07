use crate::{
    chk::{get_all_string_references, get_string, ChunkName, ParsedChunk},
    merge_raw_chunks,
    mpq::{get_chk_from_mpq_filename, get_chk_from_mpq_in_memory},
    parse_chk, parse_merged_chunks,
};
use anyhow::Result;
use futures::FutureExt;
use std::fs::read;
use walkdir::{DirEntry, WalkDir};

fn for_all_test_maps<F: Fn(DirEntry) + Sync>(func: F) {
    let processed_maps = WalkDir::new(format!("{}/test_vectors", env!("CARGO_MANIFEST_DIR")))
        .into_iter()
        //.par_bridge()
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
fn test_parse_merged_chunks() {
    for_all_test_maps(|e| {
        println!("file: {}", e.file_name().to_string_lossy());
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
fn specific_test_0c0c_bound() -> Result<()> {
    unsafe {
        let filename = format!(
            "{}/test_vectors/OcOc Bound 2(p).scx",
            env!("CARGO_MANIFEST_DIR")
        );
        let cstr = std::ffi::CString::new(filename.as_str())?;

        let mut mpq_handle = 0 as stormlib_bindings::HANDLE;

        stormlib_bindings::SFileSetLocale(0);

        if stormlib_bindings::SFileOpenArchive(cstr.as_ptr(), 0, 0, &mut mpq_handle as *mut _)
            == false
        {
            return Err(anyhow::anyhow!(
                "SFileOpenArchive. GetLastError: {}, filename: {filename}",
                stormlib_bindings::GetLastError()
            ));
        }

        let mut chk_data: Vec<u8> = vec![0; 32 * 1024 * 1024];
        let mut archive_file_handle = 0 as stormlib_bindings::HANDLE;
        if stormlib_bindings::SFileOpenFileEx(
            mpq_handle,
            std::ffi::CString::new("staredit\\scenario.chk")?.as_ptr(),
            0,
            &mut archive_file_handle as *mut _,
        ) == false
        {
            return Err(anyhow::anyhow!(
                "SFileOpenFileEx. GetLastError: {}",
                stormlib_bindings::GetLastError()
            ));
        }

        let mut size: u32 = 0;
        if stormlib_bindings::SFileReadFile(
            archive_file_handle,
            chk_data.as_mut_ptr() as *mut _,
            chk_data.len() as u32,
            &mut size as *mut _,
            0 as *mut _,
        ) == false
        {
            let last_error = stormlib_bindings::GetLastError();
            if last_error != stormlib_bindings::ERROR_HANDLE_EOF || size == chk_data.len() as u32 {
                return Err(anyhow::anyhow!(
                    "SFileReadFile. GetLastError: {}",
                    last_error,
                ));
            }
        }

        chk_data.resize(size as usize, 0);
        anyhow::ensure!(chk_data.len() > 0);
    }

    anyhow::Ok(())
}

#[test]
fn specific_test_sniper_seed() -> Result<()> {
    unsafe {
        let filename = format!(
            "{}/test_vectors/Sniper - Seed vA.scx",
            env!("CARGO_MANIFEST_DIR")
        );
        let cstr = std::ffi::CString::new(filename.as_str())?;

        let mut mpq_handle = 0 as stormlib_bindings::HANDLE;
        if stormlib_bindings::SFileOpenArchive(cstr.as_ptr(), 0, 0, &mut mpq_handle as *mut _)
            == false
        {
            return Err(anyhow::anyhow!(
                "SFileOpenArchive. GetLastError: {}, filename: {filename}",
                stormlib_bindings::GetLastError()
            ));
        }

        stormlib_bindings::SFileSetLocale(0);

        let mut chk_data: Vec<u8> = vec![0; 32 * 1024 * 1024];
        let mut archive_file_handle = 0 as stormlib_bindings::HANDLE;
        if stormlib_bindings::SFileOpenFileEx(
            mpq_handle,
            std::ffi::CString::new("staredit\\scenario.chk")?.as_ptr(),
            0,
            &mut archive_file_handle as *mut _,
        ) == false
        {
            return Err(anyhow::anyhow!(
                "SFileOpenFileEx. GetLastError: {}",
                stormlib_bindings::GetLastError()
            ));
        }

        let mut size: u32 = 0;
        if stormlib_bindings::SFileReadFile(
            archive_file_handle,
            chk_data.as_mut_ptr() as *mut _,
            chk_data.len() as u32,
            &mut size as *mut _,
            0 as *mut _,
        ) == false
        {
            let last_error = stormlib_bindings::GetLastError();
            if last_error != stormlib_bindings::ERROR_HANDLE_EOF || size == chk_data.len() as u32 {
                return Err(anyhow::anyhow!(
                    "SFileReadFile. GetLastError: {}",
                    last_error,
                ));
            }
        }

        chk_data.resize(size as usize, 0);
        anyhow::ensure!(chk_data.len() > 0);
    }

    anyhow::Ok(())
}

#[test]
fn test_specific_map_0c0c_bound_protected_by_acmp_version_1_dot_74() {
    // Unknown protector with a bunch of MPQ hacks?
    let chk = crate::get_chk_from_mpq_filename(format!(
        "{}/test_vectors/OcOc Bound 2(p).scx",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();

    let raw_chunks = parse_chk(&chk);
    let merged_chunks = merge_raw_chunks(&raw_chunks);
    let parsed_chunks = parse_merged_chunks(&merged_chunks).unwrap();

    assert!(parsed_chunks.get(&ChunkName::VCOD).is_some());
}

#[test]
fn test_specific_map_sniper_seed_protected_by_smc_version_2_dot_9() {
    // SMC V2.9
    let chk = crate::get_chk_from_mpq_filename(format!(
        "{}/test_vectors/Sniper - Seed vA.scx",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();

    let raw_chunks = parse_chk(&chk);
    let merged_chunks = merge_raw_chunks(&raw_chunks);
    let parsed_chunks = parse_merged_chunks(&merged_chunks).unwrap();

    assert!(parsed_chunks.get(&ChunkName::VCOD).is_some());
}

#[test]
fn test_specific_map_protected_by_smlp_version_2_dot_5_dot_00() {
    // SMLP 2.5.00
    let chk = crate::get_chk_from_mpq_filename(format!(
        "{}/test_vectors/Lnm Series Bound 12(p).scx",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();

    let raw_chunks = parse_chk(&chk);
    let merged_chunks = merge_raw_chunks(&raw_chunks);
    let parsed_chunks = parse_merged_chunks(&merged_chunks).unwrap();

    assert!(parsed_chunks.get(&ChunkName::VCOD).is_some());
}

#[test]
fn test_specific_map_protected_by_unknown_protector() {
    // Unknown protector with a bunch of MPQ hacks?
    let chk = crate::get_chk_from_mpq_filename(format!(
        "{}/test_vectors/______4VZ015__.scx",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();

    let raw_chunks = parse_chk(&chk);
    let merged_chunks = merge_raw_chunks(&raw_chunks);
    let parsed_chunks = parse_merged_chunks(&merged_chunks).unwrap();

    assert!(parsed_chunks.get(&ChunkName::VCOD).is_some());
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

#[test]
fn test_get_string_on_all_maps() {
    for_all_test_maps(|e| {
        let chk = crate::get_chk_from_mpq_filename(e.path().to_string_lossy().to_string()).unwrap();
        let raw_chunks = crate::parse_chk(chk.as_slice());
        let merged_chunks = crate::merge_raw_chunks(raw_chunks.as_slice());
        let map = crate::parse_merged_chunks(&merged_chunks).unwrap();

        let string_refs = get_all_string_references(&map).unwrap();

        for string_ref in string_refs {
            get_string(&map, string_ref as usize).unwrap();
        }
    });
}

#[test]
fn test_constrain_encoding_detection_algorithm() {
    let f = |s: &str| {
        let mut root = env!("CARGO_MANIFEST_DIR").to_owned();
        root.push_str(format!("/test_vectors/{s}").as_str());

        let mpq = std::fs::read(std::path::Path::new(root.as_str())).unwrap();
        let chk = crate::get_chk_from_mpq_in_memory(mpq.as_slice()).unwrap();
        let raw_chunks = crate::parse_chk(chk.as_slice());
        let merged_chunks = crate::merge_raw_chunks(raw_chunks.as_slice());
        let map = crate::parse_merged_chunks(&merged_chunks).unwrap();

        //let encoding_order = guess_encoding_order(&map).unwrap();

        let sprp_scenario_index = if let Some(ParsedChunk::SPRP(x)) = map.get(&ChunkName::SPRP) {
            *x.scenario_name_string_number
        } else {
            unreachable!();
        };

        get_string(&map, sprp_scenario_index as usize).unwrap()
        // get_string(&map, &encoding_order, sprp_scenario_index as usize).unwrap()
    };

    let test_vectors = [
        ("폭피[뿌요뿌요]", "폭탄피하기[뿌요뿌요].scx"),
        ("JØNÎ$  ßøûñÐ(beta)", "»JoNiS»BoUnD».scx"),
        (
            "\u{0013}\u{0002}Ðúst BóüÑÐ\u{0012}\u{0006}Dust BouND      .",
            "D u s t BouND P .scx",
        ),
        ("\u{4}Poo\u{6}p \u{3}Boun\u{6}d", "poop bound.scx"),
        (
            "\u{3}C\u{4}r\u{6}escent \u{7}B\u{6}ound",
            "Crescent Bound.scx",
        ),
        (
            "\u{4}도라에몽\u{4} 의\u{4} \u{4}S\u{5}unken ",
            "[[[[[도라에몽의성큰(빨�__짱 (1).scx",
        ),
        (
            "\u{1}마린키우기 \u{7}E\u{6}cstasy \u{3}EVF",
            "Ecstasy EVF.scx",
        ),
        ("갓 타워디펜스4VZ015겨울", "______4VZ015__.scx"),
        (
            "\u{2}Can \u{1}You \u{2}Stop \u{1}1 \u{4}Unit? \u{3}§tack",
            "Can u stop 1 unit (stack) (1).scx",
        ),
        (
            "\u{3}Marine Special Forces \u{7}Re",
            "마린키우기_오리지널_re_정식_1.62.scx",
        ),
    ];

    for (a, b) in test_vectors.into_iter() {
        assert_eq!(a, f(b));
    }
}

#[test]
fn test_constrain_encoding_detection_algorithm2() {
    let f = |s: &str, index: usize| {
        let mut root = env!("CARGO_MANIFEST_DIR").to_owned();
        root.push_str(format!("/test_vectors/{s}").as_str());

        let mpq = std::fs::read(std::path::Path::new(root.as_str())).unwrap();
        let chk = crate::get_chk_from_mpq_in_memory(mpq.as_slice()).unwrap();
        let raw_chunks = crate::parse_chk(chk.as_slice());
        let merged_chunks = crate::merge_raw_chunks(raw_chunks.as_slice());
        let map = crate::parse_merged_chunks(&merged_chunks).unwrap();

        get_string(&map, index).unwrap()
    };

    let test_vectors = [
        (
            "\u{3}Marine Special Forces \u{7}Re",
            "마린키우기_오리지널_re_정식_1.62.scx",
            1,
        ),
        (
            "[오리지널 \"Marine Special Forces\" 헌정맵]\r\n오리지널 마린키우기를 리메이크 했습니다.\r\n기존에 있던 마린키우기를 수정하는 것이\r\n아니라 새롭게 제작하였습니다.\r\n\r\n수정일 : 19.07.02\r\n버전 : 정식 1.62\r\n제작 : 리메이커",
            "마린키우기_오리지널_re_정식_1.62.scx",
            2,
        ),
        (
            "\u{3}Marine Special Forces \u{7}Re",
            "MarineSpecialForces_Re_[확장판]_1.31V.scx",
            1,
        ),
        (
            "[오리지널 \"Marine Special Forces\" 헌정맵]\r\n오리지널 마린키우기를 리메이크 했습니다.\r\n기존에 있던 마린키우기를 수정하는 것이\r\n아니라 새롭게 제작하였습니다.\r\n\r\n수정일 : 19.10.06\r\n버전 : 정식 1.31 [확장판]\r\n제작 : 리메이커",
            "MarineSpecialForces_Re_[확장판]_1.31V.scx",
            2,
        ),
        (
            "쓰레기통",
            "Can u stop 1 unit (stack) (1).scx",
            23,
        ),
        (
            "\r\n\u{3}고스트는 한번에 둘 이상 생산할수 없습니다.\r\n",
            "Can u stop 1 unit (stack) (1).scx",
            25,
        ),
        (
            "마린키우기 Let It Snow",
            "마린키우기 Let It Snow - 3.5.scx",
            1,
        ),
        (
            "제가 처음로 제작하는 맵 입니다.\r\n맵 지형 제작 -Men- 감사합니다.\r\nMade By: Lucas Spia\r\nThanks to \r\n-Men- ,Mininii\r\n",
            "마린키우기 Let It Snow - 3.5.scx",
            2,
        ),
        (
            "",
            "마린키우기 Let It Snow - 3.5.scx",
            3,
        ),
        (
            "마린키우기 Snow 3.5 정식버전",
            "마린키우기 Let It Snow - 3.5.scx",
            4,
        ),
        (
            "적 데몬 3.5 정식버전",
            "마린키우기 Let It Snow - 3.5.scx",
            5,
        ),
        (
            "Snow 3.5 정식버전",
            "마린키우기 Let It Snow - 3.5.scx",
            6,
        ),
    ];

    for (a, b, c) in test_vectors.into_iter() {
        assert_eq!(a, f(b, c));
    }
}

#[test]
fn specific_test_rise_of_empires() -> Result<()> {
    let filename = format!(
        "{}/test_vectors/[6]Rise of Empires v6.07e.scx",
        env!("CARGO_MANIFEST_DIR")
    );

    let chk_data = crate::get_chk_from_mpq_filename(filename).unwrap();
    let raw_chunks = parse_chk(&chk_data);
    let merged_chunks = merge_raw_chunks(&raw_chunks);
    let parsed_chunks = parse_merged_chunks(&merged_chunks)?;

    assert!(parsed_chunks.get(&ChunkName::SPRP).is_some());

    anyhow::Ok(())
}
