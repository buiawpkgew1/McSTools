# Efficient Blueprint Toolkit Based on Tauri v2.0

**[简体中文](README.md)** | **English**

[![Stars](https://img.shields.io/github/stars/guapi-exe/McSTools?style=flat-square&label=Stars)](https://github.com/guapi-exe/McSTools/stargazers)
[![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Framework-Tauri%202.0-blue)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-GPLv3-green)]()

> A cross-platform toolkit built with Rust, delivering exceptional memory safety and native-level performance

---

##  Project
- **High-Performance Core**: Achieves ultimate memory safety and efficient computation through Rust
- **Cross-Platform Support**: Built with Tauri framework, compatible with Windows/macOS/Linux
- **Modular Design**: Functional components are freely extensible to meet advanced development needs

---

##  Implemented Features

### Blueprint Management
- **Version Control System**  
  Complete blueprint iteration history tracking and diff comparison
- **Smart Material Analysis**  
  Automatically calculates building material usage and resource consumption
- **Data Insights**  
  Native source data

### Blueprint Processing
- **Format Conversion Engine**  
  `Supports mutual conversion between 4 mainstream blueprint formats`（*.schem ↔ *.nbt ↔ ...）
- **Smart Block Replacement**
    - Simple Mode: Preserves target block attributes, only replaces ID
    - Precision Mode: Fully overwrites block ID and attribute sets

### Creative Tools
- **Themes**  
  Customizable UI theme colors/layouts/visual elements
- **Map Art Generator**  
  ▨ Flat Mode: Fast pixel art conversion  
  ▦ 3D Mode: Three-dimensional voxel art construction

---

##  In-Development Features
- **BE Blueprint Adaptation**  
  Support for Bedrock Edition blueprint parsing and conversion
- **Blueprint Splitter**  
  Intelligent large blueprint splitting and reassembly module

---

##  Planned Development Roadmap
- **Cloud Sync**  
  Cross-device blueprint library synchronization

---

![:Views](https://count.getloli.com/@guapi-exe_McSTools?name=guapi-exe_McSTools&theme=original-new&padding=8&offset=0&align=top&scale=1&pixelated=1&darkmode=auto)
##  Build Instructions
```bash
# Install dependencies
pnpm install

# Run in dev mode
pnpm run tauri dev

# Build for production
pnpm run tauri build