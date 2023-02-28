use nix::errno::{errno, Errno};
use std::ffi::CString;

fn main() {
    let path = "/".to_string();
    let path = CString::new(path.into_bytes()).unwrap();
    for i in -1..=30 {
        Errno::clear();
        let pc_value = unsafe { libc::pathconf(path.as_ptr(), i) };

        println!("Hello, world! {} {} {}", i, pc_value, errno());
    }
}
