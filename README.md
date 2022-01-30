# zo-abi

The abi is a repository for interfacing with the 01 program either through a rust client or through CPIs from another Solana program.

### Devnet token faucet
Replace `<WALLET>`, `<MINT>`, and `<AMOUNT>`

```bash 
curl -XPOST 'https://devnet-faucet.01.xyz?owner=<WALLET>&mint=<MINT>&amount=<AMOUNT>'
```

SOL can be deposited directly using native lamports. You can get SOL either through Solana cli airdrop or at any airdrop faucet.

### Usage examples

`zo-abi` can be used just like any other project built with `anchor-lang`.

#### CPI from a Solana program

Below is an example of a CPI call into the `zo` program calling the `deposit` instruction. For more information, see the
[official anchor documentation on CPI](https://project-serum.github.io/anchor/tutorials/tutorial-3.html).

Note that as with all `anchor-lang` programs, the `cpi` feature of this package must be enabled.

```rust
use anchor_lang::prelude::*;
use zo_abi::{self as zo, program::ZoAbi as Zo};

#[program]
mod my_program {
    use super::*;

    pub fn do_deposit(ctx: Context<DoDeposit>, amount: u64) -> ProgramResult {
        let cpi_program = ctx.accounts.zo_program.to_account_info();
        let cpi_accounts = zo::cpi::accounts::Deposit {
            authority: ctx.accounts.authority.to_account_info(),
            state: ctx.accounts.zo_state.to_account_info(),
            state_signer: ctx.accounts.zo_state_signer.to_account_info(),
            cache: ctx.accounts.zo_cache.to_account_info(),
            margin: ctx.accounts.zo_margin.to_account_info(),
            vault: ctx.accounts.zo_vault.to_account_info(),
            token_account: ctx.accounts.token_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        zo::cpi::deposit(cpi_ctx, false, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct DoDeposit<'info> {
    pub authority: Signer<'info>,
    pub zo_state: AccountLoader<'info, zo::State>,
    pub zo_state_signer: UncheckedAccount<'info>,
    #[account(mut)]
    pub zo_cache: AccountLoader<'info, zo::Cache>,
    #[account(mut)]
    pub zo_margin: AccountLoader<'info, zo::Margin>,
    pub zo_vault: UncheckedAccount<'info>,
    pub zo_program: Program<'info, Zo>,
    pub token_account: UncheckedAccount<'info>,
    pub token_program: UncheckedAccount<'info>,
}
```

#### Client

For a more comprehensive example, see the
[official `anchor-client` example](https://github.com/project-serum/anchor/blob/master/client/example/src/main.rs).

```rust
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signer::keypair::read_keypair_file;
use anchor_client::{Client, Cluster};
use std::rc::Rc;
use zo_abi as zo;

fn main() {
    let payer = read_keypair_file("/path/to/keypair/file").unwrap();
    let client = Client::new(Cluster::Devnet, Rc::new(payer));
    let program = client.program(zo::ID);

    // Loading a program account.
    let zo_state: zo::State = program.account(zo::STATE_ID).unwrap();
    let zo_cache: zo::Cache = program.account(zo_state.cache).unwrap();

    let (zo_state_signer, _) = Pubkey::find_program_address(&[zo::STATE_ID.as_ref()], &zo::ID);

    // Calling an instruction.
    program
        .request()
        .args(zo::instruction::Deposit {
            repay_only: false,
            amount: 1_000_000,
        })
        .accounts(zo::accounts::Deposit {
            authority: program.payer(),
            state: zo::STATE_ID,
            state_signer: zo_state_signer,
            cache: zo_state.cache,
            token_program: spl_token::ID,
            // ..., etc.
        })
        .send()
        .unwrap();
}
```
