use std::os::unix::process;

use pinocchio::{ProgramResult, account_info::AccountInfo, entrypoint, pubkey::Pubkey};

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

entrypoint!(process_instruction);
fn process_instruction(_program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    Ok(())
}
