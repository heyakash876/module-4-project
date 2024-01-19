use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorAccount {
    pub operand1: u32,
    pub operand2: u32,
    pub result: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Calculator account does not have the correct program ID");
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut calculator_account = CalculatorAccount::try_from_slice(&account.data.borrow())?;

    match instruction_data[0] {
        0 => {
            calculator_account.result = calculator_account.operand1 + calculator_account.operand2;
        }
        1 => {
            calculator_account.result = calculator_account.operand1.saturating_sub(calculator_account.operand2);
        }
        _ => {
            msg!("Invalid instruction");
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Calculated result: {}", calculator_account.result);

    Ok(())
}

