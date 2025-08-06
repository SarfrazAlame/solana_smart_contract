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
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;

    let instruction_type = InstructionType::try_from_slice(&instruction_data)?;

    match instruction_type {
        InstructionType::Increment(value)=>{
            msg!("Executing increase");
            counter_data.count += value;
        },
        InstructionType::Decrement(value)=>{
            msg!("Executing decrease");
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut());

    Ok(())
}

