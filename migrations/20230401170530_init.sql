CREATE TABLE IF NOT EXISTS process_metadata (
    name TEXT,
    status TEXT,
    memory TEXT,
    vmemory TEXT,
    cpu_usage NUMBER,
    disk_read TEXT,
    disk_write TEXT,
    created_at TEXT
);
