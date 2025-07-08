#[cfg(not(target_arch = "x86_64"))]
use lilium_sys::sys::handle::HandlePtr;
#[cfg(not(target_arch = "x86_64"))]
use lilium_sys::sys::thread::GetThreadPointer;
use lilium_sys::sys::thread::{tls_alloc_dyn, tls_free_dyn, tls_register_destructor};

pub type Key = isize;

pub fn create(dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    let k = unsafe { tls_alloc_dyn(crate::mem::size_of::<*mut u8>()) };

    if k < 0 {
        panic!("Allocating dyn TLS failed");
    }

    let key = -k;

    if let Some(dtor) = dtor {
        unsafe {
            tls_register_destructor(core::mem::transmute(dtor), key);
        }
    }

    key
}

pub unsafe fn destroy(key: Key) {
    unsafe {
        tls_free_dyn(key);
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        #[inline]
        #[cfg(any(not(target_thread_local), test))]
        pub unsafe fn get(key: Key) -> *mut u8 {
            let val: *mut u8;
            unsafe { crate::arch::asm!("mov {dest}, qword ptr fs:[{src}]", dest = lateout(reg) val, src = in(reg) key, options(nostack, readonly, pure, preserves_flags)); }
            val
        }
        pub unsafe fn set(key: Key, val: *mut u8) {
            unsafe { crate::arch::asm!("mov qword ptr fs:[{dest}], {val}", dest = in(reg) key, val = in(reg) val, options(nostack, preserves_flags)); }
        }
    } else {
        #[inline]
        #[cfg(any(not(target_thread_local), test))]
        pub unsafe fn get(key: Key) -> *mut u8 {
            let ptr = unsafe { GetThreadPointer(HandlePtr::null()) };

            unsafe { ptr.wrapping_offset_bytes(key).cast().read()}
        }
        pub unsafe fn set(key: Key, val: *mut u8) {
            let ptr = unsafe { GetThreadPointer(HandlePtr::null()) };

            unsafe { ptr.wrapping_offset_bytes(key).cast().write(val);}
        }
    }
}
