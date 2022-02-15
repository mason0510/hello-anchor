use anchor_lang::prelude::*;

declare_id!("448KahuLdmiQpbrdGziRuQ6Y3mYG3uKWCWzcV3U9UWz6");

#[program]
mod hello_anchor {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
