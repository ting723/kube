# 构建阶段
FROM rust:alpine AS builder

# 替换Alpine镜像源为国内源（阿里云）
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 设置Rust代理环境变量
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV CARGO_REGISTRIES_CRATES_IO_REPLACE_WITH=rsproxy
ENV CARGO_REGISTRIES_RSPROXY_INDEX=https://rsproxy.cn/crates.io-index
ENV RUSTUP_DIST_SERVER=https://rsproxy.cn/rustup
ENV RUSTUP_UPDATE_ROOT=https://rsproxy.cn/rustup/rustup

# 安装构建依赖
RUN apk add --no-cache musl-dev git

# 设置工作目录
WORKDIR /app

# 缓存依赖步骤
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs

# 添加musl目标
RUN rustup target add x86_64-unknown-linux-musl

# 构建依赖（缓存）
RUN cargo build --release --target x86_64-unknown-linux-musl

# 复制实际源代码
COPY src ./src

# 触发重新编译
RUN touch src/main.rs

# 构建项目
RUN cargo build --release --target x86_64-unknown-linux-musl

# 运行阶段（仅保留构建产物，无运行配置）
FROM alpine:latest

# 替换Alpine镜像源为国内源（阿里云）
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# 仅复制构建产物
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/kube-tui /app/release/linux/kube-tui