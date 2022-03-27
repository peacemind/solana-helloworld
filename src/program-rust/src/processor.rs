use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::state::GreetingAccount;

use crate::instruction::HelloInstruction;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Hello World Rust program processor");
        msg!("Instruction data: {:?}", instruction_data);
        let instruction = HelloInstruction::unpack(instruction_data)?;
        msg!("Instruction data after unpack: {:?}", instruction);

        // Iterating accounts is safer than indexing
        let accounts_iter = &mut accounts.iter();
    
        // Get the account to say hello to
        let account = next_account_info(accounts_iter)?;
    
        // The account must be owned by the program in order to modify its data
        if account.owner != program_id {
            msg!("Greeted account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }
    
        // Increment and store the number of times the account has been greeted
        let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;

        match instruction {
            HelloInstruction::Increment => {
                greeting_account.counter += 1;
            },
            HelloInstruction::Decrement => {
                greeting_account.counter -= 1;
            },
            HelloInstruction::Set(val) => {
                greeting_account.counter = val;
            }
        }

        greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    
        msg!("Greeted {} time(s)!", greeting_account.counter);
    
        Ok(())    
    }
}