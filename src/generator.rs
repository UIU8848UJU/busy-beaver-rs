//! 候选机器生成器。
//!
//! 这里负责枚举所有 n 状态、m 符号 Busy Beaver 候选机器。
//! 当前采用回溯 + 回调的方式：生成一张转移表，就立刻交给搜索器处理。

use crate::machine::TransitionTable;
use crate::model::{Direction, RuleKey, State, Symbol, Transition};

/// @brief 图灵机候选规则表生成器。
#[derive(Clone, Debug)]
pub struct MachineGenerator {
    state_count: u8,
    symbol_count: u8,
}

impl MachineGenerator {
    /// @brief 创建生成器。
    ///
    /// @param state_count 普通状态数量，例如 BB(3) 就是 3。
    /// @param symbol_count 符号数量，经典 Busy Beaver 为 2。
    pub fn new(state_count: u8, symbol_count: u8) -> Self {
        Self {
            state_count,
            symbol_count,
        }
    }

    /// @param callback 每生成一张完整转移表，就调用一次 callback。
    pub fn for_each<F>(&self, mut callback: F)
    where
        F: FnMut(TransitionTable),
    {
        let keys = self.generate_rule_keys();
        let candidates = self.generate_transition_candidates();

        let mut current = TransitionTable::new();

        self.backtrack_for_each(0, &keys, &candidates, &mut current, &mut callback);
    }

    /// @brief 生成所有规则入口。
    ///
    /// 对于 BB(3)，规则入口是：
    /// A0, A1, B0, B1, C0, C1。
    fn generate_rule_keys(&self) -> Vec<RuleKey> {
        let mut keys = Vec::new();

        for state_id in 0..self.state_count {
            for symbol in 0..self.symbol_count {
                keys.push((State::Normal(state_id), symbol));
            }
        }

        keys
    }

    /// @brief 生成单条规则所有可能的候选转移。
    ///
    /// 对于 BB(3)，一条规则有：
    /// 写 0/1，左/右移动，下一个状态 A/B/C/H。
    fn generate_transition_candidates(&self) -> Vec<Transition> {
        let mut candidates = Vec::new();

        for write in 0..self.symbol_count {
            for direction in [Direction::Left, Direction::Right] {
                for next_state in self.generate_next_states() {
                    candidates.push(Transition {
                        write,
                        direction,
                        next_state,
                    });
                }
            }
        }
        candidates
    }

    /// @brief 生成可跳转的下一个状态集合。
    ///
    /// 包含所有普通状态和 Halt 状态。
    fn generate_next_states(&self) -> Vec<State> {
        let mut states = Vec::new();

        for state_id in 0..self.state_count {
            states.push(State::Normal(state_id));
        }

        states.push(State::Halt);

        states
    }

    /// @brief 回溯生成完整转移表。
    ///
    /// 每个规则入口都尝试所有候选转移，直到组成一张完整表。
    fn backtrack_for_each<F>(
        &self,
        index: usize,
        keys: &[RuleKey],
        candidates: &[Transition],
        current: &mut TransitionTable,
        callback: &mut F,
    ) where
        F: FnMut(TransitionTable),
    {
        if index == keys.len() {
            callback(current.clone());
            return;
        }
        let key = keys[index];

        for transition in candidates {
            current.insert(key, *transition);
            self.backtrack_for_each(index + 1, keys, candidates, current, callback);
            current.remove(&key);
        }
    }
}
