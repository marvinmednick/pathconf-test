use nix::errno::{errno, Errno};
use std::env;
use std::ffi::CString;

fn main() {
    println!(
        "Rust pathconf results via libc on platform {}",
        env::consts::OS,
    );
    let path = "/".to_string();
    let path = CString::new(path.into_bytes()).unwrap();
    for i in -1..=30 {
        Errno::clear();
        let pc_value = unsafe { libc::pathconf(path.as_ptr(), i) };
        let err = errno();
        let mut err_info = "".to_string();
        if err != 0 {
            err_info = format!("Error reported: {err}");
        }

        println!(
            "\tPathconf results: Index: {:4} value: {:6} {}",
            i, pc_value, err_info,
        );
    }
}
