
# 🦫 busy-beaver-rs
<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/language-Rust-orange?logo=rust&logoColor=white" alt="Rust" />
  </a>
  <a href="https://doc.rust-lang.org/cargo/">
    <img src="https://img.shields.io/badge/build-Cargo-informational?logo=rust&logoColor=white" alt="Cargo" />
  </a>
  <a href="https://en.wikipedia.org/wiki/Busy_beaver">
    <img src="https://img.shields.io/badge/topic-Busy%20Beaver-blueviolet" alt="Busy Beaver" />
  </a>
  <a href="https://www.docker.com/">
    <img src="https://img.shields.io/badge/container-Docker-blue?logo=docker&logoColor=white" alt="Docker" />
  </a>
  <a href="https://docs.rs/rayon/latest/rayon/">
    <img src="https://img.shields.io/badge/parallel-RayON-green" alt="Rayon" />
  </a>
  <a href="#">
    <img src="https://img.shields.io/badge/status-active%20development-yellow" alt="Status" />
  </a>
</p>

`busy-beaver-rs` 是一个用 **Rust** 编写的 Busy Beaver 搜索实验项目。

它源自一个经典问题：

> 在 `n` 个状态、`2` 个符号组成的所有图灵机状态转移表中，枚举所有可能的“写入 / 移动 / 跳转”组合；  
> 从全 `0` 磁带开始运行，只保留最终会 `Halt` 的机器；  
> 然后统计其中：
>
> - 停机时磁带上留下 `1` 最多的机器
> - 运行步数最多的机器

---

## 🧠 Busy Beaver 是什么？

对于 `BB(n)`，我们通常关注两个值：

| 记号 | 含义 |
|---|---|
| `Σ(n)` | 所有会停机的 `n` 状态机器中，停机时磁带上留下 `1` 的最大数量 |
| `S(n)` | 所有会停机的 `n` 状态机器中，运行步数的最大值 |

本项目当前目标不是直接挑战理论极限，而是先从工程角度完成：

```text
BB(2) -> BB(3) -> 优化搜索性能 -> 再评估 BB(4) / BB(5)
````

---

## 📈 为什么这个问题很难？

Busy Beaver 的搜索空间增长极快。

对于 `n` 个状态、`2` 个符号的图灵机，每个规则 key 可以理解为：

```text
状态 × 当前符号
```

例如 `BB(3)` 中有：

```text
A0, A1
B0, B1
C0, C1
```

一共有：

```text
n × 2
```

个规则槽位。

每个槽位都要选择一个转移规则：

```text
写入符号 / 移动方向 / 下一状态
```

所以随着 `n` 增大，候选机器数量会爆炸式增长。

例如粗略理解：

```text
BB(2): 搜索空间还能接受
BB(3): 已经明显吃力
BB(4): 朴素暴力搜索基本不可接受
BB(5): 需要更强的理论剪枝与工程优化
```

因此本项目会按版本逐步优化：

```text
结果语义拆分
数据结构优化
数组引擎
多核并行
候选机器编码
剪枝策略
```

---

## 🚀 Quick Start

### 1. 启动开发环境

```bash
docker compose up -d
```

### 2. 进入容器

```bash
docker compose exec rust-bb bash
```

### 3. 编译 / 运行 / 测试

```bash
cargo build
cargo run
cargo test
```

---

## 🧪 运行示例

默认运行 `BB(2)`：

```bash
cargo run
```

显式运行 `BB(2)`：

```bash
cargo run -- 2
```

运行 `BB(3)`：

```bash
cargo run -- 3
```

指定最大运行步数和进度输出间隔：

```bash
cargo run -- 3 1000 100000
```

第三个参数为 `0` 时关闭进度输出：

```bash
cargo run -- 2 100 0
```

---

## 📊 当前进度

目前已经完成：

* `BB(2)` 的完整枚举与运行流程
* `BB(3)` 的基础流程
* 基础状态转移表生成
* 基础图灵机模拟器
* 停机机器统计
* `Σ(n)` 和 `S(n)` 冠军组拆分
* 搜索进度输出
* 运行耗时统计
* 数组形式状态转移表
* 双向数组纸带
* 冠军组输出默认只展示前 5 台机器，避免终端输出过长

当前观察：

```text
BB(2): 可以跑完，数组引擎后在当前开发环境可进入亚秒级
BB(3): 明显吃力，在 Jetson 上运行 30 分钟仍未完成，继续等待意义不大，没有进入不知道是卡住了还是真的没跑通。
```

这也说明当前版本仍然是朴素搜索实现，后续需要重点优化数据结构和并行搜索。

---

## 🧱 当前核心流程

```text
生成所有状态转移表
        ↓
