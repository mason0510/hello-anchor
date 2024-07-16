use anchor_lang::prelude::*;

declare_id!("448KahuLdmiQpbrdGziRuQ6Y3mYG3uKWCWzcV3U9UWz6");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> anchor_lang::Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

