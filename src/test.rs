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
fn test_constrain_encoding_detection_algorithm() {
    let f = |s: &str| {
        let mut root = env!("CARGO_MANIFEST_DIR").to_owned();
        root.push_str(format!("/test_vectors/{s}").as_str());

        let mpq = std::fs::read(std::path::Path::new(root.as_str())).unwrap();
        let chk = crate::get_chk_from_mpq_in_memory(mpq.as_slice()).unwrap();
        let raw_chunks = crate::parse_chk(chk.as_slice());
        let merged_chunks = crate::merge_rawchunks(raw_chunks.as_slice());
        let chk_dump = crate::get_parsed_chk(&merged_chunks).unwrap();

        assert!(chk_dump.mtxm.len() > 0);
        assert!(chk_dump.map_ver > 0);

        chk_dump.scenario_name
    };

    let test_vectors = [
        ("폭탄피하기[뿌요뿌요].scx", "폭피[뿌요뿌요]"),
        ("»JoNiS»BoUnD».scx", "JØNÎ$  ßøûñÐ(beta)"),
        (
            "D u s t BouND P .scx",
            "\u{0013}\u{0002}Ðúst BóüÑÐ\u{0012}\u{0006}Dust BouND      .",
        ),
        ("poop bound.scx", "\u{4}Poo\u{6}p \u{3}Boun\u{6}d"),
        (
            "Crescent Bound.scx",
            "\u{3}C\u{4}r\u{6}escent \u{7}B\u{6}ound",
        ),
        (
            "[[[[[도라에몽의성큰(빨�__짱 (1).scx",
            "\u{4}도라에몽\u{4} 의\u{4} \u{4}S\u{5}unken ",
        ),
        (
            "Ecstasy EVF.scx",
            "\u{1}마린키우기 \u{7}E\u{6}cstasy \u{3}EVF",
        ),
    ];

    test_vectors
        .into_par_iter()
        .for_each(|(a, b)| assert_eq!(f(a), b));
}

#[test]
fn test_get_chk_from_mpq_filename() {
    crate::test::for_all_test_maps(|e| {
        assert!(
            crate::get_chk_from_mpq_filename(e.path().to_string_lossy().to_string())
                .unwrap()
                .len()
                > 0
        );
    });
}

#[test]
fn test_get_chk_from_mpq_in_memory() {
    crate::test::for_all_test_maps(|e| {
        assert_eq!(
            crate::get_chk_from_mpq_in_memory(std::fs::read(e.path()).unwrap().as_slice()).unwrap(),
            crate::get_chk_from_mpq_filename(e.path().to_string_lossy().to_string()).unwrap()
        );
    });
}
