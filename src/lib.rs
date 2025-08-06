use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    entrypoint
};

#[derive(BorshSerialize, BorshDeserialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshSerialize,BorshDeserialize)]
struct Counter{
    count:u32
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
)->ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(&instruction_data)?;

    match instruction_type {
        InstructionType::Increment(value)=>{

        },
        InstructionType::Decrement(value)=>{

        }
    }

    Ok(())
}

