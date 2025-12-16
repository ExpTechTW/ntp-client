use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtpRecord {
    pub id: Option<i64>,
    pub offset: f64,
    pub delay: f64,
    pub server: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedBatch {
    pub records: Vec<NtpRecord>,
}

#[derive(Debug, Clone, Default)]
pub struct QueryFilter {
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub server: Option<String>,
    pub min_offset: Option<f64>,
    pub max_offset: Option<f64>,
    pub min_delay: Option<f64>,
    pub max_delay: Option<f64>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub order_desc: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbStats {
    pub total_records: usize,
    pub earliest_timestamp: Option<i64>,
    pub latest_timestamp: Option<i64>,
    pub servers: Vec<String>,
    pub db_size_bytes: u64,
}

lazy_static::lazy_static! {
    static ref DB_CONNECTION: Mutex<Option<Connection>> = Mutex::new(None);
}

fn get_db_path() -> PathBuf {
    let app_data = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    let app_dir = app_data.join("ntp-client");
    std::fs::create_dir_all(&app_dir).ok();
    app_dir.join("history.db")
}

pub fn init_db() -> SqliteResult<()> {
    let path = get_db_path();
    let conn = Connection::open(&path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ntp_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            offset REAL NOT NULL,
            delay REAL NOT NULL,
            server TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS compressed_batches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            start_time INTEGER NOT NULL,
            end_time INTEGER NOT NULL,
            record_count INTEGER NOT NULL,
            data BLOB NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timestamp ON ntp_records(timestamp)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_server ON ntp_records(server)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_offset ON ntp_records(offset)",
        [],
    )?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_batch_time ON compressed_batches(start_time, end_time)",
        [],
    )?;

    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;

    let mut guard = DB_CONNECTION.lock().unwrap();
    *guard = Some(conn);

    Ok(())
}

fn get_connection() -> SqliteResult<std::sync::MutexGuard<'static, Option<Connection>>> {
    let guard = DB_CONNECTION.lock().unwrap();
    if guard.is_none() {
        drop(guard);
        init_db()?;
        Ok(DB_CONNECTION.lock().unwrap())
    } else {
        Ok(guard)
    }
}

pub fn insert_record(record: &NtpRecord) -> SqliteResult<i64> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    conn.execute(
        "INSERT INTO ntp_records (offset, delay, server, timestamp) VALUES (?1, ?2, ?3, ?4)",
        params![record.offset, record.delay, record.server, record.timestamp],
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn insert_records(records: &[NtpRecord]) -> SqliteResult<usize> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let mut stmt = conn.prepare(
        "INSERT INTO ntp_records (offset, delay, server, timestamp) VALUES (?1, ?2, ?3, ?4)",
    )?;

    let mut count = 0;
    for record in records {
        stmt.execute(params![
            record.offset,
            record.delay,
            record.server,
            record.timestamp
        ])?;
        count += 1;
    }

    Ok(count)
}

pub fn query_records(filter: &QueryFilter) -> SqliteResult<Vec<NtpRecord>> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let mut sql = String::from("SELECT id, offset, delay, server, timestamp FROM ntp_records WHERE 1=1");
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(start) = filter.start_time {
        sql.push_str(" AND timestamp >= ?");
        params_vec.push(Box::new(start));
    }

    if let Some(end) = filter.end_time {
        sql.push_str(" AND timestamp <= ?");
        params_vec.push(Box::new(end));
    }

    if let Some(ref server) = filter.server {
        sql.push_str(" AND server = ?");
        params_vec.push(Box::new(server.clone()));
    }

    if let Some(min) = filter.min_offset {
        sql.push_str(" AND offset >= ?");
        params_vec.push(Box::new(min));
    }

    if let Some(max) = filter.max_offset {
        sql.push_str(" AND offset <= ?");
        params_vec.push(Box::new(max));
    }

    if let Some(min) = filter.min_delay {
        sql.push_str(" AND delay >= ?");
        params_vec.push(Box::new(min));
    }

    if let Some(max) = filter.max_delay {
        sql.push_str(" AND delay <= ?");
        params_vec.push(Box::new(max));
    }

    if filter.order_desc {
        sql.push_str(" ORDER BY timestamp DESC");
    } else {
        sql.push_str(" ORDER BY timestamp ASC");
    }

    if let Some(limit) = filter.limit {
        sql.push_str(&format!(" LIMIT {}", limit));
    }

    if let Some(offset) = filter.offset {
        sql.push_str(&format!(" OFFSET {}", offset));
    }

    let mut stmt = conn.prepare(&sql)?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

    let rows = stmt.query_map(params_refs.as_slice(), |row| {
        Ok(NtpRecord {
            id: Some(row.get(0)?),
            offset: row.get(1)?,
            delay: row.get(2)?,
            server: row.get(3)?,
            timestamp: row.get(4)?,
        })
    })?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row?);
    }

    Ok(records)
}

