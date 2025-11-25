struct Machine {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    chunk: Vec<usize>,
}

impl Machine {
    const FN_TABLE: [fn(&mut Machine); 8] = [
        Machine::adv,
        Machine::bxl,
        Machine::bst,
        Machine::jnz,
        Machine::bxz,
        Machine::out,
        Machine::bdv,
        Machine::cdv,
    ];

    #[allow(unused)]
    fn new_test() -> Self {
        Self {
            a: 729,
            b: 0,
            c: 0,
            ip: 0,
            chunk: vec![0, 1, 5, 4, 3, 0],
        }
    }

    fn new() -> Self {
        Self {
            a: 30344604,
            b: 0,
            c: 0,
            ip: 0,
            chunk: vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0],
        }
    }

    fn exec(mut self) {
        while self.ip < self.chunk.len() {
            Machine::FN_TABLE[self.read()](&mut self);
        }
    }

    fn read(&mut self) -> usize {
        self.ip += 1;
        self.chunk[self.ip - 1]
    }

    fn adv(&mut self) {
        self.a = self.dv();
    }

    fn bxl(&mut self) {
        self.b ^= self.read();
    }

    fn bst(&mut self) {
        self.b = self.combo() % 8;
    }

    fn jnz(&mut self) {
        let literal = self.read();
        let n = if self.a > 0 { &mut self.ip } else { &mut 0 };
        *n = literal;
    }

    fn bxz(&mut self) {
        _ = self.read();
        self.b ^= self.c;
    }

    fn out(&mut self) {
        print!("{},", self.combo() % 8);
    }

    fn bdv(&mut self) {
        self.b = self.dv();
    }

    fn cdv(&mut self) {
        self.c = self.dv();
    }

    fn dv(&mut self) -> usize {
        self.a / 2usize.pow(self.combo() as u32)
    }

    fn combo(&mut self) -> usize {
        match self.read() {
            n if n < 4 => n,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

pub fn solve_1() -> usize {
    Machine::new().exec();
    println!();
    0
}
