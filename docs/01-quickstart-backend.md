### 快速上手：本地运行后端

本指南帮助你在本地快速启动 QiLuo 后端服务。

#### 1. 环境准备
- Rust >= 1.70（建议 `rustup` 安装）
- MySQL >= 8.0 或 SQLite >= 3.35
- Redis >= 6.0（可选，用于缓存与验证码）

#### 2. 拉取代码并进入目录
```bash
git clone https://github.com/chelunfu/qiluo_admin.git
cd qiluo_admin
```

#### 3. 配置数据库与缓存
- 复制并编辑 `config/development.yaml`：
  - `database.uri`：MySQL 例如 `mysql://user:pass@localhost:3306/qiluoopen`
  - 或使用 SQLite（示例见注释）
  - `cache.url`：Redis 连接串，默认 `redis://127.0.0.1`

建议通过环境变量覆盖：
```bash
export DATABASE_URL="mysql://qiluo:Qiluo123@localhost:3306/qiluoopen"
export REDIS_URL="redis://127.0.0.1"
```

#### 4. 初始化数据库
- 导入根目录的 `qiluo.sql`（或 `migration/` 中的迁移）到目标库。

#### 5. 启动后端
```bash
cargo run
```
默认监听 `0.0.0.0:5001`，静态目录 `data/static`，前端目录 `data/web`。

#### 6. 登录后台
- 管理后台默认由前端项目提供。默认跨域允许 `http://localhost:4000`。
- 默认账户：`admin / 123456`（首次登录请修改）。

#### 7. 常见问题
- 端口占用：修改 `config/development.yaml` 中 `server.port`。
- 日志路径：`logger.log_dir`（默认为 `data/log`）。
- JWT 过期：`auth.jwt.expiration`（秒）。
