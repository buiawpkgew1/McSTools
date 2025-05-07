import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import {toast} from "./others.ts";


export const chuck_update = async() => {
    const update = await check();
    if (update) {

        toast.info(`发现新版本: ${update.version} from ${update.date} with notes ${update.body}`, {
            timeout: 3000
        });
        let downloaded = 0;
        let contentLength = 0;
        await update.downloadAndInstall((event) => {
            switch (event.event) {
                case 'Started':
                    contentLength = event.data.contentLength;
                    console.log(`started downloading ${event.data.contentLength} bytes`);
                    break;
                case 'Progress':
                    downloaded += event.data.chunkLength;
                    console.log(`downloaded ${downloaded} from ${contentLength}`);
                    break;
                case 'Finished':
                    console.log('download finished');
                    break;
            }
        });

        console.log('update installed');
        toast.info(`更新完毕即将重启`, {
            timeout: 3000
        });
        await relaunch();
    }
}
