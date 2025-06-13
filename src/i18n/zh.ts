export default {
  common: {
    save: '保存',
    cancel: '取消',
    delete: '删除',
    edit: '编辑',
    create: '创建',
    search: '搜索',
    loading: '加载中...',
    success: '成功',
    error: '错误',
    confirm: '确认',
    back: '返回'
  },
  menu: {
    home: '首页',
    settings: '设置',
    about: '关于'
  },
  about: {
    title: '关于',
    version: '版本: v{version}',
    actions: {
      checkUpdate: '检测更新',
      changelog: '更新日志',
      github: 'Github',
      website: '官方网站',
      sponsor: '赞助项目',
      faq: 'FAQ→'
    },
    description: {
      title: '软件说明',
      content: '软件使用tauri 后端基于rust，前端使用vue.\n分离化设计让软件性能得到保障，rust安全设计，性能更好，内存安全高效。'
    },
    schematicSite: {
      title: '蓝图站',
      description: '支持多种蓝图的网站，提供私有和公开多种模式，可以在线预览蓝图\n提供在线的蓝图转换功能，材料统计，可将网站中蓝图快速导入到本地',
      visit: '前往网站→'
    },
    tauri: {
      title: 'Tauri 2.0',
      tooltip: '软件基于tauri 2.0开发制作',
      currentVersion: '当前版本: {version}'
    },
    license: {
      title: 'GNU Affero General Public License',
      tooltip: '允许修改和分发，但必须开源修改后的代码并保留版权声明，禁止未经授权商业使用',
      copyright: '© 2025 MCS Tools. All rights licensed under AGPL-3.0',
      viewLicense: '查看完整协议'
    },
    developers: {
      title: '核心开发人员',
      tooltip: '参与开发及代表明确遵守AGPL V3协议,改版 转发请注明所有开发人员和协议',
      author: '作者'
    }
  },
  home: {
    title: '蓝图工具箱',
    stats: {
      localSchematics: '本地蓝图总数',
      cloudSchematics: '云端蓝图总数',
      welcome: '欢迎回来'
    },
    upload: {
      title: '蓝图处理',
      dragDrop: '拖放文件或点击上传',
      supportedFormats: '支持格式：nbt、litematic、schem、 json、 mcstruct（最大50MB）允许多选',
      selectFile: '选择文件',
      uploadSuccess: '成功上传 {count} 个文件',
      uploadError: '发生错误:{error}'
    },
    supportedTypes: {
      title: '支持蓝图类型',
      vanilla: {
        title: '香草结构',
        desc: '我的世界原版支持的蓝图格式，机械动力也采用了这种格式'
      },
      buildingGadgets: {
        title: '建筑小帮手',
        desc: '科技包最常见的辅组建筑工具'
      },
      litematica: {
        title: '建筑投影',
        desc: '生电玩家活下去的必备工具'
      },
      worldEdit: {
        title: '创世神',
        desc: '老牌建筑工具，延用至今，新版axiom也采用了这种蓝图格式'
      },
      bedrock: {
        title: 'MC BE',
        desc: '我的世界BE采用的蓝图格式，目前未完全适配'
      }
    }
  },
  settings: {
    darkMode: '深色模式',
    lightMode: '浅色模式',
    title: '设置',
    update: {
      title: '更新设置',
      autoUpdate: '启用自动更新',
      source: '更新源设置',
      sourcePlaceholder: '选择或输入更新源',
      sourceNoData: '输入有效的HTTP地址'
    },
    debug: {
      title: '调试模式',
      enable: '启用调试模式',
      open: '开启调试',
      openDev: '开启调试DEV'
    },
    theme: {
      title: '跟随主题',
      autoTheme: '启用系统跟随(页面主题将跟随windows主题变化)'
    },
    resources: {
      title: '资源文件',
      clear: '清除资源文件(将删除所有资源文件，你存储的蓝图)',
      clearConfirm: '确认清除',
      clearWarning: '清除将导致数据全部丢失，建议先进行备份',
      openFolder: '打开资源文件夹',
      openFolderBtn: '打开目录'
    },
    language: {
      title: '语言设置',
      select: '语言选择'
    }
  },
  messages: {
    clearSuccess: '已清除资源文件，将在5s后重启',
    error: '发生了一个错误: {error}',
    fetchError: '获取原理图失败: {error}'
  },
  tools: {
    title: '蓝图编辑',
    upload: '上传蓝图',
    noSchematic: '未选取目标蓝图',
    schematicId: '蓝图ID: {id}',
    tabs: {
      schematic: '蓝图详情',
      history: '版本管理',
      split: '蓝图分割',
      replace: '方块替换',
      convert: '蓝图转换',
      data: '源数据查看',
      stats: '材料统计',
      threeD: '结构预览'
    },
    convert: {
      title: '蓝图转换',
      tip: '大型蓝图的转换耗时可能过长请耐心等待',
      oneClickConvert: '一键转换',
      convertToFormat: '转换到改格式',
      confirmStart: '确认开始',
      confirmExport: '确认导出',
      cancel: '取消',
      targetVersion: '目标输出版本',
      waitingTip: '大型蓝图转换需要一定时间等待',
      noParams: '无转换参数，大型蓝图转换需要一定时间等待',
      alreadyExists: '已存在',
      formats: {
        vanilla: {
          title: '香草结构蓝图',
          desc: '适配 Minecraft 原版结构方块格式',
          ext: 'nbt'
        },
        litematica: {
          title: '投影蓝图',
          desc: '适配 我的世界建筑投影蓝图格式',
          ext: 'litematic'
        },
        worldEdit: {
          title: '创世神',
          desc: '适配与新版1.16 + 创世神模组和最新版 axios',
          ext: 'schem',
          versions: {
            latest: '0: WE最新格式',
            legacy: '1: WE 1.16-'
          }
        },
        buildingGadgets: {
          title: '建筑小帮手',
          desc: '适配与1.12 + 建筑小帮手 3个 变种格式蓝图',
          ext: 'json',
          versions: {
            latest: '0: 小帮手最新格式',
            modern: '1: 小帮手1.16+',
            legacy: '2: 小帮手1.12+'
          }
        },
        bedrock: {
          title: 'MC BE',
          desc: '适配与1.18 + 我的世界BE原版 结构方块格式',
          ext: 'mcstructure'
        }
      },
      meta: {
        extension: '后缀类型',
        originalSize: '原始大小',
        version: '版本',
        subVersion: '子版本',
        exists: '已存在',
        gzipCompression: 'Gzip压缩',
        hasSubVersions: '存在子版本'
      }
    }
  },
  schematics: {
    title: '蓝图仓库',
    local: '本地蓝图',
    web: '网络蓝图',
    upload: '上传蓝图',
    source: '站点源',
    sites: {
      mcs: 'MCS:www.mcschematic.top',
      cms: 'CMS:www.creativemechanicserver.com'
    }
  },
  report: {
    title: '问题反馈',
    subtitle: '反馈渠道',
    tip: '有问题先不要盲目，乱求医。先尝试自己解决一下！',
    channels: {
      github: {
        title: 'GitHub issue',
        desc: '通过Github issue向我们反馈bug和问题'
      },
      qqGroup: {
        title: 'QQ群聊',
        desc: '加入官方Q群反馈问题bug'
      },
      qqChannel: {
        title: 'QQ频道',
        desc: '加入官方QQ频道反馈问题bug'
      }
    },
    placeholder: '还莫有，这只是个占位符'
  },
  others: {
    title: '工具箱',
    tabs: {
      mapArt: '地图画',
      redstoneMusic: '红石音乐'
    }
  },
  individuation: {
    title: '个性化设置',
    opacity: {
      title: '不透明度',
      value: '{value}%'
    },
    theme: {
      title: '主题配色',
      options: {
        grey: '默认灰白',
        blue: '蔚蓝主题',
        darkBlue: '深蓝之夜',
        green: '清新绿意',
        orange: '活力橙',
        yellow: '菠萝黄',
        brown: '橡木棕',
        greyDark: '暗色模式'
      }
    },
    background: {
      title: '背景设置',
      imageInfo: '图片信息',
      fileName: '文件名',
      fileSize: '文件大小',
      resolution: '分辨率',
      layoutMode: '布局方式',
      layoutModes: {
        stretch: '拉伸填充',
        repeat: '平铺重复',
        contain: '适应屏幕',
        cover: '完整覆盖'
      },
      actions: {
        clear: '清除背景',
        refresh: '刷新背景',
        select: '选择背景文件'
      }
    }
  }
} 