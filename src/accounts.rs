//! Accounts structs for Raydium AMM / Liquidity.

use anchor_lang::prelude::*;

/// Accounts for an `pre_initialize` instruction.
#[derive(Accounts, Clone)]
pub struct PreInitialize<'info> {
    /// AMM target orders account, a PDA create with seed = [program_id,serum_market_id, b"target_associated_seed"]
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// Withdraw queue Account, a PDA create with seed = [program_id,serum_market_id, b"withdraw_associated_seed"].
    /// To save withdraw dest_coin & dest_pc account with must cancle orders.
    #[account(mut)]
    pub pool_withdraw_queue: AccountInfo<'info>,
    /// Amm authority, a PDA create with seed = [b"amm authority"]
    pub amm_authority: AccountInfo<'info>,
    /// Pool lp mint account, a PDA create with seed = [program_id,serum_market_id, b"lp_mint_associated_seed"].
    /// Must be empty, owned by $authority.
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// Coin mint account
    pub coin_mint: AccountInfo<'info>,
    /// Pc mint account
    pub pc_mint: AccountInfo<'info>,
    /// Pool_token_coin Account.  a PDA create with seed = [program_id,serum_market_id, b"coin_vault_associated_seed"].
    /// Must be non zero, owned by $authority
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// Pool_token_pc Account, a PDA create with seed = [program_id,serum_market_id, b"pc_vault_associated_seed"].
    /// Must be non zero, owned by $authority.
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// Token_temp_lp Account, a PDA create with seed = [program_id,serum_market_id, b"temp_lp_token_associated_seed"].
    /// To save withdraw lp with must cancle orders as temp to transfer later.
    #[account(mut)]
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    /// Serum market Account. serum_dex program is the owner.
    pub serum_market: AccountInfo<'info>,
    /// The user wallet create the pool
    #[account(signer)]
    pub user_wallet: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts for an `initialize` instruction.
#[derive(Accounts, Clone)]
pub struct Initialize<'info> {
    /// The new amm Account to be create, a PDA create with seed = [program_id,serum_market_id, b"amm_associated_seed"]
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// Amm authority, a PDA create with seed = [b"amm authority"]
    pub amm_authority: AccountInfo<'info>,
    /// Amm open_orders Account, a PDA create with seed = [program_id,serum_market_id, b"open_order_associated_seed"]
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// Pool lp mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// Coin mint account
    pub coin_mint: AccountInfo<'info>,
    /// Pc mint account
    pub pc_mint: AccountInfo<'info>,
    /// Pool_token_coin Account. Must be non zero, owned by $authority
    pub pool_coin_token_account: AccountInfo<'info>,
    /// Pool_token_pc Account. Must be non zero, owned by $authority.
    pub pool_pc_token_account: AccountInfo<'info>,
    /// Withdraw queue Account. To save withdraw dest_coin & dest_pc account with must cancle orders.
    #[account(mut)]
    pub pool_withdraw_queue: AccountInfo<'info>,
    /// Pool target orders account
    #[account(mut)]
    pub pool_target_orders_account: AccountInfo<'info>,
    /// Token_dest_lp Account. To deposit the initial pool token supply, user_wallet is the owner.
    #[account(mut)]
    pub pool_lp_token_account: AccountInfo<'info>,
    /// Token_temp_lp Account. To save withdraw lp with must cancle orders as temp to transfer later.
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    /// Serum dex program.
    pub serum_program: AccountInfo<'info>,
    /// Serum market Account. serum_dex program is the owner.
    pub serum_market: AccountInfo<'info>,
    /// The user wallet create the pool
    #[account(signer)]
    pub user_wallet: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts for an `deposit` instruction.
#[derive(Accounts, Clone)]
pub struct Deposit<'info> {
    /// Amm Account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// Amm authority, a PDA account derived with seed `amm authority` and amm program address
    pub amm_authority: AccountInfo<'info>,
    /// AMM open_orders Account.
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// AMM target orders account. To store plan orders infomations.
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// LP mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// Pool_token_coin account, $authority can transfer amount.
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// Pool_token_pc account, $authority can transfer amount.
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// Serum market account, serum_dex program is the owner.
    pub serum_market: AccountInfo<'info>,
    /// User coin token account to deposit into.
    #[account(mut)]
    pub user_coin_token_account: AccountInfo<'info>,
    /// User pc token account to deposit into.
    #[account(mut)]
    pub user_pc_token_account: AccountInfo<'info>,
    /// User lp token account, to deposit the generated tokens, user is the owner
    #[account(mut)]
    pub user_lp_token_account: AccountInfo<'info>,
    /// User wallet account
    #[account(signer)]
    pub user_owner: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `withdraw` instruction.
