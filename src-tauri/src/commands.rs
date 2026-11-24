use crate::date::compute_range;
use crate::db::Db;
use crate::models::{Item, ItemFilter, ItemInput, PageResult, Tag};
use chrono::Local;
use rusqlite::types::Value;
use rusqlite::{params, params_from_iter, Connection, OptionalExtension};
use tauri::State;

fn build_where(f: &ItemFilter) -> (String, Vec<Value>) {
    let mut conds: Vec<String> = Vec::new();
    let mut vals: Vec<Value> = Vec::new();

    match f.status.as_deref() {
        Some("pending") => conds.push("end_date IS NULL".to_string()),
        Some("done") => conds.push("end_date IS NOT NULL".to_string()),
        _ => {}
    }

    if let Some(range) = f.range.as_deref() {
        if range == "custom" {
            if let Some(sd) = f.start_date.as_deref() {
                if !sd.is_empty() {
                    conds.push("start_date >= ?".to_string());
                    vals.push(Value::Text(sd.to_string()));
                }
            }
            if let Some(ed) = f.end_date.as_deref() {
                if !ed.is_empty() {
                    conds.push("start_date <= ?".to_string());
                    vals.push(Value::Text(ed.to_string()));
                }
            }
        } else {
            let r = compute_range(range);
            let s = r.start.format("%Y-%m-%d").to_string();
            let e = r.end.format("%Y-%m-%d").to_string();
            conds.push("start_date <= ?".to_string());
            vals.push(Value::Text(s));
            if range == "today" {
                conds.push("(end_date IS NULL OR end_date = ?)".to_string());
            } else {
                conds.push("(end_date IS NULL OR end_date <= ?)".to_string());
            }
            vals.push(Value::Text(e));
        }
    }

    if let Some(tag_ids) = f.tag_ids.as_deref() {
        if !tag_ids.is_empty() {
            let ph = vec!["?"; tag_ids.len()].join(",");
            conds.push(format!(
                "id IN (SELECT item_id FROM item_tag WHERE tag_id IN ({}))",
                ph
            ));
            for t in tag_ids {
                vals.push(Value::Integer(*t));
            }
        }
    }

    let where_sql = if conds.is_empty() {
        "1=1".to_string()
    } else {
        conds.join(" AND ")
    };
    (where_sql, vals)
}

fn get_tags(conn: &Connection, item_id: i64) -> Result<Vec<Tag>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.name, t.color FROM tag t JOIN item_tag it ON t.id = it.tag_id \
             WHERE it.item_id = ? ORDER BY t.name",
        )
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([item_id], |r| Ok(Tag { id: r.get(0)?, name: r.get(1)?, color: r.get(2)? }))
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

fn get_item_by_id(conn: &Connection, id: i64) -> Result<Item, String> {
    let row: (String, String, String, Option<String>, String, Option<String>) = conn
        .query_row(
            "SELECT code, name, start_date, end_date, detail, jira FROM item WHERE id = ?",
            [id],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            },
        )
        .map_err(|e| e.to_string())?;
    let tags = get_tags(conn, id)?;
    let status = if row.3.is_none() { "pending" } else { "done" }.to_string();
    Ok(Item {
        id,
        code: row.0,
        name: row.1,
        start_date: row.2,
        end_date: row.3,
        detail: row.4,
        jira: row.5,
        tags,
        status,
    })
}

fn generate_code(conn: &Connection) -> Result<String, String> {
    let today = Local::now().format("%Y%m%d").to_string();
    let prefix = format!("item-{}", today);
    let pattern = format!("{}%", prefix);
    let last: Option<String> = conn
        .query_row(
            "SELECT code FROM item WHERE code LIKE ? ORDER BY code DESC LIMIT 1",
            [&pattern],
            |r| r.get(0),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    let next = match last {
        Some(c) => {
            let suffix = c.trim_start_matches(&prefix);
            suffix.parse::<u32>().unwrap_or(0) + 1
        }
        None => 1,
    };
    Ok(format!("{}{:02}", prefix, next))
}

fn set_item_tags(conn: &Connection, item_id: i64, tag_ids: &[i64]) -> Result<(), String> {
    conn.execute("DELETE FROM item_tag WHERE item_id = ?", [item_id])
        .map_err(|e| e.to_string())?;
    for tag_id in tag_ids {
        conn.execute(
            "INSERT OR IGNORE INTO item_tag (item_id, tag_id) VALUES (?, ?)",
            params![item_id, tag_id],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn list_items(state: State<Db>, filter: ItemFilter) -> Result<PageResult, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let (where_sql, vals) = build_where(&filter);

    let count_sql = format!("SELECT COUNT(*) FROM item WHERE {}", where_sql);
    let total: i64 = conn
        .query_row(&count_sql, params_from_iter(vals.iter()), |r| r.get(0))
        .map_err(|e| e.to_string())?;

    let offset = ((filter.page.max(1) - 1) * filter.page_size.max(1)) as i64;
    let limit = filter.page_size.max(1) as i64;
    let list_sql = format!(
        "SELECT id, code, name, start_date, end_date, detail, jira FROM item \
         WHERE {} ORDER BY start_date DESC, id DESC LIMIT ? OFFSET ?",
        where_sql
    );

    let mut list_vals = vals;
    list_vals.push(Value::Integer(limit));
    list_vals.push(Value::Integer(offset));

    let raw: Vec<(i64, String, String, String, Option<String>, String, Option<String>)> = {
        let mut stmt = conn.prepare(&list_sql).map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(params_from_iter(list_vals.iter()), |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                ))
            })
            .map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    let mut items = Vec::new();
    for (id, code, name, start_date, end_date, detail, jira) in raw {
        let tags = get_tags(&conn, id)?;
        let status = if end_date.is_none() { "pending" } else { "done" }.to_string();
        items.push(Item {
            id,
            code,
            name,
            start_date,
            end_date,
            detail,
            jira,
            tags,
            status,
        });
    }

    Ok(PageResult { items, total })
}

#[tauri::command]
pub fn create_item(state: State<Db>, input: ItemInput) -> Result<Item, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let code = generate_code(&conn)?;
    conn.execute(
        "INSERT INTO item (code, name, start_date, end_date, detail, jira) \
         VALUES (?, ?, ?, ?, ?, ?)",
        params![
            code,
            input.name,
            input.start_date,
            input.end_date,
            input.detail,
            input.jira
        ],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    set_item_tags(&conn, id, &input.tags)?;
    tracing::info!("创建事项: {} (id={})", code, id);
    get_item_by_id(&conn, id)
}

#[tauri::command]
pub fn update_item(state: State<Db>, id: i64, input: ItemInput) -> Result<Item, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE item SET name = ?, start_date = ?, end_date = ?, detail = ?, jira = ?, \
         updated_at = datetime('now','localtime') WHERE id = ?",
        params![
            input.name,
            input.start_date,
            input.end_date,
            input.detail,
            input.jira,
            id
        ],
    )
    .map_err(|e| e.to_string())?;
    set_item_tags(&conn, id, &input.tags)?;
    tracing::info!("更新事项: id={}", id);
    get_item_by_id(&conn, id)
}

