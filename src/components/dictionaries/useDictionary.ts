import { NButton, NHighlight, NIcon, NSelect, NSkeleton, NTooltip, NProgress, type GlobalThemeOverrides, useThemeVars, type SelectGroupOption, type SelectOption } from "naive-ui";
import { CSSProperties, h, ref, RendererElement, RendererNode, VNode } from "vue";
import { Result } from "../../tauri/abstract";
import { SelectBaseOption, type Value } from "naive-ui/es/select/src/interface";
import { BlockRound, ClearRound, CodeRound, DoneOutlineSharp, EmergencyRound, GradeOutlined, GradeRound, GradeSharp, NearbyErrorRound, PlaylistAddCheckCircleTwotone, RefreshOutlined } from "@vicons/material";
import Loader from '../loaders/Loader1.vue';
import {tauri_events} from '../../tauri/events';
type Node =  VNode<RendererNode, RendererElement, {
    [key: string]: any;
}>
type SelectedValue = Dictionary & (SelectOption | SelectGroupOption);
export const useDictionary = (placeholder: string, update_callback: (dict: Dictionary|null) => void, rescan_callback: () => void) =>
{
    const themeVars = useThemeVars();
    const options = ref<SelectedValue[]>([]);
    const selected = ref<Value|null|undefined>(null);
    const is_loading = ref(false);
    const search_patterns = ref<string[]>();
    const count = ref(0);
    //процесс загрузки
    const process = ref(0);
    const unselect = () =>
    {
        selected.value = null;
        options.value = [];
        count.value = 0;
        process.value = 0;
    };
    const load_process = tauri_events.load_process(async (p) =>
    {
        process.value = p.payload;
    });
    
    const status = ref<'warning'|'success'|'error'>('success');
    const load_options = (dict: Result<Dictionary[]>)  =>
    {
        unselect();
        status.value = 'success';
        if(dict.error)
        {
            status.value = 'error';
            unselect();
        }
        else
        {
            const dictionary = dict.get_value();
            const dm = dictionary.map(o =>
            {
                const org = 
                {
                    label: o.name,
                    value: o.id,
                    id: o.id,
                    name: o.name,
                    key: o.id,
                    parserType: o.parserType,
                    disabled: (o.parserType == -1 || o.parserType == 2)
                } as SelectedValue
                return org;
            });
            if(dm.length == 1)
            {
                if(dm[0].parserType != -1 && dm[0].parserType != 2)
                {
                    selected.value = dm[0].value as string;
                    update_callback(dm[0]);
                }
            }
            else
            {
                //selected.value = null;
                update_callback(null);
            }
            options.value = dm;
            count.value = options.value.length;
        }
    }
    const filter = (pattern: string, option: Object): boolean => 
    {
        const exists = (option as SelectedValue).name.toLowerCase().includes(pattern);
        if(exists)
            count.value++;
        return exists;
    };

    const label = (option: SelectOption): Node =>
    {
        return h('div', 
            {
                style:
                {
                    display: 'flex',
                    flexDirection: 'row',
                    alignItems: 'center',
                    width: '100%',
                    fontSize: '15px',
                    height: '100%'
                } as CSSProperties
            },
            [
                h(NHighlight,
                {
                    text: option.label as string,
                    patterns: search_patterns.value,
                    highlightStyle: 
                    { 
                        fontSize: '16px',
                        borderRadius: themeVars.value.borderRadius,
                        display: 'inline-block',
                        color: themeVars.value.baseColor,
                        background: themeVars.value.primaryColor,
                        transition: `all .3s ${themeVars.value.cubicBezierEaseInOut}` 
                    } as CSSProperties,
                    style:
                    {
                        flexGrow: '2',
                        textWrap: 'wrap',
                    } as CSSProperties

                }),
                h(NTooltip, {
                    placement: 'left'
                },
                {
                    default:() => 
                    {
                        switch(option.parserType)
                        {
                            case -1:
                            {
                                return "Парсер не найден"
                            }
                            case 0:
                            {
                                return "Используется парсер по умолчанию"
                            }
                            case 1:
                            {
                                return "Используется специальный парсер"
                            }
                            case 2:
                            {
                                return "Не найдено ни одного документа"
                            }
                        }
                    },
                    trigger:() => 
                    {
                        switch(option.parserType)
                        {
                            case -1:
                            {
                                return  h(NIcon,
                                    {
                                        color: 'rgb(204,51,51)',
                                        size: '25px',
                                        style:
                                        {
                                            //filter: 'blur(1px)'
                                        }   as CSSProperties
                                    },
                                    {
                                        default: () => h(BlockRound)
                                    })
                            }
                            case 0:
                            {
                                return  h(NIcon,
                                    {
                                        color: 'rgb(115,140,136)',
                                        size: '25px',
                                        style:
                                        {
                                            //filter: 'blur(1px)'
                                        }   as CSSProperties
                                    },
                                    {
                                        default: () => h(GradeRound)
                                    })
                            }
                            case 1:
                            {
                                return  h(NIcon,
                                    {
                                        color: '#78e378',
                                        size: '25px',
                                        style:
                                        {
                                            //filter: 'blur(1px)'
                                        }   as CSSProperties
                                    },
                                    {
                                        default: () => h(GradeRound)
                                    })
                            }
                            case 2:
                            {
                                return  h(NIcon,
                                    {
                                        color: 'rgb(204,51,51)',
                                        size: '25px',
                                        style:
                                        {
                                            //filter: 'blur(1px)'
                                        }   as CSSProperties
                                    },
                                    {
                                        default: () => h(ClearRound)
                                    })
                            }
                        }
                    }
                }),
            ])
    }

    const retry_button = () =>
    {
        return h(NButton, 
            {
                onClick:() =>
                {
                    status.value = 'warning';
                    rescan_callback()
                }
            }, 
            {
                default:() => "Повторить запрос",
                icon:() =>
                    h(NIcon,{color: '#72cc3e', component: RefreshOutlined})
            });
    }

    const empty = () =>
    {
        if(status.value == 'error')
        {
            return  retry_button();
        }
        else
        {
            if(is_loading.value)
            {
                return h('div',
                    {
                        style:
                        {
                            display : 'flex',
                            flexDirection:'column',
                            alignContent: 'center',
                            justifyContent: 'center',
                            alignItems: 'center',
                        } as CSSProperties
                    },
                    [
                        h('div', {style: {fontSize: '16px'}}, "Ожидайте, идет загрузка...."),
                        h(NProgress, 
                        {
                            type: 'line',
                            indicatorPlacement: 'inside',
                            processing: true,
                            percentage: process.value
                        }),
                        h('div', 
                        {
                            style:
                            {
                                padding: '70px 32px'
                            } as CSSProperties
                        }, h(Loader))
                    ]
                )
            }
            else
            {
                return h('div', "Ничего не найдено")
            }
        }
    }
    const themeOverrides: GlobalThemeOverrides = {
    Select: {
        peers: {
        InternalSelection: {
            textColor: '#FF0000'
        },
        InternalSelectMenu: {
            height: '1220',
            paddingLarge: '100'
        },
        }
    },
    }

    const select_element = () => 
    {
        return  h(
                NSelect,
                {
                    value: selected.value,
                    options: options.value,
                    virtualScroll: true,
                    loading: is_loading.value,
                    placeholder: placeholder,
                    filterable: true,
                    status: status.value,
                    size: 'large',
                    filter: filter,
                    renderLabel: label,
                    themeOverrides: themeOverrides,
                    
                    onUpdateValue:(val: string, option: SelectBaseOption|null) =>
                    {
                        let s = options.value.findIndex(i=> i.value == val);
                        //emits.emit('select', options.value[s]);
                        update_callback(options.value[s]);
                        selected.value = (options.value[s].id)
                        search_patterns.value = undefined;
                    },
                    onSearch(value) 
                    {
                        count.value = 0;
                        if(value.length > 0)
                        {
                            search_patterns.value = [value];
                        }
                        else
                        {
                            count.value = options.value.length;
                            search_patterns.value = undefined;
                        }
                    },
                },
                {
                    action:() => h('div', `Количество: ${search_patterns.value ? count.value /2 : count.value}`),
                    empty:() => empty()
                }
        )
    }

    return { select_element, is_loading, load_options, unselect};
}