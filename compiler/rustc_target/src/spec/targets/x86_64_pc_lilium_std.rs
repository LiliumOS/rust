use crate::spec::{Cc, LinkerFlavor, Lld, StackProbeType, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::lilium::opts();
    base.cpu = "x86-64".into();
    base.env = "std".into();
    base.vendor = "pc".into();
    base.plt_by_default = false;
    base.max_atomic_width = Some(64);
    base.stack_probes = StackProbeType::Inline;
    base.static_position_independent_executables = true;

    base.linker_flavor = LinkerFlavor::Gnu(Cc::Yes, Lld::No);
    base.linker = Some("x86_64-lilium-std-cc".into());

    Target {
        llvm_target: "x86_64-pc-lilium-std".into(),
        metadata: TargetMetadata {
            description: Some("64-bit Lilium (x86)".into()),
            tier: None,
            host_tools: Some(false),
            std: Some(false), // For now
        },
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: base,
    }
}
