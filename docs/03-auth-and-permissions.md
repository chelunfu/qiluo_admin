### 认证与鉴权：Captcha 与 JWT 登录

本文介绍登录流程、验证码接口、以及携带 JWT 访问受保护接口的方法。

#### 1) 获取验证码（白名单接口）
- 路径：`GET /api/sys/auth/get_captcha`
- Query：`client_id=<字符串>`
- 响应：
```json
{
  "captcha_on_off": true,
  "uuid": "1234567890123456789",
  "img": "data:image/png;base64,..."
}
```

#### 2) 登录获取 JWT（白名单接口）
- 路径：`POST /api/sys/auth/login`
- Body（JSON）：
```json
{
  "username": "admin",
  "password": "123456",
  "captchaid": "<上一步返回的 uuid>",
  "client_id": "<与上一步相同>",
  "captcha": "<图片上的文本>",
  "email": null,
  "phone": null
}
```
- 成功响应：
```json
{
  "token": {
    "token": "<jwt-string>",
    "token_type": "Bearer",
    "exp": 1710000000,
    "exp_in": 604800
  },
  "userinfo": {
    "username": "admin",
    "nickname": "Admin",
    "email": null,
    "phone": null,
    "avatar": "http://localhost:5001/<path>"
  }
}
```

#### 3) 刷新 Token（受保护接口）
- 路径：`PUT /api/sys/user/refersh_token`
- Header：`Authorization: Bearer <jwt>`
- 响应：与 `token` 结构相同。

#### 4) 访问受保护接口
- 受保护的系统接口挂在 `/api/sys/...`，需要在 Header 中携带 `Authorization: Bearer <jwt>`。

#### 5) 权限模型
- 框架采用 RBAC：用户-角色-菜单/API 多维度权限。
- 中间件链路：`AuthMid`（JWT 解析、白名单校验）→ `ApiMid`（接口权限）→ `OperateLogMid`（操作日志）。

#### 6) 常见错误
- `Invalid or expired verification code`：验证码 uuid 过期或无效。
- `Incorrect password`：用户名/密码不正确。
- `Invalid token`/`Token expired`：JWT 无效或过期。
