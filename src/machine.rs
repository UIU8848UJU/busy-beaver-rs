//! 单台图灵机执行器
//!
//! `Machine` 不负责搜索，也不负责生成规则表
//! 它只负责：给定一张转移表，从空白纸带开始一步一步运行

use std::collections::HashMap;

use crate::model::{Direction, RuleKey, State, Symbol, Transition};
use crate::tape::Tape;

/// @brief 状态转移表
pub type TransitionTable = HashMap<RuleKey, Transition>;

/// @brief 单台机器的运行状态
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Running,           // 机器正在运行
    Halted,            // 机器停机
    MissingTransition, // 机器遇到了未定义的转移规则
    StepLimitReached,  // 机器运行超过了预设的最大步数应对与无解的情况
}

/// @brief 图灵机模拟器。
#[derive(Clone, Debug)]
pub struct MachineSnapshot {
    pub state: State,
    pub head: i64,
    pub steps: u64,
    pub ones: usize,
}

#[derive(Clone, Debug)]
pub struct Machine {
    table: TransitionTable,
    tape: Tape,
    head: i64,
    state: State,
    steps: u64,
}

impl Machine {
    /// @brief 创建一台机器。
    ///
    /// @param table 状态转移表。
    /// @return 从 A 状态、head=0、空白纸带开始的新机器。
    pub fn new(table: TransitionTable) -> Self {
        Self {
            table,
            tape: Tape::new(),
            head: 0,
            state: State::Normal(0),
            steps: 0,
        }
    }

    /// @brief 执行单步转移。
    ///
    /// @return 当前运行状态。
    pub fn step(&mut self) -> RunStatus {
        if self.state == State::Halt {
            return RunStatus::Halted;
        }

        let symbol = self.tape.read(self.head);
        let key = (self.state, symbol);

        let transition = match self.table.get(&key) {
            Some(transition) => *transition,
            None => return RunStatus::MissingTransition,
        };

        self.tape.write(self.head, transition.write);

        match transition.direction {
            Direction::Left => self.head -= 1,
            Direction::Right => self.head += 1,
        }

        self.state = transition.next_state;
        self.steps += 1;

        if self.state == State::Halt {
            RunStatus::Halted
        } else {
            RunStatus::Running
        }
    }

    /// @brief 持续运行，直到停机或达到最大步数。
    ///
    /// @param max_steps 最大运行步数。
    /// @return 最终运行状态。
    pub fn run(&mut self, max_steps: u64) -> RunStatus {
        while self.steps < max_steps {
            match self.step() {
                RunStatus::Running => {}
                status => return status,
            }
        }

        RunStatus::StepLimitReached
    }

    /// @brief 获取已运行步数。
    pub fn steps(&self) -> u64 {
        self.steps
    }

    /// @brief 获取纸带上 1 的数量。
    pub fn ones(&self) -> usize {
        self.tape.count_ones()
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn snapshot(&self) -> MachineSnapshot {
        MachineSnapshot {
            state: self.state,
            head: self.head,
            steps: self.steps,
            ones: self.ones(),
        }
    }

    pub fn table(&self) -> &TransitionTable {
        &self.table
    }
}
