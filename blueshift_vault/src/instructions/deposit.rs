use pinocchio::{AccountView, ProgramResult, error::ProgramError, instruction};
use pinocchio_system::instructions::Transfer;


// 存款指令执行以下步骤：

// 验证金库为空（lamports 为零），以防止重复存款
// 确保存款金额超过基本账户的免租金最低限额
// 使用对系统程序的 CPI 将 lamports 从所有者转移到金库
pub struct DepositAccounts<'a> {
    pub owner: &'a AccountView,
    pub vault: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for DepositAccounts<'a> {
        type Error = ProgramError;

    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [owner, vault, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        if !owner.is_signer() {
            return Err(ProgramError::InvalidAccountOwner);
        }
        if !vault.owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }
        if vault.lamports().ne(&0) {
           return Err(ProgramError::InvalidAccountData); 
        }
        
        let (vault_key, _) = pinocchio::Address::find_program_address(&[b"vault", owner.address().as_ref()], &crate::ID);
        if vault.address() != &vault_key {
            return Err(ProgramError::InvalidSeeds);
        }
        
        Ok(Self { owner, vault })
    }
    

}

pub struct DepositInstructionData {
    pub amount: u64
}

impl<'a> TryFrom<&'a [u8]> for DepositInstructionData{
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u64>() {
            return Err(ProgramError::InvalidInstructionData);
        }
        let amount: u64  = u64::from_le_bytes(data.try_into().unwrap());
        if amount.eq(&0) {
            return Err(ProgramError::InvalidInstructionData);
        }
        Ok(Self { amount })
    }
}

pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub instruction_data: DepositInstructionData
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountView])> for Deposit<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountView])) -> Result<Self, Self::Error> {
        let accounts = DepositAccounts::try_from(accounts)?;
        let instruction_data = DepositInstructionData::try_from(data)?;
        Ok(Self { accounts, instruction_data })
    }
}
impl<'a> Deposit<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;
    pub fn process(&mut self) -> ProgramResult{
        Transfer{
            from: self.accounts.owner,
            to: self.accounts.vault,
            lamports: self.instruction_data.amount,
        }.invoke()?;
        Ok(())
    }
}