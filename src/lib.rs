#![allow(unsafe_op_in_unsafe_fn)]

use std::hash::{Hash, Hasher};

/// A fixed-capacity or heap-allocated buffer storing elements of type `E`.
///
/// Small buffers (up to 256 elements) are stored inline; larger ones use heap allocation.
pub enum Str<E: Sized> {
    B8    { buf: [E;   8], len: u8    },
    B16   { buf: [E;  16], len: u8    },
    B32   { buf: [E;  32], len: u8    },
    B64   { buf: [E;  64], len: u8    },
    B128  { buf: [E; 128], len: u8    },
    B256  { buf: [E; 256], len: u8    },
    Boxed { buf: Box<[E]>, len: usize },
    Empty
}

/// UTF-8 string specialization using a `Str<u8>` buffer.
pub type UTF8 = Str<u8>;

impl<E> Str<E> {
    /// Returns a slice of the stored elements.
    pub unsafe fn as_slice(&self) -> &[E] {
        let (ptr, len) = match self {
            Self::B8    { buf, len } => (buf.as_ptr(), *len as _),
            Self::B16   { buf, len } => (buf.as_ptr(), *len as _),
            Self::B32   { buf, len } => (buf.as_ptr(), *len as _),
            Self::B64   { buf, len } => (buf.as_ptr(), *len as _),
            Self::B128  { buf, len } => (buf.as_ptr(), *len as _),
            Self::B256  { buf, len } => (buf.as_ptr(), *len as _),
            Self::Boxed { buf, len } => (buf.as_ptr(), *len     ),
            Self::Empty => (std::ptr::null(), 0)
        };

        &*std::ptr::slice_from_raw_parts(ptr, len)
    }

    /// Returns a raw pointer to the buffer.
    pub fn as_ptr(&self) -> *const E {
        match self {
            Self::B8    { buf, .. } => buf.as_ptr(),
            Self::B16   { buf, .. } => buf.as_ptr(),
            Self::B32   { buf, .. } => buf.as_ptr(),
            Self::B64   { buf, .. } => buf.as_ptr(),
            Self::B128  { buf, .. } => buf.as_ptr(),
            Self::B256  { buf, .. } => buf.as_ptr(),
            Self::Boxed { buf, .. } => buf.as_ptr(),
            Self::Empty => std::ptr::null()
        }
    }

    /// Returns an unsafe mutable raw pointer to the buffer.
    pub unsafe fn as_mut_ptr(&mut self) -> *mut E {
        match self {
            Self::B8    { buf, .. } => buf.as_mut_ptr(),
            Self::B16   { buf, .. } => buf.as_mut_ptr(),
            Self::B32   { buf, .. } => buf.as_mut_ptr(),
            Self::B64   { buf, .. } => buf.as_mut_ptr(),
            Self::B128  { buf, .. } => buf.as_mut_ptr(),
            Self::B256  { buf, .. } => buf.as_mut_ptr(),
            Self::Boxed { buf, .. } => buf.as_mut_ptr(),
            Self::Empty => std::ptr::null_mut()
        }
    }

    /// Returns the number of elements in the buffer.
    pub fn len(&self) -> usize {
        match self {
            Self::B8    { len, .. } |
            Self::B16   { len, .. } |
            Self::B32   { len, .. } |
            Self::B64   { len, .. } |
            Self::B128  { len, .. } |
            Self::B256  { len, .. } => *len as _,
            Self::Boxed { len, .. } => *len     ,
            Self::Empty => 0
        }
    }
}

impl<E> PartialEq for Str<E> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ptr() == other.as_ptr() && self.len() == other.len()
    }
}
impl<E> Eq for Str<E> { }

impl<E> PartialOrd for Str<E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.len().partial_cmp(&other.len())
    }
}

impl<E> Ord for Str<E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len().cmp(&other.len())
    }
}

impl<E: Hash> Hash for Str<E> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        
        match self {
            Str::Empty => { }
            Str::B8    { buf, len } => (&buf[..*len as usize]).hash(state),
            Str::B16   { buf, len } => (&buf[..*len as usize]).hash(state),
            Str::B32   { buf, len } => (&buf[..*len as usize]).hash(state),
            Str::B64   { buf, len } => (&buf[..*len as usize]).hash(state),
            Str::B128  { buf, len } => (&buf[..*len as usize]).hash(state),
            Str::B256  { buf, len } => (&buf[..*len as usize]).hash(state),
            Str::Boxed { buf, len } => (&buf[..*len]).hash(state),
        }
    }
}

impl AsRef<str> for UTF8 {
    fn as_ref(&self) -> &str {
        unsafe { std::mem::transmute(self.as_slice()) }
    }
}

