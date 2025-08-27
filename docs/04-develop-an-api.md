### API 开发：新增一个系统接口

以在 `/api/sys` 下新增一个受保护的接口为例。

#### 1) 选择模块并添加路由
- 系统接口集中在 `src/api/sys_controll/mod.rs`。
- 参考已有分组（如 `sys_user()`、`menu()`），新增函数并在 `router_sys()` 中通过 `.nest("/xxx", your_fn())` 挂载。

示例（伪代码）：
```rust
fn sys_hello() -> WebPath {
    WebPath::new().route(
        "/hello",
        WebPathType::Get,
        Some("问候"),
        get(your_service::hello),
    )
}
```

#### 2) 编写处理函数
- 在对应的 `src/service/sys/` 下创建 `your_service.rs`，实现 `async fn hello(...) -> impl IntoResponse`。
- 可通过注入 `UserInfo` 获取当前登录用户。

示例（伪代码）：
```rust
pub async fn hello(user: UserInfo) -> impl IntoResponse {
    ApiResponse::ok(format!("hello, {}", user.username))
}
```

#### 3) 编译运行并测试
```bash
cargo run
```
调用：
- `GET /api/sys/hello`，Header：`Authorization: Bearer <jwt>`。

#### 4) API 自动登记说明
- 路由汇总在 `WebApi::routers()` 中展开后，会通过异步任务将接口信息更新到权限系统中（用于 API 权限管理）。

#### 5) 白名单接口（可选）
- 将无需鉴权的接口挂到 `white_sys()` 中：
```rust
pub fn white_sys() -> Router {
    Router::new().nest(
        "/sys",
        Router::new()
            .nest("/auth", auth())
            .route("/hello_public", get(hello_public)),
    )
}
```
