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
        const search_view = () => h(Searcher);
        return {search_view}
    },
    render ()
    {
        return this.search_view();
    }
})