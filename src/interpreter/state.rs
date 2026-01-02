use std::fmt::{self, Error};

use console::style;

/// Collection of read-write registers
#[derive(Default)]
pub struct Registers([u8; 15]);

impl Registers {
    /// Read a value from a register.
    /// Returns `None` if no such register
    #[inline]
    pub fn r(&self, reg: u8) -> Option<u8> {
        match reg {
            0 => Some(0),
            1..=15 => Some(self.0[(reg as usize) - 1]),
            _ => None,
        }
    }

    /// Write a value to a register.
    /// Returns `Err` if no such register
    #[inline]
    pub fn w(&mut self, reg: u8, v: u8) -> Result<(), ()> {
        match reg {
            0 => Ok(()),
            1..=15 => {
                self.0[(reg as usize) - 1] = v;
                Ok(())
            }
            _ => Err(()),
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..=7 {
            let left = self.r(i).ok_or(Error)?;
            let right = self.r(i + 8).ok_or(Error)?;
            write!(f, "{:<3}: 0x{:02x} ", style(format!("r{}", i)).bold(), left)?;
            writeln!(
                f,
                "{:<3}: 0x{:02x} ",
                style(format!("r{}", i + 8)).bold(),
                right
            )?;
        }
        Ok(())
    }
}

/// Collection of ALU flags
pub struct Flags {
    pub zero: bool,
    pub overflow: bool,
}

impl Default for Flags {
    fn default() -> Self {
        (true, false).into()
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "zero: {}, overflow: {}", self.zero, self.overflow)
    }
}

impl From<(bool, bool)> for Flags {
    #[inline]
    fn from(value: (bool, bool)) -> Self {
        Self {
            zero: value.0,
            overflow: value.1,
        }
    }
}

/// Virtual machine state
#[derive(Default)]
pub struct State {
    /// Program counter
    pub pc: u16,
    /// Registers
    pub regs: Registers,
    /// ALU Flags
    pub flags: Flags,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PC:    0x{:04x}", self.pc)?;
        writeln!(f, "Flags: {}", self.flags)?;
        writeln!(f, "Registers:")?;
        write!(f, "{}", self.regs)
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}
