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

    pub fn update(
        ctx: Context<Update>,
        open: u128, // TODO: really unwieldy, but I'm not sure if I can pass an object (bc Serialize is req'd) and use zero copy
        high: u128,
        low: u128,
        close: u128,
        volume: u128,
        unix_time: u128,
    ) -> ProgramResult {
        let mut candle_frame = ctx.accounts.candle_frame.load_mut()?;
        candle_frame.append({
            Candle {
                open,
                high,
                low,
                close,
                volume,
                unix_time,
            }
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(zero)]
    pub candle_frame: Loader<'info, CandleFrame>,
    #[account(signer)]
    pub updater: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut, has_one = updater)]
    pub candle_frame: Loader<'info, CandleFrame>,
    #[account(signer)]
    pub updater: AccountInfo<'info>,
}

#[account(zero_copy)]
pub struct CandleFrame {
    pub updater: Pubkey, // 32
    head: u64,           // 8
    // pub resolution : string like?
    // pub base: Pubkey, // TODO: add base and quote addresses to frame creation
    // pub quote: Pubkey,
    pub candles: [Candle; 30], // 96 * 30 = 2880
}

impl CandleFrame {
    fn append(&mut self, candle: Candle) {
        self.candles[CandleFrame::index_of(self.head)] = candle;
        self.head = (self.head + 1) % 30;
        if self.head == 30 {
            self.head = 0
        }
    }
    fn index_of(value: u64) -> usize {
        std::convert::TryInto::try_into(value).unwrap() // mod by max # of candles
    }
}

#[zero_copy]
pub struct Candle {
    // 16 * 6 = 96
    open: u128,
    high: u128,
    low: u128,
    close: u128,
    volume: u128,
    unix_time: u128,
}