#[derive(Accounts, Clone)]
pub struct Withdraw<'info> {
    /// Amm account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// Amm authority Account
    pub amm_authority: AccountInfo<'info>,
    /// amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// amm target_orders Account. To store plan orders infomations.
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// pool lp mint account. Must be empty, owned by $authority.
    #[account(mut)]
    pub lp_mint: AccountInfo<'info>,
    /// pool_token_coin Amm Account to withdraw FROM,
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// pool_token_pc Amm Account to withdraw FROM,
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// withdraw queue Account
    #[account(mut)]
    pub pool_withdraw_queue: AccountInfo<'info>,
    /// token_temp_lp Account
    #[account(mut)]
    pub pool_temp_lp_token_account: AccountInfo<'info>,
    /// serum dex program id
    pub serum_program: AccountInfo<'info>,
    /// serum market Account. serum_dex program is the owner.
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// coin_vault Account
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// pc_vault Account
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// vault_signer Account
    pub serum_vault_signer: AccountInfo<'info>,
    /// user lp token Account. Source lp, amount is transferable by $authority.
    #[account(mut)]
    pub user_lp_token_account: AccountInfo<'info>,
    /// user token coin Account. user Account to credit.
    #[account(mut)]
    pub user_coin_token_account: AccountInfo<'info>,
    /// user token pc Account. user Account to credit.
    #[account(mut)]
    pub user_pc_token_account: AccountInfo<'info>,
    /// User wallet account
    #[account(signer)]
    pub user_owner: AccountInfo<'info>,
    /// Serum event queue account
    #[account(mut)]
    pub serum_event_q: AccountInfo<'info>,
    // Serum bid account
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    // Serum ask account
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `swap_base_in` instruction.
#[derive(Accounts, Clone)]
pub struct SwapBaseIn<'info> {
    /// amm Account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// Amm authority Account
    pub amm_authority: AccountInfo<'info>,
    /// amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// amm target_orders Account
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// pool_token_coin Amm Account to swap FROM or To,
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// pool_token_pc Amm Account to swap FROM or To,
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// serum dex program id
    pub serum_program: AccountInfo<'info>,
    /// serum market Account. serum_dex program is the owner.
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// bids Account
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// asks Account
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// event_q Account
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    /// coin_vault Account
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// pc_vault Account
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// vault_signer Account
    #[account(mut)]
    pub serum_vault_signer: AccountInfo<'info>,
    /// user source token Account. user Account to swap from.
    #[account(mut)]
    pub user_source_token_account: AccountInfo<'info>,
    /// user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_destination_token_account: AccountInfo<'info>,
    /// user owner Account
    #[account(signer)]
    pub user_source_owner: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}

/// Accounts for an `swap_base_out` instruction.
#[derive(Accounts, Clone)]
pub struct SwapBaseOut<'info> {
    /// amm Account
    #[account(mut)]
    pub amm: AccountInfo<'info>,
    /// Amm authority Account
    pub amm_authority: AccountInfo<'info>,
    /// amm open_orders Account
    #[account(mut)]
    pub amm_open_orders: AccountInfo<'info>,
    /// amm target_orders Account
    #[account(mut)]
    pub amm_target_orders: AccountInfo<'info>,
    /// pool_token_coin Amm Account to swap FROM or To,
    #[account(mut)]
    pub pool_coin_token_account: AccountInfo<'info>,
    /// pool_token_pc Amm Account to swap FROM or To,
    #[account(mut)]
    pub pool_pc_token_account: AccountInfo<'info>,
    /// serum dex program id
    pub serum_program: AccountInfo<'info>,
    /// serum market Account. serum_dex program is the owner.
    #[account(mut)]
    pub serum_market: AccountInfo<'info>,
    /// bids Account
    #[account(mut)]
    pub serum_bids: AccountInfo<'info>,
    /// asks Account
    #[account(mut)]
    pub serum_asks: AccountInfo<'info>,
    /// event_q Account
    #[account(mut)]
    pub serum_event_queue: AccountInfo<'info>,
    /// coin_vault Account
    #[account(mut)]
    pub serum_coin_vault_account: AccountInfo<'info>,
    /// pc_vault Account
    #[account(mut)]
    pub serum_pc_vault_account: AccountInfo<'info>,
    /// vault_signer Account
    #[account(mut)]
    pub serum_vault_signer: AccountInfo<'info>,
    /// user source token Account. user Account to swap from.
    #[account(mut)]
    pub user_source_token_account: AccountInfo<'info>,
    /// user destination token Account. user Account to swap to.
    #[account(mut)]
    pub user_destination_token_account: AccountInfo<'info>,
    /// user owner Account
    #[account(signer)]
    pub user_source_owner: AccountInfo<'info>,
    #[account(address = spl_token::ID)]
    pub spl_token_program: AccountInfo<'info>,
}
