# bf-rs
**bf-rs** is a Brainf\*\*k interpreter written in Rust. It's currently in early development.

This is also a preparation for my next project, **bfc-rs** which will be a compiler which compiles into Brainf\*\*k code.

# How to run
Should be as simple as running `./demo.sh` or `demo.bat` depending on your OS, assuming you got `cargo` on your path.

# How does Read (`,`) work?
Whenever the program halts without exiting (unless of course you've done something wrong), you're given the option to enter your input. Usually Brainf\*\*k only expects a single char as input, thus if you enter more than one char, it will add the rest to a queue that will be exhausted before askig again.

# Syntax?
Should support the full Brainf\*\*k spec. Unsupported symbols are ignored.

I did however add support for actual comments. Write `;` and the rest of the line with be ignored by the lexer. This is useful when writing long descriptions where you use symbols used by Brainf\*\*k.