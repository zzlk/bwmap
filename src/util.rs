pub(crate) fn parse_slice<T: Copy>(s: &[u8]) -> T {
    if std::mem::size_of::<T>() != s.len() {
        panic!(
            "sizeof<t>: {}, s.len(): {}",
            std::mem::size_of::<T>(),
            s.len()
        );
    }

    unsafe { *(s.as_ptr() as *const T) }
}

pub(crate) fn reinterpret_slice<T: Copy + Sized>(s: &[u8]) -> &[T] {
    if s.len() % std::mem::size_of::<T>() != 0 {
        panic!();
    }

    unsafe {
        std::slice::from_raw_parts(s.as_ptr() as *const T, s.len() / std::mem::size_of::<T>())
    }
}

// pub(crate) fn parse_null_terminated_string(s: &[i8]) -> &str {
//     let mut index = 0;
//     for &c in s {
//         if c == 0i8 {
//             break;
//         }

//         index += 1;
//     }

//     std::str::from_utf8(reinterpret_slice2(&s[0..index])).unwrap()
// }

// pub(crate) fn reinterpret_slice2<T: Copy + Sized>(s: &[i8]) -> &[T] {
//     if s.len() % std::mem::size_of::<T>() != 0 {
//         panic!();
//     }

//     unsafe {
//         std::slice::from_raw_parts(s.as_ptr() as *const T, s.len() / std::mem::size_of::<T>())
//     }
// }
