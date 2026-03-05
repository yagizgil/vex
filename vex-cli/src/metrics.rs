use std::time::{Duration, Instant};

/// PhaseMetrics holds information about how a single step (like Lexing) performed.
pub struct PhaseMetrics {
    pub name: String,         // Name of the step (e.g. "Lexing")
    pub duration: Duration,   // How long it took
    pub memory_used_kb: u64,  // How much RAM it used (diff from start)
}

/// MetricsCollector helps us measure time and memory usage.
pub struct MetricsCollector {
    #[cfg(feature = "performance-stats")]
    system: sysinfo::System, // Used to read system info
    #[cfg(feature = "performance-stats")]
    pid: sysinfo::Pid,       // ID of our current running process
    
    start_time: Option<Instant>,
    #[allow(dead_code)]
    initial_memory: u64,     // RAM usage before we started the phase
}

impl MetricsCollector {
    pub fn new() -> Self {
        #[cfg(feature = "performance-stats")]
        {
            use sysinfo::System;
            let mut system = System::new_all();
            system.refresh_all();
            let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
            
            let initial_memory = system.process(pid)
                .map(|p| p.memory())
                .unwrap_or(0);

            Self {
                system,
                pid,
                start_time: None,
                initial_memory,
            }
        }
        #[cfg(not(feature = "performance-stats"))]
        {
            Self {
                start_time: None,
                initial_memory: 0,
            }
        }
    }

    pub fn start_phase(&mut self) {
        self.start_time = Some(Instant::now());
        
        #[cfg(feature = "performance-stats")]
        {
            use sysinfo::{ProcessesToUpdate, ProcessRefreshKind};
            self.system.refresh_processes_specifics(
                ProcessesToUpdate::Some(&[self.pid]),
                ProcessRefreshKind::new().with_memory(),
            );
        }
    }

    pub fn end_phase(&mut self, name: &str) -> PhaseMetrics {
        let duration = self.start_time.expect("Phase started without timer").elapsed();
        
        #[allow(unused_mut)]
        let mut memory_diff = 0;
        #[cfg(feature = "performance-stats")]
        {
            use sysinfo::{ProcessesToUpdate, ProcessRefreshKind};
            self.system.refresh_processes_specifics(
                ProcessesToUpdate::Some(&[self.pid]),
                ProcessRefreshKind::new().with_memory(),
            );
            
            let current_memory = self.system.process(self.pid)
                .map(|p| p.memory())
                .unwrap_or(0);
            
            memory_diff = current_memory.saturating_sub(self.initial_memory);
        }

        PhaseMetrics {
            name: name.to_string(),
            duration,
            memory_used_kb: memory_diff,
        }
    }

    pub fn print_stats(metrics: &[PhaseMetrics]) {
        use colored::*;
        
        #[cfg(feature = "performance-stats")]
        {
            println!("\n{}", "--- Performance Statistics ---".bold().magenta());
            println!("{:<15} | {:<12} | {:<12}", "Phase", "Duration", "Memory (KB)");
            println!("{}", "-".repeat(45));

            let mut total_duration = Duration::ZERO;
            for m in metrics {
                total_duration += m.duration;
                let dur_str = format!("{:?}", m.duration);
                println!(
                    "{:<15} | {:<12} | {:<12}",
                    m.name.cyan(),
                    dur_str.green(),
                    m.memory_used_kb.to_string().yellow()
                );
            }
            
            println!("{}", "-".repeat(45));
            println!("{:<15} | {:<12}", "Total", format!("{:?}", total_duration).bold().green());
            println!();
        }
        #[cfg(not(feature = "performance-stats"))]
        {
            println!("\n{}", "--- Performance Statistics ---".bold().magenta());
            println!("{:<15} | {:<12}", "Phase", "Duration");
            println!("{}", "-".repeat(30));

            let mut total_duration = Duration::ZERO;
            for m in metrics {
                total_duration += m.duration;
                let dur_str = format!("{:?}", m.duration);
                println!(
                    "{:<15} | {:<12}",
                    m.name.cyan(),
                    dur_str.green()
                );
            }
            
            println!("{}", "-".repeat(30));
            println!("{:<15} | {:<12}", "Total", format!("{:?}", total_duration).bold().green());
            println!("\n{}", "(Note: Compile with --features performance-stats for memory insights)".bright_black());
        }
    }
}
