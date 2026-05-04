//! 无限纸带抽象。
//!
//! Busy Beaver 的纸带理论上是无限长的，初始全为 0。
//! 工程上不可能真的创建无限数组，因此这里只存储被写成非 0 的格子。

use std::collections::HashMap;

use crate::model::Symbol;

/// @brief 稀疏纸带。
///
/// 只保存写过的非零格子；未保存的位置默认读取为 0。
#[derive(Clone, Debug, Default)]
pub struct Tape {
    pub cells: HashMap<i64, Symbol>,
}

impl Tape {
    /// @brief 创建一条空白纸带。
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }

    /// @brief 读取指定位置的符号。
    ///
    /// @param position 纸带坐标。
    /// @return 当前坐标上的符号；如果没有写过，则返回 0。
    pub fn read(&self, position: i64) -> Symbol {
        *self.cells.get(&position).unwrap_or(&0)
    }

    /// @brief 写入指定位置的符号。
    ///
    /// @param position 纸带坐标。
    /// @param symbol 要写入的符号。
    ///
    /// 如果写入 0，就从稀疏表中删除该位置，保持“默认 0”的语义。
    pub fn write(&mut self, position: i64, symbol: Symbol) {
        if symbol == 0 {
            self.cells.remove(&position);
        } else {
            self.cells.insert(position, symbol);
        }
    }

    /// @brief 统计纸带上 1 的数量。
    pub fn count_ones(&self) -> usize {
        self.cells.values().filter(|&&s| s == 1).count()
    }

    /// @brief 获取底层存储，主要用于调试和测试。
    pub fn cells(&self) -> &HashMap<i64, Symbol> {
        &self.cells
    }
}