impl std::ops::Deref for UTF8 {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self.as_slice()) }
    }
}

impl From<&str> for UTF8 {
    fn from(value: &str) -> Self {
        let bytes = value.as_bytes();
        let len   = bytes.len();
        match len {
            0 => Self::Empty,

            1..=8 => {
                let mut buf = [0u8; 8];
                buf[..len].copy_from_slice(bytes);
                Self::B8 { buf, len: len as u8 }
            }

            9..=16 => {
                let mut buf = [0u8; 16];
                buf[..len].copy_from_slice(bytes);
                Self::B16 { buf, len: len as u8 }
            }

            17..=64 => {
                let mut buf = [0u8; 64];
                buf[..len].copy_from_slice(bytes);
                Self::B64 { buf, len: len as u8 }
            }

            65..=128 => {
                let mut buf = [0u8; 128];
                buf[..len].copy_from_slice(bytes);
                Self::B128 { buf, len: len as u8 }
            }

            129..=256 => {
                let mut buf = [0u8; 256];
                buf[..len].copy_from_slice(bytes);
                Self::B256 { buf, len: len as u8 }
            }

            _ => Self::Boxed { buf: Box::from(value.as_bytes()), len }
        }
    }
}

impl From<&[u8]> for UTF8 {
    fn from(slice: &[u8]) -> Self {
        let len = slice.len();
        match len {
            0 => Self::Empty,

            1..=8 => {
                let mut buf = [0u8; 8];
                buf[..len].copy_from_slice(slice);
                Self::B8 { buf, len: len as u8 }
            }

            9..=16 => {
                let mut buf = [0u8; 16];
                buf[..len].copy_from_slice(slice);
                Self::B16 { buf, len: len as u8 }
            }

            17..=64 => {
                let mut buf = [0u8; 64];
                buf[..len].copy_from_slice(slice);
                Self::B64 { buf, len: len as u8 }
            }

            65..=128 => {
                let mut buf = [0u8; 128];
                buf[..len].copy_from_slice(slice);
                Self::B128 { buf, len: len as u8 }
            }

            129..=256 => {
                let mut buf = [0u8; 256];
                buf[..len].copy_from_slice(slice);
                Self::B256 { buf, len: len as u8 }
            }

            _ => Self::Boxed { buf: Box::from(slice), len }
        }
    }
}

impl From<String> for UTF8 {
    fn from(value: String) -> Self {
        let len = value.len();
        match len {
            0       => Self::Empty,
            1..=256 => Self::from(value.as_str()),
            _       => UTF8::Boxed { buf: value.into_boxed_str().into_boxed_bytes(), len }
        }
    }
}

impl From<UTF8> for String {
    fn from(value: UTF8) -> Self {
        match value {
            UTF8::Empty => String::new(),
            UTF8::Boxed { buf, len } => unsafe { String::from_raw_parts(Box::into_raw(buf) as *mut u8, len as usize, len)}
            _ => value.as_ref().to_string()
        }
    }
}

#[cfg(feature = "napi")]
mod napi_impl {
    use napi::{bindgen_prelude::FromNapiValue, Status, sys::*, *};
    use std::os::raw::c_char;
    use crate::UTF8;
    
    impl FromNapiValue for UTF8 {
        unsafe fn from_napi_value(env: napi_env, value: napi_value) -> Result<Self> {
            let mut needed = 0;
            let status = napi_get_value_string_utf8(env, value, std::ptr::null_mut(), 0, &mut needed);

            if status != 0 /* napi_ok */ {
                return Err(Error::new(Status::from(status), "Failed to get string size"));
            }

            if needed <= 256 {
                let mut written = 0;
                let mut buf     = [0u8; 257];
                let status = napi_get_value_string_utf8(env, value, buf.as_mut_ptr() as *mut c_char, needed + 1, &mut written);

                if status != 0 /* napi_ok */ {
                    return Err(Error::new(Status::from(status), "Failed stack read"));
                }

                return std::str::from_utf8(&buf[..written])
                    .map_err(|error| Error::from_reason(error.to_string()))
                    .map(UTF8::from)
            }

            let mut written = 0;
            let mut vec     = Vec::with_capacity(needed + 1);

            let status = napi_get_value_string_utf8(env, value, vec.as_mut_ptr() as *mut c_char, needed + 1, &mut written);

            if status != 0 /* napi_ok */ {
                return Err(Error::new(Status::from(status), "Failed heap read"));
            }

            vec.set_len(written);
            String::from_utf8(vec)
                .map_err(|error| Error::from_reason(error.to_string()))
                .map(UTF8::from)
        }
    }
}
