type Dictionary = 
{
    id: string,
    name: string,
    /**
     * `-1` данный документ не поддерживается  
     * `0` используется дефолтный парсер  
     * `1` есть кастомный парсер  
     * 
     */
    parserType: number,
}