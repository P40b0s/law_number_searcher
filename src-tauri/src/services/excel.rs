use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Url, Workbook, XlsxError};
use crate::types::Number;



fn merge_format(color: u32, align: FormatAlign) -> Format
{
    Format::new()
        .set_border(FormatBorder::Thin)
        .set_bold()
        .set_background_color(Color::RGB(color))
        .set_font_size(18)
        .set_text_wrap()
        .set_align(align)
}
const NOT_PUBLIC_COLOR: u32 = 0xe56c6c;
const PUBLIC_COLOR: u32 = 0x7de255;
const CHECKED_COLOR: u32 = 0x979d95;
const WHITE_COLOR: u32 = 0xffffff;

pub fn export(organ_name: &str, type_name: &str, off_site: Option<&str>, numbers: &[Number]) -> Result<(),  XlsxError>
{
    let mut workbook = Workbook::new();

    let col_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_bold()
        .set_background_color(Color::RGB(0x88e378))
        .set_font_size(16)
        .set_align(FormatAlign::Center);

    let not_public_format = Format::new()
        .set_background_color(Color::RGB(NOT_PUBLIC_COLOR))
        .set_border_bottom(FormatBorder::Thin)
        .set_font_size(14)
        .set_text_wrap()
        .set_border_right(FormatBorder::Thin)
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center);

    let checked_format = Format::new()
        .set_background_color(Color::RGB(CHECKED_COLOR))
        .set_border_bottom(FormatBorder::Thin)
        .set_text_wrap()
        .set_font_size(14)
        .set_align(FormatAlign::Center)
        .set_border_right(FormatBorder::Thin)
        .set_align(FormatAlign::VerticalCenter);
    
    let public_format = Format::new()
        .set_background_color(Color::RGB(PUBLIC_COLOR))
        .set_font_size(14)
        .set_text_wrap()
        .set_border_right(FormatBorder::Thin)
        .set_border_bottom(FormatBorder::Thin)
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center);


    let worksheet = workbook.add_worksheet();
    // установка ширины столбцов
    worksheet.set_column_width(0, 40)?;
    worksheet.set_column_width(1, 30)?;
    worksheet.set_column_width(2, 10)?;
    worksheet.set_column_width(3, 15)?;
    worksheet.set_column_width(4, 40)?;
    worksheet.set_column_width(5, 30)?;
    
    let mut last_row = 0;
    let mut not_public_count = 0;
    let mut public_count = 0;
    let mut checked_count = 0;
    //первая строка
    worksheet.merge_range(last_row, 0, last_row, 5, organ_name, &merge_format(WHITE_COLOR, FormatAlign::Center))?;
    last_row += 1;
    worksheet.merge_range(last_row, 0, last_row, 5, type_name, &merge_format(WHITE_COLOR, FormatAlign::Center))?;
    last_row += 1;
    if let Some(s) = off_site
    {
        worksheet.merge_range(last_row, 0, last_row, 5, "", &merge_format(WHITE_COLOR, FormatAlign::Center))?;
        worksheet.write_with_format(last_row, 0, Url::new(s).set_text("Переход на региональный сайт опубликования >>>" ), &merge_format(WHITE_COLOR, FormatAlign::Center))?;
        last_row += 1;
    }
    worksheet.write_with_format(last_row, 0, "Орган", &col_format)?;
    worksheet.write_with_format(last_row, 1, "Вид документа", &col_format)?;
    worksheet.write_with_format(last_row, 2, "Год", &col_format)?;
    worksheet.write_with_format(last_row, 3, "Номер", &col_format)?;
    worksheet.write_with_format(last_row, 4, "Статус", &col_format)?;
    worksheet.write_with_format(last_row, 5, "Комментарий", &col_format)?;
    last_row += 1;
    let mut year: Option<u32> = None;
    for n in numbers
    {
        if year.is_none()
        {
            year = Some(n.year);
        }
        let note = match n.note.as_ref()
        {
            Some(nt) => [" \n(", nt, ")"].concat(),
            None => "".to_owned()
        };
        match n.status
        {
            0 => 
            {
                worksheet.write_with_format(last_row, 0, organ_name, &not_public_format)?;
                worksheet.write_with_format(last_row, 1, type_name, &not_public_format)?;
                worksheet.write_with_format(last_row, 2, n.year, &not_public_format)?;
                worksheet.write_with_format(last_row, 3, &n.number, &not_public_format)?;
                worksheet.write_with_format(last_row, 4, "не опубликован на pravo.gov.ru", &not_public_format)?;
                worksheet.write_with_format(last_row, 5, note, &not_public_format)?;
                not_public_count +=1;
            },
            1 => 
            {
                worksheet.write_with_format(last_row, 0, organ_name, &checked_format)?;
                worksheet.write_with_format(last_row, 1, type_name, &checked_format)?;
                worksheet.write_with_format(last_row, 2, n.year, &checked_format)?;
                worksheet.write_with_format(last_row, 3, &n.number, &checked_format)?;
                worksheet.write_with_format(last_row, 4, "не опубликован на pravo.gov.ru, проверен", &checked_format)?;
                worksheet.write_with_format(last_row, 5, note, &checked_format)?;
                checked_count +=1;
            },
            2 => 
            {
                worksheet.write_with_format(last_row, 0, organ_name, &public_format)?;
                worksheet.write_with_format(last_row, 1, type_name, &public_format)?;
                worksheet.write_with_format(last_row, 2, n.year, &public_format)?;
                worksheet.write_with_format(last_row, 3, &n.number, &public_format)?;
                worksheet.write_with_format(last_row, 4, "опубликован на региональном сайте", &public_format)?;
                worksheet.write_with_format(last_row, 5, note, &public_format)?;
                public_count +=1;
            },
            _ => ()
        }
        last_row += 1;
    }
    worksheet.merge_range(last_row, 0, last_row, 5, "", &merge_format(WHITE_COLOR, FormatAlign::Left))?;
    last_row += 1;
    worksheet.merge_range(last_row, 0, last_row, 5, &["Не опубликовано на pravo.gov.ru: ".to_owned(), not_public_count.to_string()].concat(), &merge_format(NOT_PUBLIC_COLOR, FormatAlign::Left))?;
    last_row += 1;
    worksheet.merge_range(last_row, 0, last_row, 5, &["Опубликовано на региональном сайте: ".to_owned(), public_count.to_string()].concat(), &merge_format(PUBLIC_COLOR, FormatAlign::Left))?;
    last_row += 1;
    worksheet.merge_range(last_row, 0, last_row, 5, &["Не опубликовано на pravo.gov.ru, проверено: ".to_owned(), checked_count.to_string()].concat(), &merge_format(CHECKED_COLOR, FormatAlign::Left))?;
    last_row += 1;
    worksheet.merge_range(last_row, 0, last_row, 5, &["Всего: ".to_owned(), numbers.len().to_string()].concat(), &merge_format(WHITE_COLOR, FormatAlign::Left))?;
    let _nme = worksheet.set_name(year.as_ref().unwrap().to_string());
    //let date_now = utilites::Date::now().format(utilites::DateFormat::DotDate);
    let name = [organ_name.to_owned(), " - ".to_owned() , type_name.to_owned(), " (".to_owned(), year.as_ref().unwrap().to_string(), ")".to_owned()].concat();
    let _ = std::fs::create_dir("reports");
    workbook.save(["reports/".to_owned(), name, ".xlsx".to_owned()].concat())?;

    Ok(())
}
#[cfg(test)]
mod tests
{
    use crate::types::Number;

