import { NButton, NHighlight, NIcon, NSelect, NSkeleton, NTooltip, NProgress, type GlobalThemeOverrides, useThemeVars, type SelectGroupOption, type SelectOption, NTag } from "naive-ui";
import { CSSProperties, h, ref, RendererElement, RendererNode, VNode } from "vue";
import { Result } from "../../tauri/abstract";
import { SelectBaseOption, type Value } from "naive-ui/es/select/src/interface";
import { AccountTreeRound, BlockRound, BuildCircleRound, ClearRound, CodeRound, DoneOutlineSharp, EmergencyRound, ErrorOutlineFilled, ForkRightRound, GradeOutlined, GradeRound, GradeSharp, NearbyErrorRound, PlaylistAddCheckCircleTwotone, RefreshOutlined } from "@vicons/material";
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
    const process_description = ref("");
    const unselect = () =>
    {
        selected.value = null;
        options.value = [];
        count.value = 0;
        process.value = 0;
    };
    const load_process = tauri_events.load_process(async (p) =>
    {
        process.value = p.payload.percent;
        process_description.value = p.payload.description;
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
                    numberExample: o.numberExample,
                    disabled: (o.parserType == -1 || o.parserType == 2),
                    dataType: o.dataType,
                    alternativeSite: o.alternativeSite
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


    const right_select_panel = (option: SelectOption & Dictionary) =>
    {
        const number_example = () =>
        {
            if(option.dataType == 'type' && option.numberExample)
            {
                return h(NTooltip,
                    {
                        placement: 'left'
                    },
                    {
                        trigger:() => h(NTag , {type: 'info'}, {default:()=> option.numberExample}),
                        default:() => "Шаблон номера для текущего вида документа",
                    })
            }
        }

        const parser_type = (type: number): [string, Node] =>
        {
            switch(type)
            {
                default:
                case -1:
                {
                    return ["Формат номера не поддерживается", 
                            h(NIcon,
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
                    ]
                }
                case 2:
                {
                    return ["Не найдено ни одного документа",
                            h(NIcon,
                                {
                                    color: 'rgb(204,51,51)',
                                    size: '25px',
                                    style:
                                    {
                                        //filter: 'blur(1px)'
                                    }   as CSSProperties
                                },
                                {
                                    default: () => h(ErrorOutlineFilled)
                                })
                    ]
                }
            }
        }
        // const parser_type_for_types = (type: number): [string, Node] =>
        // {
        //     switch(type)
        //     {
        //         default:
        //         case -1:
        //         {
        //             return ["Формат номера не поддерживается", 
        //                     h(NIcon,
        //                         {
        //                             color: 'rgb(204,51,51)',
        //                             size: '25px',
        //                             style:
        //                             {
        //                                 //filter: 'blur(1px)'
        //                             }   as CSSProperties
        //                         },
        //                         {
        //                             default: () => h(BlockRound)
        //                         })
        //             ]
        //         }
        //         case 1:
        //         {
        //             return ["Используется специальный парсер",
        //                     h(NIcon,
        //                         {
        //                             color: '#78e378',
        //                             size: '25px',
        //                             style:
        //                             {
        //                                 //filter: 'blur(1px)'
        //                             }   as CSSProperties
        //                         },
        //                         {
        //                             default: () => h(BuildCircleRound)
        //                         })
        //             ]
        //         }
        //         case 2:
        //         {
        //             return ["Не найдено ни одного документа",
        //                     h(NIcon,
        //                         {
        //                             color: 'rgb(204,51,51)',
        //                             size: '25px',
        //                             style:
        //                             {
        //                                 //filter: 'blur(1px)'
        //                             }   as CSSProperties
        //                         },
        //                         {
        //                             default: () => h(ErrorOutlineFilled)
        //                         })
        //             ]
        //         }
        //     }
        // }

        const number_parser = () =>
        {
            if(option.dataType == 'organ' && option.parserType == 1)
            {
                return h(NTooltip, 
                    {
                        placement: 'left'
                    },
                    {
                        default:() => "Используется специальный парсер для номера",
                        trigger:() =>  h(NIcon,
                            {
                                color: '#78e378',
                                size: '25px',
                                style:
                                {
                                    //filter: 'blur(1px)'
                                }   as CSSProperties
                            },
                            {
                                default: () => h(BuildCircleRound)
                            })
                    })
            }
            if(option.dataType == 'type' && option.parserType != 0 && option.parserType != 1)
            {
                const ptype = parser_type(option.parserType);
                return h(NTooltip, 
                    {
                        placement: 'left'
                    },
                    {
                        default:() => ptype[0],
                        trigger:() => ptype[1]
                    })
            }
        }

        const site_parser = () =>
        {
            if(option.dataType == 'organ' && option.alternativeSite)
            {
                return h(NTooltip, 
                    {
                        placement: 'left'
                    },
                    {
                        default:() => "Есть парсер для альтернативного сайта опубликования " + option.alternativeSite,
                        trigger:() =>  h(NIcon,
                            {
                                color: '#78e378',
                                size: '25px',
                                style:
                                {
                                }   as CSSProperties
                            },
                            {
                                default: () => h(ForkRightRound)
                            })
                    })
            }
        }
        const panel =  [
                number_example(),
                number_parser(),
                site_parser()  
        ]
        return panel.filter(f=>f != undefined);
    }

    const label = (option: SelectOption & Dictionary): Node =>
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
                
            ].concat(right_select_panel(option)))
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
                        h('div', {style: {fontSize: '16px'}}, "Ожидайте, идет загрузка. " + process_description.value),
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