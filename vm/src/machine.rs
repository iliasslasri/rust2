use std::io::{self, Write};

pub const MEMORY_SIZE: usize = 4096;
const NREGS: usize = 16;

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Machine {
    regs: [u32; NREGS],
    machine_memory: [u8; MEMORY_SIZE],
}

#[derive(Debug)]
pub enum Error {
    /// Attempt to create a machine with too large a memory
    MemoryOverflow,
    /// Attempt to set a register out of r0 to r15
    RegIndexOutOfRange,
    /// Error while writing to the output
    WriteError,
    /// Unknown instruction
    UnknownInstruction,
    /// Memory adress out of range
    MemAddressOutOfRange,
}

impl Machine {
    /// Create a new machine in its reset state. The `memory` parameter will
    /// be copied at the beginning of the machine memory.
    ///
    /// # Errors
    /// This function returns an error when the memory exceeds `MEMORY_SIZE`.
    #[must_use]
    pub fn new(memory: &[u8]) -> Result<Self> {
        let regs: [u32; 16] = [0; 16];

        if memory.len() > MEMORY_SIZE {
            // MemoryOverflow
            return std::result::Result::Err(Error::MemoryOverflow);
        }

        let end = memory.len() as usize;
        let mut machine_memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
        for i in 0..end {
            machine_memory[i] = memory[i];
        }
        std::result::Result::Ok(Machine {
            regs,
            machine_memory,
        })
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
        // exec_after_end_of_address_space
        if (self.regs[0] as usize) >= MEMORY_SIZE {
            return std::result::Result::Err(Error::UnknownInstruction);
        }
        // get the instruction in IP, note that the data storage uses little-endian
        // theres no problem in reading 4 bytes every time
        let instruction: [u8; 4];
        if (self.regs[0] as usize) + 1 >= MEMORY_SIZE {
            instruction = [self.machine_memory[(self.regs[0] as usize) + 0], 0, 0, 0];
        } else if self.regs[0] as usize + 2 >= MEMORY_SIZE {
            // MemoryOverflow
            instruction = [
                self.machine_memory[(self.regs[0] as usize) + 0],
                self.machine_memory[(self.regs[0] as usize) + 1],
                0,
                0,
            ];
        } else if self.regs[0] as usize + 3 >= MEMORY_SIZE {
            // MemoryOverflow
            instruction = [
                self.machine_memory[(self.regs[0] as usize) + 0],
                self.machine_memory[(self.regs[0] as usize) + 1],
                self.machine_memory[(self.regs[0] as usize) + 2],
                0,
            ];
        } else {
            instruction = [
                self.machine_memory[(self.regs[0] as usize) + 0],
                self.machine_memory[(self.regs[0] as usize) + 1],
                self.machine_memory[(self.regs[0] as usize) + 2],
                self.machine_memory[(self.regs[0] as usize) + 3],
            ];
        }

        // decode the instruction
        if instruction[0] == 1 {
            if (self.regs[0] as usize + 4) > MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            // move if
            if instruction[1] >= 16 || instruction[2] >= 16 || instruction[3] >= 16 {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            self.regs[0] += 4;
            if self.regs[instruction[3] as usize] != 0 {
                self.regs[instruction[1] as usize] = self.regs[instruction[2] as usize];
            }
            std::result::Result::Ok(false)
        } else if instruction[0] == 2 {
            // store
            self.regs[0] += 3;
            // store the content of register rⱼ into the memory starting at address pointed by register rᵢ using little-endian representation.
            if instruction[1] >= 16 || instruction[2] >= 16 {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            let mut address: u32 = self.regs[instruction[1] as usize] as u32;
            if (address as usize) + 3 >= MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            // register to copy
            let ri = self.regs[instruction[2] as usize];
            for i in 0..4 {
                self.machine_memory[address as usize] = (ri >> (i * 8)) as u8;
                address += 1;
            }
            std::result::Result::Ok(false)
        } else if instruction[0] == 3 {
            // load
            self.regs[0] += 3;
            if (self.regs[0] as usize + 3) > MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            if (instruction[2]) >= 16 {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            let mut address: usize = self.regs[instruction[2] as usize] as usize;
            // address out of range
            if address + 3 >= MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            let mut word: u32 = 0;
            for i in 0..4 {
                word += (self.machine_memory[address] as u32) << (i * 8);
                address += 1;
            }
            if (instruction[1]) >= 16 {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            self.regs[instruction[1] as usize] = word;
            std::result::Result::Ok(false)
        } else if instruction[0] == 4 {
            // loadimm
            self.regs[0] += 4;
            if (self.regs[0] as usize + 4) > MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            let word: i16 = (instruction[2] as u16 + ((instruction[3] as u16) << 8)) as i16;
            if (instruction[1]) >= 16 {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            self.regs[instruction[1] as usize] = word as u32;
            std::result::Result::Ok(false)
        } else if instruction[0] == 5 {
            // sub
            self.regs[0] += 4;
            if (self.regs[0] as usize + 4) > MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            // sub_with_wraparound_neg
            if instruction[1] >= 16 || instruction[2] >= 16 || instruction[3] >= 16 {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            self.regs[instruction[1] as usize] =
                self.regs[instruction[2] as usize].wrapping_sub(self.regs[instruction[3] as usize]);
            return std::result::Result::Ok(false);
        } else if instruction[0] == 6 {
            // out
            self.regs[0] += 2;
            if (self.regs[0] as usize + 4) > MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            let c: char = self.regs[instruction[1] as usize] as u8 as char;
            let mut buf = [0; 4]; // Buffer to hold UTF-8 encoding
            let utf8_bytes = c.encode_utf8(&mut buf).as_bytes(); // Get UTF-8 bytes of the character

            if fd.write_all(utf8_bytes).is_err() {
                // Handle WriteError
                return std::result::Result::Err(Error::WriteError);
            }

            std::result::Result::Ok(false)
        } else if instruction[0] == 7 {
            if ((self.regs[0] + 1) as usize) > MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            self.regs[0] += 1;
            // exit the current program
            std::result::Result::Ok(true)
        } else if instruction[0] == 8 {
            self.regs[0] += 2;
            if (self.regs[0] as usize + 4) > MEMORY_SIZE {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            if (instruction[1]) >= 16 {
                return std::result::Result::Err(Error::MemAddressOutOfRange);
            }
            // out number 8 rᵢ: output the signed number stored in register rᵢ in decimal.
            let mut number: i32 = self.regs[instruction[1] as usize] as i32;
            if number < 0 {
                if fd.write_all(&['-' as u8]).is_err() {
                    // WriteError
                    return std::result::Result::Err(Error::WriteError);
                }
            }
            number = number.abs();
            let mut digits: Vec<u8> = Vec::new();
            while number > 0 {
                digits.push((number % 10) as u8 + '0' as u8);
                number /= 10;
            }
            digits.reverse();
            if fd.write_all(&digits).is_err() {
                // WriteError
                return std::result::Result::Err(Error::WriteError);
            }
            std::result::Result::Ok(false)
        } else {
            std::result::Result::Err(Error::UnknownInstruction)
        }
    }

    /// Similar to [`step_on`](Machine::step_on).
    /// If output instructions are run, they print on standard output.
    pub fn step(&mut self) -> Result<bool> {
        self.step_on(&mut io::stdout().lock())
    }

    /// Reference onto the machine current set of registers.
    #[must_use]
    pub fn regs(&self) -> &[u32] {
        &self.regs
    }

    /// Sets a register to the given value.
    pub fn set_reg(&mut self, reg: usize, value: u32) -> Result<()> {
        if reg > NREGS {
            // RegIndexOutOfRange
            return std::result::Result::Err(Error::RegIndexOutOfRange);
        }
        self.regs[reg] = value as u32;
        std::result::Result::Ok(())
    }

    /// Reference onto the machine current memory.
    #[must_use]
    pub fn memory(&self) -> &[u8] {
        &self.machine_memory
    }
}