pub fn query_by_time_range(start: i64, end: i64) -> SqliteResult<Vec<NtpRecord>> {
    query_records(&QueryFilter {
        start_time: Some(start),
        end_time: Some(end),
        ..Default::default()
    })
}

pub fn query_by_server(server: &str, limit: Option<usize>) -> SqliteResult<Vec<NtpRecord>> {
    query_records(&QueryFilter {
        server: Some(server.to_string()),
        limit,
        order_desc: true,
        ..Default::default()
    })
}

pub fn query_recent(count: usize) -> SqliteResult<Vec<NtpRecord>> {
    query_records(&QueryFilter {
        limit: Some(count),
        order_desc: true,
        ..Default::default()
    })
}

pub fn query_outliers(threshold: f64, limit: Option<usize>) -> SqliteResult<Vec<NtpRecord>> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let limit_sql = limit.map(|l| format!(" LIMIT {}", l)).unwrap_or_default();
    let sql = format!(
        "SELECT id, offset, delay, server, timestamp FROM ntp_records
         WHERE ABS(offset) > ?1 ORDER BY timestamp DESC{}",
        limit_sql
    );

    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![threshold], |row| {
        Ok(NtpRecord {
            id: Some(row.get(0)?),
            offset: row.get(1)?,
            delay: row.get(2)?,
            server: row.get(3)?,
            timestamp: row.get(4)?,
        })
    })?;

    let mut records = Vec::new();
    for row in rows {
        records.push(row?);
    }

    Ok(records)
}

pub fn get_db_stats() -> SqliteResult<DbStats> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let total: usize = conn.query_row("SELECT COUNT(*) FROM ntp_records", [], |row| row.get(0))?;

    let earliest: Option<i64> = conn
        .query_row("SELECT MIN(timestamp) FROM ntp_records", [], |row| row.get(0))
        .ok();

    let latest: Option<i64> = conn
        .query_row("SELECT MAX(timestamp) FROM ntp_records", [], |row| row.get(0))
        .ok();

    let mut stmt = conn.prepare("SELECT DISTINCT server FROM ntp_records")?;
    let servers: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    let path = get_db_path();
    let db_size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

    Ok(DbStats {
        total_records: total,
        earliest_timestamp: earliest,
        latest_timestamp: latest,
        servers,
        db_size_bytes: db_size,
    })
}

pub fn archive_old_records(before_timestamp: i64) -> SqliteResult<usize> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let mut stmt = conn.prepare(
        "SELECT id, offset, delay, server, timestamp FROM ntp_records WHERE timestamp < ?1 ORDER BY timestamp",
    )?;
    let rows = stmt.query_map(params![before_timestamp], |row| {
        Ok(NtpRecord {
            id: Some(row.get(0)?),
            offset: row.get(1)?,
            delay: row.get(2)?,
            server: row.get(3)?,
            timestamp: row.get(4)?,
        })
    })?;

    let records: Vec<NtpRecord> = rows.filter_map(|r| r.ok()).collect();
    if records.is_empty() {
        return Ok(0);
    }

    let count = records.len();
    let start_time = records.first().map(|r| r.timestamp).unwrap_or(0);
    let end_time = records.last().map(|r| r.timestamp).unwrap_or(0);

    let batch = CompressedBatch { records };
    let compressed = compress_batch(&batch)?;

    conn.execute(
        "INSERT INTO compressed_batches (start_time, end_time, record_count, data, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            start_time,
            end_time,
            count as i64,
            compressed,
            chrono::Utc::now().timestamp_millis()
        ],
    )?;

    conn.execute(
        "DELETE FROM ntp_records WHERE timestamp < ?1",
        params![before_timestamp],
    )?;

    Ok(count)
}

fn compress_batch(batch: &CompressedBatch) -> SqliteResult<Vec<u8>> {
    let serialized = bincode::serialize(batch).map_err(|e| {
        rusqlite::Error::ToSqlConversionFailure(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )))
    })?;

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&serialized).map_err(|e| {
        rusqlite::Error::ToSqlConversionFailure(Box::new(e))
    })?;

    encoder.finish().map_err(|e| {
        rusqlite::Error::ToSqlConversionFailure(Box::new(e))
    })
}

fn decompress_batch(data: &[u8]) -> SqliteResult<CompressedBatch> {
    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Blob, Box::new(e))
    })?;

    bincode::deserialize(&decompressed).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Blob, Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        )))
    })
}

pub fn query_archived_records(start: i64, end: i64) -> SqliteResult<Vec<NtpRecord>> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let mut stmt = conn.prepare(
        "SELECT data FROM compressed_batches
         WHERE start_time <= ?2 AND end_time >= ?1",
    )?;

    let rows = stmt.query_map(params![start, end], |row| {
        let data: Vec<u8> = row.get(0)?;
        Ok(data)
    })?;

    let mut all_records = Vec::new();
    for row in rows {
        if let Ok(data) = row {
            if let Ok(batch) = decompress_batch(&data) {
                for record in batch.records {
                    if record.timestamp >= start && record.timestamp <= end {
                        all_records.push(record);
                    }
                }
            }
        }
    }

    all_records.sort_by_key(|r| r.timestamp);
    Ok(all_records)
}

