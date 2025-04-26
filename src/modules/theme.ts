import {ref} from "vue";
import {appStore} from "./store.ts";
import {getBackgroundBase64Url} from "./uploadImage.ts";

export const opacity = ref(1)
export const backgroundOpacity = ref(1);
export const layoutMode = ref('cover');
export const backgroundStr = ref<string | null>(null)

export const initTheme = async () => {
    const bgPath = await appStore.get('backgroundImage', '')
    backgroundOpacity.value = await appStore.get('backgroundOpacity', 0.9);
    layoutMode.value = await appStore.get('layoutMode', 'cover');
    opacity.value = await appStore.get('opacity', 0.8);
    if (bgPath) {
        try {
            backgroundStr.value = await getBackgroundBase64Url(bgPath)
        } catch (error) {
            console.error('背景加载失败:', error)
            backgroundStr.value = 'null'
        }
    }
}