    #[test]
    fn test_export()
    {
        let _ = logger::StructLogger::new_default();
        let organ_name = "Республика Бурятия";
        let type_name = "Закон";
        let off_site = "https://egov-buryatia.ru/npa_template";
        let numbers = vec![
            Number
            {
                signatory_authority: uuid::Uuid::default(),
                type_id: uuid::Uuid::default(),
                year: 2025,
                number: "1-БУР".to_owned(),
                note: None,
                status: 0
            },
            Number
            {
                signatory_authority: uuid::Uuid::default(),
                type_id: uuid::Uuid::default(),
                year: 2025,
                number: "2-БУР".to_owned(),
                note: Some("опубликовывался".to_owned()),
                status: 1
            },
            Number
            {
                signatory_authority: uuid::Uuid::default(),
                type_id: uuid::Uuid::default(),
                year: 2025,
                number: "3-БУР".to_owned(),
                note: None,
                status: 2
            },
            Number
            {
                signatory_authority: uuid::Uuid::default(),
                type_id: uuid::Uuid::default(),
                year: 2025,
                number: "4-БУР".to_owned(),
                note: None,
                status: 1
            }
        ];
        let tt = super::export(organ_name, type_name, Some(off_site), &numbers);
        logger::debug!("{:?}", tt);
    }
}