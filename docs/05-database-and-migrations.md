### 数据库与迁移：初始化与模型概览

#### 1) 初始化数据库
- 使用根目录 `qiluo.sql` 创建数据库结构与基础数据。
- 或在 `config/development.yaml` 中开启 `database.auto_migrate: true`（开发环境建议）

#### 2) 连接配置
- `database.uri` 支持 MySQL 与 SQLite：
  - MySQL：`mysql://user:pass@host:3306/qiluoopen`
  - SQLite：`sqlite://data/qiluo.db?mode=rwc`

#### 3) 连接池与日志
- `min_connections`、`max_connections`、`connect_timeout`、`idle_timeout` 可调优性能。
- `enable_logging` 为 SQL 语句日志开关。

#### 4) 模型与结构
- 领域模型位于 `src/model/`：
  - `sys/args`：请求/响应参数对象（如 `LoginParams`, `CaptchaImage`）
  - `sys/model`：数据库实体与查询结构（如 `SysUserModel` 等）

#### 5) 常见问题
- 权限或用户初始化失败：确认已导入 `qiluo.sql` 并存在默认账户。
- 连接失败：检查 `DATABASE_URL`、数据库是否启动、账号权限与防火墙。
