import { darkTheme, lightTheme} from 'naive-ui';
import { ref } from 'vue';
const height = ref("40");
export const useSelectHeight = () =>
{
    return {height}
}