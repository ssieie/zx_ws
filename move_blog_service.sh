#!/bin/bash

PROJECT_DIR="/usr/local/blog/build/server"
TARGET_DIR="/usr/local/blog/build/server_build"

# 进入项目目录
cd "$PROJECT_DIR" || { echo "无法进入目录 $PROJECT_DIR"; exit 1; }

# 执行 Cargo 构建
cargo build --release

# 检查构建是否成功
if [ $? -ne 0 ]; then
    echo "构建失败"
    exit 1
fi

# 移动文件到目标目录
SOURCE_FILE="$PROJECT_DIR/target/release/blog-service"
DEST_FILE="$TARGET_DIR/blog-service"

# 检查源文件是否存在
if [ ! -f "$SOURCE_FILE" ]; then
    echo "源文件 $SOURCE_FILE 不存在"
    exit 1
fi

# 创建目标目录（如果不存在）
mkdir -p "$TARGET_DIR"

# 移动文件
mv "$SOURCE_FILE" "$DEST_FILE"

# 检查移动操作是否成功
if [ $? -eq 0 ]; then
    echo "文件成功移动到 $DEST_FILE"
else
    echo "文件移动失败"
    exit 1
fi

# 执行 blog-service
echo "正在执行 blog-service..."
"$DEST_FILE"

# 检查执行是否成功
if [ $? -eq 0 ]; then
    echo "blog-service 执行成功"
else
    echo "blog-service 执行失败"
    exit 1
fi