use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameRecord {
    pub id: String,
    pub game_id: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub purchase_date: Option<String>,
    pub start_date: Option<String>,
    pub expire_date: Option<String>,
    pub amount: Option<f64>,
    pub note: Option<String>,
    pub images: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HardwareType {
    pub id: String,
    pub name: String,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hardware {
    pub id: String,
    pub type_id: String,
    pub name: String,
    pub brand: Option<String>,
    pub purchase_date: Option<String>,
    pub purchase_channel: Option<String>,
    pub warranty_months: Option<i32>,
    pub warranty_expire: Option<String>,
    pub price: Option<f64>,
    pub order_no: Option<String>,
    pub note: Option<String>,
    pub images: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumableType {
    pub id: String,
    pub name: String,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consumable {
    pub id: String,
    pub type_id: String,
    pub name: String,
    pub brand: Option<String>,
    pub purchase_date: Option<String>,
    pub expire_date: Option<String>,
    pub usage_duration_days: Option<i32>,
    pub start_use_date: Option<String>,
    pub expected_end_date: Option<String>,
    pub stock_qty: Option<i32>,
    pub single_duration_days: Option<i32>,
    pub max_duration_days: Option<i32>,
    pub note: Option<String>,
    pub images: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedicineCategory {
    pub id: String,
    pub name: String,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Medicine {
    pub id: String,
    pub category_id: String,
    pub name: String,
    pub specification: Option<String>,
    pub purchase_date: Option<String>,
    pub expire_date: Option<String>,
    pub indication: Option<String>,
    pub usage_dosage: Option<String>,
    pub contraindication: Option<String>,
    pub storage_condition: Option<String>,
    pub remaining_qty: Option<String>,
    pub note: Option<String>,
    pub images: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub expired: i64,
    pub expiring_soon: i64,
    pub this_month: i64,
    pub total: i64,
}

pub fn new_id() -> String {
    Uuid::new_v4().to_string()
}
