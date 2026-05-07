use std::borrow::Cow;

use crate::spec::{
    SplitDebuginfo, Target, TargetMetadata, TargetOptions, PanicStrategy,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "ez80".into(),
        metadata: TargetMetadata {
            description: Some("eZ80 target, TI-84+CE support".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 24,
        data_layout: "e-m:z-p:24:8-p1:16:8-p2:8:8-p3:16:8-p4:24:8-i16:8-i24:8-i32:8-i48:8-i64:8-i96:8-f32:8-f64:8-a:8-n8:16:24-S8".into(),
        arch: "ez80".into(),
        options: TargetOptions {
            features: "+24bit-mode".into(),
            code_model: None,
            need_explicit_cpu: false,
            max_atomic_width: None,
            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
            singlethread: true,
            panic_strategy: PanicStrategy::Abort,
            os: "ti".into(),
            vendor: "ti".into(),
            linker: Some("ez80-link".into()),
            disable_redzone: true,
            dynamic_linking: true,
            link_script_exe: Some(include_str!("../../../../../CEdev/meta/linker_script_app.ld").into()),
            link_script_dylib: Some(include_str!("../../../../../CEdev/meta/linker_script.ld").into()),
            generate_arange_section: false,
            ..Default::default()
        },
    }
}
