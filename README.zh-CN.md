# 祺洛 (QiLuo) - Rust 快速开发平台
<a href="https://www.qiluo.vip/" target="_blank">
  <img src="https://www.qiluo.vip/logo.png" alt="QiLuo" style="max-width: 100%;">
</a>

**高性能 · 安全可靠 · 企业级应用开发框架**

<div style="display: flex; flex-wrap: wrap; gap: 5px;">
<a href="https://www.qiluo.vip/" target="_blank"><img src="https://www.qiluo.vip/badge/QiLuo.svg" alt="QiLuo"></a>
<a href="https://www.rust-lang.org/" target="_blank"><img src="https://www.qiluo.vip/badge/Rust-latest-orange.svg" alt="Rust"></a>
<a href="https://github.com/tokio-rs/axum" target="_blank"><img src="https://www.qiluo.vip/badge/Axum-latest-blue.svg" alt="Axum"></a>
<a href="https://github.com/SeaQL/sea-orm" target="_blank"><img src="https://www.qiluo.vip/badge/SeaORM-latest-green.svg" alt="Sea-ORM"></a>
<a href="https://vuejs.org/" target="_blank"><img src="https://www.qiluo.vip/badge/Vue.js-3.4.x-brightgreen.svg" alt="Vue.js"></a>
<a href="LICENSE" target="_blank"><img src="https://www.qiluo.vip/badge/License-MIT-yellow.svg" alt="License"></a>
</div>

## 📖 项目概述

祺洛是一个基于 Rust 技术栈开发的企业级快速开发平台，采用现代化的架构设计，提供完整的后台管理系统解决方案。平台集成了用户管理、权限控制、系统监控、微信公众号管理等核心功能，适用于中小型企业的信息化建设。

## ✨ 核心特性

- **🦀 Rust 生态** — 基于 Rust + Axum + Sea-ORM + JWT 构建，性能卓越，内存安全
- **🔐 权限管理** — 完整的 RBAC 权限体系，支持用户、角色、菜单、API 多维度权限控制
- **📱 微信集成** — 内置微信公众号管理，支持菜单配置、消息管理、用户管理、自动回复
- **⚡ 高性能** — 异步架构，支持高并发，响应速度快
- **🌐 分布式** — 支持集群部署，多数据源，分布式架构
- **📊 系统监控** — 实时服务器状态监控，在线用户管理，操作日志追踪
- **⏰ 定时任务** — 在线配置定时任务，支持 Cron 表达式
- **🎨 现代化界面** — 基于 Vue 的管理后台，界面美观，操作便捷

## 🛠️ 技术栈

### 后端技术

| 技术 | 说明 | 版本 |
|------|------|------|
| **Rust** | 系统开发语言 | >= 1.70.0 |
| **Axum** | Web 框架 | 最新版 |
| **Sea-ORM** | ORM 框架 | 最新版 |
| **Tokio** | 异步运行时 | 最新版 |
| **MySQL/SQLite** | 数据库 | MySQL >= 8.0 或 SQLite >= 3.35 |
| **Redis** | 缓存服务 | >= 6.0 |
| **JWT** | 认证机制 | 最新版 |
| **tracing** | 日志系统 | 最新版 |

### 前端技术

| 技术 | 说明 | 版本 |
|------|------|------|
| **Vue** | 前端框架 | 3.4.x |
| **Vite** | 构建工具 | 5.2.x |
| **TypeScript** | 类型系统 | 5.4.x |
| **Pinia** | 状态管理 | 2.1.x |
| **Vue Router** | 路由管理 | 4.3.x |
| **Element Plus** | UI 组件库 | 2.7.x |
| **Axios** | HTTP 请求 | 最新版 |
| **ECharts** | 图表库 | 最新版 |

## 🚀 快速开始

### 系统要求

在运行项目之前，请确保您的系统已安装以下软件：

