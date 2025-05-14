/// A fixed-capacity or heap-allocated buffer storing elements of type `E`.
///
/// Small buffers (up to 256 elements) are stored inline; larger ones use heap allocation.
pub enum Str<E> {
    B8   { buf: [E;   8], len: u8 },
    B16  { buf: [E;  16], len: u8 },
    B32  { buf: [E;  32], len: u8 },
    B64  { buf: [E;  64], len: u8 },
    B128 { buf: [E; 128], len: u8 },
    B256 { buf: [E; 256], len: u8 },
    Heap(Box<[E]>)                 ,
    Empty
}

/// UTF-8 string specialization using a `Str<u8>` buffer.
pub type UTF8 = Str<u8>;

impl<E> Str<E> {
    /// Returns a slice of the stored elements.
    pub fn as_slice(&self) -> &[E] {
        match self {
            Str::Empty             => &[],
            Str::Heap(slice)       => slice,
            Str::B8   { buf, len } => &buf[..(*len as usize)],
            Str::B16  { buf, len } => &buf[..(*len as usize)],
            Str::B32  { buf, len } => &buf[..(*len as usize)],
            Str::B64  { buf, len } => &buf[..(*len as usize)],
            Str::B128 { buf, len } => &buf[..(*len as usize)],
            Str::B256 { buf, len } => &buf[..(*len as usize)],
        }
    }

    /// Returns a raw pointer to the first element of the buffer.
    pub fn as_ptr(&self) -> *const E {
        self.as_slice().as_ptr()
    }

    /// Returns the number of elements in the buffer.
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<E> PartialEq for Str<E> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}
impl<E> Eq for Str<E> { }

impl<E> PartialOrd for Str<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.len().cmp(&other.len()))
    }
}

impl<E> Ord for Str<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len().cmp(&other.len())
    }
}

impl AsRef<str> for UTF8 {
    fn as_ref(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_slice()) }
    }
}

impl std::ops::Deref for UTF8 {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(self.as_slice()) }
    }
}

impl From<&str> for UTF8 {
    fn from(s: &str) -> Self {
        let bytes = s.as_bytes();
        let len   = bytes.len();
        match len {
            0 => Str::Empty,

            1..=8 => {
                let mut buf = [0u8; 8];
                buf[..len].copy_from_slice(bytes);
                Str::B8 { buf, len: len as u8 }
            }

            9..=16 => {
                let mut buf = [0u8; 16];
                buf[..len].copy_from_slice(bytes);
                Str::B16 { buf, len: len as u8 }
            }

            17..=64 => {
                let mut buf = [0u8; 64];
                buf[..len].copy_from_slice(bytes);
                Str::B64 { buf, len: len as u8 }
            }

            65..=128 => {
                let mut buf = [0u8; 128];
                buf[..len].copy_from_slice(bytes);
                Str::B128 { buf, len: len as u8 }
            }

            129..=256 => {
                let mut buf = [0u8; 256];
                buf[..len].copy_from_slice(bytes);
                Str::B256 { buf, len: len as u8 }
            }

            _ => Str::Heap(s.as_bytes().to_vec().into_boxed_slice()),
        }
    }
}

#[cfg(feature = "napi")]
mod napi_impl {
    use std::os::raw::c_char;
    use napi::{
        bindgen_prelude::FromNapiValue,
        sys::{self, napi_env, napi_get_value_string_utf8, napi_value}
    };

    use crate::UTF8;

    impl FromNapiValue for UTF8 {
        unsafe fn from_napi_value(env: napi_env, value: napi_value) -> napi::Result<Self> {
            let mut str_size: usize = 0;

            let status = unsafe {
                napi_get_value_string_utf8(env, value, std::ptr::null_mut(), 0, &mut str_size)
            };

            if status != sys::Status::napi_ok {
                return Err(napi::Error::new(
                    napi::Status::from(status),
                    "Failed to get string size",
                ));
            }

            let mut stc_buf = [0u8; 300];
            let mut dyn_buf    = Vec::with_capacity(str_size + 1);
            let mut status = 0i32;
            let mut copied = 0   ;

            let slice = if str_size <= 300 {
                status = unsafe {
                    napi_get_value_string_utf8(env, value, stc_buf.as_mut_ptr() as *mut c_char, str_size + 1, &mut copied)
                };

                &stc_buf[..copied]
            } else {
                unsafe {
                    napi_get_value_string_utf8(env, value, dyn_buf.as_mut_ptr() as *mut c_char, dyn_buf.len(), &mut copied);
                }
                
                unsafe { dyn_buf.set_len(copied); }
                dyn_buf.truncate(copied);
                dyn_buf.as_slice()
            };

            if status != sys::Status::napi_ok {
                return Err(napi::Error::new(
                    napi::Status::from(status),
                    "Failed to get string content",
                ));
            }

            let s = std::str::from_utf8(slice).map_err(|_| {
                napi::Error::new(napi::Status::InvalidArg, "Invalid UTF-8 received from napi")
            })?;

            Ok(UTF8::from(s))
        }
    }
}
