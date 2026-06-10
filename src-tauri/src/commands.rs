use tauri::State;
use tokio::sync::Mutex;
use rusqlite::Connection;

use crate::db;
use crate::models::*;

pub struct AppState {
    pub db: Mutex<Option<Connection>>,
}

#[tauri::command]
pub async fn init_db(state: State<'_, AppState>) -> Result<String, String> {
    let db_path = db::get_db_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    db::init_db(&conn).map_err(|e| e.to_string())?;
    db::seed_default_types(&conn).map_err(|e| e.to_string())?;
    
    let mut db = state.db.lock().await;
    *db = Some(conn);
    
    Ok("数据库初始化成功".to_string())
}

// ===== 游戏 =====

#[tauri::command]
pub async fn add_game(state: State<'_, AppState>, name: String, icon: Option<String>) -> Result<Game, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let id = new_id();
    conn.execute(
        "INSERT INTO games (id, name, icon) VALUES (?1, ?2, ?3)",
        (&id, &name, &icon),
    ).map_err(|e| e.to_string())?;
    
    Ok(Game {
        id,
        name,
        icon,
        created_at: None,
    })
}

#[tauri::command]
pub async fn get_games(state: State<'_, AppState>) -> Result<Vec<Game>, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let mut stmt = conn.prepare("SELECT id, name, icon, created_at FROM games ORDER BY name")
        .map_err(|e| e.to_string())?;
    
    let games = stmt.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            icon: row.get(2)?,
            created_at: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(games)
}

#[tauri::command]
pub async fn add_game_record(
    state: State<'_, AppState>,
    game_id: String,
    record_type: String,
    purchase_date: Option<String>,
    start_date: Option<String>,
    expire_date: Option<String>,
    amount: Option<f64>,
    note: Option<String>,
) -> Result<GameRecord, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let id = new_id();
    conn.execute(
        "INSERT INTO game_records (id, game_id, type, purchase_date, start_date, expire_date, amount, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (&id, &game_id, &record_type, &purchase_date, &start_date, &expire_date, &amount, &note),
    ).map_err(|e| e.to_string())?;
    
    Ok(GameRecord {
        id,
        game_id,
        record_type,
        purchase_date,
        start_date,
        expire_date,
        amount,
        note,
        images: Some("[]".to_string()),
        created_at: None,
    })
}

