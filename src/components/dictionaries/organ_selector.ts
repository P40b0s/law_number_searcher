import 
{
    h,
    defineComponent,
	onMounted,
  } from 'vue'

import { searcher_commands } from '../../tauri/commands';
import './select_item_content.css';
import { useDictionary } from './useDictionary';

const props =
{
	placeholder:
	{
		type: String ,
		default: ""
	}
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
		const load = async () =>
		{
			is_loading.value = true;
			let p = (await searcher_commands.get_exists_parsers()).get_value();
			let dict = await searcher_commands.get_signatory_authorites();
			options.value = load_options(dict, p);
			count.value = options.value.length;
			is_loading.value = false;
		}
		onMounted(async ()=>
		{
			await load();
		})
		return select_element
	},
	render () 
	{
			return h(this)
	}
})

export default comp;