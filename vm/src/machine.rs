use std::io::{self, Write};

pub const MEMORY_SIZE: usize = 4096;
const NREGS: usize = 16;

const IP: usize = 0;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Machine {
    // Implement me!
}

#[derive(Debug)]
pub enum Error {
    /// Attempt to create a machine with too large a memory
    MemoryOverflow,
    // Add some more entries to represent different errors
}

impl Machine {
    /// Create a new machine in its reset state. The `memory` parameter will
    /// be copied at the beginning of the machine memory.
    ///
    /// # Errors
    /// This function returns an error when the memory exceeds `MEMORY_SIZE`.
    #[must_use]
    pub fn new(memory: &[u8]) -> Result<Self> {
        unimplemented!() // Implement me!
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on `fd`.
    pub fn run_on<T: Write>(&mut self, fd: &mut T) -> Result<()> {
        while !self.step_on(fd)? {}
        Ok(())
    }

    /// Run until the program terminates or until an error happens.
    /// If output instructions are run, they print on standard output.
    pub fn run(&mut self) -> Result<()> {
        self.run_on(&mut io::stdout().lock())
    }

    /// Execute the next instruction by doing the following steps:
    ///   - decode the instruction located at IP (register 0)
    ///   - increment the IP by the size of the instruction
    ///   - execute the decoded instruction
    ///
    /// If output instructions are run, they print on `fd`.
    /// If an error happens at either of those steps, an error is
    /// returned.
    ///
    /// In case of success, `true` is returned if the program is
    /// terminated (upon encountering an exit instruction), or
    /// `false` if the execution must continue.
    pub fn step_on<T: Write>(&mut self, fd: &mut T) -> Result<bool> {
        unimplemented!() // Implement me!
    }

    /// Similar to [`step_on`](Machine::step_on).
    /// If output instructions are run, they print on standard output.
    pub fn step(&mut self) -> Result<bool> {
        self.step_on(&mut io::stdout().lock())
    }

    /// Reference onto the machine current set of registers.
    #[must_use]
    pub fn regs(&self) -> &[u32] {
        unimplemented!() // Implement me!
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, reg: usize, value: u32) -> Result<()> {
        unimplemented!() // Implement me!
    }

    /// Reference onto the machine current memory.
    #[must_use]
    pub fn memory(&self) -> &[u8] {
        unimplemented!() // Implement me!
    }
}
