#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::account_info::AccountInfo;
    use solana_program::clock::Epoch;
    use solana_program::pubkey::Pubkey;
    use std::mem;
    use borsh::BorshDeserialize;
    use borsh::BorshSerialize;
    use calculator_program::CalculatorAccount;
    use calculator_program::process_instruction;
    #[test]
    fn test_addition() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<CalculatorAccount>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = vec![0]; // 0 for addition
        let accounts = vec![account];

        let mut calculator_account = CalculatorAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();
        calculator_account.operand1 = 10;
        calculator_account.operand2 = 5;
        calculator_account.serialize(&mut &mut accounts[0].data.borrow_mut()[..]).unwrap(); 

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let calculator_account = CalculatorAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();

        assert_eq!(calculator_account.result, 15);
    }

    #[test]
    fn test_subtraction() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<CalculatorAccount>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = vec![1]; // 1 for subtraction
        let accounts = vec![account];

        let mut calculator_account = CalculatorAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();
        calculator_account.operand1 = 10;
        calculator_account.operand2 = 5;
        calculator_account.serialize(&mut &mut accounts[0].data.borrow_mut()[..]).unwrap(); 

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let calculator_account = CalculatorAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();

        assert_eq!(calculator_account.result, 5);
    }
}
