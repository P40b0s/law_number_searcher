use std::{cell::OnceCell, fmt::Display, str::FromStr};
use hashbrown::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::ExtractorError;
use futures::future::{BoxFuture, FutureExt};

pub struct Number
{
    pub number: u32,
    pub postfix: Option<String>,
    //хотел сделать ссылки, но если брать префикс и постфикс из регекса то малыми жертвами ссылки сделать не получится
    //TODO попробовать заменить на Arc
    pub prefix: Option<String>
}


pub trait NumberExtractorPlugin<'a> : 'a where Self: Send + Sync
{
    ///uid органа подписавшего документ
    fn signatory_authority(&self) -> &'static str;
    ///чистый номер без букв и символов, для вычисления очередности
    fn get_raw_number<'b>(&'a self, act_type: &str,  number: &'b str) -> Result<Number, crate::error::ExtractorError>;
    ///поддерживается ли данный номер текущим парсером
    fn number_is_support(&'a self, number: &str) -> bool;
    ///получение списка пропущеных номеров
    fn get_skip_numbers<'b: 'a>(&'a self,  act_type: &str, numbers: Vec<String>) -> Result<Vec<String>, crate::error::ExtractorError>
    {
        if numbers.len() == 0
        {
            return Ok(Vec::<String>::new());
        }
        //logger::debug!("{:?}", &numbers);
        let mut raw_numbers = Vec::with_capacity(numbers.len());
        let mut max = 0;
        let mut min = 1;
        let mut prefix: Option<String> = None;
        let mut postfix: Option<String> = None;
        for n in numbers
        {
            let raw_number = self.get_raw_number(act_type, &n)?;
            if raw_number.prefix.is_some() && prefix.is_none()
            {
                prefix = Some(raw_number.prefix.unwrap_or_default().to_owned());
            }
            if raw_number.postfix.is_some() && postfix.is_none()
            {
                postfix = Some(raw_number.postfix.unwrap_or_default().to_owned());
            }
            max = max.max(raw_number.number);
            min = min.min(raw_number.number);
            raw_numbers.push(raw_number.number);
        }
        raw_numbers.sort();
        let mut skipped = Vec::new();
        for n in min..max
        {
            //если номер не найден добавляем его в список пропущеных
            if let Err(_) = raw_numbers.binary_search(&n)
            {
                let mut formatted_str: Vec<String> = Vec::with_capacity(3);
                if let Some(pr) = prefix.as_ref()
                {
                    formatted_str.push(pr.to_owned());
                }
                formatted_str.push(n.to_string());
                if let Some(po) = postfix.as_ref()
                {
                    formatted_str.push(po.to_owned());
                }
                //logger::debug!("{}", &formatted_str.concat());
                skipped.push(formatted_str.concat());
            }
        }
        Ok(skipped)
    }
}

pub trait OffSiteParser where Self: Send + Sync
{
    ///страница альтернативного сайта опубликования
    fn official_publication_url(&self) -> &'static str;
    ///проверка найденых пропущеных номеров на альтернативном сайте опубликования
    fn check_numbers_on_alternative_site<'a>(&'a self, sa: &'a str, act_type: &'a str, year: u32) 
    -> BoxFuture<'a, Result<Vec<String>, crate::error::ExtractorError>>;
}
