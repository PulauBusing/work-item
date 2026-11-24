use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub type Db = Mutex<Connection>;

const SCHEMA: &str = r#"
PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,
    detail TEXT NOT NULL DEFAULT '',
    jira TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now','localtime')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now','localtime'))
);
CREATE TABLE IF NOT EXISTS tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#409EFF'
);
CREATE TABLE IF NOT EXISTS item_tag (
    item_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (item_id, tag_id),
    FOREIGN KEY (item_id) REFERENCES item(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
);
"#;

pub fn init(app: &AppHandle) -> Result<Db, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    let db_path = data_dir.join("work-item.db");
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute_batch(SCHEMA).map_err(|e| e.to_string())?;
    // 迁移：为已存在的旧 tag 表补充 color 列
    let has_color: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('tag') WHERE name = 'color'",
            [],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    if has_color == 0 {
        conn.execute(
            "ALTER TABLE tag ADD COLUMN color TEXT NOT NULL DEFAULT '#409EFF'",
            [],
        )
        .map_err(|e| e.to_string())?;
    }
    tracing::info!("数据库已初始化: {}", db_path.display());
    Ok(Mutex::new(conn))
}
