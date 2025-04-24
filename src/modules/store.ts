import { LazyStore } from '@tauri-apps/plugin-store';

class AppStore {
    private store: LazyStore;

    constructor() {
        this.store = new LazyStore('settings.json');
    }

    async get<T>(key: string, defaultValue: T): Promise<T> {
        const value = await this.store.get(key);
        console.log(value)
        return value !== undefined ? (value as T) : defaultValue;
    }

    async set<T>(key: string, value: T): Promise<void> {
        await this.store.set(key, value);
        await this.store.save();
    }
}

export const appStore = new AppStore();