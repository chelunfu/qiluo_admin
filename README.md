# QiLuo - Rust Rapid Development Platform
<a href="https://www.qiluo.vip/" target="_blank">
  <img src="https://www.qiluo.vip/logo.png" alt="QiLuo" style="max-width: 100%;">
</a>

**High Performance · Secure and Reliable · Enterprise-grade Application Development Framework**

<div style="display: flex; flex-wrap: wrap; gap: 5px;">
<a href="https://www.qiluo.vip/" target="_blank"><img src="https://www.qiluo.vip/badge/QiLuo.svg" alt="QiLuo"></a>
<a href="https://www.rust-lang.org/" target="_blank"><img src="https://www.qiluo.vip/badge/Rust-latest-orange.svg" alt="Rust"></a>
<a href="https://github.com/tokio-rs/axum" target="_blank"><img src="https://www.qiluo.vip/badge/Axum-latest-blue.svg" alt="Axum"></a>
<a href="https://github.com/SeaQL/sea-orm" target="_blank"><img src="https://www.qiluo.vip/badge/SeaORM-latest-green.svg" alt="SeaORM"></a>
<a href="https://vuejs.org/" target="_blank"><img src="https://www.qiluo.vip/badge/Vue.js-3.4.x-brightgreen.svg" alt="Vue.js"></a>
<a href="LICENSE" target="_blank"><img src="https://www.qiluo.vip/badge/License-MIT-yellow.svg" alt="License"></a>
</div>

## 📖 Project Overview

QiLuo is an enterprise-grade rapid development platform built on the Rust technology stack. It adopts modern architectural design and provides a complete backend management system solution. The platform integrates core functions such as user management, permission control, system monitoring, and WeChat official account management, suitable for information construction of small and medium-sized enterprises.

## ✨ Core Features

- **🦀 Rust Ecosystem** — Built on Rust + Axum + Sea-ORM + JWT, with excellent performance and memory safety
- **🔐 Permission Management** — Complete RBAC permission system, supporting multi-dimensional permission control for users, roles, menus, and APIs
- **📱 WeChat Integration** — Built-in WeChat official account management, supporting menu configuration, message management, user management, and automatic replies
- **⚡ High Performance** — Asynchronous architecture, supporting high concurrency and fast response speed
- **🌐 Distributed** — Supports cluster deployment, multiple data sources, and distributed architecture
- **📊 System Monitoring** — Real-time server status monitoring, online user management, and operation log tracking
- **⏰ Scheduled Tasks** — Online configuration of scheduled tasks, supporting Cron expressions
- **🎨 Modern Interface** — Vue-based management backend with a beautiful interface and convenient operation

## 🛠️ Technology Stack

### Backend Technology

| Technology | Description | Version |
|------|------|------|
| **Rust** | System development language | >= 1.70.0 |
| **Axum** | Web framework | Latest version |
| **Sea-ORM** | ORM framework | Latest version |
| **Tokio** | Asynchronous runtime | Latest version |
| **MySQL/SQLite** | Database | MySQL >= 8.0 or SQLite >= 3.35 |
| **Redis** | Cache service | >= 6.0 |
| **JWT** | Authentication mechanism | Latest version |
| **tracing** | Logging system | Latest version |

### Frontend Technology

| Technology | Description | Version |
|------|------|------|
| **Vue** | Frontend framework | 3.4.x |
| **Vite** | Build tool | 5.2.x |
| **TypeScript** | Type system | 5.4.x |
| **Pinia** | State management | 2.1.x |
| **Vue Router** | Routing management | 4.3.x |
| **Element Plus** | UI component library | 2.7.x |
| **Axios** | HTTP request | Latest version |
| **ECharts** | Chart library | Latest version |

## 🚀 Quick Start

### System Requirements

Before running the project, ensure your system has the following software installed:

