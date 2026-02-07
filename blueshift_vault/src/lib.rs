#![no_std]
use pinocchio::{AccountView, Address, ProgramResult, address::address, entrypoint, error::ProgramError, nostd_panic_handler};

use crate::instructions::{deposit::Deposit, withdraw::Withdraw};
nostd_panic_handler!();
entrypoint!(process_instruction);

mod instructions;

pub const ID: Address = address!("22222222222222222222222222222222222222222222");

fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8]
) -> ProgramResult{
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}