#[tauri::command]
pub fn delete_item(state: State<Db>, id: i64) -> Result<(), String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM item WHERE id = ?", [id])
        .map_err(|e| e.to_string())?;
    tracing::info!("删除事项: id={}", id);
    Ok(())
}

#[tauri::command]
pub fn toggle_item_done(state: State<Db>, id: i64) -> Result<Item, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let done: bool = conn
        .query_row("SELECT end_date IS NOT NULL FROM item WHERE id = ?", [id], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    let today = Local::now().format("%Y-%m-%d").to_string();
    if done {
        conn.execute(
            "UPDATE item SET end_date = NULL, updated_at = datetime('now','localtime') WHERE id = ?",
            [id],
        )
        .map_err(|e| e.to_string())?;
    } else {
        conn.execute(
            "UPDATE item SET end_date = ?, updated_at = datetime('now','localtime') WHERE id = ?",
            params![today, id],
        )
        .map_err(|e| e.to_string())?;
    }
    tracing::info!("切换完成状态: id={}", id);
    get_item_by_id(&conn, id)
}

#[tauri::command]
pub fn list_tags(state: State<Db>) -> Result<Vec<Tag>, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, color FROM tag ORDER BY name")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |r| Ok(Tag { id: r.get(0)?, name: r.get(1)?, color: r.get(2)? }))
        .map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_tag(state: State<Db>, name: String, color: String) -> Result<Tag, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err("标签名不能为空".to_string());
    }
    // 同名标签已存在则直接返回
    let existing = conn
        .query_row(
            "SELECT id, name, color FROM tag WHERE name = ?",
            [&trimmed],
            |r| Ok(Tag { id: r.get(0)?, name: r.get(1)?, color: r.get(2)? }),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    if let Some(t) = existing {
        return Ok(t);
    }
    conn.execute("INSERT INTO tag (name, color) VALUES (?, ?)", params![trimmed, color])
        .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    tracing::info!("创建标签: {} (id={})", trimmed, id);
    Ok(Tag { id, name: trimmed, color })
}

#[tauri::command]
pub fn update_tag(state: State<Db>, id: i64, name: String, color: String) -> Result<Tag, String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err("标签名不能为空".to_string());
    }
    conn.execute(
        "UPDATE tag SET name = ?, color = ? WHERE id = ?",
        params![trimmed, color, id],
    )
    .map_err(|e| e.to_string())?;
    tracing::info!("更新标签: id={}", id);
    Ok(Tag { id, name: trimmed, color })
}

#[tauri::command]
pub fn export_items(state: State<Db>, path: String, filter: ItemFilter) -> Result<(), String> {
    let conn = state.lock().map_err(|e| e.to_string())?;
    let markdown = build_markdown(&conn, &filter)?;
    std::fs::write(&path, markdown).map_err(|e| e.to_string())?;
    Ok(())
}

fn build_markdown(conn: &Connection, filter: &ItemFilter) -> Result<String, String> {
    let (where_sql, vals) = build_where(filter);
    let sql = format!(
        "SELECT name, start_date FROM item WHERE {} ORDER BY start_date ASC, id ASC",
        where_sql
    );

    let raw: Vec<(String, String)> = {
        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map(params_from_iter(vals.iter()), |r| Ok((r.get(0)?, r.get(1)?)))
            .map_err(|e| e.to_string())?;
        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    let mut out = String::from("# 工作日志\n\n");
    let mut current_date: Option<String> = None;
    let mut idx = 0;
    for (name, start_date) in raw {
        if current_date.as_deref() != Some(start_date.as_str()) {
            if current_date.is_some() {
                out.push('\n');
            }
            out.push_str(&format!("## {}\n", start_date));
            current_date = Some(start_date);
            idx = 1;
        }
        out.push_str(&format!("{}. {}\n", idx, name));
        idx += 1;
    }
    Ok(out)
}
