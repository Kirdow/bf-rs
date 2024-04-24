use std::fmt::Display;


#[derive(Clone)]
pub enum Sym {
    IncPtr,
    DecPtr,
    Inc,
    Dec,
    WriteOut,
    ReadIn,
    Begin(u64),
    End(u64)
}

impl Display for Sym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Sym::*;
        match self {
            IncPtr => write!(f, ">"),
            DecPtr => write!(f, "<"),
            Inc => write!(f, "+"),
            Dec => write!(f, "-"),
            WriteOut => write!(f, "."),
            ReadIn => write!(f, ","),
            Begin(end_index) => write!(f, "[ {}", end_index),
            End(begin_index) => write!(f, "] {}", begin_index)
        }
    }
}