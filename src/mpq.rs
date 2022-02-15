use crate::chk::verify_is_valid_chk;
use tracing::instrument;

#[instrument]
pub fn get_chk_from_mpq_filename(filename: String) -> anyhow::Result<Vec<u8>, anyhow::Error> {
    // This is really not the rust way to do things but stormlib_bindings is internally not threadsafe so what we can do.
    lazy_static::lazy_static! {
        static ref LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    }

    let cstr = std::ffi::CString::new(filename.as_str())?;

    let _lock = LOCK.lock().unwrap();
    unsafe {
        //let zzz = std::ffi::CString::new(filename.as_str())?;
        let mut mpq_handle = 0 as stormlib_bindings::HANDLE;
        if stormlib_bindings::SFileOpenArchive(cstr.as_ptr(), 0, 0, &mut mpq_handle as *mut _)
            == false
        {
            return Err(anyhow::anyhow!(
                "SFileOpenArchive. GetLastError: {}, filename: {filename}",
                stormlib_bindings::GetLastError()
            ));
        }

        scopeguard::defer! {
            if stormlib_bindings::SFileCloseArchive(mpq_handle) == false {
                println!(
                    "{:?}",
                    anyhow::anyhow!(
                        "SFileCloseArchive. GetLastError: {}, filename: {filename}",
                        stormlib_bindings::GetLastError()
                    )
                );
            }
        };

        let try_map_with_locale = |filename: &str, locale| {
            //println!("try_map_with_locale: {filename}, locale: {locale}");

            let cstr = std::ffi::CString::new(filename)?;

            stormlib_bindings::SFileSetLocale(locale); // Set locale, this function never fails.
            let mut chk_data: Vec<u8> = vec![0; 32 * 1024 * 1024];
            let mut archive_file_handle = 0 as stormlib_bindings::HANDLE;
            if stormlib_bindings::SFileOpenFileEx(
                mpq_handle,
                cstr.as_ptr(),
                0,
                &mut archive_file_handle as *mut _,
            ) == false
            {
                println!(
                    "SFileOpenFileEx. GetLastError: {}, filename: {filename}, locale: {locale}",
                    stormlib_bindings::GetLastError()
                );
                return Err(anyhow::anyhow!(
                    "SFileOpenFileEx. GetLastError: {}, filename: {filename}, locale: {locale}",
                    stormlib_bindings::GetLastError()
                ));
            }

            scopeguard::defer! {
                if stormlib_bindings::SFileCloseFile(archive_file_handle) == false {
                    println!(
                        "{:?}",
                        anyhow::anyhow!(
                            "SFileCloseFile. GetLastError: {}, filename: {filename}, locale: {locale}",
                            stormlib_bindings::GetLastError()
                        )
                    );
                }
            };

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
                if last_error != stormlib_bindings::ERROR_HANDLE_EOF
                    || size == chk_data.len() as u32
                {
                    println!(
                        "SFileReadFile. GetLastError: {}, filename: {filename}, locale: {locale}",
                        last_error,
                    );

                    return Err(anyhow::anyhow!(
                        "SFileReadFile. GetLastError: {}, filename: {filename}, locale: {locale}",
                        last_error,
                    ));
                }
            }

            chk_data.resize(size as usize, 0);

            Ok(chk_data)
        };

        let locales = [
            0x404, 0x405, 0x407, 0x409, 0x40a, 0x40c, 0x410, 0x411, 0x412, 0x415, 0x416, 0x419,
            0x809, 0,
        ];

        // PROTECTION: Some maps put fake scenario.chk files at different locales. Try to find the real one by trying a lot of them.
        for locale in locales {
            if let Ok(x) = try_map_with_locale("staredit\\scenario.chk", locale) {
                // println!("got candidate");
                // PROTECTION: Some maps put very small scenario.chk files in the mpq at different locales, ignore them.
                if verify_is_valid_chk(x.as_slice()) {
                    return Ok(x);
                }
            }
        }

        return Err(anyhow::anyhow!(
            "Couldn't find scenario.chk the legit way: {}, file: {}",
            stormlib_bindings::GetLastError(),
            filename
        ));

        // If none of the files found are useful then fall back to enumarating every single file in the MPQ and check to see if it is a CHK file.;
        // Until I'm confident i won't need this again, I'll just leave it here for now.
        // {
        //     let mut find_data: stormlib_bindings::SFILE_FIND_DATA =
        //         stormlib_bindings::SFILE_FIND_DATA {
        //             cFileName: [0i8; 1024],
        //             szPlainName: 0 as *mut i8,
        //             dwHashIndex: 0,
        //             dwBlockIndex: 0,
        //             dwFileSize: 0,
        //             dwFileFlags: 0,
        //             dwCompSize: 0,
        //             dwFileTimeLo: 0,
        //             dwFileTimeHi: 0,
        //             lcLocale: 0,
        //         };

        //     let search_handle = stormlib_bindings::SFileFindFirstFile(
        //         mpq_handle,
        //         format!("{}\0", "*").as_ptr() as *const i8,
        //         &mut find_data as *mut stormlib_bindings::SFILE_FIND_DATA,
        //         0 as *mut _,
        //     );

        //     scopeguard::defer! {
        //         if stormlib_bindings::SFileFindClose(search_handle) == false {
        //             println!(
        //                 "{:?}",
        //                 anyhow::anyhow!(
        //                     "SFileFindClose. GetLastError: {}, filename: {filename}",
        //                     stormlib_bindings::GetLastError()
        //                 )
        //             );
        //         }
        //     }

        //     loop {
        //         if search_handle == 0 as stormlib_bindings::HANDLE {
        //             return Err(anyhow::anyhow!(
        //                 "search_handle == 0. No more files in MPQ. GetLastError: {}",
        //                 stormlib_bindings::GetLastError()
        //             ));
        //         }

        //         let new_filename =
        //             crate::util::parse_null_terminated_string(&find_data.cFileName[0..]);

        //         println!(
        //             "Enumarating Files: cFileName: {}, dwBlockIndex: {}, dwCompSize: {}, dwFileSize: {}, dwFileTimeHi: {}, dwFileTimeLo: {}, dwHashIndex: {}, lcLocale: {}",
        //             new_filename,
        //             find_data.dwBlockIndex,
        //             find_data.dwCompSize,
        //             find_data.dwFileSize,
        //             find_data.dwFileTimeHi,
        //             find_data.dwFileTimeLo,
        //             find_data.dwHashIndex,
        //             find_data.lcLocale,
        //         );

        //         if let Ok(x) = try_map_with_locale(new_filename, find_data.lcLocale) {
        //             // PROTECTION: Some maps put very small scenario.chk files in the mpq at different locales, ignore them.
        //             if verify_is_valid_chk(x.as_slice()) {
        //                 return Ok(x);
        //             }
        //         }

        //         if stormlib_bindings::SFileFindNextFile(
        //             search_handle,
        //             &mut find_data as *mut stormlib_bindings::SFILE_FIND_DATA,
        //         ) == false
        //         {
        //             return Err(anyhow::anyhow!(
        //                 "SFileFindNextFile. No more files in MPQ. GetLastError: {}",
        //                 stormlib_bindings::GetLastError()
        //             ));
        //         }
        //     }
        // }
    }
}

pub fn get_chk_from_mpq_in_memory(mpq: &[u8]) -> anyhow::Result<Vec<u8>, anyhow::Error> {
    use std::io::Write;

    let path = {
        use std::str::FromStr;
        let uuid = uuid::Uuid::new_v4().to_simple().to_string();
        let path = std::path::PathBuf::from_str(format!("/tmp/{}.scx", uuid).as_str())?;

        let mut file = std::fs::File::create(&path)?;

        file.write(mpq)?;

        file.flush()?;

        path
    };

    scopeguard::defer! {
        if let Err(e) = std::fs::remove_file(&path) {
            println!("{:?}", e);
        }
    }

    get_chk_from_mpq_filename(path.to_string_lossy().to_string())
}
