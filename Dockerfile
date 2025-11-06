# 构建阶段
FROM docker.1ms.run/rust:alpine AS builder

# 安装构建依赖
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    pkgconfig \
    perl \
    make

WORKDIR /app

COPY . .

# 安装 MUSL 目标并构建
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# 剥离调试符号
RUN strip target/x86_64-unknown-linux-musl/release/qiluo

# 检查二进制大小
RUN ls -lh target/x86_64-unknown-linux-musl/release/qiluo

# 运行时阶段
FROM docker.1ms.run/alpine:latest

# 在单层中完成所有设置
RUN apk add --no-cache ca-certificates && \
    addgroup -S qiluo && adduser -S qiluo -G qiluo && \
    mkdir -p /app/data && \
    chown -R qiluo:qiluo /app

WORKDIR /app

# 复制二进制文件并立即设置权限（避免额外层）
COPY --from=builder --chown=qiluo:qiluo /app/target/x86_64-unknown-linux-musl/release/qiluo .

USER qiluo

# 暴露端口
EXPOSE 5001

CMD ["./qiluo"]