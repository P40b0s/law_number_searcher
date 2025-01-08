
import { Plugin, Result } from "./abstract";
import { type ExportNumbers, type Number } from "../@types/number";

class Searcher extends Plugin<
'get_signatory_authorites' 
| 'get_exists_parsers' 
| 'get_types' 
| 'get_exists_numbers' 
| 'get_lost_numbers'
| 'get_alternative_publ_site'
| 'save_number'
| 'check_alternative_publ_info'
| 'export_to_excel'> implements ICommand
{
    plugin = "plugin:searcher|";
    public async get_signatory_authorites<R extends Dictionary[]>(): Promise<Result<R>>
    {
        const r = await this.get<R>('get_signatory_authorites');
        return r;
    }
    public async get_types<R extends Dictionary[]>(sa_id: string): Promise<Result<R>>
    {
        const r = await this.get<R>('get_types', {payload: sa_id});
        return r;
    }
    public async get_exists_parsers<R extends string[]>(): Promise<Result<R>>
    {
        const r = await this.get<R>('get_exists_parsers');
        return r;
    }
    public async get_exists_numbers<R extends string[]>(signatory_authority: string, act_type: string, year: number): Promise<Result<R>>
    {
        const r = await this.get<R>('get_exists_numbers', {existsNumbersRequest: {signatory_authority, act_type, year}});
        return r;
        
    }
    public async get_lost_numbers<R extends Number[]>(signatory_authority: string, act_type: string, year: number): Promise<Result<R>>
    {
        const r = await this.get<R>('get_lost_numbers', {existsNumbersRequest: {signatory_authority, act_type, year}});
        return r;
        
    }
    public async get_alternative_publ_site<R extends string|undefined>(signatory_authority: string): Promise<Result<R>>
    {
        const r = await this.get<R>('get_alternative_publ_site', {payload: signatory_authority});
        return r;
        
    }
    public async save_number<R extends void>(number: Number): Promise<Result<R>>
    {
        const r = await this.post<Number, R>('save_number', number);
        return r;
    }
    public async check_alternative_publ_info<R extends Number[]>(numbers: Number[]): Promise<Result<R>>
    {
        const r = await this.post<Number[], R>('check_alternative_publ_info', numbers);
        return r;
    }

    public async export_to_excel<R extends void>(numbers: ExportNumbers): Promise<Result<R>>
    {
        const r = await this.post<ExportNumbers, R>('export_to_excel', numbers);
        return r;
        
    }
}


interface ICommand
{
    get_signatory_authorites<R extends Dictionary[]>(): Promise<Result<R>>;
    get_types<R extends Dictionary[]>(sa_id: string): Promise<Result<R>>;
    get_exists_parsers<R extends string[]>(): Promise<Result<R>>;
    get_exists_numbers<R extends string[]>(signatory_authority: string, act_type: string, year: number): Promise<Result<R>>;
    get_lost_numbers<R extends Number[]>(signatory_authority: string, act_type: string, year: number): Promise<Result<R>>;
    get_alternative_publ_site<R extends string>(signatory_authority: string): Promise<Result<R>>;
    save_number<R extends void>(number: Number): Promise<Result<R>>;
    check_alternative_publ_info<R extends Number[]>(numbers: Number[]): Promise<Result<R>>;
    export_to_excel<R extends void>(numbers: ExportNumbers): Promise<Result<R>>;
   
}


// class Service extends Plugin<'clear_dirs' | 'ws_server_online' | 'rescan_packet' | 'delete_packet'>
// {
//     plugin = "plugin:service|";
  
//     public async clean_dirs<R extends void>(): Promise<Result<R>>
//     {
//         return await this.get<R>('clear_dirs');
//     }
//     public async ws_server_online<R extends boolean>(): Promise<Result<R>>
//     {
//         return await this.get<R>('ws_server_online');
//     }
//     public async rescan_packet<R extends IPacket>(packet: R): Promise<Result<void>>
//     {
//         return await this.post('rescan_packet', packet);
//     }
//     public async delete_packet<R extends IPacket>(packet: R): Promise<Result<void>>
//     {
//         return await this.post('delete_packet', packet);
//     }
// }

// class Packets extends Plugin<
//    'get_packets_list'
//  | 'get_count' 
//  | 'search_packets' 
//  | 'get_files_list' 
//  | 'get_pdf_pages_count' 
//  | 'get_pdf_page' 
//  | 'get_file_body'
//  | 'get_senders'
//  | 'update_sender'>
// {
//     plugin = "plugin:packets|";
//     public async get_packets_list(limit: number, offset: number): Promise<Result<IPacket[]>>
//     {
//         return await this.get<IPacket[]>('get_packets_list', {pagination: {row: limit, offset: offset}});
//     }
//     public async search_packets(search_string: string): Promise<Result<IPacket[]>>
//     {
//         return await this.get<IPacket[]>('search_packets', {payload: search_string});
//     }
//     public async get_count(): Promise<Result<number>>
//     {
//         return await this.get<number>('get_count');
//     }
//     public async get_files_list(fr: FilesRequest): Promise<Result<File[]>>
//     {
//         return await this.get<File[]>('get_files_list', {filesRequest: {dir_name: fr.dir_name, task_name: fr.task_name}});
//     }
//     public async get_pdf_pages_count(fr: FileRequest): Promise<Result<number>>
//     {
//         return await this.get<number>('get_pdf_pages_count', {fileRequest: { file: { file_name: fr.file.file_name, file_type: fr.file.file_type, path: fr.file.path }} as FileRequest});
//     }
//     public async get_pdf_page<T extends string>(fr: FileRequest): Promise<Result<T>>
//     {
//         return await this.get<T>('get_pdf_page', {fileRequest: { file: { file_name: fr.file.file_name, file_type: fr.file.file_type, path: fr.file.path }, page_number: fr.page_number} as FileRequest});
//     }
//     public async get_file_body<T extends string>(fr: FileRequest): Promise<Result<T>>
//     {
//         return await this.get<T>('get_file_body', {fileRequest: { file: { file_name: fr.file.file_name, file_type: fr.file.file_type, path: fr.file.path }} as FileRequest});
//     }
//     public async get_senders<T extends Senders[]>(): Promise<Result<T>>
//     {
//         return await this.get<T>('get_senders');
//     }
//     public async update_sender<T extends Senders>(senders: T): Promise<Result<void>>
//     {
//         console.log("update", senders)
//         //{senders: {id: senders.id, organization: senders.organization, medo_addresse: senders.medo_addresse, contact_info: senders.contact_info, icon: senders.icon } as Senders}
//         return await this.post('update_sender', senders);
//     }
    
    
// }
// const commands_service = new Service();
// const commands_settings = new Settings();
// const commands_packets = new Packets();
// export {commands_settings, commands_service, commands_packets}
const searcher_commands: ICommand = new Searcher();
export {searcher_commands};