struct Machine {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    chunk: Vec<usize>,
    out: Vec<usize>,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            a: 30344604,
            b: 0,
            c: 0,
            ip: 0,
            chunk: vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0],
            out: Vec::new(),
        }
    }
}

impl Machine {
    const INIT_CHUNK: [usize; 16] = [2, 4, 1, 1, 7, 5, 1, 5, 4, 5, 0, 3, 5, 5, 3, 0];
    const CHUNK_LEN: usize = Self::INIT_CHUNK.len();
    const FN_TABLE: [fn(&mut Self); 8] = [
        Self::adv,
        Self::bxl,
        Self::bst,
        Self::jnz,
        Self::bxz,
        Self::out,
        Self::bdv,
        Self::cdv,
    ];

    fn new_set_a(a: usize) -> Self {
        Self {
            a,
            ..Self::default()
        }
    }

    fn exec(&mut self) {
        while self.ip < self.chunk.len() {
            Machine::FN_TABLE[self.read()](self);
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
        let combo = self.combo() % 8;
        self.out.push(combo);
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

    fn find_lowest_value() -> Option<usize> {
        Self::find_lowest_value_recurse(Self::CHUNK_LEN - 1, 0)
    }

    fn find_lowest_value_recurse(n: usize, i: usize) -> Option<usize> {
        for a in i..i + 8 {
            let mut m = Machine::new_set_a(a);
            m.exec();

            if m.out[0] == m.chunk[n] {
                if n == 0 {
                    return Some(a);
                }
                if let Some(res) = Self::find_lowest_value_recurse(n - 1, a << 3) {
                    return Some(res);
                }
            }
        }

        None
    }
}

pub fn solve_1() -> String {
    let mut m = Machine::default();
    m.exec();
    m.out
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

pub fn solve_2() -> usize {
    Machine::find_lowest_value().unwrap_or(0)
}
