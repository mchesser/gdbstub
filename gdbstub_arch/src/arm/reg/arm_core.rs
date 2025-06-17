use gdbstub::arch::Registers;

pub type F96 = [u8; 12];

/// 32-bit ARM core registers.
///
/// Source: <https://github.com/bminor/binutils-gdb/blob/master/gdb/features/arm/arm-core.xml>
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ArmCoreRegs {
    /// General purpose registers (R0-R12)
    pub r: [u32; 13],
    /// Stack Pointer (R13)
    pub sp: u32,
    /// Link Register (R14)
    pub lr: u32,
    /// Program Counter (R15)
    pub pc: u32,
    /// Floating point registers (F0-F7)
    pub f: [F96; 8],
    /// Floating Point Status Register (fps)
    pub fps: u32,
    /// Current Program Status Register (cpsr)
    pub cpsr: u32,
}

impl Registers for ArmCoreRegs {
    type ProgramCounter = u32;

    fn pc(&self) -> Self::ProgramCounter {
        self.pc
    }

    fn gdb_serialize(&self, mut write_byte: impl FnMut(Option<u8>)) {
        macro_rules! write_bytes {
            ($bytes:expr) => {
                for b in $bytes {
                    write_byte(Some(*b))
                }
            };
        }

        for reg in self.r.iter() {
            write_bytes!(&reg.to_le_bytes());
        }
        write_bytes!(&self.sp.to_le_bytes());
        write_bytes!(&self.lr.to_le_bytes());
        write_bytes!(&self.pc.to_le_bytes());

        for reg in self.f.iter() {
            write_bytes!(reg);
        }
        write_bytes!(&self.fps.to_le_bytes());

        write_bytes!(&self.cpsr.to_le_bytes());
    }

    fn gdb_deserialize(&mut self, mut bytes: &[u8]) -> Result<(), ()> {
        if bytes.len() != (17 + 25) * 4 {
            return Err(());
        }

        let mut next_reg = || {
            if bytes.len() < 4 {
                Err(())
            } else {
                use core::convert::TryInto;

                let (next, rest) = bytes.split_at(4);
                bytes = rest;
                Ok(u32::from_le_bytes(next.try_into().unwrap()))
            }
        };

        for reg in self.r.iter_mut() {
            *reg = next_reg()?
        }
        self.sp = next_reg()?;
        self.lr = next_reg()?;
        self.pc = next_reg()?;

        for reg in self.f.iter_mut() {
            reg[..4].copy_from_slice(&next_reg()?.to_le_bytes());
            reg[4..8].copy_from_slice(&next_reg()?.to_le_bytes());
            reg[8..12].copy_from_slice(&next_reg()?.to_le_bytes());
        }

        self.fps = next_reg()?;
        self.cpsr = next_reg()?;

        Ok(())
    }
}
