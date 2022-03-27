use solana_program::{
    program_error::ProgramError,
    msg
};
use std::convert::TryInto;

#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(u32), // first byte is instruction type, the rest of four bytes is the number to be set
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        msg!("HelloInstruction unpack function");

        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        match tag {
            0 => Ok(HelloInstruction::Increment),
            1 => Ok(HelloInstruction::Decrement),
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8; 4], _> = rest[..4].try_into();
                match val {
                    Ok(i) => {
                        return Ok(HelloInstruction::Set(u32::from_le_bytes(i)))
                    },
                    _ => return Err(ProgramError::InvalidInstructionData)
                }
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        }
    }
}