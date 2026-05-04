//! Busy Beaver 搜索器。
//!
//! 搜索器负责组合生成器和机器执行器：
//! 生成机器 -> 运行机器 -> 判断停机 -> 记录最优。

use crate::generator::MachineGenerator;
use crate::machine::{Machine, RunStatus, TransitionTable};

// @brief Busy Beaver 搜索配置。
#[derive(Clone, Copy, Debug)]
pub struct SearchConfig {
    pub state_count: u8,  // 普通状态数量，例如 BB(2) 为 2，BB(3) 为 3。
    pub symbol_count: u8, // 符号数量，经典 Busy Beaver 为 2。
    pub max_steps: u64,   // 最大运行步数，用于防止非停机机器无限运行。
}

impl SearchConfig {
    /// @brief 创建搜索配置。
    pub fn new(state_count: u8, symbol_count: u8, max_steps: u64) -> Self {
        Self {
            state_count,
            symbol_count,
            max_steps,
        }
    }

    /// @brief BB(2) 默认配置。
    pub fn bb2() -> Self {
        Self::new(2, 2, 100)
    }

    /// @brief BB(3) 默认配置。
    pub fn bb3() -> Self {
        Self::new(3, 2, 100)
    }
}

/// @brief 搜索结果。
#[derive(Clone, Debug)]
pub struct SearchResult {
    pub total_machines: usize,
    pub halted_machines: usize,
    pub step_limited_machines: usize,
    pub missing_transition_machines: usize,
    pub best_ones: usize,
    pub best_steps: u64,
    pub best_table: Option<TransitionTable>,
}

impl SearchResult {
    /// @brief 创建空搜索结果。
    pub fn new() -> Self {
        Self {
            total_machines: 0,
            halted_machines: 0,
            step_limited_machines: 0,
            missing_transition_machines: 0,
            best_ones: 0,
            best_steps: 0,
            best_table: None,
        }
    }
}

/// @brief Busy Beaver 搜索器。
pub struct BusyBeaverSearch {
    config: SearchConfig,
}

impl BusyBeaverSearch {
    /// @brief 创建搜索器。
    pub fn new(config: SearchConfig) -> Self {
        Self { config }
    }

    /// @brief 执行搜索。
    ///
    /// @return 搜索统计结果和当前最优机器。
    pub fn run(&self) -> SearchResult {
        let generator = MachineGenerator::new(self.config.state_count, self.config.symbol_count);

        let mut result = SearchResult::new();

        generator.for_each(|table| {
            result.total_machines += 1;

            let mut machine = Machine::new(table.clone());
            let status = machine.run(self.config.max_steps);

            match status {
                RunStatus::Halted => {
                    result.halted_machines += 1;
                    Self::try_update_best(&mut result, &machine, table);
                }
                RunStatus::StepLimitReached => {
                    result.step_limited_machines += 1;
                }
                RunStatus::MissingTransition => {
                    result.missing_transition_machines += 1;
                }
                RunStatus::Running => {}
            }
        });

        result
    }

    /// @brief 尝试更新当前最优机器。
    ///
    /// 当前策略：
    /// 1. 优先比较纸带上 1 的数量；
    /// 2. 如果 1 的数量相同，再比较运行步数。
    fn try_update_best(result: &mut SearchResult, machine: &Machine, table: TransitionTable) {
        let ones = machine.ones();
        let steps = machine.steps();

        if ones > result.best_ones {
            result.best_ones = ones;
            result.best_steps = steps;
            result.best_table = Some(table);
            return;
        }

        if ones == result.best_ones && steps > result.best_steps {
            result.best_steps = steps;
            result.best_table = Some(table);
        }
    }
}
