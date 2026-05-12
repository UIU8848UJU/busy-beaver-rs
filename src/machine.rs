//! 单台图灵机执行器
//!
//! `Machine` 不负责搜索，也不负责生成规则表
//! 它只负责：给定一张转移表，从空白纸带开始一步一步运行

use crate::model::{Direction, State, Symbol, Transition};
use crate::tape::Tape;

/// @brief 数组形式的状态转移表。
///
/// 规则索引编码：
/// index = state_id * symbol_count + symbol。
#[derive(Clone, Debug)]
pub struct TransitionTable {
    symbol_count: u8,
    transitions: Vec<Transition>,
}

impl TransitionTable {
    /// @brief 创建一张数组状态转移表。
    pub fn new(symbol_count: u8, transitions: Vec<Transition>) -> Self {
        Self {
            symbol_count,
            transitions,
        }
    }

    /// @brief 按当前状态和符号查找转移规则。
    pub fn get(&self, state: State, symbol: Symbol) -> Option<Transition> {
        let state_id = match state {
            State::Normal(id) => id,
            State::Halt => return None,
        };

        let index = state_id as usize * self.symbol_count as usize + symbol as usize;
        self.transitions.get(index).copied()
    }

    /// @brief 按规则槽位顺序遍历转移表。
    pub fn iter(&self) -> impl Iterator<Item = (State, Symbol, Transition)> + '_ {
        self.transitions
            .iter()
            .copied()
            .enumerate()
            .map(move |(index, transition)| {
                let symbol_count = self.symbol_count as usize;
                let state_id = index / symbol_count;
                let symbol = index % symbol_count;

                (State::Normal(state_id as u8), symbol as Symbol, transition)
            })
    }

    /// @brief 获取规则槽位数量。
    pub fn len(&self) -> usize {
        self.transitions.len()
    }

    /// @brief 判断转移表是否为空。
    pub fn is_empty(&self) -> bool {
        self.transitions.is_empty()
    }
}

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

        let transition = match self.table.get(self.state, symbol) {
            Some(transition) => transition,
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

#[cfg(test)]
mod tests {
    use super::TransitionTable;
    use crate::model::{Direction, State, Transition};

    #[test]
    fn transition_table_indexes_by_state_and_symbol() {
        let transitions = vec![
            Transition {
                write: 0,
                direction: Direction::Left,
                next_state: State::Normal(0),
            },
            Transition {
                write: 1,
                direction: Direction::Right,
                next_state: State::Normal(1),
            },
            Transition {
                write: 1,
                direction: Direction::Left,
                next_state: State::Halt,
            },
            Transition {
                write: 0,
                direction: Direction::Right,
                next_state: State::Normal(0),
            },
        ];
        let table = TransitionTable::new(2, transitions);

        assert_eq!(table.get(State::Normal(0), 0).unwrap().write, 0);
        assert_eq!(table.get(State::Normal(0), 1).unwrap().write, 1);
        assert_eq!(
            table.get(State::Normal(1), 0).unwrap().next_state,
            State::Halt
        );
        assert_eq!(table.get(State::Normal(1), 1).unwrap().write, 0);
        assert_eq!(table.get(State::Halt, 0), None);
    }
}
