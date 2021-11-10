use anchor_lang::prelude::*;

declare_id!("4HcMYQPwtUrcd1R1qEkNnvSw2xQhmYrRfRMrr3uqa1BU");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        //Reference to the base account
        let base_account = &mut ctx.accounts.base_account;
        //Initialize total gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, url : String ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;

        let item = Gif {
            gif_url: url.to_string(),
            user_address: *base_account.to_account_info().key
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;

        Ok(())
    }
}

// Specify what data you want in the AddGif Context.
// I create a Context named AddGif that has access to a mutable reference to base_account. That's why I do #[account(mut)].
// Basically it means I can actually change the total_gifs value stored on BaseAccount.
// Otherwise, I may change data on it within my function but it wouldn't actually change on my account.
// Now, w/ a "mutable" reference if I mess w/ base_account in my function it'll change data on the account itself.
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

//Attach certain variables to the Initialize context.
#[derive(Accounts)]
pub struct Initialize<'info> {
    //All we're doing here is telling Solana how we want to initialize BaseAccount.
    //1. init will tell Solana to create a new account owned by our current program.
    //2. payer = user tells our program who's paying for the account to be created. In this case, it's the user calling the function.
    //3. We then say space = 9000 which will allocate 9000 bytes of space for our account. You can change this # if you wanted,
    //but, 9000 bytes is enough for the program we'll be building here!
    //Why are we paying for an account? Well — storing data isn't free! How Solana works is users will pay "rent" on their accounts.
    //The rent is calculated by the number of bytes they're using.
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list : Vec<Gif>
}

//Creating a custom struct for us to work with 
//Basically this tells Anchor how to serialize/deserialize the struct.
//Remember, data is being stored in an "account" right ? That account is basically a file and we serialize our data into binary 
//format before storing it.Then, when we want to retrieve it we'll actually deserialize it.
//This line takes care of that to make sure our data is properly serialized/deserialized since we're creating a custom struct here.

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Gif {
    pub gif_url : String,
    pub user_address : Pubkey
}
