use crate::db::NumberDBO;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
///status - `0` неопубликован `1` проверен `2` опубликован на другом сайте
pub struct Number
{
    pub signatory_authority: uuid::Uuid,
    pub type_id: uuid::Uuid,
    pub year: u32,
    pub number: String,
    pub note: Option<String>,
    pub status: i8
}

impl Into<Number> for NumberDBO
{
    fn into(self) -> Number 
    {
        Number
        {
            signatory_authority: self.signatory_authority,
            type_id: self.type_id,
            year: self.year,
            number: self.number,
            note: self.note,
            status: self.status
        }
    }
}

impl Into<NumberDBO> for &mut Number
{
    fn into(self) -> NumberDBO
    {
        NumberDBO
        {
            signatory_authority: self.signatory_authority,
            type_id: self.type_id,
            year: self.year,
            number: self.number.clone(),
            note: self.note.clone(),
            status: self.status
        }
    }
}

impl Into<NumberDBO> for Number
{
    fn into(self) -> NumberDBO
    {
        NumberDBO
        {
            signatory_authority: self.signatory_authority,
            type_id: self.type_id,
            year: self.year,
            number: self.number,
            note: self.note,
            status: self.status
        }
    }
}