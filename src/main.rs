//! Busy Beaver 搜索器命令行入口。
//!
//! 用法：
//! cargo run -- 2
//! cargo run -- 3
//! cargo run -- 3 1000
//! cargo run -- 3 1000 100000
//!
//! 第一个参数：普通状态数量。
//! 第二个参数：最大运行步数，可选。
//! 第三个参数：搜索进度输出间隔，可选；0 表示不输出。

use std::env;

use busy_beaver_rs::machine::TransitionTable;
use busy_beaver_rs::model::{Direction, State};
use busy_beaver_rs::search::{BusyBeaverSearch, ChampionGroup, SearchConfig};

const MAX_PRINTED_CHAMPIONS: usize = 5;

fn main() {
    let state_count = parse_state_count();
    let max_steps = parse_max_steps(state_count);
    let progress_interval = parse_progress_interval();

    let config =
        SearchConfig::new(state_count, 2, max_steps).with_progress_interval(progress_interval);
    let search = BusyBeaverSearch::new(config);
    let result = search.run();

    println!("=== Busy Beaver BB({}) 搜索结果 ===", state_count);
    println!("最大运行步数: {}", max_steps);
    println!("搜索耗时: {:.2?}", result.elapsed);
    println!("总机器数量: {}", result.total_machines);
    println!("正常停机机器数量: {}", result.halted_machines);
    println!("达到步数上限机器数量: {}", result.step_limited_machines);
    println!(
        "缺失转移规则机器数量: {}",
        result.missing_transition_machines
    );

    println!();
    print_champion_group("Sigma(n) 最多 1 冠军组", &result.sigma_champions);

    println!();
    print_champion_group("S(n) 最长停机步数冠军组", &result.step_champions);
}

/// @brief 解析状态数量参数。
///
/// 默认值为 2，所以直接 `cargo run` 会运行 BB(2)。
fn parse_state_count() -> u8 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return 2;
    }

    args[1].parse::<u8>().unwrap_or(2)
}

/// @brief 解析最大步数参数。
///
/// 如果用户没有传入第二个参数，则根据状态数量给默认值。
fn parse_max_steps(state_count: u8) -> u64 {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        return args[2].parse::<u64>().unwrap_or(100);
    }

    match state_count {
        2 => 100,
        3 => 100,
        _ => 10_000,
    }
}

/// @brief 解析进度输出间隔参数。
fn parse_progress_interval() -> usize {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return 10_000;
    }

    args[3].parse::<usize>().unwrap_or(10_000)
}

/// @brief 打印一个 Busy Beaver 冠军组。
fn print_champion_group(title: &str, champions: &ChampionGroup) {
    println!("=== {} ===", title);

    if champions.is_empty() {
        println!("没有找到正常停机的机器");
        return;
    }

    println!("冠军机器数量: {}", champions.len());
    println!("最多 1 的数量: {}", champions.best_ones);
    println!("最长运行步数: {}", champions.best_steps);

    for (index, table) in champions
        .tables
        .iter()
        .take(MAX_PRINTED_CHAMPIONS)
        .enumerate()
    {
        println!();
        println!("--- 冠军机器 #{} ---", index + 1);
        print_transition_table(table);
    }

    if champions.len() > MAX_PRINTED_CHAMPIONS {
        println!();
        println!(
            "仅展示前 {} 台，剩余 {} 台未展示",
            MAX_PRINTED_CHAMPIONS,
            champions.len() - MAX_PRINTED_CHAMPIONS
        );
    }
}

/// @brief 打印状态转移表。
fn print_transition_table(table: &TransitionTable) {
    let mut entries: Vec<_> = table.iter().collect();

    entries.sort_by_key(|(state, symbol, _)| {
        let state_id = match state {
            State::Normal(id) => *id,
            State::Halt => u8::MAX,
        };

        (state_id, *symbol)
    });

    for (state, symbol, transition) in entries {
        println!(
            "{}{} -> {}{}{}",
            format_state(state),
            symbol,
            transition.write,
            format_direction(transition.direction),
            format_state(transition.next_state),
        );
    }
}

/// @brief 格式化状态。
///
/// A/B/C/... 用于展示普通状态，H 表示停机状态。
fn format_state(state: State) -> String {
    match state {
        State::Normal(id) => normal_state_name(id),
        State::Halt => "H".to_string(),
    }
}

/// @brief 将普通状态编号转换成人类可读名称。
///
/// 0 -> A, 1 -> B, 2 -> C。
fn normal_state_name(id: u8) -> String {
    let ch = (b'A' + id) as char;
    ch.to_string()
}

/// @brief 格式化移动方向。
fn format_direction(direction: Direction) -> &'static str {
    match direction {
        Direction::Left => "L",
        Direction::Right => "R",
    }
}
