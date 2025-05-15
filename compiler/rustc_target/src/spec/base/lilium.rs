use std::borrow::Cow;

use crate::spec::{PanicStrategy, RelroLevel, SplitDebuginfo, TargetOptions, TlsModel, cvs};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "lilium".into(),
        dynamic_linking: true,
        families: cvs!["lilium"],
        has_rpath: true,
        position_independent_executables: true,
        relro_level: RelroLevel::Full,
        has_thread_local: true,
        tls_model: TlsModel::InitialExec,
        panic_strategy: PanicStrategy::Abort, // for now
        crt_static_respected: true,
        crt_static_allows_dylibs: true,
        supported_split_debuginfo: Cow::Borrowed(&[
            SplitDebuginfo::Packed,
            SplitDebuginfo::Unpacked,
            SplitDebuginfo::Off,
        ]),
        ..Default::default()
    }
}
