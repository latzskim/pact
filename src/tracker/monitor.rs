use sqlx::{Pool, Sqlite};
use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
    time::Duration,
};
use sysinfo::{ProcessRefreshKind, RefreshKind, System, SystemExt};

use super::{duration::ProcessMonitorTask, MonitorError, MonitorExec};

pub struct MonitorConfig {
    interval: Duration,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        MonitorConfig {
            interval: Duration::from_secs(1),
        }
    }
}

pub struct Monitor {
    cfg: MonitorConfig,
    tasks: Vec<Arc<Mutex<Box<dyn MonitorExec + Send>>>>,
}

impl Monitor {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Monitor {
            cfg: MonitorConfig::default(),
            tasks: vec![Arc::new(Mutex::new(Box::new(ProcessMonitorTask::new(
                pool.clone(),
            ))))],
        }
    }

    pub fn run(&self) -> Result<(), MonitorError> {
        thread::scope(|s| {
            let sleep_interval = self.cfg.interval;

            let process_information_scope =
                RefreshKind::new().with_processes(ProcessRefreshKind::everything());

            let sys = Arc::new(RwLock::new(System::new_with_specifics(
                process_information_scope,
            )));

            // How to avoid Poisoned error? It appears when local_task panics.
            loop {
                sys.try_write()
                    .unwrap()
                    .refresh_all();

                for task in self.tasks.clone() {
                    let local_task = task.clone();
                    let sys_cloned_task = Arc::clone(&sys);

                    s.spawn(move || {
                        let _ = local_task
                            .try_lock()
                            .unwrap()
                            .run(
                                &sys_cloned_task
                                    .read()
                                    .unwrap(),
                            );
                    });
                }

                thread::sleep(sleep_interval);
            }
        });

        Ok(())
    }
}
