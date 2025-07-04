/**
学习路线图
我们将分成 5 个阶段来掌握 Tokio：
--------------------------------

📌 第 1 阶段：Tokio 基础概念
什么是 async/await？
为什么需要 Tokio？
Tokio 与标准库的 async 有何不同？


📌 第 2 阶段：Tokio 基础用法
tokio::main 宏
tokio::spawn 并发任务
tokio::sleep、定时器等工具


📌 第 3 阶段：异步 IO 编程
使用 TcpListener / TcpStream 创建异步 TCP 服务
处理多个客户端的并发连接


📌 第 4 阶段：通道通信 & 任务协调
tokio::sync::mpsc 用于异步任务之间通信
tokio::select! 用于监听多个事件

Tokio 异步通道概览
| 通道类型        | 特点               | 用途                      |
| ----------- | ---------------- | ----------------------- |
| `mpsc`      | 多发送者，单接收者        | 任务之间的单向消息发送             |
| `oneshot`   | 一次性单发单收          | 通常用于请求-响应模式             |
| `broadcast` | 多发多收（发布/订阅）      | 已学，用于聊天室广播              |
| `watch`     | 单值广播，所有订阅者可获取最新值 | 类似广播但仅保留**最新值**，常用于状态传播 |



📌 第 5 阶段：高级功能 & 真实项目实践
使用 tokio::task::JoinHandle 控制任务生命周期
异步文件 IO、异步 DNS、异步数据库连接（如：SQLx）

*/

pub mod c1;
pub mod phase1;
pub mod phase2;
pub mod phase3;
pub mod phase4;