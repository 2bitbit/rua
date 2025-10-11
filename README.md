# Rua（意为rua不死，象征一直rua）

## 用户手册
[[粘贴 cargo doc 的内容]]

## 主要功能：
1. `rua`进入rua列表，呈现一行行的记录。格式：key : command，按command的字典序排列。用户执行`rua`后按下对应的一个键盘键，就能直接执行对应的命令。（直接退出列表，并且把rua命令的执行记录删去，只留下对应命令的执行记录）（如果不存在则保持不动。按esc退出rua列表。注意不要打印改条rua历史记录）

2. `rua <key> <command>` 把一个command添加到rua列表

是否合适？

”安全 (Safety) 和 无畏并发 (Concurrency)。“的优点如何融进去？

先前的redis基础能否创意融进去？或作为一个扩展功能？

如果你哪点不清楚，请随时问我。

## Code Joke
Rubus means "rua 不死", in contrast to "rua 嘀死"(`Rudis`).