use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod permissionless_candles {
    use super::*;

    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let mut candle_frame = ctx.accounts.candle_frame.load_init()?;
        candle_frame.updater = *ctx.accounts.updater.key;
        Ok(())
    }

    // pub fn update(ctx: Context<Update>) -> ProgramResult {
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = updater, space = 8 + 2914)]
    pub candle_frame: Loader<'info, CandleFrame>,
    #[account(mut)]
    pub updater: Signer<'info>,
    pub system_program: Program<'info, System>,
}

//Need to verify that the right person is updating
// #[derive(Accounts)]
// pub struct Update<'info> {
//     #[account(init, payer = updater, space = 8 + 2914)]
//     pub candle_frame: Loader<'info, CandleFrame>,
//     #[account(mut)]
//     pub updater: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }


#[account(zero_copy)]
pub struct CandleFrame {
    pub updater: Pubkey,       // 32
    head: u8, // 1
    tail: u8, // 1
    // pub resolution : not sure
    pub candles: [Candle ; 30] // 96 * 30 = 2880
}

impl CandleFrame {
    fn append(&mut self, candle: Candle) {
        self.candles[CandleFrame::index_of(self.head)] = candle;
        if CandleFrame::index_of(self.head + 1) == CandleFrame::index_of(self.tail) {
            self.tail += 1;
        }
        self.head += 1;
    }
    fn index_of(counter: u8) -> usize {
        std::convert::TryInto::try_into(counter % 30).unwrap() // mod by max # of candles
    }
}

#[zero_copy]
pub struct Candle { // 16 * 6 = 96
    open: u128,
    high: u128,
    low: u128,
    close: u128,
    volume: u128,
    unix_time: u128,
}
