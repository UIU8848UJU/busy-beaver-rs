//! Busy Beaver 的领域模型定义。
//!
//! 这里放状态、符号、方向、转移规则等基础类型。
//! 这些类型本身不负责执行，只负责描述“机器长什么样”。

/// @brief 纸带符号类型。
///
/// 当前只研究经典二符号 Busy Beaver，因此符号为 0 或 1。
pub type Symbol = u8;

/// @brief 图灵机状态。
///
/// `Normal(0)` 表示 A 状态，`Normal(1)` 表示 B 状态，
/// `Normal(2)` 表示 C 状态，以此类推。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum State {
    Normal(u8),
    Halt,
}

/// @brief 读写头移动方向。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

/// @brief 一条状态转移规则。
///
/// 对应 Busy Beaver 规则中的：
///
/// 当前状态 + 当前符号 -> 写入符号 + 移动方向 + 下一个状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Transition {
    pub write: Symbol,
    pub direction: Direction,
    pub next_state: State,
}