- **Rust** >= 1.70.0 ([安装指南](https://rustup.rs/))
- **MySQL** >= 8.0 或 **SQLite** >= 3.35
- **Redis** >= 6.0 (可选，用于缓存)
- **Node.js** >= 16.0.0 (前端开发)

### 1. 获取源码

#### 后端地址

#### 克隆后端项目
##### GitHub <https://github.com/chelunfu/qiluo_admin.git>
```bash
git clone https://github.com/chelunfu/qiluo_admin.git
cd qiluo_admin
```
##### GitCode <https://gitcode.com/will_csdn_go/qiluo_admin.git>
![star](https://gitcode.com/will_csdn_go/qiluo_admin/star/badge.svg) 
```bash
git clone https://gitcode.com/will_csdn_go/qiluo_admin.git
cd qiluo_admin
```

##### Gitee <https://gitee.com/chenlunfu/qiluo_admin.git>
```bash
git clone https://gitee.com/chenlunfu/qiluo_admin.git
cd qiluo_admin
```

#### 前端地址

#### 克隆前端项目
---
##### GitHub <https://github.com/chelunfu/qiluo_vue.git>
```bash
git clone https://github.com/chelunfu/qiluo_vue.git
cd qiluo_vue
```
---
##### GitCode <https://gitcode.com/will_csdn_go/qiluo_vue.git>
![star](https://gitcode.com/will_csdn_go/qiluo_admin/star/badge.svg) 
```bash
git clone https://gitcode.com/will_csdn_go/qiluo_vue.git
cd qiluo_vue
```
---
##### Gitee <https://gitee.com/chenlunfu/qiluo_vue.git>
```bash
git clone https://gitee.com/chenlunfu/qiluo_vue.git
cd qiluo_vue
```
---
#### Tauri版本地址
---
##### GitHub <https://github.com/chelunfu/qiluo_tauri.git>
```bash
git clone https://github.com/chelunfu/qiluo_tauri.git
cd qiluo_tauri
```
---
##### GitCode <https://gitcode.com/will_csdn_go/qiluo_tauri.git>
![star](https://gitcode.com/will_csdn_go/qiluo_admin/star/badge.svg) 
```bash
git clone https://gitcode.com/will_csdn_go/qiluo_tauri.git
cd qiluo_tauri
```
---
##### Gitee <https://gitee.com/chenlunfu/qiluo_tauri.git>
```bash
git clone https://gitee.com/chenlunfu/qiluo_tauri.git
cd qiluo_tauri
```
---
### 2. 配置数据库

编辑 `config/` 目录下的配置文件，设置数据库连接信息：

```toml
[database]
url = "mysql://username:password@localhost:3306/qiluo"
# 或使用 SQLite
# url = "sqlite:data/qiluo.db"
```

创建数据库，并导入 qiluo.sql 文件。

### 3. 启动项目

#### 启动后端服务

```bash
# 开发模式运行
cargo run
```
#### 启动前端项目

```bash
# 安装依赖
pnpm install
# 启动开发服务器
pnpm run dev
```

### 4. 访问系统

服务启动后，访问以下地址：

- **管理后台**：<http://localhost:4000>

## 🔧 生产环境部署

### 后端部署

```bash
# 构建 Release 版本
cargo build --release

# 运行 Release 版本
./target/release/qiluo
```

### 前端部署

```bash
# 生产环境构建
pnpm build:pro

# 构建产物位于 dist 目录，可部署到任意静态服务器
```

## 📁 项目结构

### 后端目录结构

```
qiluo/
├── 📁 config/               # 配置文件
├── 📁 data/                 # 数据存储目录
│   ├── 📁 img/              # 图片资源
│   ├── 📁 log/              # 日志文件
│   ├── 📁 static/           # 静态资源
│   ├── 📁 upload/           # 上传文件
│   └── 📁 web/              # 前端文件
├── 📁 migration/            # 数据库迁移
├── 📁 src/                  # 源代码
│   ├── 📁 api/              # API 控制器
│   │   ├── 📁 sys_controll/ # 系统管理接口
│   │   └── 📁 wechat/       # 微信接口
│   ├── 📁 cache/            # 缓存管理
│   ├── 📁 common/           # 公共模块
│   ├── 📁 config/           # 配置模块
│   ├── 📁 midle_ware/       # 中间件
│   ├── 📁 model/            # 数据模型
│   ├── 📁 service/          # 业务服务
│   └── 📁 worker/           # 后台任务
└── 📄 Cargo.toml            # 项目配置
```

### 前端目录结构

```
qiluo_vue/
├── 📁 public/               # 静态资源目录
├── 📁 src/                  # 源代码目录
│   ├── 📁 api/              # API 接口定义
│   ├── 📁 assets/           # 项目资源文件
│   ├── 📁 axios/            # Axios 请求配置
│   ├── 📁 components/       # 公共组件
│   ├── 📁 constants/        # 常量定义
│   ├── 📁 directives/       # 自定义指令
│   ├── 📁 hooks/            # 自定义 Hook
│   ├── 📁 layout/           # 布局组件
│   ├── 📁 locales/          # 国际化资源
│   ├── 📁 plugins/          # 插件配置
│   ├── 📁 router/           # 路由配置
│   ├── 📁 store/            # 状态管理
│   ├── 📁 styles/           # 样式文件
│   ├── 📁 utils/            # 工具函数
│   ├── 📁 views/            # 页面视图
│   ├── 📄 App.vue           # 根组件
│   ├── 📄 main.ts           # 应用入口文件
│   └── 📄 permission.ts     # 权限控制
├── 📁 types/                # TypeScript 类型定义
├── 📄 index.html            # HTML 模板
├── 📄 package.json          # 项目依赖
└── 📄 vite.config.ts        # Vite 配置
```

## 教程地址
1. **主教程地址**  
   - **链接**：https://www.qiluo.vip/tutorial/  
   - **描述**：这是教程的根页面，可能包含所有教程的目录、索引或概述。建议从这里开始浏览整体内容。

2. **快速上手教程**  
   - **链接**：https://www.qiluo.vip/tutorial/getting-started/new-model.html  
   - **描述**：专注于新手快速入门，具体主题是“new-model”（可能涉及创建新模型）。适合初学者快速上手平台的基本操作。可配合test模块可供阅读理解

3. **自动生成代码教程**  
   - **链接**：https://www.qiluo.vip/tutorial/getting-started/auto-generate.html  
   - **描述**：专注于“auto-generate”（自动生成代码）的功能。可能包括步骤、示例和最佳实践，帮助用户自动化代码生成过程。

### 建议使用流程
- **步骤1**：访问主教程地址（https://www.qiluo.vip/tutorial/）了解整体框架。
- **步骤2**：如果您是新手，先阅读快速上手教程（new-model），掌握基础。
- **步骤3**：深入学习自动生成代码教程，应用到实际场景中。
## 📖 功能模块

### 系统管理

- 👥 **用户管理** — 用户增删改查、密码重置、头像上传
- 🔐 **角色管理** — 角色创建、权限分配、用户关联
- 🏢 **部门管理** — 组织架构树形管理
- 📋 **菜单管理** — 系统菜单配置和权限控制
- 📚 **数据字典** — 系统字典数据管理
- 📊 **系统监控** — 服务器状态、在线用户监控
- 📝 **日志管理** — 操作日志、登录日志记录
- ⏰ **定时任务** — 任务配置、执行监控
- 🔌 **API 权限** — 接口级权限控制

### 微信管理

- 📱 **公众号管理** — 多账号配置管理
- 📋 **菜单管理** — 自定义菜单创建发布
- 💬 **消息管理** — 消息收发和历史记录
- 👤 **用户管理** — 粉丝信息管理
- 🤖 **自动回复** — 关键词智能回复

## 🔑 默认账户

初次运行系统时，可使用以下默认管理员账户登录：

- **用户名**：admin
- **密码**：123456

> ⚠️ **安全提示**：首次登录后请立即修改默认密码！

## 🌐 浏览器支持

本系统支持现代浏览器，不支持 IE：

| [<img src="http://www.qiluo.vip/assets/icon/edge_48x48.png" alt="Edge" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Edge | [<img src="http://www.qiluo.vip/assets/icon/firefox_48x48.png" alt="Firefox" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Firefox | [<img src="http://www.qiluo.vip/assets/icon/chrome_48x48.png" alt="Chrome" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Chrome | [<img src="http://www.qiluo.vip/assets/icon/safari_48x48.png" alt="Safari" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Safari |
| :-: | :-: | :-: | :-: |
| last 2 versions | last 2 versions | last 2 versions | last 2 versions |

## 📷 系统截图

<table>
  <tr>
    <td><img src="https://www.qiluo.vip/assets/images/1.jpg" alt="截图1" /></td>
    <td><img src="https://www.qiluo.vip/assets/images/2.jpg" alt="截图2" /></td>
  </tr>
  <tr>
    <td><img src="https://www.qiluo.vip/assets/images/3.jpg" alt="截图3" /></td>
    <td><img src="https://www.qiluo.vip/assets/images/4.jpg" alt="截图4" /></td>
  </tr>
  <tr>
    <td><img src="https://www.qiluo.vip/assets/images/5.jpg" alt="截图5" /></td>
    <td><img src="https://www.qiluo.vip/assets/images/6.jpg" alt="截图6" /></td>
  </tr>
  <tr>
    <td><img src="https://www.qiluo.vip/assets/images/7.jpg" alt="截图7" /></td>
    <td><img src="https://www.qiluo.vip/assets/images/8.jpg" alt="截图8" /></td>
  </tr>
</table>

> 更多截图请访问[官方网站](https://www.qiluo.vip)查看

## 🐛 问题反馈

如果您在使用过程中遇到问题，请通过以下方式反馈：

1. **GitHub Issues**：[Git Issuse](https://github.com/chelunfu/qiluo_admin/issues)
2. **Git Code**:[GitCode Issuse](https://gitcode.com/will_csdn_go/qiluo_admin/issues)
3. **Gitee**:[Gitee Issuse](https://gitcode.com/will_csdn_go/qiluo_admin/issues)
4. **官方网站**：<https://www.qiluo.vip>
5. **技术交流**：欢迎加入我们的技术交流[QQ群](https://qm.qq.com/q/zI4N0SkwnI)，与其他用户交流，分享你的问题，并寻求他们的帮助。

## 🤝 贡献指南

我们欢迎任何形式的贡献，包括但不限于：

- 🐛 提交 Bug 报告
- 💡 提出新功能建议
- 📝 完善文档
- 🔧 提交代码补丁

在提交贡献之前，请阅读我们的 [贡献指南](CONTRIBUTING.md)。

## 💰 成为赞助商

如果你愿意支持本项目的发展，可以通过以下方式成为赞助商：

<div align="center">
  <table>
    <tr>
      <td align="center"><img src="https://www.qiluo.vip/assets/sponsor/wechat.jpg" width="200" alt="微信支付" /><br><strong>微信支付</strong></td>
      <td align="center"><img src="https://www.qiluo.vip/assets/sponsor/alipay.jpg" width="200" alt="支付宝" /><br><strong>支付宝</strong></td>
    </tr>
  </table>
</div>

## 📄 许可证

本项目采用 MIT 许可证，详情请参阅 [LICENSE](LICENSE) 文件。

## 🙏 致谢

感谢所有支持这个项目的赞助商和贡献者，你们的支持让这个项目能够更好地发展。

特别感谢以下开源项目：

- [Rust](https://www.rust-lang.org/ "Rust Programming Language")
- [Vue.js](https://vuejs.org/ "Vue.js Framework")
- [Axum](https://github.com/tokio-rs/axum "Axum Web Framework")
- [Tokio](https://tokio.rs/)
- [VuePress](https://vuepress.vuejs.org/ "VuePress Framework")
- [VitePress](https://vitepress.vuejs.org/ "VitePress Framework")
- [Element Plus](https://element-plus.org/ "Element Plus Framework")
- [UnoCSS](https://github.com/unocss/unocss/ "UnoCSS Framework")
- [element-plus-admin](https://element-plus-admin-doc.cn/ "element-plus-admin Framework")
- [sea-orm](https://github.com/SeaQL/sea-orm "sea-orm ORM Framework")
- [tauri](https://tauri.app/ "tauri Framework")
- [sidekiq](https://github.com/film42/sidekiq-rs")

---

<div align="center">

**⭐ 如果这个项目对您有帮助，请给我一个 Star！**

</div>
