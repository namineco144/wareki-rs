use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use wareki_core::{to_wareki as core_to_wareki, from_wareki as core_from_wareki};
use chrono::Datelike;

#[repr(C)]
pub struct CWareki {
    pub era_name: *mut c_char,
    pub year: u32,
    pub is_error: bool,
}

#[repr(C)]
pub struct CGregorianDate {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub is_error: bool,
}

#[no_mangle]
pub extern "C" fn to_wareki(year: i32, month: u32, day: u32) -> CWareki {
    match core_to_wareki(year, month, day) {
        Ok(w) => {
            let c_str = CString::new(w.era_name()).unwrap();
            CWareki {
                era_name: c_str.into_raw(),
                year: w.year,
                is_error: false,
            }
        }
        Err(_) => CWareki {
            era_name: std::ptr::null_mut(),
            year: 0,
            is_error: true,
        },
    }
}

#[no_mangle]
pub extern "C" fn from_wareki(era_name: *const c_char, year: u32, month: u32, day: u32) -> CGregorianDate {
    if era_name.is_null() {
        return CGregorianDate { year: 0, month: 0, day: 0, is_error: true };
    }

    let c_str = unsafe { CStr::from_ptr(era_name) };
    let r_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return CGregorianDate { year: 0, month: 0, day: 0, is_error: true },
    };

    match core_from_wareki(r_str, year, month, day) {
        Ok(d) => CGregorianDate {
            year: d.year(),
            month: d.month(),
            day: d.day(),
            is_error: false,
        },
        Err(_) => CGregorianDate { year: 0, month: 0, day: 0, is_error: true },
    }
}

#[no_mangle]
pub extern "C" fn free_wareki_string(s: *mut c_char) {
    if s.is_null() { return; }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
