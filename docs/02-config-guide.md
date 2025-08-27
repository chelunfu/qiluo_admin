### 配置说明：development.yaml 与环境变量

#### 配置文件位置
`config/development.yaml` 控制服务、数据库、缓存、日志等关键参数。

#### 关键段落
- `server`：
  - `domainname`: 生成头像等完整资源 URL 的前缀（如 `http://localhost:5001`）
  - `binding`: 监听地址，默认 `0.0.0.0`
  - `host`: 对外展示主机名（用于日志、友好显示）
  - `port`: 监听端口，默认 `5001`
  - `static_dir` / `web_dir` / `upload_dir`：静态/前端/上传目录
  - `ssl.enable`：是否启用 HTTPS（证书路径见 `ssl.key`/`ssl.cert`）
  - `middlewares.cors.allow_origins`：允许的跨域来源（默认包含 `http://localhost:4000`）

- `database`：
  - `uri`：数据库连接串，支持模板 `{{ get_env(name="DATABASE_URL", default="...") }}`
  - `auto_migrate`：启动时自动迁移（建议开发开启，生产关闭）
  - 连接池参数：`min_connections`/`max_connections`/`connect_timeout`/`idle_timeout`

- `cache`：
  - `cache_type`: `redis` 或 `memory`
  - `url`: Redis 地址，支持 `{{ get_env(name="REDIS_URL", default="redis://127.0.0.1") }}`
  - `namespace`：键前缀，`pool_size`：连接池大小

- `auth.jwt`：
  - `secret`：JWT 密钥（务必在生产更换）
  - `expiration`：过期秒数（默认 7 天）

- `logger`：
  - `level`: `trace|debug|info|warn|error`
  - `format`: `compact|pretty|json`
  - `log_dir`/`file_name`：文件日志目录与前缀

#### 使用环境变量覆盖
YAML 模板支持 `get_env`：
```yaml
database:
  uri: {{ get_env(name="DATABASE_URL", default="mysql://qiluo:Qiluo123@localhost:3306/qiluoopen") }}
cache:
  url: {{ get_env(name="REDIS_URL", default="redis://127.0.0.1") }}
```
在 shell 中：
```bash
export DATABASE_URL="mysql://user:pass@localhost:3306/qiluoopen"
export REDIS_URL="redis://127.0.0.1"
```

#### 端口与地址策略
- 对内监听：`binding: 0.0.0.0`，`port: 5001`
- 对外展示：`domainname` 与 `host`，分别用于资源拼接与日志显示
