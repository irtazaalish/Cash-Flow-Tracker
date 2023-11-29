use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, Utc};
use crate::schema::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub amount: f64,
    pub description: Option<String>,
    pub transaction_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "budgets"]
pub struct Budget {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub amount: f64,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub balance: f64,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "recurring_transactions"]
pub struct RecurringTransaction {
    pub id: i32,
    pub user_id: i32,
    pub category_id: i32,
    pub amount: f64,
    pub description: Option<String>,
    pub frequency: Option<chrono::Duration>,
    pub next_transaction_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "audit_log"]
pub struct AuditLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub action: String,
    pub details: Option<String>,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Identifiable)]
#[table_name = "user_preferences"]
#[primary_key(user_id)]
pub struct UserPreference {
    pub user_id: i32,
    pub theme: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "currencies"]
pub struct Currency {
    pub id: i32,
    pub code: String,
    pub name: String,
}
