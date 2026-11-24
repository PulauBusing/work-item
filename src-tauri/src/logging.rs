use tauri::{AppHandle, Manager};

pub fn init(app: &AppHandle) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let log_dir = data_dir.join("logs");
    std::fs::create_dir_all(&log_dir).map_err(|e| e.to_string())?;
    let file_appender = tracing_appender::rolling::daily(&log_dir, "work-item.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(false)
        .init();
    // 保持后台写入线程存活到进程结束
    std::mem::forget(guard);
    Ok(())
}
