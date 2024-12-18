import { NHighlight, NIcon, NSelect, NTooltip, useThemeVars, type SelectGroupOption, type SelectOption } from "naive-ui";
import { CSSProperties, h, ref, RendererElement, RendererNode, VNode } from "vue";
import { Result } from "../../tauri/abstract";
import { SelectBaseOption, type Value } from "naive-ui/es/select/src/interface";
import { EmergencyRound } from "@vicons/material";

type Node =  VNode<RendererNode, RendererElement, {
    [key: string]: any;
}>
type SelectedValue = Dictionary & (SelectOption | SelectGroupOption) & {parser: boolean};
export const useDictionary = (placeholder: string, update_callback: (dict: Dictionary) => void) =>
{
    const themeVars = useThemeVars();
    const options = ref<SelectedValue[]>([]);
    const selected = ref<Value|null|undefined>(null);
    const is_loading = ref(false);
    const search_patterns = ref<string[]>();
    const count = ref(0);

    const load_options = (dict: Result<Dictionary[]>, parsers: string[]): SelectedValue[]  =>
    {
        if(dict.error)
            return [];
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
                    parser: parsers.some(s=>s == o.id)
                } as SelectedValue
                return org;
            });
            if(dm.length == 1)
            {
                selected.value = dm[0].value as string;
                update_callback(dm[0]);
            }
            else
                selected.value = null;
                
            return dm;
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
                option.parser ?
                h(NTooltip, {
                    placement: 'bottom'
                },
                {
                    default:() => "Для данного вида документа есть парсер",
                    trigger:() => 
                    h(NIcon,
                    {
                        color: '#78e378',
                        size: '30px',
                    },
                    {
                        default: () => h(EmergencyRound)
                    })
                }) : [],
            ])
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
                }
        )
    }

    return { select_element, is_loading, load_options, options, count };
}