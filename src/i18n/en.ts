export default {
  common: {
    save: 'Save',
    cancel: 'Cancel',
    delete: 'Delete',
    edit: 'Edit',
    create: 'Create',
    search: 'Search',
    loading: 'Loading...',
    success: 'Success',
    error: 'Error',
    confirm: 'Confirm',
    back: 'Back'
  },
  menu: {
    home: 'Home',
    settings: 'Settings',
    about: 'About'
  },
  home: {
    title: 'Schematic Toolbox',
    stats: {
      localSchematics: 'Local Schematics',
      cloudSchematics: 'Cloud Schematics',
      welcome: 'Welcome Back'
    },
    upload: {
      title: 'Schematic Processing',
      dragDrop: 'Drag and drop files or click to upload',
      supportedFormats: 'Supported formats: nbt, litematic, schem, json, mcstruct (max 50MB) Multiple files allowed',
      selectFile: 'Select File',
      uploadSuccess: 'Successfully uploaded {count} files',
      uploadError: 'Error occurred: {error}'
    },
    supportedTypes: {
      title: 'Supported Schematic Types',
      vanilla: {
        title: 'Vanilla Structure',
        desc: 'Minecraft vanilla supported schematic format, also used by Create mod'
      },
      buildingGadgets: {
        title: 'Building Gadgets',
        desc: 'Most common building assistant tool in tech modpacks'
      },
      litematica: {
        title: 'Litematica',
        desc: 'Essential tool for technical Minecraft players'
      },
      worldEdit: {
        title: 'WorldEdit',
        desc: 'Classic building tool, still in use today, also adopted by newer tools like Axiom'
      },
      bedrock: {
        title: 'MC BE',
        desc: 'Minecraft Bedrock Edition schematic format, currently not fully supported'
      }
    }
  },
  settings: {
    darkMode: 'Dark Mode',
    lightMode: 'Light Mode',
    title: 'Settings',
    update: {
      title: 'Update Settings',
      autoUpdate: 'Enable Auto Update',
      source: 'Update Source',
      sourcePlaceholder: 'Select or enter update source',
      sourceNoData: 'Enter a valid HTTP address'
    },
    debug: {
      title: 'Debug Mode',
      enable: 'Enable Debug Mode',
      open: 'Open Debug',
      openDev: 'Open Debug DEV'
    },
    theme: {
      title: 'Theme Follow',
      autoTheme: 'Enable System Theme Follow (Page theme will follow Windows theme changes)'
    },
    resources: {
      title: 'Resource Files',
      clear: 'Clear Resource Files (This will delete all resource files and your stored blueprints)',
      clearConfirm: 'Confirm Clear',
      clearWarning: 'Clearing will cause all data to be lost, please backup first',
      openFolder: 'Open Resource Folder',
      openFolderBtn: 'Open Directory'
    },
    language: {
      title: 'Language Settings',
      select: 'Language Selection'
    }
  },
  messages: {
    clearSuccess: 'Resource files cleared, will restart in 5s',
    error: 'An error occurred: {error}',
    fetchError: 'Failed to fetch schematic: {error}'
  },
  tools: {
    title: 'Schematic Editor',
    upload: 'Upload Schematic',
    noSchematic: 'No Schematic Selected',
    schematicId: 'Schematic ID: {id}',
    tabs: {
      schematic: 'Schematic Details',
      history: 'Version Control',
      split: 'Split Schematic',
      replace: 'Block Replacement',
      convert: 'Schematic Conversion',
      data: 'Source Data View',
      stats: 'Material Statistics',
      threeD: '3D Preview'
    },
    convert: {
      title: 'Schematic Conversion',
      tip: 'Large schematic conversion may take some time, please be patient',
      oneClickConvert: 'One-Click Convert',
      convertToFormat: 'Convert to Format',
      confirmStart: 'Confirm Start',
      confirmExport: 'Confirm Export',
      cancel: 'Cancel',
      targetVersion: 'Target Output Version',
      waitingTip: 'Large schematic conversion requires some waiting time',
      noParams: 'No conversion parameters, large schematic conversion requires some waiting time',
      alreadyExists: 'Already Exists',
      formats: {
        vanilla: {
          title: 'Vanilla Structure',
          desc: 'Compatible with Minecraft vanilla structure block format',
          ext: 'nbt'
        },
        litematica: {
          title: 'Litematica',
          desc: 'Compatible with Minecraft Litematica schematic format',
          ext: 'litematic'
        },
        worldEdit: {
          title: 'WorldEdit',
          desc: 'Compatible with WorldEdit mod 1.16+ and latest axios',
          ext: 'schem',
          versions: {
            latest: '0: Latest WE Format',
            legacy: '1: WE 1.16-'
          }
        },
        buildingGadgets: {
          title: 'Building Gadgets',
          desc: 'Compatible with Building Gadgets 1.12+ three variant formats',
          ext: 'json',
          versions: {
            latest: '0: Latest BG Format',
            modern: '1: BG 1.16+',
            legacy: '2: BG 1.12+'
          }
        },
        bedrock: {
          title: 'MC BE',
          desc: 'Compatible with Minecraft Bedrock Edition 1.18+ structure block format',
          ext: 'mcstructure'
        }
      },
      meta: {
        extension: 'Extension Type',
        originalSize: 'Original Size',
        version: 'Version',
        subVersion: 'Sub Version',
        exists: 'Exists',
        gzipCompression: 'Gzip Compression',
        hasSubVersions: 'Has Sub Versions'
      }
    }
  },
  schematics: {
    title: 'Schematic Repository',
    local: 'Local Schematics',
    web: 'Web Schematics',
    upload: 'Upload Schematic',
    source: 'Source Site',
    sites: {
      mcs: 'MCS:www.mcschematic.top',
      cms: 'CMS:www.creativemechanicserver.com'
    }
  },
  report: {
    title: 'Issue Report',
    subtitle: 'Feedback Channels',
    tip: 'Before seeking help, try to solve the problem yourself first!',
    channels: {
      github: {
        title: 'GitHub issue',
        desc: 'Report bugs and issues through GitHub issues'
      },
      qqGroup: {
        title: 'QQ Group',
        desc: 'Join our official QQ group to report bugs and issues'
      },
      qqChannel: {
        title: 'QQ Channel',
        desc: 'Join our official QQ channel to report bugs and issues'
      }
    },
    placeholder: 'Not available yet, this is just a placeholder'
  },
  others: {
    title: 'Toolbox',
    tabs: {
      mapArt: 'Map Art',
      redstoneMusic: 'Redstone Music'
    }
  },
  individuation: {
    title: 'Personalization',
    opacity: {
      title: 'Opacity',
      value: '{value}%'
    },
    theme: {
      title: 'Theme Color',
      options: {
        grey: 'Default Grey',
        blue: 'Azure Theme',
        darkBlue: 'Dark Blue Night',
        green: 'Fresh Green',
        orange: 'Vibrant Orange',
        yellow: 'Pineapple Yellow',
        brown: 'Oak Brown',
        greyDark: 'Dark Mode'
      }
    },
    background: {
      title: 'Background Settings',
      imageInfo: 'Image Information',
      fileName: 'File Name',
      fileSize: 'File Size',
      resolution: 'Resolution',
      layoutMode: 'Layout method',
      layoutModes: {
        stretch: 'Stretch Fill',
        repeat: 'Tile Repeat',
        contain: 'Fit Screen',
        cover: 'Full Cover'
      },
      actions: {
        clear: 'Clear Background',
        refresh: 'Refresh Background',
        select: 'Select Background File'
      }
    }
  },
  about: {
    title: 'About',
    version: 'Version: v{version}',
    actions: {
      checkUpdate: 'Check for Updates',
      changelog: 'Changelog',
      github: 'Github',
      website: 'Official Website',
      sponsor: 'Sponsor Project',
      faq: 'FAQ→'
    },
    description: {
      title: 'Software Description',
      content: 'The software uses Tauri backend based on Rust and Vue frontend.\nModular design ensures software performance, Rust\'s safety design provides better performance and memory safety.'
    },
    schematicSite: {
      title: 'Schematic Site',
      description: 'A website supporting multiple schematic formats, offering both private and public modes, with online schematic preview\nProvides online schematic conversion, material statistics, and quick import of schematics from the website to local',
      visit: 'Visit Site→'
    },
    tauri: {
      title: 'Tauri 2.0',
      tooltip: 'Software developed using Tauri 2.0',
      currentVersion: 'Current Version: {version}'
    },
    license: {
      title: 'GNU Affero General Public License',
      tooltip: 'Allows modification and distribution but requires open-sourcing modified code and retaining copyright notices, prohibits unauthorized commercial use',
      copyright: '© 2025 MCS Tools. All rights licensed under AGPL-3.0',
      viewLicense: 'View Full License'
    },
    developers: {
      title: 'Core Developers',
      tooltip: 'All developers and representatives must comply with AGPL V3 protocol, modifications and distributions must credit all developers and the license',
      author: 'Author'
    }
  }
} 