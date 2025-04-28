use anchor_lang::prelude::*;

declare_id!("62cUMnHaFETt81n2AeHLEVpvSD6awPyh1wrGbhXZ6s8g");

#[program]
pub mod anchor_project {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
