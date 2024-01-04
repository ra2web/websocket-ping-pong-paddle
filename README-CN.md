# WebSocket 乒乓拍

[English Version Doc](README.md)

## 介绍
WebSocket 乒乓拍是一个简单而强大的工具，使用Rust编写，为服务器和客户端提供实时、双向的通信。它的设计目的是处理WebSocket连接中的Ping和Pong帧，以维护WebSocket连接并确保其活跃性。

这是绝佳的给Rust入门者学习基于Websocket编程的案例。同时也是网页红警RA2WEB提供给有意向参与网关编写的同学的入门项目。

## 功能
- 监听传入的WebSocket连接。
- 对Ping帧进行Pong帧响应。
- 原样返回非标准的Ping消息。
- 使用异步编程模型，有效利用系统资源。

## 使用场景
这个工具可以用于：
- 本地或远程调试WebSocket连接。
- 通过定期发送Ping帧并期待Pong帧来测试WebSocket连接的活跃性。
- 反射非标准的Ping消息，这可以用于WebSocket上的自定义通信协议。

## 未来展望
随着项目的发展，我们计划添加更多功能，如：
- 可定制的Ping和Pong消息。
- 连接健康和性能的日志和度量。
- 支持安全的WebSocket连接（wss://）。

## 入门指南
要使用WebSocket乒乓拍，请按照以下步骤操作：

1. 克隆仓库：
   ```bash
   git clone https://github.com/ra2web/websocket-ping-pong-paddle.git
   ```

2. 导航到项目目录：
   ```bash
   cd websocket-ping-pong-paddle
   ```

3. 使用默认设置运行程序（监听0.0.0.0:11451）有点臭：
   ```bash
   cargo run
   ```

   或指定自定义地址和端口：
   ```bash
   cargo run 127.0.0.1 12345
   ```

## 贡献
尽管这个鸟东西不大，但是也欢迎贡献！如果您遇到任何问题或有改进的建议，请在GitHub仓库上打开一个问题或提交一个拉取请求。

## 许可证
[Apache License 2.0](LICENSE).
