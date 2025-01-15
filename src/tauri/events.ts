import { event} from "@tauri-apps/api"; 
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { AbstractEvents, Plugin, Unlistener } from "./abstract";

type ProcessEventType = 
{
  percent: number,
  description: string
}
/**
 * задаем дженерик в виде литеральных типов, и создаем перечень эвентов
 */
export class TauriEvents extends AbstractEvents<
    'load-process'
  | 'load-base-process' >
{
  public async load_process(func: (arg: event.Event<ProcessEventType>) => void): Promise<Unlistener>
  {
    return await this.subscribe('load-process', func)
  }
  public async load_base_process(func: (arg: event.Event<ProcessEventType>) => void): Promise<Unlistener>
  {
    return await this.subscribe('load-base-process', func)
  }
}
const tauri_events = new TauriEvents();
export {tauri_events}