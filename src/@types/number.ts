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