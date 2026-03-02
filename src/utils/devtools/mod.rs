#[cfg(feature = "inspector")]
pub mod inspector;

#[macro_export]
macro_rules! inspect {
    ($stage:expr, $tokens:expr, $ast:expr, $($arg:tt)*) => {
        #[cfg(feature = "inspector")]
        {
            let message = format!($($arg)*);
            $crate::utils::devtools::inspector::record($stage, &message, $tokens, $ast);
        }
    };
}


#[macro_export]
macro_rules! inspect_dump {
    ($filename:expr) => {
        #[cfg(feature = "inspector")]
        {
            $crate::utils::devtools::inspector::dump_to_file($filename);
        }
    };
    () => {
        #[cfg(feature = "inspector")]
        {
            $crate::utils::devtools::inspector::dump_to_file("vex_crash_report.json");
        }
    };
}