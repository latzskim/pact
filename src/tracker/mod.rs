mod duration;
mod monitor;

use sysinfo::System;

pub use self::monitor::Monitor;
pub use self::monitor::MonitorConfig;

pub enum MonitorError {}

pub trait MonitorExec {
    fn run(&self, sys: &System) -> Result<(), MonitorError>;
}
