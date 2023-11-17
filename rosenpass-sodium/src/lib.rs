use libsodium_sys as libsodium;
use std::os::raw::c_void;

macro_rules! sodium_call {
    ($name:ident, $($args:expr),*) => { ::rosenpass_util::attempt!({
        anyhow::ensure!(unsafe{libsodium::$name($($args),*)} > -1,
            "Error in libsodium's {}.", stringify!($name));
        Ok(())
    })};
    ($name:ident) => { sodium_call!($name, ) };
}

#[inline]
pub fn init() -> anyhow::Result<()> {
    log::trace!("initializing libsodium");
    sodium_call!(sodium_init)
}

#[inline]
pub fn memcmp(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len()
        && unsafe {
            let r = libsodium::sodium_memcmp(
                a.as_ptr() as *const c_void,
                b.as_ptr() as *const c_void,
                a.len(),
            );
            r == 0
        }
}

#[inline]
pub fn compare(a: &[u8], b: &[u8]) -> i32 {
    assert!(a.len() == b.len());
    unsafe { libsodium::sodium_compare(a.as_ptr(), b.as_ptr(), a.len()) }
}

#[inline]
pub fn increment(v: &mut [u8]) {
    unsafe {
        libsodium::sodium_increment(v.as_mut_ptr(), v.len());
    }
}