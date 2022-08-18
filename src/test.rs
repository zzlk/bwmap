use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

pub(crate) fn get_all_test_maps() -> impl Iterator<Item = DirEntry> {
    let vec = WalkDir::new(format!("{}/test_vectors", env!("CARGO_MANIFEST_DIR")))
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
        .collect::<Vec<_>>();

    assert_eq!(vec.len(), 185);

    vec.into_iter()
}
