use sqlx::{Pool, Sqlite};
use sysinfo::{Process, ProcessExt, SystemExt};
use tokio::runtime;

use super::MonitorExec;

pub(super) struct ProcessMonitorTask {
    pool: Pool<Sqlite>,
}

impl MonitorExec for ProcessMonitorTask {
    fn run(&self, sys: &sysinfo::System) -> Result<(), super::MonitorError> {
        let rt = runtime::Runtime::new().unwrap();

        for (_, p) in sys.processes() {
            rt.block_on(self.store_process_metadata(p));
        }

        Ok(())
    }
}

impl ProcessMonitorTask {
    pub(super) fn new(pool: Pool<Sqlite>) -> Self {
        ProcessMonitorTask { pool }
    }

    async fn store_process_metadata(&self, p: &Process) {
        sqlx::query("INSERT INTO process_metadata(name, status, memory, vmemory, cpu_usage, disk_read, disk_write, created_at) 
                    VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);")
                    .bind(p.name())
                    .bind(p.status().to_string()) 
                    .bind(p.memory().to_string()) // sqlite does not support u64... 
                    .bind(p.virtual_memory().to_string())
                    .bind(p.cpu_usage())
                    .bind(p.disk_usage().read_bytes.to_string())
                    .bind(p.disk_usage().written_bytes.to_string())
                    .bind(chrono::Utc::now().to_rfc3339())
                    .execute(&self.pool).await.unwrap();
    }
}
