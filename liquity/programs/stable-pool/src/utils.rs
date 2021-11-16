use anchor_lang::prelude::*;

pub fn get_market_price()->u64 {
    253
}


pub fn authority_id(
    program_id: &Pubkey,
    my_info: &Pubkey,
    nonce: u8,
) -> Result<Pubkey, PubkeyError> {
    Pubkey::create_program_address(&[&my_info.to_bytes()[..32], &[nonce]], program_id)
}

/// issue a spl_token `Transfer` instruction.
pub fn token_transfer<'a>(
    pool: &Pubkey,
    token_program: AccountInfo<'a>,
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    nonce: u8,
    amount: u64,
) -> Result<(), ProgramError> {
    let pool_bytes = pool.to_bytes();
    let authority_signature_seeds = [&pool_bytes[..32], &[nonce]];
    let signers = &[&authority_signature_seeds[..]];
    let ix = spl_token::instruction::transfer(
        token_program.key,
        source.key,
        destination.key,
        authority.key,
        &[],
        amount,
    )?;
    invoke_signed(
        &ix,
        &[source, destination, authority, token_program],
        signers,
    )
} 
pub fn token_mint_to<'a>(
    owner: &Pubkey,
    token_program: AccountInfo<'a>,
    mint: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    nonce: u8,
    amount: u64,
) -> Result<(), ProgramError> {
    let owner_bytes = owner.to_bytes();
    let authority_signature_seeds = [&owner_bytes[..32], &[nonce]];
    let signers = &[&authority_signature_seeds[..]];
    let ix = spl_token::instruction::mint_to(
        token_program.key,
        mint.key,
        destination.key,
        authority.key,
        &[],
        amount,
    )?;

    invoke_signed(&ix, &[mint, destination, authority, token_program], signers)
}

pub fn token_burn<'a>(
    owner: &Pubkey,
    token_program: AccountInfo<'a>,
    mint: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    nonce: u8,
    amount: u64,
) -> Result<(), ProgramError> {
    let owner_bytes = owner.to_bytes();
    let authority_signature_seeds = [&owner_bytes[..32], &[nonce]];
    let signers = &[&authority_signature_seeds[..]];

    let ix = spl_token::instruction::burn(
        token_program.key,
        destination.key,
        mint.key,
        authority.key,
        &[],
        amount
    )?;
    
    invoke_signed(&ix, &[mint, destination, authority, token_program], signers)
}

pub fn apply_pending_rewards(
    trove_manager_data:&TroveManager, 
    borrower_trove:&mut Trove, 
    reward_snapshot:&mut RewardSnapshot, 
    default_pool_data:&mut DefaultPool, 
    active_pool_data:&mut ActivePool)
{
    if has_pending_rewards(trove_manager_data, borrower_trove, reward_snapshot) {
        if borrower_trove.is_active() {
            // Compute pending rewards
            let pending_sol_reward = get_pending_sol_reward(trove_manager_data,borrower_trove, reward_snapshot);
            let pending_solusd_debt_reward = get_pending_solusd_debt_reward(trove_manager_data, borrower_trove, reward_snapshot);

            // Apply pending rewards to trove's state
            borrower_trove.coll = borrower_trove.coll + pending_sol_reward;
            borrower_trove.debt = borrower_trove.debt + pending_solusd_debt_reward;

            reward_snapshot.update_trove_reward_snapshots(trove_manager_data);
            // Transfer from DefaultPool to ActivePool
            move_pending_trove_reward_to_active_pool(
                trove_manager_data, 
                pending_sol_reward, 
                pending_solusd_debt_reward,
                default_pool_data,
                active_pool_data
            );
        }
    }
}