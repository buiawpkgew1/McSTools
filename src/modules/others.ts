import { useToast } from "vue-toastification";
export const toast = useToast();

export const getBlockIcon = (blockId: string) => {
    const block = blockId.split(':');
    return new URL(`../assets/icon/icon-exports-x32/${block[0]}__${block[1]}.png`, import.meta.url).href
};