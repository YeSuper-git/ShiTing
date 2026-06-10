use rusqlite::{Connection, Result};
use std::path::PathBuf;
use dirs::data_dir;

pub fn get_db_path() -> PathBuf {
    let mut path = data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("com.shiting.app");
    std::fs::create_dir_all(&path).ok();
    path.push("shiting.db");
    path
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        CREATE TABLE IF NOT EXISTS games (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            icon TEXT,
            created_at TEXT DEFAULT (datetime('now'))
        );
        
        CREATE TABLE IF NOT EXISTS game_records (
            id TEXT PRIMARY KEY,
            game_id TEXT NOT NULL,
            type TEXT NOT NULL,
            purchase_date TEXT,
            start_date TEXT,
            expire_date TEXT,
            amount REAL,
            note TEXT,
            images TEXT DEFAULT '[]',
            created_at TEXT DEFAULT (datetime('now')),
            FOREIGN KEY (game_id) REFERENCES games(id)
        );
        
        CREATE TABLE IF NOT EXISTS hardware_types (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            sort_order INTEGER DEFAULT 0
        );
        
        CREATE TABLE IF NOT EXISTS hardwares (
            id TEXT PRIMARY KEY,
            type_id TEXT NOT NULL,
            name TEXT NOT NULL,
            brand TEXT,
            purchase_date TEXT,
            purchase_channel TEXT,
            warranty_months INTEGER,
            warranty_expire TEXT,
            price REAL,
            order_no TEXT,
            note TEXT,
            images TEXT DEFAULT '[]',
            created_at TEXT DEFAULT (datetime('now')),
            FOREIGN KEY (type_id) REFERENCES hardware_types(id)
        );
        
        CREATE TABLE IF NOT EXISTS consumable_types (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            sort_order INTEGER DEFAULT 0
        );
        
        CREATE TABLE IF NOT EXISTS consumables (
            id TEXT PRIMARY KEY,
            type_id TEXT NOT NULL,
            name TEXT NOT NULL,
            brand TEXT,
            purchase_date TEXT,
            expire_date TEXT,
            usage_duration_days INTEGER,
            start_use_date TEXT,
            expected_end_date TEXT,
            stock_qty INTEGER DEFAULT 0,
            single_duration_days INTEGER,
            max_duration_days INTEGER,
            note TEXT,
            images TEXT DEFAULT '[]',
            created_at TEXT DEFAULT (datetime('now')),
            FOREIGN KEY (type_id) REFERENCES consumable_types(id)
        );
        
        CREATE TABLE IF NOT EXISTS medicine_categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            sort_order INTEGER DEFAULT 0
        );
        
        CREATE TABLE IF NOT EXISTS medicines (
            id TEXT PRIMARY KEY,
            category_id TEXT NOT NULL,
            name TEXT NOT NULL,
            specification TEXT,
            purchase_date TEXT,
            expire_date TEXT,
            indication TEXT,
            usage_dosage TEXT,
            contraindication TEXT,
            storage_condition TEXT,
            remaining_qty TEXT,
            note TEXT,
            images TEXT DEFAULT '[]',
            created_at TEXT DEFAULT (datetime('now')),
            FOREIGN KEY (category_id) REFERENCES medicine_categories(id)
        );
    ")?;
    
    Ok(())
}

pub fn seed_default_types(conn: &Connection) -> Result<()> {
    // 硬件类型
    let hw_types = ["显卡", "显示器", "主板", "CPU", "内存", "硬盘", "电源", "机箱", "外设", "其他"];
    for (i, name) in hw_types.iter().enumerate() {
        conn.execute(
            "INSERT OR IGNORE INTO hardware_types (id, name, sort_order) VALUES (?1, ?2, ?3)",
            (format!("hw-type-{}", i), name, i as i32),
        )?;
    }
    
    // 耗材类型
    let con_types = ["净水滤芯", "空气滤芯", "电池", "灯泡", "清洁用品", "其他"];
    for (i, name) in con_types.iter().enumerate() {
        conn.execute(
            "INSERT OR IGNORE INTO consumable_types (id, name, sort_order) VALUES (?1, ?2, ?3)",
            (format!("con-type-{}", i), name, i as i32),
        )?;
    }
    
    // 药品分类
    let med_cats = ["感冒发烧", "肠胃消化", "外伤消毒", "慢性病", "维生素保健", "儿童用药", "其他"];
    for (i, name) in med_cats.iter().enumerate() {
        conn.execute(
            "INSERT OR IGNORE INTO medicine_categories (id, name, sort_order) VALUES (?1, ?2, ?3)",
            (format!("med-cat-{}", i), name, i as i32),
        )?;
    }
    
    Ok(())
}
