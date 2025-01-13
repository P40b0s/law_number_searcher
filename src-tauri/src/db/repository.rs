use std::sync::Arc;
use sqlx::{encode::IsNull, error::BoxDynError, sqlite::SqliteRow, Any, Database, Decode, Encode, FromRow, Row, SqlitePool, Value};
use utilites::Date;

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
    async fn save_number(&self, number: NumberDBO) -> Result<(), Error>;
    async fn get_number(&self, sa: &str, ty: &str, year: u32, number: &str) -> Result<Option<NumberDBO>, Error>;
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
    status INTEGER DEFAULT 0,
    PRIMARY KEY(signatory_authority, type_id, year, number)
    );
    CREATE INDEX IF NOT EXISTS 'numbers_idx' ON numbers (signatory_authority, type_id, year, number, status);
    COMMIT;"
}

pub struct NumberDBO
{
    pub signatory_authority: uuid::Uuid,
    pub type_id: uuid::Uuid,
    pub year: u32,
    pub number: String,
    pub note: Option<String>,
    pub status: i8
}
impl FromRow<'_, SqliteRow> for NumberDBO
{
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
    {
        let signatory_authority: &str =  row.try_get("signatory_authority")?;
        let type_id: &str = row.try_get("type_id")?;
        let year: u32 = row.try_get("year")?;
        let number: String = row.try_get("number")?;
        let note: Option<String> = row.try_get("note")?;
        let status: i8 = row.try_get("status")?;
        let obj = NumberDBO {
            signatory_authority: uuid::Uuid::parse_str(signatory_authority).unwrap(),
            type_id: uuid::Uuid::parse_str(type_id).unwrap(),
            year,
            number,
            note,
            status
        };
        Ok(obj)
    }
}




impl IRepository for Repository
{
   
    async fn save_number(&self, number: NumberDBO) -> Result<(), Error>
    {
        let connection = Arc::clone(&self.connection);
        let sql = "UPDATE numbers SET note = $1, status = $2 WHERE signatory_authority = $3 AND number = $4 AND year = $5 AND type_id = $6";
        let r = sqlx::query(&sql)
        .bind(number.note.as_ref())
        .bind(number.status)
        .bind(number.signatory_authority.to_string())
        .bind(number.number)
        .bind(number.year)
        .bind(number.type_id.to_string())
        .execute(&*connection).await?;
        Ok(())
    }
    async fn get_number(&self, sa: &str, ty: &str, year: u32, number: &str) -> Result<Option<NumberDBO>, Error>
    {
        let pool = Arc::clone(&self.connection);
        let sql = "SELECT signatory_authority, type_id, year, number, note, status FROM numbers WHERE signatory_authority = $1 AND type_id = $2 AND year = $3 AND number = $4";
        let r = sqlx::query_as::<_, NumberDBO>(&sql)
        .bind(sa)
        .bind(ty)
        .bind(year)
        .bind(number)
        .fetch_optional(&*pool).await?;
        // if r.is_err()
        // {
        //     logger::error!("{}", r.as_ref().err().unwrap());
        //     return Err(Error::SqlxError(r.err().unwrap()));
        // }
        Ok(r)
    }
}

