use cfg_if::cfg_if;

mod prototypes;

cfg_if! {
    if #[cfg(feature = "macos_dynlink")] {
        mod dynlink;
        pub(super) use dynlink::*;
    } else {
        mod externlink;
        pub(super) use externlink::*;
    }
}
