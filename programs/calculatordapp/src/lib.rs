use anchor_lang::prelude::*;

declare_id!("HgqwqYTqTvDYbarZCBijebg5wCzqkjSYxT4T1vbH75he");

#[program]
pub mod calculatordapp {
    use super::*;

    pub fn create(ctx: Context<Create>, init_message: String) -> {
        let calculator = &mut ctx.account.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
}

#derive(Accounts)
pub struct create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}
