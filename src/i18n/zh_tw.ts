export default {
    common: {
        save: '儲存',
        cancel: '取消',
        delete: '刪除',
        edit: '編輯',
        create: '建立',
        search: '搜尋',
        loading: '載入中...',
        success: '成功',
        error: '錯誤',
        confirm: '確認',
        back: '返回'
    },
    menu: {
        home: '首頁',
        settings: '設定',
        about: '關於'
    },
    about: {
        title: '關於',
        version: '版本: v{version}',
        actions: {
            checkUpdate: '檢查更新',
            changelog: '更新日誌',
            github: 'Github',
            website: '官方網站',
            sponsor: '贊助專案',
            faq: '常見問題→'
        },
        description: {
            title: '軟體說明',
            content: '軟體採用tauri後端基於rust，前端使用vue。\n分離化設計讓軟體效能得到保障，rust安全設計，效能更好，記憶體安全高效。'
        },
        schematicSite: {
            title: '藍圖站',
            description: '支援多種藍圖的網站，提供私有和公開多種模式，可以在線預覽藍圖\n提供線上的藍圖轉換功能，材料統計，可將網站中藍圖快速匯入到本地',
            visit: '前往網站→'
        },
        tauri: {
            title: 'Tauri 2.0',
            tooltip: '軟體基於tauri 2.0開發製作',
            currentVersion: '目前版本: {version}'
        },
        license: {
            title: 'GNU Affero General Public License',
            tooltip: '允許修改和分發，但必須開源修改後的程式碼並保留版權宣告，禁止未經授權商業使用',
            copyright: '© 2025 MCS Tools. 所有權利依據 AGPL-3.0 授權',
            viewLicense: '檢視完整協議'
        },
        developers: {
            title: '核心開發人員',
            tooltip: '參與開發及代表明確遵守AGPL V3協議,改版 轉發請註明所有開發人員和協議',
            author: '作者'
        }
    },
    home: {
        title: '藍圖工具箱',
        stats: {
            localSchematics: '本地藍圖總數',
            cloudSchematics: '雲端藍圖總數',
            welcome: '歡迎回來'
        },
        upload: {
            title: '藍圖處理',
            dragDrop: '拖放檔案或點選上傳',
            supportedFormats: '支援格式：nbt、litematic、schem、 json、 mcstruct（最大50MB）允許多選',
            selectFile: '選擇檔案',
            uploadSuccess: '成功上傳 {count} 個檔案',
            uploadError: '發生錯誤:{error}'
        },
        supportedTypes: {
            title: '支援藍圖類型',
            vanilla: {
                title: '香草結構',
                desc: '我的世界原版支援的藍圖格式，機械動力也採用了這種格式'
            },
            buildingGadgets: {
                title: '建築小幫手',
                desc: '科技包最常見的輔助建築工具'
            },
            litematica: {
                title: '建築投影',
                desc: '生電玩家必備工具'
            },
            worldEdit: {
                title: '創世神',
                desc: '經典建築工具，沿用至今，新版axiom也採用了這種藍圖格式'
            },
            bedrock: {
                title: 'MC BE',
                desc: '我的世界BE採用的藍圖格式，目前未完全適配'
            }
        }
    },
    settings: {
        darkMode: '深色模式',
        lightMode: '淺色模式',
        title: '設定',
        update: {
            title: '更新設定',
            autoUpdate: '啟用自動更新',
            source: '更新源設定',
            sourcePlaceholder: '選擇或輸入更新源',
            sourceNoData: '輸入有效的HTTP地址'
        },
        debug: {
            title: '偵錯模式',
            enable: '啟用偵錯模式',
            open: '開啟偵錯',
            openDev: '開啟開發者模式'
        },
        theme: {
            title: '跟隨主題',
            autoTheme: '啟用系統跟隨(頁面主題將跟隨windows主題變化)'
        },
        resources: {
            title: '資源檔案',
            clear: '清除資源檔案(將刪除所有資源檔案，你儲存的藍圖)',
            clearConfirm: '確認清除',
            clearWarning: '清除將導致資料全部遺失，建議先進行備份',
            openFolder: '開啟資源資料夾',
            openFolderBtn: '開啟目錄'
        },
        language: {
            title: '語言設定',
            select: '語言選擇'
        }
    },
    messages: {
        clearSuccess: '已清除資源檔案，將在5秒後重新啟動',
        error: '發生錯誤: {error}',
        fetchError: '取得原理圖失敗: {error}'
    },
    tools: {
        title: '藍圖編輯',
        upload: '上傳藍圖',
        noSchematic: '未選取目標藍圖',
        schematicId: '藍圖ID: {id}',
        tabs: {
            schematic: '藍圖詳情',
            history: '版本管理',
            split: '藍圖分割',
            replace: '方塊替換',
            convert: '藍圖轉換',
            data: '源資料檢視',
            stats: '材料統計',
            threeD: '結構預覽'
        },
        convert: {
            title: '藍圖轉換',
            tip: '大型藍圖的轉換耗時可能過長請耐心等待',
            oneClickConvert: '一鍵轉換',
            convertToFormat: '轉換到該格式',
            confirmStart: '確認開始',
            confirmExport: '確認匯出',
            cancel: '取消',
            targetVersion: '目標輸出版本',
            waitingTip: '大型藍圖轉換需要一定時間等待',
            noParams: '無轉換參數，大型藍圖轉換需要一定時間等待',
            alreadyExists: '已存在',
            formats: {
                vanilla: {
                    title: '香草結構藍圖',
                    desc: '適配 Minecraft 原版結構方塊格式',
                    ext: 'nbt'
                },
                litematica: {
                    title: '投影藍圖',
                    desc: '適配 我的世界建築投影藍圖格式',
                    ext: 'litematic'
                },
                worldEdit: {
                    title: '創世神',
                    desc: '適配於新版1.16 + 創世神模組和最新版 axios',
                    ext: 'schem',
                    versions: {
                        latest: '0: WE最新格式',
                        legacy: '1: WE 1.16-'
                    }
                },
                buildingGadgets: {
                    title: '建築小幫手',
                    desc: '適配於1.12 + 建築小幫手 三種變種格式藍圖',
                    ext: 'json',
                    versions: {
                        latest: '0: 小幫手最新格式',
                        modern: '1: 小幫手1.16+',
                        legacy: '2: 小幫手1.12+'
                    }
                },
                bedrock: {
                    title: 'MC BE',
                    desc: '適配於1.18 + 我的世界BE原版 結構方塊格式',
                    ext: 'mcstructure'
                }
            },
            meta: {
                extension: '副檔名類型',
                originalSize: '原始大小',
                version: '版本',
                subVersion: '子版本',
                exists: '已存在',
                gzipCompression: 'Gzip壓縮',
                hasSubVersions: '存在子版本'
            }
        }
    },
    schematics: {
        title: '藍圖倉庫',
        local: '本地藍圖',
        web: '網路藍圖',
        upload: '上傳藍圖',
        source: '站點來源',
        sites: {
            mcs: 'MCS:www.mcschematic.top',
            cms: 'CMS:www.creativemechanicserver.com'
        }
    },
    report: {
        title: '問題回報',
        subtitle: '回報管道',
        tip: '有問題先不要盲目亂求醫。先嘗試自己解決一下！',
        channels: {
            github: {
                title: 'GitHub issue',
                desc: '透過Github issue向我們回報bug和問題'
            },
            qqGroup: {
                title: 'QQ群聊',
                desc: '加入官方Q群回報問題'
            },
            qqChannel: {
                title: 'QQ頻道',
                desc: '加入官方QQ頻道回報問題'
            }
        },
        placeholder: '還沒有，這只是個佔位符'
    },
    others: {
        title: '工具箱',
        tabs: {
            mapArt: '地圖畫',
            redstoneMusic: '紅石音樂'
        }
    },
    individuation: {
        title: '個人化設定',
        opacity: {
            title: '不透明度',
            value: '{value}%'
        },
        theme: {
            title: '主題配色',
            options: {
                grey: '預設灰白',
                blue: '蔚藍主題',
                darkBlue: '深藍之夜',
                green: '清新綠意',
                orange: '活力橘',
                yellow: '鳳梨黃',
                brown: '橡木棕',
                greyDark: '暗色模式'
            }
        },
        background: {
            title: '背景設定',
            imageInfo: '圖片資訊',
            fileName: '檔案名稱',
            fileSize: '檔案大小',
            resolution: '解析度',
            layoutMode: '佈局方式',
            layoutModes: {
                stretch: '拉伸填充',
                repeat: '平鋪重複',
                contain: '適應螢幕',
                cover: '完整覆蓋'
            },
            actions: {
                clear: '清除背景',
                refresh: '重新整理背景',
                select: '選擇背景檔案'
            }
        }
    }
}