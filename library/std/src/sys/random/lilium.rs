use lilium_sys::sys::random::{GetRandomBytes, RANDOM_DEVICE};

use crate::sync::atomic::{AtomicBool, Ordering};

pub fn fill_bytes(bytes: &mut [u8]) {
    static RAND_DEV_SUPPORTED: AtomicBool = AtomicBool::new(true);
    let len = bytes.len();

    loop {
        if !RAND_DEV_SUPPORTED.load(Ordering::Relaxed) {
            panic!("Default Random Device not functional for some reason. Panicking")
        }
        let res = unsafe { GetRandomBytes(bytes.as_mut_ptr().cast(), len, &RANDOM_DEVICE) };

        if res < 0 {
            if res == -0x101 || res == -0x100 {
                continue;
            } else {
                RAND_DEV_SUPPORTED.store(false, Ordering::Relaxed);
            }
        } else {
            break;
        }
    }
}
