use crate::types::Sym;

pub enum ProgramState {
    Running,
    Ended
}

pub struct Program {
    code: Vec<Sym>,
    state: Vec<u8>,
    ptr: u64,
    ip: u64
}

impl Program {
    pub fn new(code: Vec<Sym>) -> Self {
        Self {
            code,
            state: vec![0; 30000],
            ptr: 0,
            ip: 0
        }
    }

    pub fn step(&mut self) -> Result<ProgramState, String> {
        let next = self.code.get(self.ip as usize);
        if next.is_none() {
            return Err(format!("Failed to step. IP {} > {}", self.ip, self.code.len()));
        }

        //println!("{}: {} {}({} {})", self.ip, next.unwrap(), self.ptr, self.state[0], self.state[1]);


        match next.unwrap() {
            Sym::IncPtr => self.ptr += 1,
            Sym::DecPtr => self.ptr -= 1,
            Sym::Inc => self.state[self.ptr as usize] = self.state[self.ptr as usize].wrapping_add(1),
            Sym::Dec => self.state[self.ptr as usize] = self.state[self.ptr as usize].wrapping_sub(1),
            Sym::WriteOut => print!("{}", self.state[self.ptr as usize] as char),
            Sym::ReadIn => {},
            Sym::Begin(end_ptr) => self.ip = end_ptr.to_owned(),
            Sym::End(begin_ptr) => {
                if self.state[self.ptr as usize] != 0 {
                    self.ip = begin_ptr.to_owned();
                }
            }
        }
    
        self.ip += 1;
    
        if self.ip >= self.code.len() as u64 {
            Ok(ProgramState::Ended)
        } else {
            Ok(ProgramState::Running)
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            match self.step() {
                Err(e) => return Err(e),
                Ok(state) => {
                    if let ProgramState::Ended = state {
                        return Ok(());
                    }
                }
            }
        }
    }
}