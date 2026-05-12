//! Busy Beaver 搜索器。
//!
//! 搜索器负责组合生成器和机器执行器：
//! 生成机器 -> 运行机器 -> 判断停机 -> 记录最优。

use crate::generator::MachineGenerator;
use crate::machine::{Machine, RunStatus, TransitionTable};
use std::time::{Duration, Instant};

// @brief Busy Beaver 搜索配置。
#[derive(Clone, Copy, Debug)]
pub struct SearchConfig {
    pub state_count: u8,          // 普通状态数量，例如 BB(2) 为 2，BB(3) 为 3。
    pub symbol_count: u8,         // 符号数量，经典 Busy Beaver 为 2。
    pub max_steps: u64,           // 最大运行步数，用于防止非停机机器无限运行。
    pub progress_interval: usize, // 每搜索多少台机器输出一次进度；0 表示不输出。
}

impl SearchConfig {
    /// @brief 创建搜索配置。
    pub fn new(state_count: u8, symbol_count: u8, max_steps: u64) -> Self {
        Self {
            state_count,
            symbol_count,
            max_steps,
            progress_interval: 10_000,
        }
    }

    /// @brief 设置搜索进度输出间隔。
    pub fn with_progress_interval(mut self, progress_interval: usize) -> Self {
        self.progress_interval = progress_interval;
        self
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

/// @brief 一类 Busy Beaver 冠军组。
#[derive(Clone, Debug)]
pub struct ChampionGroup {
    pub best_ones: usize,
    pub best_steps: u64,
    pub tables: Vec<TransitionTable>,
}

impl ChampionGroup {
    /// @brief 创建空冠军组。
    pub fn new() -> Self {
        Self {
            best_ones: 0,
            best_steps: 0,
            tables: Vec::new(),
        }
    }

    /// @brief 是否找到了至少一台冠军机器。
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    /// @brief 冠军机器数量。
    pub fn len(&self) -> usize {
        self.tables.len()
    }
}

/// @brief 搜索结果。
#[derive(Clone, Debug)]
pub struct SearchResult {
    pub total_machines: usize,
    pub halted_machines: usize,
    pub step_limited_machines: usize,
    pub missing_transition_machines: usize,
    pub sigma_champions: ChampionGroup,
    pub step_champions: ChampionGroup,
    pub elapsed: Duration,
}

impl SearchResult {
    /// @brief 创建空搜索结果。
    pub fn new() -> Self {
        Self {
            total_machines: 0,
            halted_machines: 0,
            step_limited_machines: 0,
            missing_transition_machines: 0,
            sigma_champions: ChampionGroup::new(),
            step_champions: ChampionGroup::new(),
            elapsed: Duration::ZERO,
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
        let started_at = Instant::now();

        generator.for_each(|table| {
            result.total_machines += 1;

            let mut machine = Machine::new(table.clone());
            let status = machine.run(self.config.max_steps);

            match status {
                RunStatus::Halted => {
                    result.halted_machines += 1;
                    Self::try_update_champions(&mut result, &machine, &table);
                }
                RunStatus::StepLimitReached => {
                    result.step_limited_machines += 1;
                }
                RunStatus::MissingTransition => {
                    result.missing_transition_machines += 1;
                }
                RunStatus::Running => {}
            }

            if self.should_print_progress(result.total_machines) {
                println!(
                    "[progress] searched={} halted={} step_limited={} missing={} elapsed={:.2?}",
                    result.total_machines,
                    result.halted_machines,
                    result.step_limited_machines,
                    result.missing_transition_machines,
                    started_at.elapsed()
                );
            }
        });

        result.elapsed = started_at.elapsed();
        result
    }

    /// @brief 判断当前搜索数量是否应该打印进度。
    fn should_print_progress(&self, total_machines: usize) -> bool {
        self.config.progress_interval > 0 && total_machines % self.config.progress_interval == 0
    }

    /// @brief 分别更新 Sigma(n) 和 S(n) 冠军组。
    fn try_update_champions(result: &mut SearchResult, machine: &Machine, table: &TransitionTable) {
        let ones = machine.ones();
        let steps = machine.steps();

        Self::try_update_sigma_champions(&mut result.sigma_champions, ones, steps, table);
        Self::try_update_step_champions(&mut result.step_champions, ones, steps, table);
    }

    /// @brief 更新 Sigma(n)：停机时纸带上 1 最多的冠军组。
    fn try_update_sigma_champions(
        champions: &mut ChampionGroup,
        ones: usize,
        steps: u64,
        table: &TransitionTable,
    ) {
        if champions.is_empty() || ones > champions.best_ones {
            champions.best_ones = ones;
            champions.best_steps = steps;
            champions.tables.clear();
            champions.tables.push(table.clone());
            return;
        }

        if ones == champions.best_ones {
            champions.best_steps = champions.best_steps.max(steps);
            champions.tables.push(table.clone());
        }
    }

    /// @brief 更新 S(n)：运行步数最多的停机冠军组。
    fn try_update_step_champions(
        champions: &mut ChampionGroup,
        ones: usize,
        steps: u64,
        table: &TransitionTable,
    ) {
        if champions.is_empty() || steps > champions.best_steps {
            champions.best_ones = ones;
            champions.best_steps = steps;
            champions.tables.clear();
            champions.tables.push(table.clone());
            return;
        }

        if steps == champions.best_steps {
            champions.best_ones = champions.best_ones.max(ones);
            champions.tables.push(table.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BusyBeaverSearch, SearchConfig};

    #[test]
    fn bb2_search_tracks_sigma_and_step_champions_separately() {
        let config = SearchConfig::bb2().with_progress_interval(0);
        let result = BusyBeaverSearch::new(config).run();

        assert_eq!(result.total_machines, 20_736);
        assert_eq!(result.sigma_champions.best_ones, 4);
        assert_eq!(result.sigma_champions.best_steps, 6);
        assert!(!result.sigma_champions.is_empty());

        assert_eq!(result.step_champions.best_steps, 6);
        assert_eq!(result.step_champions.best_ones, 4);
        assert!(!result.step_champions.is_empty());
    }
}
