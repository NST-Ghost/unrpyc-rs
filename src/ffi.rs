use std::ffi::{c_char, CStr, CString};
use std::path::Path;
use crate::reader::{read_rpyc_file, decompress_data};
use crate::rpa::RpaArchive;

#[no_mangle]
pub unsafe extern "C" fn unrpyc_decompile(input_path: *const c_char, output_path: *const c_char) -> i32 {
    let input = match CStr::from_ptr(input_path).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let _output = if !output_path.is_null() {
        match CStr::from_ptr(output_path).to_str() {
            Ok(s) => Some(s),
            Err(_) => return -2,
        }
    } else {
        None
    };

    // 1. Read file
    let raw_data = match read_rpyc_file(Path::new(input)) {
        Ok(d) => d,
        Err(_) => return -3,
    };

    // 2. Decompress
    let _decompressed = match decompress_data(&raw_data) {
        Ok(d) => d,
        Err(_) => return -4,
    };

    // TODO: Full decompilation logic to .rpy string
    // For now, this is a placeholder that validates the file can be read and decompressed
    0
}

#[no_mangle]
pub unsafe extern "C" fn unrpyc_extract_rpa(archive_path: *const c_char, output_dir: *const c_char) -> i32 {
    let archive = match CStr::from_ptr(archive_path).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let out_dir = match CStr::from_ptr(output_dir).to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    let mut rpa = match RpaArchive::open(archive) {
        Ok(r) => r,
        Err(_) => return -3,
    };

    if let Err(_) = std::fs::create_dir_all(out_dir) {
        return -4;
    }

    let files = rpa.list_files();
    for file in files {
        if let Ok(Some(data)) = rpa.extract_file(&file) {
            let out_path = Path::new(out_dir).join(&file);
            if let Some(parent) = out_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Err(_) = std::fs::write(out_path, data) {
                return -5;
            }
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn unrpyc_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
