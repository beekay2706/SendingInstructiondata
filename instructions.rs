use solana_program::{program_error::ProgramError};
use std::convert::TryInto;
// The various Instruction functions are defined in this module

#[derive(Debug)]
pub enum Helloinstructions {
    Increment,
    Decrement,
    Set(u32)
}

impl Helloinstructions{
pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> 
{
let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

match tag{
    0 => return Ok(Helloinstructions::Increment),
    1 => return Ok(Helloinstructions::Decrement),
    2 => {
        if rest.len() != 4 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let val: Result<[u8 ; 4],_> = rest[..4].try_into();
        match val {
         Ok(i) => {
                return Ok(Helloinstructions::Set(u32::from_le_bytes(i)))
            },
            _ => return Err(ProgramError::InvalidInstructionData)
        }
        
         },
        _ => return Err(ProgramError::InvalidInstructionData)
    }
}

}