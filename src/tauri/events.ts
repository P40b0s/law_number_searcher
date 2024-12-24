import { event} from "@tauri-apps/api"; 
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { AbstractEvents, Plugin, Unlistener } from "./abstract";

/**
 * задаем дженерик в виде литеральных типов, и создаем перечень эвентов
 */
export class TauriEvents extends AbstractEvents<
  'load-process' >
{
    public async load_process(func: (arg: event.Event<number>) => void): Promise<Unlistener>
    {
        return await this.subscribe('load-process', func)
    }
}
const tauri_events = new TauriEvents();
export {tauri_events}