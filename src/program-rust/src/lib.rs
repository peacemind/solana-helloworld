use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub mod entrypoint;
pub mod processor;
pub mod instruction;
pub mod state;

use crate::entrypoint::process_instruction;
use crate::state::GreetingAccount;

use crate::instruction::HelloInstruction;

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
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
        // 0 - increment
        // 1 - decrement
        // 2 - set 
        //      1-4 le
        let arr = u32::to_le_bytes(100);
        let mut instruction_data = [2; 5];

        for i in 0..4 {
            instruction_data[i+1] = arr[i];
        }
        // let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            100
        );
        let mut instruction_data = [0; 5];
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            101
        );
    }
}
