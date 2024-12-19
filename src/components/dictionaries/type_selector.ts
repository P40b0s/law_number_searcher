import 
{
    h,
    defineComponent,
    PropType,
	watch,
  } from 'vue'
import { searcher_commands } from '../../tauri/commands';
import './select_item_content.css';
import { useDictionary } from './useDictionary';
const props = 
{
    selected_organ: 
    {
        type: Object as PropType<Dictionary>,
    },
	placeholder:
	{
		type: String ,
		default: ""
	}
	// options:
	// {
	// 	type: Function as PropType<()=> Promise<Result<Dictionary[]>>>,
	// 	default: () => ({}),
	// }
} as const

const comp =  defineComponent({
props: props,
emits:
{
	'select': (value: Dictionary|null) => true,
},
	setup (props, emits)
	{
		const {
			select_element,
			is_loading,
			load_options,
			unselect
		} = useDictionary(props.placeholder, (d) => emits.emit('select', d), async () => await load(props.selected_organ));
		const load = async (organ: Dictionary| undefined) =>
		{
			if(organ != undefined)
			{
				is_loading.value = true;
				unselect();
				let dict = await searcher_commands.get_types(organ.id);
				load_options(dict);
				is_loading.value = false;
			}
		}
		watch(()=> props.selected_organ, async (n, o) =>
		{
			await load(n);
		})

		return select_element
	},
	render () 
	{
		return h(this)
	}
})

export default comp;