#[tauri::command]
pub async fn get_game_records(state: State<'_, AppState>, game_id: String) -> Result<Vec<GameRecord>, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let mut stmt = conn.prepare(
        "SELECT id, game_id, type, purchase_date, start_date, expire_date, amount, note, images, created_at FROM game_records WHERE game_id = ?1 ORDER BY expire_date DESC"
    ).map_err(|e| e.to_string())?;
    
    let records = stmt.query_map([&game_id], |row| {
        Ok(GameRecord {
            id: row.get(0)?,
            game_id: row.get(1)?,
            record_type: row.get(2)?,
            purchase_date: row.get(3)?,
            start_date: row.get(4)?,
            expire_date: row.get(5)?,
            amount: row.get(6)?,
            note: row.get(7)?,
            images: row.get(8)?,
            created_at: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(records)
}

// ===== 硬件 =====

#[tauri::command]
pub async fn get_hardware_types(state: State<'_, AppState>) -> Result<Vec<HardwareType>, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let mut stmt = conn.prepare("SELECT id, name, sort_order FROM hardware_types ORDER BY sort_order")
        .map_err(|e| e.to_string())?;
    
    let types = stmt.query_map([], |row| {
        Ok(HardwareType {
            id: row.get(0)?,
            name: row.get(1)?,
            sort_order: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(types)
}

#[tauri::command]
pub async fn add_hardware(
    state: State<'_, AppState>,
    type_id: String,
    name: String,
    brand: Option<String>,
    purchase_date: Option<String>,
    purchase_channel: Option<String>,
    warranty_months: Option<i32>,
    price: Option<f64>,
    order_no: Option<String>,
    note: Option<String>,
) -> Result<Hardware, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let id = new_id();
    let warranty_expire = warranty_months.and_then(|months| {
        purchase_date.as_ref().and_then(|pd| {
            chrono::NaiveDate::parse_from_str(pd, "%Y-%m-%d").ok().map(|d| {
                d + chrono::Duration::days(months as i64 * 30)
            })
        })
    }).map(|d| d.format("%Y-%m-%d").to_string());
    
    conn.execute(
        "INSERT INTO hardwares (id, type_id, name, brand, purchase_date, purchase_channel, warranty_months, warranty_expire, price, order_no, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (&id, &type_id, &name, &brand, &purchase_date, &purchase_channel, &warranty_months, &warranty_expire, &price, &order_no, &note),
    ).map_err(|e| e.to_string())?;
    
    Ok(Hardware {
        id,
        type_id,
        name,
        brand,
        purchase_date,
        purchase_channel,
        warranty_months,
        warranty_expire,
        price,
        order_no,
        note,
        images: Some("[]".to_string()),
        created_at: None,
    })
}

#[tauri::command]
pub async fn get_hardwares(state: State<'_, AppState>) -> Result<Vec<Hardware>, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let mut stmt = conn.prepare(
        "SELECT id, type_id, name, brand, purchase_date, purchase_channel, warranty_months, warranty_expire, price, order_no, note, images, created_at FROM hardwares ORDER BY warranty_expire ASC"
    ).map_err(|e| e.to_string())?;
    
    let hardwares = stmt.query_map([], |row| {
        Ok(Hardware {
            id: row.get(0)?,
            type_id: row.get(1)?,
            name: row.get(2)?,
            brand: row.get(3)?,
            purchase_date: row.get(4)?,
            purchase_channel: row.get(5)?,
            warranty_months: row.get(6)?,
            warranty_expire: row.get(7)?,
            price: row.get(8)?,
            order_no: row.get(9)?,
            note: row.get(10)?,
            images: row.get(11)?,
            created_at: row.get(12)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(hardwares)
}

// ===== 耗材 =====

#[tauri::command]
pub async fn get_consumable_types(state: State<'_, AppState>) -> Result<Vec<ConsumableType>, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let mut stmt = conn.prepare("SELECT id, name, sort_order FROM consumable_types ORDER BY sort_order")
        .map_err(|e| e.to_string())?;
    
    let types = stmt.query_map([], |row| {
        Ok(ConsumableType {
            id: row.get(0)?,
            name: row.get(1)?,
            sort_order: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(types)
}

#[tauri::command]
pub async fn get_consumables(state: State<'_, AppState>) -> Result<Vec<Consumable>, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let mut stmt = conn.prepare(
        "SELECT id, type_id, name, brand, purchase_date, expire_date, usage_duration_days, start_use_date, expected_end_date, stock_qty, single_duration_days, max_duration_days, note, images, created_at FROM consumables ORDER BY expected_end_date ASC"
    ).map_err(|e| e.to_string())?;
    
    let consumables = stmt.query_map([], |row| {
        Ok(Consumable {
            id: row.get(0)?,
            type_id: row.get(1)?,
            name: row.get(2)?,
            brand: row.get(3)?,
            purchase_date: row.get(4)?,
            expire_date: row.get(5)?,
            usage_duration_days: row.get(6)?,
            start_use_date: row.get(7)?,
            expected_end_date: row.get(8)?,
            stock_qty: row.get(9)?,
            single_duration_days: row.get(10)?,
            max_duration_days: row.get(11)?,
            note: row.get(12)?,
            images: row.get(13)?,
            created_at: row.get(14)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(consumables)
}

// ===== 药品 =====

#[tauri::command]
pub async fn get_medicine_categories(state: State<'_, AppState>) -> Result<Vec<MedicineCategory>, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let mut stmt = conn.prepare("SELECT id, name, sort_order FROM medicine_categories ORDER BY sort_order")
        .map_err(|e| e.to_string())?;
    
    let cats = stmt.query_map([], |row| {
        Ok(MedicineCategory {
            id: row.get(0)?,
            name: row.get(1)?,
            sort_order: row.get(2)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(cats)
}

#[tauri::command]
pub async fn get_medicines(state: State<'_, AppState>) -> Result<Vec<Medicine>, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let mut stmt = conn.prepare(
        "SELECT id, category_id, name, specification, purchase_date, expire_date, indication, usage_dosage, contraindication, storage_condition, remaining_qty, note, images, created_at FROM medicines ORDER BY expire_date ASC"
    ).map_err(|e| e.to_string())?;
    
    let medicines = stmt.query_map([], |row| {
        Ok(Medicine {
            id: row.get(0)?,
            category_id: row.get(1)?,
            name: row.get(2)?,
            specification: row.get(3)?,
            purchase_date: row.get(4)?,
            expire_date: row.get(5)?,
            indication: row.get(6)?,
            usage_dosage: row.get(7)?,
            contraindication: row.get(8)?,
            storage_condition: row.get(9)?,
            remaining_qty: row.get(10)?,
            note: row.get(11)?,
            images: row.get(12)?,
            created_at: row.get(13)?,
        })
    }).map_err(|e| e.to_string())?
    .collect::<Result<Vec<_>, _>>()
    .map_err(|e| e.to_string())?;
    
    Ok(medicines)
}

// ===== 仪表盘 =====

#[tauri::command]
pub async fn get_dashboard_stats(state: State<'_, AppState>) -> Result<DashboardStats, String> {
    let db = state.db.lock().await;
    let conn = db.as_ref().ok_or("数据库未初始化")?;
    
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let week_later = (chrono::Local::now() + chrono::Duration::days(7)).format("%Y-%m-%d").to_string();
    let month_end = chrono::Local::now().format("%Y-%m-31").to_string();
    
    // 游戏记录统计
    let expired: i64 = conn.query_row(
        "SELECT COUNT(*) FROM game_records WHERE expire_date < ?1",
        [&today],
        |row| row.get(0),
    ).unwrap_or(0);
    
    let expiring_soon: i64 = conn.query_row(
        "SELECT COUNT(*) FROM game_records WHERE expire_date >= ?1 AND expire_date <= ?2",
        [&today, &week_later],
        |row| row.get(0),
    ).unwrap_or(0);
    
    let this_month: i64 = conn.query_row(
        "SELECT COUNT(*) FROM game_records WHERE expire_date >= ?1 AND expire_date <= ?2",
        [&today, &month_end],
        |row| row.get(0),
    ).unwrap_or(0);
    
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM game_records",
        [],
        |row| row.get(0),
    ).unwrap_or(0);
    
    Ok(DashboardStats {
        expired,
        expiring_soon,
        this_month,
        total,
    })
}
