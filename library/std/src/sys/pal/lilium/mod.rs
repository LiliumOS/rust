use lilium_sys::sys::except::{ExceptionStatusInfo, UnmanagedException};
use lilium_sys::sys::result::SysResult;
use lilium_sys::uuid::{Uuid, parse_uuid};

const ABORTED: Uuid = parse_uuid("466fbae6-be8b-5525-bd04-ee7153b74f55");

pub fn abort_internal() -> ! {
    unsafe {
        UnamangedException(&ExceptionStatusInfo {
            except_code: ABORTED,
            except_info: 0,
            except_reason: 0,
        })
    }
}

pub type RawOsError = SysResult;

pub fn cvt(x: SysResult) -> crate::io::Result<()> {
    if x < 0 { Err(crate::io::Error::from_raw_os_error(x)) } else { Ok(()) }
}
