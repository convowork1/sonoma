//! Example Rust-based BPF noop program

extern crate sonoma_program;
use sonoma_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

sonoma_program::entrypoint!(process_instruction);
#[allow(clippy::unnecessary_wraps)]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
