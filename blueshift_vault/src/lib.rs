use pinocchio::{AccountView, Address, ProgramResult, address::address, entrypoint, error::ProgramError};

mod instructions;

pub const ID: Address = address!("22222222222222222222222222222222222222222222");

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8]
) -> ProgramResult{
    match instruction_data.split_first() {
        
        _ => Err(ProgramError::InvalidInstructionData)
    }
}