pub fn delete_before(timestamp: i64) -> SqliteResult<usize> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let deleted_records: usize = conn.execute(
        "DELETE FROM ntp_records WHERE timestamp < ?1",
        params![timestamp],
    )?;

    conn.execute(
        "DELETE FROM compressed_batches WHERE end_time < ?1",
        params![timestamp],
    )?;

    Ok(deleted_records)
}

pub fn clear_all() -> SqliteResult<()> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    conn.execute("DELETE FROM ntp_records", [])?;
    conn.execute("DELETE FROM compressed_batches", [])?;
    conn.execute("VACUUM", [])?;

    Ok(())
}

pub fn optimize_db() -> SqliteResult<()> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    conn.execute("VACUUM", [])?;
    conn.execute("ANALYZE", [])?;

    Ok(())
}

pub fn aggregate_by_hour(start: i64, end: i64) -> SqliteResult<Vec<(i64, f64, f64, usize)>> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let mut stmt = conn.prepare(
        "SELECT (timestamp / 3600000) * 3600000 as hour,
                AVG(offset) as avg_offset,
                AVG(delay) as avg_delay,
                COUNT(*) as count
         FROM ntp_records
         WHERE timestamp >= ?1 AND timestamp <= ?2
         GROUP BY hour
         ORDER BY hour",
    )?;

    let rows = stmt.query_map(params![start, end], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, usize>(3)?,
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}

pub fn aggregate_by_day(start: i64, end: i64) -> SqliteResult<Vec<(i64, f64, f64, usize)>> {
    let guard = get_connection()?;
    let conn = guard.as_ref().unwrap();

    let mut stmt = conn.prepare(
        "SELECT (timestamp / 86400000) * 86400000 as day,
                AVG(offset) as avg_offset,
                AVG(delay) as avg_delay,
                COUNT(*) as count
         FROM ntp_records
         WHERE timestamp >= ?1 AND timestamp <= ?2
         GROUP BY day
         ORDER BY day",
    )?;

    let rows = stmt.query_map(params![start, end], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, f64>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, usize>(3)?,
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }

    Ok(results)
}

#[tauri::command]
pub async fn db_init() -> Result<(), String> {
    init_db().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_insert_record(offset: f64, delay: f64, server: String, timestamp: i64) -> Result<i64, String> {
    let record = NtpRecord {
        id: None,
        offset,
        delay,
        server,
        timestamp,
    };
    insert_record(&record).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_insert_batch(records: Vec<NtpRecord>) -> Result<usize, String> {
    insert_records(&records).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_query_recent(count: usize) -> Result<Vec<NtpRecord>, String> {
    query_recent(count).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_query_range(start: i64, end: i64) -> Result<Vec<NtpRecord>, String> {
    query_by_time_range(start, end).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_query_by_server(server: String, limit: Option<usize>) -> Result<Vec<NtpRecord>, String> {
    query_by_server(&server, limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_query_outliers(threshold: f64, limit: Option<usize>) -> Result<Vec<NtpRecord>, String> {
    query_outliers(threshold, limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_query(
    start_time: Option<i64>,
    end_time: Option<i64>,
    server: Option<String>,
    min_offset: Option<f64>,
    max_offset: Option<f64>,
    min_delay: Option<f64>,
    max_delay: Option<f64>,
    limit: Option<usize>,
    offset: Option<usize>,
    order_desc: Option<bool>,
) -> Result<Vec<NtpRecord>, String> {
    let filter = QueryFilter {
        start_time,
        end_time,
        server,
        min_offset,
        max_offset,
        min_delay,
        max_delay,
        limit,
        offset,
        order_desc: order_desc.unwrap_or(false),
    };
    query_records(&filter).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_stats() -> Result<DbStats, String> {
    get_db_stats().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_archive(before_timestamp: i64) -> Result<usize, String> {
    archive_old_records(before_timestamp).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_query_archived(start: i64, end: i64) -> Result<Vec<NtpRecord>, String> {
    query_archived_records(start, end).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_delete_before(timestamp: i64) -> Result<usize, String> {
    delete_before(timestamp).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_clear() -> Result<(), String> {
    clear_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_optimize() -> Result<(), String> {
    optimize_db().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_aggregate_hourly(start: i64, end: i64) -> Result<Vec<(i64, f64, f64, usize)>, String> {
    aggregate_by_hour(start, end).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_aggregate_daily(start: i64, end: i64) -> Result<Vec<(i64, f64, f64, usize)>, String> {
    aggregate_by_day(start, end).map_err(|e| e.to_string())
}
