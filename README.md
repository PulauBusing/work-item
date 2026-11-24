# 工作事项 (work-item)

个人桌面应用，用于记录工作事项，便于事项计划和工作日志记录。

## 功能特性

- **事项管理**：新增、编辑、删除、完成（完成时自动记录完成日期）
- **事项要素**：唯一编号（`item-YYYYMMDD+两位自增`）、名称、开始日期、完成日期、标签、详情（Markdown）、关联 Jira
- **三个清单视图**：事项清单、今日事项、本周事项，均支持状态筛选（默认未完成）、标签筛选、分页、列表/卡片两种布局
- **事项统计**：按今日/本周/上周/本月/上月/日期区间筛选，支持 Markdown 日志导出（按日期分组）
- **标签系统**：标签由「背景色 + 文字」组成，支持新建、改名、改色，多选筛选
- **主题切换**：内置浅色 / 深色主题，持久化记忆
- **界面**：可折叠侧边栏、图标化操作（悬停显示文字）、Markdown 预览独立弹窗

## 技术栈

- [Tauri 2](https://tauri.app/) + [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Vite](https://vitejs.dev/)
- UI 组件：[Element Plus](https://element-plus.org/)
- 状态管理：[Pinia](https://pinia.vuejs.org/)
- Markdown：[md-editor-v3](https://github.com/imzbf/md-editor-v3)
- 数据库：[SQLite](https://www.sqlite.org/)，通过 Rust 端 [rusqlite](https://github.com/rusqlite/rusqlite) 访问（bundled）
- 日期处理：[dayjs](https://day.js.org/)（前端）/ [chrono](https://github.com/chronotope/chrono)（Rust）

## 开发环境

- Node.js 18+
- Rust stable 工具链
- 目标平台：macOS / Windows（个人使用，非商业）

## 快速开始

```bash
# 安装前端依赖
npm install

# 开发模式（热更新）
npm run tauri dev

# 前端类型检查 + 打包
npm run build

# 打包安装包
npm run tauri build
```

> **macOS 打包说明**：在 macOS Tahoe (26) 及更新版本上，`npm run tauri build` 的 dmg 生成步骤可能因 `/Volumes` 只读而失败（`.app` 会正常产出）。请改用 `./scripts/make-dmg.sh` 一键构建 `.app` 与 `.dmg`。

## 数据存储

数据保存在 Tauri 的应用数据目录下的 SQLite 数据库文件中。正式包使用应用标识符 `com.wind.workitem` 对应的目录，开发模式使用临时标识符目录，两者数据互不影响。

## 目录结构

```
work-item/
├── src/                  # 前端源码
│   ├── components/       # Vue 组件（列表、表单、详情、标签编辑等）
│   ├── views/            # 视图（统计）
│   ├── store/            # Pinia store
│   ├── utils/            # 工具函数（颜色等）
│   └── types.ts          # 类型定义
├── src-tauri/            # Tauri 后端
│   └── src/              # Rust 源码（数据库、命令、日期计算）
└── scripts/              # 构建脚本（make-dmg.sh）
```
