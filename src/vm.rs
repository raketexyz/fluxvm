use std::collections::HashMap;
use std::fs::File;
use std::os::unix::prelude::FromRawFd;
use std::io::Write;

use crate::arch::Opcode;

pub struct VM {
    /// The instruction pointer.
    pub ip: usize,
    /// The stack.
    pub stack: Vec<u32>,
    /// The loaded binary.
    pub memory: Vec<u8>,
    /// File descriptor table.
    pub fdt: HashMap<u32, File>,
    /// Next file descriptor.
    fd: u32,
}

impl VM {
    pub fn new(binary: Vec<u8>) -> Self {
        // Verify magic number
        assert!(binary[0] == 0xf1);
        assert!(binary[1] == 0x0f);
        assert!(binary[2] == 0x0a);
        assert!(binary[3] == 0x00);

        // Verify version
        assert!(binary[4] == 0x00);
        assert!(binary[5] == 0x00);
        assert!(binary[6] == 0x00);
        assert!(binary[7] == 0x00);

        // Read the bytes of the entry point address
        let mut ep = [0u8; 8];
        ep[4] = binary[8];
        ep[5] = binary[9];
        ep[6] = binary[10];
        ep[7] = binary[11];

        // Load the bytes of the entry point address as a big-endian
        let ip = usize::from_be_bytes(ep);

        Self {
            ip,
            stack: Vec::new(),
            memory: binary,
            fdt: HashMap::from([
                (0, unsafe { File::from_raw_fd(0) }),
                (1, unsafe { File::from_raw_fd(1) }),
                (2, unsafe { File::from_raw_fd(2) }),
            ]),
            fd: 3,
        }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        while self.ip < self.memory.len() {
            let bytecode = self.fetch()?;
            //println!("Encountered opcode {:x} at {:x}", bytecode, self.ip);

            match Opcode::try_from(bytecode)? {
                Opcode::Halt => {
                    //println!("Halting");
                    break
                }
                Opcode::IConst => {
                    let value = self.fetch()?;
                    self.stack.push(value);
                    //println!("Pushed iconst {:x}", value);
                }
                Opcode::Syscall => {
                    let code = self.fetch()?;
                    match code {
                        0 => self.open()?,
                        1 => self.write()?,
                        _ => Err(format!("Unknown syscall {:x}.", code))?
                    }
                }
                Opcode::JumpIfZero => {
                    let a = self.pop()?;
                    let dest = self.fetch()?;

                    if a == 0 {
                        self.ip = dest as usize;
                    }
                }
                Opcode::IAdd => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a + b);
                }
                Opcode::IMul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a * b);
                }
                Opcode::ISub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a - b);
                }
                Opcode::IDiv => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a / b);
                }
                Opcode::Dup => {
                    let a = self.pop()?;
                    self.stack.push(a);
                    self.stack.push(a);
                }
            }
        }

        Ok(())
    }

    fn open(&mut self) -> Result<(), String> {
        let buf = self.pop()?;
        let mut path = String::new();

        for i in 0..self.memory.len() - buf as usize {
            if self.memory[buf as usize + i] == 0 {
                break;
            }
            path.push(char::from(self.memory[buf as usize + i]));
        }

        self.fdt.insert(
            self.fd,
            File::options()
                .create(true)
                .write(true)
                .open(path)
                .map_err(|e| e.to_string())?
        );
        self.stack.push(self.fd);
        self.fd += 1;

        Ok(())
    }

    fn write(&mut self) -> Result<(), String> {
        let len = self.pop()?;
        let buf = self.pop()?;
        let fd = self.pop()?;

        let bytes = self.memory.get(buf as usize..(buf + len) as usize)
            .ok_or("Input buffer out of range.")?;

        self.fdt.get_mut(&fd).ok_or_else(|| "Unknown fd.".to_string())?
            .write(bytes).map_err(|e| e.to_string())?;

        Ok(())
    }

    fn fetch(&mut self) -> Result<u32, String> {
        let bytes = self.memory.get(self.ip..self.ip + 4).ok_or("Out of bounds.")?;
        self.ip += 4;

        Ok(u32::from_be_bytes(bytes.try_into().map_err(|_| "Error loading instruction.")?))
    }

    fn pop(&mut self) -> Result<u32, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow.".into())
    }
}
