use std::ffi::OsStr;

pub fn getString(strr: Option<&OsStr>) -> String {
    let res = match strr.unwrap().to_str() {
        Some(n) => { String::from(n) }
        _ => String::from("")
    };
    res
}