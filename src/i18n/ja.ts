export default {
  common: {
    save: '保存',
    cancel: 'キャンセル',
    delete: '削除',
    edit: '編集',
    create: '作成',
    search: '検索',
    loading: '読み込み中...',
    success: '成功',
    error: 'エラー',
    confirm: '確認',
    back: '戻る'
  },
  menu: {
    home: 'ホーム',
    settings: '設定',
    about: '概要'
  },
  home: {
    title: 'シェマティックツールボックス',
    stats: {
      localSchematics: 'ローカルシェマティック',
      cloudSchematics: 'クラウドシェマティック',
      welcome: 'おかえりなさい'
    },
    upload: {
      title: 'シェマティック処理',
      dragDrop: 'ファイルをドラッグ＆ドロップするか、クリックしてアップロード',
      supportedFormats: '対応フォーマット：nbt、litematic、schem、json、mcstruct（最大50MB）複数ファイル可',
      selectFile: 'ファイルを選択',
      uploadSuccess: '{count}個のファイルをアップロードしました',
      uploadError: 'エラーが発生しました：{error}'
    },
    supportedTypes: {
      title: '対応シェマティックタイプ',
      vanilla: {
        title: 'バニラ構造',
        desc: 'Minecraftのバニラでサポートされているシェマティック形式、Create modでも使用'
      },
      buildingGadgets: {
        title: 'Building Gadgets',
        desc: 'テックモッドパックで最も一般的な建築補助ツール'
      },
      litematica: {
        title: 'Litematica',
        desc: 'テクニカルMinecraftプレイヤーにとって必須のツール'
      },
      worldEdit: {
        title: 'WorldEdit',
        desc: 'クラシックな建築ツール、現在も使用され、Axiomなどの新しいツールでも採用'
      },
      bedrock: {
        title: 'MC BE',
        desc: 'Minecraft Bedrock Editionのシェマティック形式、現在完全には対応していません'
      }
    }
  },
  settings: {
    darkMode: 'ダークモード',
    lightMode: 'ライトモード',
    title: '設定',
    update: {
      title: '更新設定',
      autoUpdate: '自動更新を有効にする',
      source: '更新ソース設定',
      sourcePlaceholder: '更新ソースを選択または入力',
      sourceNoData: '有効なHTTPアドレスを入力してください'
    },
    debug: {
      title: 'デバッグモード',
      enable: 'デバッグモードを有効にする',
      open: 'デバッグを開く',
      openDev: 'デバッグDEVを開く'
    },
    theme: {
      title: 'テーマ追従',
      autoTheme: 'システムテーマ追従を有効にする（ページテーマがWindowsテーマの変更に追従します）'
    },
    resources: {
      title: 'リソースファイル',
      clear: 'リソースファイルをクリア（すべてのリソースファイルと保存されたブループリントが削除されます）',
      clearConfirm: 'クリアを確認',
      clearWarning: 'クリアするとすべてのデータが失われます。先にバックアップしてください',
      openFolder: 'リソースフォルダを開く',
      openFolderBtn: 'ディレクトリを開く'
    },
    language: {
      title: '言語設定',
      select: '言語選択'
    }
  },
  messages: {
    clearSuccess: 'リソースファイルをクリアしました。5秒後に再起動します',
    error: 'エラーが発生しました: {error}',
    fetchError: 'シェマティックの取得に失敗しました: {error}'
  },
  tools: {
    title: 'シェマティックエディタ',
    upload: 'シェマティックをアップロード',
    noSchematic: 'シェマティックが選択されていません',
    schematicId: 'シェマティックID: {id}',
    tabs: {
      schematic: 'シェマティック詳細',
      history: 'バージョン管理',
      split: 'シェマティック分割',
      replace: 'ブロック置換',
      convert: 'シェマティック変換',
      data: 'ソースデータ表示',
      stats: '材料統計',
      threeD: '3Dプレビュー'
    },
    convert: {
      title: 'シェマティック変換',
      tip: '大きなシェマティックの変換には時間がかかる場合があります、お待ちください',
      oneClickConvert: 'ワンクリック変換',
      convertToFormat: 'フォーマットに変換',
      confirmStart: '開始確認',
      confirmExport: 'エクスポート確認',
      cancel: 'キャンセル',
      targetVersion: '出力バージョン',
      waitingTip: '大きなシェマティックの変換には待ち時間が必要です',
      noParams: '変換パラメータなし、大きなシェマティックの変換には待ち時間が必要です',
      alreadyExists: '既に存在',
      formats: {
        vanilla: {
          title: 'バニラ構造',
          desc: 'Minecraft バニラ構造ブロック形式に対応',
          ext: 'nbt'
        },
        litematica: {
          title: 'Litematica',
          desc: 'Minecraft Litematica シェマティック形式に対応',
          ext: 'litematic'
        },
        worldEdit: {
          title: 'WorldEdit',
          desc: 'WorldEdit mod 1.16+ と最新の axios に対応',
          ext: 'schem',
          versions: {
            latest: '0: 最新WE形式',
            legacy: '1: WE 1.16-'
          }
        },
        buildingGadgets: {
          title: 'Building Gadgets',
          desc: 'Building Gadgets 1.12+ の3つのバリアント形式に対応',
          ext: 'json',
          versions: {
            latest: '0: 最新BG形式',
            modern: '1: BG 1.16+',
            legacy: '2: BG 1.12+'
          }
        },
        bedrock: {
          title: 'MC BE',
          desc: 'Minecraft Bedrock Edition 1.18+ 構造ブロック形式に対応',
          ext: 'mcstructure'
        }
      },
      meta: {
        extension: '拡張子タイプ',
        originalSize: '元のサイズ',
        version: 'バージョン',
        subVersion: 'サブバージョン',
        exists: '存在',
        gzipCompression: 'Gzip圧縮',
        hasSubVersions: 'サブバージョンあり'
      }
    }
  },
  schematics: {
    title: 'シェマティックリポジトリ',
    local: 'ローカルシェマティック',
    web: 'ウェブシェマティック',
    upload: 'シェマティックをアップロード',
    source: 'ソースサイト',
    sites: {
      mcs: 'MCS:www.mcschematic.top',
      cms: 'CMS:www.creativemechanicserver.com'
    }
  },
  report: {
    title: '問題報告',
    subtitle: 'フィードバックチャネル',
    tip: '問題が発生したら、まず自分で解決を試みてください！',
    channels: {
      github: {
        title: 'GitHub issue',
        desc: 'GitHub issueを通じてバグや問題を報告'
      },
      qqGroup: {
        title: 'QQグループ',
        desc: '公式QQグループに参加してバグや問題を報告'
      },
      qqChannel: {
        title: 'QQチャンネル',
        desc: '公式QQチャンネルに参加してバグや問題を報告'
      }
    },
    placeholder: 'まだ利用できません、これはプレースホルダーです'
  },
  others: {
    title: 'ツールボックス',
    tabs: {
      mapArt: 'マップアート',
      redstoneMusic: 'レッドストーンミュージック'
    }
  },
  individuation: {
    title: 'パーソナライズ設定',
    opacity: {
      title: '不透明度',
      value: '{value}%'
    },
    theme: {
      title: 'テーマカラー',
      options: {
        grey: 'デフォルトグレー',
        blue: 'アジュールテーマ',
        darkBlue: 'ダークブルーナイト',
        green: 'フレッシュグリーン',
        orange: 'ビビッドオレンジ',
        yellow: 'パイナップルイエロー',
        brown: 'オークブラウン',
        greyDark: 'ダークモード'
      }
    },
    background: {
      title: '背景設定',
      imageInfo: '画像情報',
      fileName: 'ファイル名',
      fileSize: 'ファイルサイズ',
      resolution: '解像度',
      layoutMode: 'レイアウトモード',
      layoutModes: {
        stretch: '引き伸ばし',
        repeat: 'タイル繰り返し',
        contain: '画面に合わせる',
        cover: '全体カバー'
      },
      actions: {
        clear: '背景をクリア',
        refresh: '背景を更新',
        select: '背景ファイルを選択'
      }
    },
    font: {
      title: 'フォント設定',
      fontInfo: 'フォント情報',
      fileName: 'ファイル名',
      fileSize: 'ファイルサイズ',
      actions: {
        clear: 'フォントをクリア',
        refresh: 'フォントを更新',
        select: 'フォントファイルを選択'
      },
      effect: {
        title: 'フォントプレビュー',
        content1: '日本語フォントデモ',
        content2: '太字効果：日本語フォントデモ',
        content3: 'The quick brown fox jumps over the lazy dog.',
        content4: 'Italic style shows elegance in typography.',
        content5: '標準：0123456789',
        content6: '特殊スタイル：① 𝟙𝟚𝟛₄₅₆ ⓺⓻⓼⓽',
      }
    }
  },
  about: {
    title: '概要',
    version: 'バージョン: v{version}',
    actions: {
      checkUpdate: 'アップデート確認',
      changelog: '変更履歴',
      github: 'Github',
      website: '公式サイト',
      sponsor: 'プロジェクト支援',
      faq: 'FAQ→'
    },
    description: {
      title: 'ソフトウェア説明',
      content: 'ソフトウェアはRustベースのTauriバックエンドとVueフロントエンドを使用しています。\nモジュール設計によりソフトウェアのパフォーマンスを確保し、Rustの安全性設計により優れたパフォーマンスとメモリ安全性を提供します。'
    },
    schematicSite: {
      title: 'シェマティックサイト',
      description: '複数のシェマティック形式をサポートするウェブサイトで、プライベートとパブリックの両方のモードを提供し、オンラインでシェマティックをプレビューできます\nオンラインのシェマティック変換、材料統計、ウェブサイトからのシェマティックのローカルへのクイックインポートを提供',
      visit: 'サイトへ→'
    },
    tauri: {
      title: 'Tauri 2.0',
      tooltip: 'Tauri 2.0を使用して開発されたソフトウェア',
      currentVersion: '現在のバージョン: {version}'
    },
    license: {
      title: 'GNU Affero General Public License',
      tooltip: '修正と配布を許可しますが、修正したコードをオープンソース化し、著作権表示を保持する必要があり、無許可の商用利用は禁止されています',
      copyright: '© 2025 MCS Tools. All rights licensed under AGPL-3.0',
      viewLicense: 'ライセンス全文を表示'
    },
    developers: {
      title: 'コア開発者',
      tooltip: 'すべての開発者と代表者はAGPL V3ライセンスに準拠する必要があり、修正と配布にはすべての開発者とライセンスのクレジットが必要です',
      author: '作者'
    }
  }
} 