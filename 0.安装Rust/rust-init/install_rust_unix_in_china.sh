#!/bin/sh
#临时切换镜像源
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
#使用阿里云安装脚本
curl --proto '=https' --tlsv1.2 -sSf https://mirrors.aliyun.com/repo/rust/rustup-init.sh | sh