逐台运行图灵机
        ↓
判断是否 Halt
        ↓
统计停机步数与磁带上的 1
        ↓
分别更新 Σ(n) 与 S(n) 冠军组
```

---

## 🗺️ Roadmap

### ✅ v0.2-basic-search

目标：完成基础 Busy Beaver 搜索流程。

* [x] 枚举状态转移表
* [x] 模拟图灵机运行
* [x] 支持 `BB(2)`
* [x] 初步支持 `BB(3)`
* [x] 基础结果统计

---

### ✅ v0.3-result-split

目标：让结果语义更加清晰。

* [x] 将最终结果拆分为两类冠军组

  * [x] `Σ(n)`：停机时留下 `1` 最多的机器
  * [x] `S(n)`：运行步数最多的机器
* [x] 输出最长停机步数冠军组
* [x] 输出最多 `1` 冠军组
* [x] 增加搜索进度输出
* [x] 增加运行耗时统计

---

### ✅ v0.4-array-engine

目标：优化单线程性能。

当前版本中，规则表已经从 `HashMap` 改为数组索引形式。

规则 key 可以编码为：

```text
索引 = 状态索引 × 符号个数 + 当前符号
```

由于符号只有 `0` 和 `1`，所以可以写成：

```text
index = state_index * 2 + symbol
```

例如：

```text
A0 -> 0 * 2 + 0 = 0
A1 -> 0 * 2 + 1 = 1

B0 -> 1 * 2 + 0 = 2
B1 -> 1 * 2 + 1 = 3

C0 -> 2 * 2 + 0 = 4
C1 -> 2 * 2 + 1 = 5
```

计划优化项：

* [x] 状态使用 `u8` 编码
* [x] 符号使用 `u8` 编码
* [x] 转移表由 `HashMap` 改为数组 / `Vec`
* [x] 纸带改为双向 `Vec<u8>`
* [x] 规则查找从哈希查找改为数组下标访问
* [x] 避免每台机器生成时产生大量 `HashMap` 所有权转移
* [x] 降低 clone / insert / remove 带来的开销

---

### 🔜 v0.5-parallel-search

目标：吃满多核，提高搜索吞吐。

计划引入 `rayon` 数据并行库。

优化方向：

* [ ] 引入 `rayon`
* [ ] 增加机器 ID，用于编码候选机器
* [ ] 并行遍历候选机器空间
* [ ] 每个线程局部统计结果
* [ ] 最后合并全局冠军组
* [ ] 减少线程间锁竞争
* [ ] 支持进度输出

---

### 🧪 v0.6-pruning

目标：引入基础剪枝策略。

候选方向：

* [ ] 跳过明显不可达状态
* [ ] 跳过永不停机的简单循环
* [ ] 状态重命名归一化
* [ ] 左右对称消除
* [ ] 起始转移规则约束
* [ ] 最大步数限制
* [ ] 最大纸带边界限制

---

## 🧩 后续目标

短期目标：

```text
稳定跑完 BB(2)
拆分 Σ(n) 与 S(n) 结果
优化后尝试跑完 BB(3)
```

中期目标：

```text
数组引擎
多线程搜索
候选机器 ID 编码
进度统计
```

长期目标：

```text
尝试 BB(4)
研究剪枝策略
对比已知 Busy Beaver 结果
```

---

## 🛠️ Tech Stack

| 模块              | 技术                      |
| --------------- | ----------------------- |
| Language        | Rust                    |
| Build           | Cargo                   |
| Container       | Docker / Docker Compose |
| Parallel Search | Rayon                   |
| Target Platform | arch_64 / Jetson         |

---

## 📌 Project Status

当前项目处于早期实验阶段。

目前重点不是追求理论最优，而是逐步完成一个可解释、可优化、可扩展的 Busy Beaver 搜索器。

```text
先跑通
再跑快
再跑大
再剪枝
```
## 📜 Changelog

项目更新记录见：[CHANGELOG.rst](./CHANGELOG.rst)
