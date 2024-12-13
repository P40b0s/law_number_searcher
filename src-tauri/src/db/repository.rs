use std::sync::Arc;
use sqlx::SqlitePool;

use crate::Error;

pub struct Repository
{
    connection: Arc<SqlitePool>
}
impl Repository
{
    pub async fn new() -> Result<Self, Error>
    {
        let pool = Arc::new(super::new_connection("numbers").await?);
        let r1 = sqlx::query(create_table()).execute(&*pool).await;
        if r1.is_err()
        {
            logger::error!("{}", r1.as_ref().err().unwrap());
            let _ = r1?;
        };
        Ok(Self
        {
            connection: pool
        })
    }
}
pub trait IRepository
{
    async fn add_number(&self);
}


fn create_table<'a>() -> &'a str
{
    "BEGIN;
    CREATE TABLE IF NOT EXISTS numbers (
    signatory_authority TEXT NOT NULL,
    type_id TEXT NOT NULL,
    year INTEGER NOT NULL,
    number TEXT NOT NULL,
    note TEXT,
    status INTEGER DEFAULT 0
    PRIMARY KEY(signatory_authority, type_id, year, number)
    );
    CREATE INDEX IF NOT EXISTS 'numbers_idx' ON numbers (signatory_authority, type_id, year, number, status);
    COMMIT;"
}

pub struct NumberDBO
{
    signatory_authority: uuid::Uuid,
    type_id: uuid::Uuid,
    year: u32,
    number: String,
    note: Option<String>,
    status: u32
}





impl IRepository for Repository
{
   
    async fn add_number(&self) 
    {
        let connection = Arc::clone(&self.connection);

    }
}

