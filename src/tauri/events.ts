import { event} from "@tauri-apps/api"; 
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { AbstractEvents, Plugin, Unlistener } from "./abstract";

/**
 * задаем дженерик в виде литеральных типов, и создаем перечень эвентов
 */
// export class TauriEvents extends AbstractEvents<
//   'packets_update' 
// | 'error' 
// | 'task_updated' 
// | 'task_deleted' 
// | 'clean_start' 
// | 'clean_complete'
// | 'need_packets_refresh'
// | 'sender_update'>
// {
//     public async packets_update(func: (arg: event.Event<IPacket>) => void): Promise<Unlistener>
//     {
//         return await this.subscribe('packets_update', func)
//     }
//     public async error(func: (arg: event.Event<string>) => void): Promise<Unlistener>
//     {
//         return await this.subscribe('error', func)
//     }
//     public async task_updated(func: (arg: event.Event<Task>) => void): Promise<Unlistener>
//     {
//         return await this.subscribe('task_updated', func)
//     }
//     public async task_deleted(func: (arg: event.Event<string>) => void): Promise<Unlistener>
//     {
//         return await this.subscribe('task_deleted', func)
//     }
//     public async clean_start(func: (arg: event.Event<void>) => void): Promise<Unlistener>
//     {
//         return await this.subscribe('clean_start', func)
//     }
//     public async clean_complete(func: (arg: event.Event<number>) => void): Promise<Unlistener>
//     {
//         return await this.subscribe('clean_complete', func)
//     }
//     public async need_packets_refresh(func: (arg: event.Event<void>) => void): Promise<Unlistener>
//     {
//         return await this.subscribe('need_packets_refresh', func)
//     }
//     public async sender_update(func: (arg: event.Event<Senders>) => void): Promise<Unlistener>
//     {
//         return await this.subscribe('sender_update', func)
//     }
    
// }
// const events = new TauriEvents();
// export {events}