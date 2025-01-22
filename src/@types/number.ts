export type Number = 
{
    signatory_authority: string,
    type_id: string,
    year: number,
    number: string,
    note?: string,
    /**
     * `0` -  просто пропущенный номер  
     * `1` -  пропущенный номер, есть в базе данных, есть отметка (например оператор оставил запись `note`)  
     * `2` -  документ опубликован в альтернативном источнике опубликования
     */
    status: number
}
/**
 * Тип для экспорта в эксель
 */
export type ExportNumbers = 
{
    organ_name: string,
    type_name: string,
    alternative_site?: string,
    numbers: Number[]
}




export const new_number = (sa: string, ty: string, year: number, number: string, status: number, note?: string) =>
{
    return {
        signatory_authority: sa,
        type_id: ty,
        year,
        number,
        status,
        note
    } as Number
}

export const test_numbers = () =>
{
    return [
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "1-ФЗ", 1),
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "2-ФЗ", 0),
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "3-ФЗ", 1),
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "4-ФЗ", 2, "sdfsdf sdfsd"),
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "5-ФЗ", 0),
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "666-ФЗ", 0),
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "99999-ФЗ", 1),
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "1123-ФЗ", 0),
        new_number("1111-111-111-111-11", "2222-222-222-222-22", 2024, "321123-ФЗ", 2, ",kfksdkf ksdfksdkf ksdfkskdfk ksdfk"),
    ]
}