import 
{
    h,
    defineComponent,
    defineAsyncComponent,
    CSSProperties,
    Suspense
  } from 'vue'

import { NButton, NCard, NSpin, NTabPane, NTabs} from 'naive-ui';
import Searcher from '../components/Searcher.vue';
import { searcher_commands } from '../tauri/commands';
//import Loader2 from './Loader/Loader2.vue';

export default defineComponent({
    setup (props) 
    {
    const tab_view = () =>
    {
        return h(NTabs,
            {
                justifyContent: 'space-evenly',
                type: 'line',
                size: 'large',
                animated: true,
                defaultValue: "searcher",
                style:
                {
                      height: '100%'
                }
            },
            {
                default:() => [searcher_tab(), temp_tab()]
            }
        )
    }

    const searcher_tab = () => 
    {
        return h(NTabPane,
            {
                tab: 'Поиск',
                name: 'searcher',
                style:
                {
                    height: '100%'
                }
            },
            {
                default:() => h(Searcher)
            }
        )
    }
    const temp_tab = () => 
        {
            return h(NTabPane,
                {
                    tab: 'Отправители',
                    name: 'temp',
                    style:
                    {
                        height: '100%'
                    }
                },
                {
                    default:() =>
                    h('div', "asdasd asdasd asd asd asd asd ")
                }
            )
        }
    // const settings_tab = () => 
    // {
    //     return h(NTabPane,
    //         {
    //             tab: 'Настройки',
    //             name: 'set',
    //             style:
    //             {
    //                 height: '100%'
    //             }
    //         },
    //         {
    //             default:() => 
    //             h(Suspense, 
    //             null,
    //             {
    //                 default:()=> h(SettingsEditor),
    //                 fallback:() => h(Loader2)
    //             })
    //         }
    //     )
    // }

    return {tab_view}
    },
    render ()
    {
        return this.tab_view();
    }
})