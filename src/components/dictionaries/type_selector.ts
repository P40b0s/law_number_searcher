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
	'select': (value: Dictionary) => true,
},
	setup (props, emits)
	{
		const {
			select_element,
			is_loading,
			load_options,
			options,
			count
		} = useDictionary(props.placeholder, (d) => emits.emit('select', d));
		watch(()=> props.selected_organ, async (n, o) =>
		{
			if(n != undefined)
			{
				is_loading.value = true;
				let dict = await searcher_commands.get_types(n.id);
				options.value = load_options(dict, []);
				count.value = options.value.length;
				is_loading.value = false;
			}
		})
		return select_element
	},
	render () 
	{
		return h(this)
	}
})

export default comp;