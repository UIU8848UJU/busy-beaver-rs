//! 无限纸带抽象。
//!
//! Busy Beaver 的纸带理论上是无限长的，初始全为 0。
//! 工程上按需扩展正向和负向两段数组。

use crate::model::Symbol;

/// @brief 稀疏纸带。
///
/// 非负坐标存入 `positive_cells`，负坐标存入 `negative_cells`。
/// 负坐标映射规则：-1 -> 0，-2 -> 1，以此类推。
#[derive(Clone, Debug, Default)]
pub struct Tape {
    positive_cells: Vec<Symbol>,
    negative_cells: Vec<Symbol>,
}

impl Tape {
    /// @brief 创建一条空白纸带。
    pub fn new() -> Self {
        Self {
            positive_cells: Vec::new(),
            negative_cells: Vec::new(),
        }
    }

    /// @brief 读取指定位置的符号。
    ///
    /// @param position 纸带坐标。
    /// @return 当前坐标上的符号；如果没有写过，则返回 0。
    pub fn read(&self, position: i64) -> Symbol {
        let (cells, index) = self.cells_and_index(position);
        cells.get(index).copied().unwrap_or(0)
    }

    /// @brief 写入指定位置的符号。
    ///
    /// @param position 纸带坐标。
    /// @param symbol 要写入的符号。
    pub fn write(&mut self, position: i64, symbol: Symbol) {
        let (cells, index) = self.cells_and_index_mut(position);

        if index >= cells.len() {
            cells.resize(index + 1, 0);
        }

        cells[index] = symbol;
    }

    /// @brief 统计纸带上 1 的数量。
    pub fn count_ones(&self) -> usize {
        self.positive_cells.iter().filter(|&&s| s == 1).count()
            + self.negative_cells.iter().filter(|&&s| s == 1).count()
    }

    /// @brief 当前已分配的纸带单元数量。
    pub fn allocated_cells(&self) -> usize {
        self.positive_cells.len() + self.negative_cells.len()
    }

    fn cells_and_index(&self, position: i64) -> (&[Symbol], usize) {
        if position >= 0 {
            (&self.positive_cells, position as usize)
        } else {
            (&self.negative_cells, (-position - 1) as usize)
        }
    }

    fn cells_and_index_mut(&mut self, position: i64) -> (&mut Vec<Symbol>, usize) {
        if position >= 0 {
            (&mut self.positive_cells, position as usize)
        } else {
            (&mut self.negative_cells, (-position - 1) as usize)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tape;

    #[test]
    fn tape_reads_unallocated_cells_as_zero() {
        let tape = Tape::new();

        assert_eq!(tape.read(0), 0);
        assert_eq!(tape.read(10), 0);
        assert_eq!(tape.read(-10), 0);
    }

    #[test]
    fn tape_supports_positive_and_negative_positions() {
        let mut tape = Tape::new();

        tape.write(0, 1);
        tape.write(3, 1);
        tape.write(-1, 1);
        tape.write(-4, 1);

        assert_eq!(tape.read(0), 1);
        assert_eq!(tape.read(3), 1);
        assert_eq!(tape.read(-1), 1);
        assert_eq!(tape.read(-4), 1);
        assert_eq!(tape.count_ones(), 4);
    }
}
