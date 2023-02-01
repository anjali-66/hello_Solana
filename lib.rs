// Import anchor
use anchor_lang::prelude::*;
//Declare Program ID
//with Anchor ,We have to declare our program 's public  key .This is being used by 
//to improve the security of our program.
declare_id!("65rL3ia7WpUzYW4A3sMaWoWgsgoT6idQJ5F3gWQF9hLX");
#[program]
mod hello_world {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {

        msg!("Hello, World!");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Hello {}