- **Rust** >= 1.70.0 ([Installation Guide](https://rustup.rs/))
- **MySQL** >= 8.0 or **SQLite** >= 3.35
- **Redis** >= 6.0 (optional, for caching)
- **Node.js** >= 16.0.0 (for frontend development)

### 1. Obtain Source Code

#### Backend address:

#### Clone the backend project
---
##### GitHub <https://github.com/chelunfu/qiluo_admin.git>
```bash
git clone https://github.com/chelunfu/qiluo_admin.git
cd qiluo_admin
```
---
##### GitCode <https://gitcode.com/will_csdn_go/qiluo_admin.git>
![star](https://gitcode.com/will_csdn_go/qiluo_admin/star/badge.svg) 
```bash
git clone https://gitcode.com/will_csdn_go/qiluo_admin.git
cd qiluo_admin
```
---
##### Gitee <https://gitee.com/chenlunfu/qiluo_admin.git>
```bash
git clone https://gitee.com/chenlunfu/qiluo_admin.git
cd qiluo_admin
```
---
#### Frontend address

#### Clone the frontend project
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

#### Tauri address
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
### 2. Configure Database

Edit the configuration file in the `config/` directory to set database connection information:

```toml
[database]
url = "mysql://username:password@localhost:3306/qiluo"
# Or use SQLite
# url = "sqlite:data/qiluo.db"
```

Create the database and import the qiluo.sql file.

### 3. Start the Project

#### Start the backend service

```bash
# Run in development mode
cargo run
```

#### Start the frontend project

```bash
# Install dependencies
pnpm install

# Start the development server
pnpm run dev
```

### 4. Access the System

After the service starts, access the following address:

- **Management backend**: <http://localhost:4000>

## 🔧 Production Environment Deployment

### Backend Deployment

```bash
# Build Release version
cargo build --release

# Run Release version
./target/release/qiluo
```

### Frontend Deployment

```bash
# Build for production environment
pnpm build:pro

# The build output is in the dist directory, which can be deployed to any static server
```

## 📁 Project Structure

### Backend Directory Structure

```
qiluo/
├── 📁 config/               # Configuration files
├── 📁 data/                 # Data storage directory
│   ├── 📁 img/              # Image resources
│   ├── 📁 log/              # Log files
│   ├── 📁 static/           # Static resources
│   ├── 📁 upload/           # Uploaded files
│   └── 📁 web/              # Frontend files
├── 📁 migration/            # Database migrations
├── 📁 src/                  # Source code
│   ├── 📁 api/              # API controllers
│   │   ├── 📁 sys_controll/ # System management interfaces
│   │   └── 📁 wechat/       # WeChat interfaces
│   ├── 📁 cache/            # Cache management
│   ├── 📁 common/           # Common modules
│   ├── 📁 config/           # Configuration module
│   ├── 📁 midle_ware/       # Middleware
│   ├── 📁 model/            # Data models
│   ├── 📁 service/          # Business services
│   └── 📁 worker/           # Background tasks
└── 📄 Cargo.toml            # Project configuration
```

### Frontend Directory Structure

```
qiluo_vue/
├── 📁 public/               # Static resources directory
├── 📁 src/                  # Source code directory
│   ├── 📁 api/              # API interface definitions
│   ├── 📁 assets/           # Project resource files
│   ├── 📁 axios/            # Axios request configuration
│   ├── 📁 components/       # Common components
│   ├── 📁 constants/        # Constant definitions
│   ├── 📁 directives/       # Custom directives
│   ├── 📁 hooks/            # Custom Hooks
│   ├── 📁 layout/           # Layout components
│   ├── 📁 locales/          # Internationalization resources
│   ├── 📁 plugins/          # Plugin configurations
│   ├── 📁 router/           # Routing configuration
│   ├── 📁 store/            # State management
│   ├── 📁 styles/           # Style files
│   ├── 📁 utils/            # Utility functions
│   ├── 📁 views/            # Page views
│   ├── 📄 App.vue           # Root component
│   ├── 📄 main.ts           # Application entry file
│   └── 📄 permission.ts     # Permission control
├── 📁 types/                # TypeScript type definitions
├── 📄 index.html            # HTML template
├── 📄 package.json          # Project dependencies
└── 📄 vite.config.ts        # Vite configuration
```

## Tutorial Addresses


1. **Main Tutorial Address**  
   - **Link**: https://www.qiluo.vip/tutorial/  
   - **Description**: This is the root page of the tutorials, which may include a directory, index, or overview of all tutorials. It is recommended to start browsing the overall content from here.

2. **Quick Start Tutorial**  
   - **Link**: https://www.qiluo.vip/tutorial/getting-started/new-model.html  
   - **Description**: Focused on quick entry for beginners, with a specific topic on "new-model" (which may involve creating new models). Suitable for beginners to quickly get started with the platform's basic operations. Can be paired with the test module for reading and understanding.

3. **Auto-Generate Code Tutorial**  
   - **Link**: https://www.qiluo.vip/tutorial/getting-started/auto-generate.html  
   - **Description**: Focused on the "auto-generate" (automatic code generation) feature. It may include steps, examples, and best practices to help users automate the code generation process.

## 📖 Function Modules

### System Management

- 👥 **User Management** — User CRUD, password reset, avatar upload
- 🔐 **Role Management** — Role creation, permission assignment, user association
- 🏢 **Department Management** — Organizational structure tree management
- 📋 **Menu Management** — System menu configuration and permission control
- 📚 **Data Dictionary** — System dictionary data management
- 📊 **System Monitoring** — Server status, online user monitoring
- 📝 **Log Management** — Operation logs, login log records
- ⏰ **Scheduled Tasks** — Task configuration, execution monitoring
- 🔌 **API Permissions** — Interface-level permission control

### WeChat Management

- 📱 **Official Account Management** — Multi-account configuration management
- 📋 **Menu Management** — Custom menu creation and publishing
- 💬 **Message Management** — Message sending/receiving and history records
- 👤 **User Management** — Fan information management
- 🤖 **Auto-reply** — Keyword intelligent reply

## 🔑 Default Account

When running the system for the first time, you can log in with the following default administrator account:

- **Username**: admin
- **Password**: 123456

> ⚠️ **Security Note**: Please change the default password immediately after the first login!

## 🌐 Browser Support

This system supports modern browsers and does not support IE:

| [<img src="http://www.qiluo.vip/assets/icon/edge_48x48.png" alt="Edge" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Edge | [<img src="http://www.qiluo.vip/assets/icon/firefox_48x48.png" alt="Firefox" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Firefox | [<img src="http://www.qiluo.vip/assets/icon/chrome_48x48.png" alt="Chrome" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Chrome | [<img src="http://www.qiluo.vip/assets/icon/safari_48x48.png" alt="Safari" width="24px" height="24px" />](http://godban.github.io/browsers-support-badges/)</br>Safari |
| :-: | :-: | :-: | :-: |
| last 2 versions | last 2 versions | last 2 versions | last 2 versions |

## 📷 System Screenshots

<table>
  <tr>
    <td><img src="https://www.qiluo.vip/assets/images/1.jpg" alt="Screenshot 1" /></td>
    <td><img src="https://www.qiluo.vip/assets/images/2.jpg" alt="Screenshot 2" /></td>
  </tr>
  <tr>
    <td><img src="https://www.qiluo.vip/assets/images/3.jpg" alt="Screenshot 3" /></td>
    <td><img src="https://www.qiluo.vip/assets/images/4.jpg" alt="Screenshot 4" /></td>
  </tr>
  <tr>
    <td><img src="https://www.qiluo.vip/assets/images/5.jpg" alt="Screenshot 5" /></td>
    <td><img src="https://www.qiluo.vip/assets/images/6.jpg" alt="Screenshot 6" /></td>
  </tr>
  <tr>
    <td><img src="https://www.qiluo.vip/assets/images/7.jpg" alt="Screenshot 7" /></td>
    <td><img src="https://www.qiluo.vip/assets/images/8.jpg" alt="Screenshot 8" /></td>
  </tr>
</table>

> For more screenshots, please visit the [official website](https://www.qiluo.vip)

## 🐛 Issue Feedback

If you encounter problems during use, please provide feedback through the following methods:

1. **GitHub Issues**：[Git Issuse](https://github.com/chelunfu/qiluo_admin/issues)
2. **Git Code**:[GitCode Issuse](https://gitcode.com/will_csdn_go/qiluo_admin/issues)
3. **Gitee**:[Gitee Issuse](https://gitcode.com/will_csdn_go/qiluo_admin/issues)
4. **Official Website**: <https://www.qiluo.vip>
5. **Technical Exchange**: Welcome to join our technical exchange [QQ group](https://qm.qq.com/q/zI4N0SkwnI) to communicate with other users, share your problems, and seek their help.

## 🤝 Contribution Guide

We welcome contributions in any form, including but not limited to:

- 🐛 Submitting bug reports
- 💡 Proposing new feature suggestions
- 📝 Improving documentation
- 🔧 Submitting code patches

Before submitting contributions, please read our [Contribution Guide](CONTRIBUTING.md).

## 💰 Become a Sponsor

If you would like to support the development of this project, you can become a sponsor through the following ways:

<div align="center">
  <table>
    <tr>
      <td align="center"><img src="https://www.qiluo.vip/assets/sponsor/wechat.jpg" width="200" alt="WeChat Pay" /><br><strong>WeChat Pay</strong></td>
      <td align="center"><img src="https://www.qiluo.vip/assets/sponsor/alipay.jpg" width="200" alt="Alipay" /><br><strong>Alipay</strong></td>
    </tr>
  </table>
</div>

## 📄 License

This project is licensed under the MIT License. For details, please refer to the [LICENSE](LICENSE) file.

## 🙏 Acknowledgments

Thank all sponsors and contributors who support this project; your support enables this project to develop better.

Special thanks to the following open-source projects:

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

**⭐ If this project is helpful to you, please give me a Star!**

</div>