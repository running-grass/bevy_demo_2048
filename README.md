# 2048 小游戏

基于Rust Bevy游戏引擎的 2048 小游戏

## 已完成
- 游戏主体
- 基于actions自动部署到github page

## TODO
- 使用 matrix 重构数据结构及矩阵运算
- 生成微信小程序
- 优化代码体积

## Web使用
### 安装 trunk
```bash
cargo install trunk
```

### 启动开发模式
```bash
trunk serve
```

### 构建
```bash
trunk build
```

### 发布模式
```bash
trunk build --release
```