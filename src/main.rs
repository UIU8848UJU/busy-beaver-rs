use std::collections::HashMap;

# 这个#号是注释吗，我看到下面好像是枚举了一个状态和移动方式
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum State {
    A,
    B,
    Halt,
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Left,
    Right,
}

// 这里因该是结构体了，好像是过度函数？
#[derive(Clone, Copy, Debug)]
struct Transition {
    write: u8,
    movement: Move,
    next: State,
}

// 这个结构体因该是bb机了，有头有情况有步数有表单有磁盘
struct Machine {
    table: HashMap<(State, u8), Transition>,
    tape: HashMap<i64, u8>,
    head: i64,
    state: State,
    steps: u64,
}

//这个impl是什么意思，fn我估计是函数，self我估计是指向自己，new是构造函数
impl Machine {
    fn new(table: HashMap<(State, u8), Transition>) -> Self {
        Self {
            table,
            tape: HashMap::new(),
            head: 0,
            state: State::A,
            steps: 0,
        }
    }

    //这里我看懂了fn是函数，->是指向的返回值。但是内部get应该是类似于STL的get
    fn read_symbol(&self) -> u8 {
        *self.tape.get(&self.head).unwrap_or(&0)
    }

    // 这里是写入状态了
    fn write_symbol(&mut self, symbol: u8) {
        self.tape.insert(self.head, symbol);
    }

    // 这里是移动头部，不过match是什么意思呢
    fn move_head(&mut self, movement: Move) {
        match movement {
            Move::Left => self.head -= 1,
            Move::Right => self.head += 1,
        }
    }

    // 这里的mut是什么类型
    fn step(&mut self) -> bool {
        if self.state == State::Halt {
            return false;
        }

        // 还有=>和somes是什么东西？
        let symbol = self.read_symbol();
        let transition = match self.table.get(&(self.state, symbol)) {
            Some(t) => *t,
            None => return false,
        };

        self.write_symbol(transition.write);
        self.move_head(transition.movement);
        self.state = transition.next;
        self.steps += 1;

        self.state != State::Halt
    }

    fn run(&mut self, max_steps: u64) {
        while self.steps < max_steps {
            if !self.step() {
                break;
            }
        }
    }

    fn count_ones(&self) -> usize {
        self.tape.values().filter(|&&v| v == 1).count()
    }
}

fn main() {
    // 这个因该是声明类型了，哦~mut是不是可变的意思？
    let mut table = HashMap::new();

    // BB(2) champion:
    // A0 -> 1RB
    // A1 -> 1LB
    // B0 -> 1LA
    // B1 -> 1RH
    table.insert((State::A, 0), Transition {
        write: 1,
        movement: Move::Right,
        next: State::B,
    });

    table.insert((State::A, 1), Transition {
        write: 1,
        movement: Move::Left,
        next: State::B,
    });

    table.insert((State::B, 0), Transition {
        write: 1,
        movement: Move::Left,
        next: State::A,
    });

    table.insert((State::B, 1), Transition {
        write: 1,
        movement: Move::Right,
        next: State::Halt,
    });

    let mut machine = Machine::new(table);
    machine.run(100);

    println!("state: {:?}", machine.state);
    println!("steps: {}", machine.steps);
    println!("ones: {}", machine.count_ones());
    println!("tape: {:?}", machine.tape);
}