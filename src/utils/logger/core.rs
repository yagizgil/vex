use std::sync::atomic::{AtomicBool};

pub static REPORT_ENABLED: AtomicBool = AtomicBool::new(false);