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

// pub(crate) fn parse_slice2<T: Copy>(s: &mut std::io::Cursor<&[u8]>) -> Result<T, anyhow::Error> {
//     let mut buffer = Vec::new();

//     buffer.resize(std::mem::size_of::<T>(), 0); // TODO: could do this at compile time when rust suppports generic const expressions

//     s.read_exact(buffer.as_mut_slice())?;

//     Ok(unsafe { *(buffer.as_ptr() as *const T) })
// }

// pub(crate) fn parse_slice_to_vec<T: Copy>(
//     s: &mut std::io::Cursor<&[u8]>,
//     count: usize,
// ) -> Result<Vec<T>, anyhow::Error> {
//     let mut buffer = Vec::new();

//     buffer.resize(count * std::mem::size_of::<T>(), 0);

//     s.read(buffer.as_mut_slice())?;

//     unsafe {
//         Ok(Vec::from(std::slice::from_raw_parts(
//             buffer.as_ptr() as *const T,
//             buffer.len() / std::mem::size_of::<T>(),
//         )))
//     }
// }

pub(crate) fn reinterpret_slice<T: Sized>(s: &[u8]) -> &[T] {
    if s.len() % std::mem::size_of::<T>() != 0 {
        panic!();
    }

    unsafe {
        std::slice::from_raw_parts(s.as_ptr() as *const T, s.len() / std::mem::size_of::<T>())
    }
}

pub(crate) fn reinterpret_signed_slice<T: Sized>(s: &[i8]) -> &[T] {
    if s.len() % std::mem::size_of::<T>() != 0 {
        panic!();
    }

    unsafe {
        std::slice::from_raw_parts(s.as_ptr() as *const T, s.len() / std::mem::size_of::<T>())
    }
}

pub(crate) fn reinterpret_slice2<T: Sized>(s: &[u8]) -> Result<&[T], anyhow::Error> {
    anyhow::ensure!(
        s.len() % std::mem::size_of::<T>() == 0,
        "s.len(): {}, std::mem::size_of::<T>(): {}",
        s.len(),
        std::mem::size_of::<T>()
    );

    Ok(unsafe {
        std::slice::from_raw_parts(s.as_ptr() as *const T, s.len() / std::mem::size_of::<T>())
    })
}

pub(crate) struct CursorSlicer<'a> {
    s: &'a [u8],
    current_offset: usize,
}

impl<'a> CursorSlicer<'a> {
    pub(crate) fn new(s: &'a [u8]) -> CursorSlicer {
        CursorSlicer {
            s,
            current_offset: 0,
        }
    }

    pub(crate) fn extract_slice<T>(&mut self, elements: usize) -> Result<&'a [T], anyhow::Error> {
        anyhow::ensure!(self.s.len() >= self.current_offset + elements * std::mem::size_of::<T>());

        let ret = reinterpret_slice2(
            &self.s[self.current_offset..self.current_offset + elements * std::mem::size_of::<T>()],
        )?;

        self.current_offset += ret.len() * std::mem::size_of::<T>();

        Ok(ret)
    }

    pub(crate) fn extract_slice_lax<T>(
        &mut self,
        elements: usize,
    ) -> Result<&'a [T], anyhow::Error> {
        anyhow::ensure!(self.s.len() >= self.current_offset);

        let elements = std::cmp::min(
            (self.s.len() - self.current_offset) / std::mem::size_of::<T>(),
            elements,
        );

        let ret = reinterpret_slice2(
            &self.s[self.current_offset..self.current_offset + elements * std::mem::size_of::<T>()],
        )?;

        self.current_offset += ret.len() * std::mem::size_of::<T>();

        Ok(ret)
    }

    pub(crate) fn extract_rest_as_slice<T>(&mut self) -> Result<&'a [T], anyhow::Error> {
        anyhow::ensure!(self.s.len() >= self.current_offset);
        anyhow::ensure!((self.s.len() - self.current_offset) % std::mem::size_of::<T>() == 0);

        let elements = (self.s.len() - self.current_offset) / std::mem::size_of::<T>();

        self.extract_slice(elements)
    }

    // If for example there is some kind of protection where one of the objects is mangled, such as [int, int, int, X] where X is 1 byte instead of 4, the lax variant will ignore the last one.
    pub(crate) fn extract_rest_as_slice_lax<T>(&mut self) -> Result<&'a [T], anyhow::Error> {
        anyhow::ensure!(self.s.len() >= self.current_offset);
        //anyhow::ensure!((self.s.len() - self.current_offset) % std::mem::size_of::<T>() == 0);

        let elements = (self.s.len() - self.current_offset) / std::mem::size_of::<T>();

        self.extract_slice(elements)
    }

    pub(crate) fn extract_ref<T>(&mut self) -> Result<&'a T, anyhow::Error> {
        anyhow::ensure!(self.s.len() >= self.current_offset + std::mem::size_of::<T>());

        let ret = &reinterpret_slice2(
            &self.s[self.current_offset..self.current_offset + std::mem::size_of::<T>()],
        )?[0];
        self.current_offset += std::mem::size_of::<T>();
        Ok(ret)
    }

    pub(crate) fn extract_ref_lax<T>(&mut self) -> Result<Option<&'a T>, anyhow::Error> {
        anyhow::ensure!(self.s.len() >= self.current_offset);

        if self.s.len() >= self.current_offset + std::mem::size_of::<T>() {
            Ok(Some(self.extract_ref()?))
        } else {
            Ok(None)
        }
    }
}

pub(crate) fn parse_null_terminated_string(s: &[i8]) -> &str {
    let mut index = 0;
    for &c in s {
        if c == 0i8 {
            break;
        }

        index += 1;
    }

    std::str::from_utf8(reinterpret_signed_slice(&s[0..index])).unwrap()
}

pub(crate) fn parse_null_terminated_bytestring_unsigned(s: &[u8]) -> &[u8] {
    let mut index = 0;
    for &c in s {
        if c == 0 {
            break;
        }

        index += 1;
    }

    &s[0..index]
}

// pub(crate) fn reinterpret_slice2<T: Copy + Sized>(s: &[i8]) -> &[T] {
//     if s.len() % std::mem::size_of::<T>() != 0 {
//         panic!();
//     }

//     unsafe {
//         std::slice::from_raw_parts(s.as_ptr() as *const T, s.len() / std::mem::size_of::<T>())
//     }
// }
