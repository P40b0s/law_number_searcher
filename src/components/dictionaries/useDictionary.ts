import { NButton, NHighlight, NIcon, NSelect, NSkeleton, NTooltip, useThemeVars, type SelectGroupOption, type SelectOption } from "naive-ui";
import { CSSProperties, h, ref, RendererElement, RendererNode, VNode } from "vue";
import { Result } from "../../tauri/abstract";
import { SelectBaseOption, type Value } from "naive-ui/es/select/src/interface";
import { CodeRound, DoneOutlineSharp, EmergencyRound, GradeOutlined, GradeRound, GradeSharp, PlaylistAddCheckCircleTwotone, RefreshOutlined } from "@vicons/material";
import Loader from '../loaders/Loader1.vue';
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
    const unselect = () =>
    {
        selected.value = null;
        options.value = [];
        count.value = 0;
    };
    
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
                    havingParser: o.havingParser,
                    disabled: !o.havingParser
                } as SelectedValue
                return org;
            });
            if(dm.length == 1)
            {
                selected.value = dm[0].value as string;
                update_callback(dm[0]);
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
                    } as CSSProperties

                }),
                option.havingParser ?
                h(NTooltip, {
                    placement: 'left'
                },
                {
                    default:() => "Парсер найден",
                    trigger:() => 
                    h(NIcon,
                    {
                        color: '#78e378',
                        size: '25px',
                        style:
                        {
                            filter: 'blur(1px)'
                        }   as CSSProperties
                    },
                    {
                        default: () => h(GradeRound)
                    })
                }) : [],
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
                    empty:() => status.value == 'error' ? retry_button() :
                    h('div', 
                    {
                        style:
                        {
                            padding: '70px 32px'
                        } as CSSProperties
                    }, h(Loader))
                }
        )
    }

    return { select_element, is_loading, load_options, unselect};
}