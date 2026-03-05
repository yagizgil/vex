#[cfg(feature = "inspector")]
use std::cell::RefCell;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "inspector", derive(serde::Serialize))]
pub struct TraceNode {
    pub name: String,
    pub args: Option<String>,
    pub result: Option<String>,
    pub children: Vec<TraceNode>,
    pub depth: usize,
}

#[cfg(feature = "inspector")]
thread_local! {
    static TRACE_STACK: RefCell<Vec<TraceNode>> = RefCell::new(Vec::new());
    static COMPLETED_TRACES: RefCell<Vec<TraceNode>> = RefCell::new(Vec::new());
}

#[cfg(feature = "inspector")]
pub fn enter_trace(name: &str, args: Option<String>) {
    TRACE_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        let depth = stack.len();
        stack.push(TraceNode {
            name: name.to_string(),
            args,
            result: None,
            children: Vec::new(),
            depth,
        });
    });
}

#[cfg(feature = "inspector")]
pub fn exit_trace(result: Option<String>) {
    TRACE_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        if let Some(mut node) = stack.pop() {
            node.result = result;
            if let Some(parent) = stack.last_mut() {
                parent.children.push(node);
            } else {
                // This was the root of this trace segment
                COMPLETED_TRACES.with(|completed| {
                    completed.borrow_mut().push(node);
                });
            }
        }
    });
}

#[cfg(feature = "inspector")]
pub fn take_traces() -> Vec<TraceNode> {
    COMPLETED_TRACES.with(|completed| {
        completed.borrow_mut().drain(..).collect()
    })
}

#[cfg(not(feature = "inspector"))]
pub fn enter_trace(_: &str, _: Option<String>) {}
#[cfg(not(feature = "inspector"))]
pub fn exit_trace(_: Option<String>) {}

#[macro_export]
macro_rules! trace_fn {
    ($name:expr) => {
        #[cfg(feature = "inspector")]
        let _trace_guard = {
            $crate::trace::enter_trace($name, None);
            Some($crate::trace::TraceGuard)
        };
        #[cfg(not(feature = "inspector"))]
        let _trace_guard: Option<()> = None;
    };
    ($name:expr, $($arg:tt)*) => {
        #[cfg(feature = "inspector")]
        let _trace_guard = {
            $crate::trace::enter_trace($name, Some(format!($($arg)*)));
            Some($crate::trace::TraceGuard)
        };
        #[cfg(not(feature = "inspector"))]
        let _trace_guard: Option<()> = None;
    };
}

#[cfg(feature = "inspector")]
pub struct TraceGuard;

#[cfg(feature = "inspector")]
impl Drop for TraceGuard {
    fn drop(&mut self) {
        crate::trace::exit_trace(None);
    }
}
