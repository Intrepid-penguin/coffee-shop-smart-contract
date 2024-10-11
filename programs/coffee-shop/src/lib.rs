use anchor_lang::prelude::*;

declare_id!("4XPmepo96zGwct8SjBmmKyMYEybHFDGibJ8FXM4FCY6F");

#[program]
pub mod coffee_store {
    use super::*;

    // Initialize the coffee store with an admin
    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey, store_name: String) -> Result<()> {
        let store = &mut ctx.accounts.store;
        store.admin = admin;
        store.store_name = store_name;
        Ok(())
    }

    // Create a coffee item (Admin only)
    pub fn create_coffee(ctx: Context<CreateCoffee>, name: String, price: u8) -> Result<()> {
        let store = &ctx.accounts.store;
        require!(
            store.admin == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );

        let coffee = &mut ctx.accounts.coffee;
        coffee.name = name;
        coffee.price = price;
        Ok(())
    }

    // Update a coffee item (Admin only)
    pub fn update_coffee(ctx: Context<UpdateCoffee>, name: String, price: u8) -> Result<()> {
        let store = &ctx.accounts.store;
        require!(
            store.admin == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );

        let coffee = &mut ctx.accounts.coffee;
        coffee.name = name;
        coffee.price = price;
        Ok(())
    }

    // Delete a coffee item (Admin only)
    pub fn delete_coffee(ctx: Context<DeleteCoffee>) -> Result<()> {
        let store = &ctx.accounts.store;
        require!(
            store.admin == ctx.accounts.user.key(),
            ErrorCode::Unauthorized
        );

        let coffee = &mut ctx.accounts.coffee;
        coffee.name = String::from("");
        coffee.price = 0;
        Ok(())
    }

    // Buy coffee
    pub fn buy_coffee(ctx: Context<BuyCoffee>) -> Result<()> {
        let coffee = &mut ctx.accounts.coffee;
        let user = &ctx.accounts.user;
        let price = coffee.price;

        // Ensure that the coffee price is valid
        require!(price > 0, ErrorCode::InvalidCoffee);

        // Transfer lamports from the user (buyer) to the admin
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &user.key(),
            &ctx.accounts.admin.key(),
            price.into(),
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[user.to_account_info(), ctx.accounts.admin.to_account_info()],
        )?;

        msg!(
            "User {} bought {} for {} lamports",
            user.key(),
            coffee.name,
            price
        );
        Ok(())
    }

    // Read a coffee item
    pub fn read_coffee(ctx: Context<ReadCoffee>) -> Result<()> {
        let coffee = &ctx.accounts.coffee;
        msg!("Coffee: {}, Price: {}", coffee.name, coffee.price);
        Ok(())
    }
}

#[account]
pub struct Coffee {
    pub name: String,
    pub price: u8,
}

#[account]
pub struct CoffeeStore {
    pub admin: Pubkey,
    pub store_name: String,
}

// Initialize context (Admin setup)
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 4 + 40)]
    pub store: Account<'info, CoffeeStore>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Create context
#[derive(Accounts)]
pub struct CreateCoffee<'info> {
    #[account(init, payer = user, space = 8 + 40 + 8)]
    pub coffee: Account<'info, Coffee>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub store: Account<'info, CoffeeStore>,
    pub system_program: Program<'info, System>,
}

// Update context
#[derive(Accounts)]
pub struct UpdateCoffee<'info> {
    #[account(mut)]
    pub coffee: Account<'info, Coffee>,
    pub user: Signer<'info>,
    #[account(mut)]
    pub store: Account<'info, CoffeeStore>,
}

// Delete context
#[derive(Accounts)]
pub struct DeleteCoffee<'info> {
    #[account(mut, close = user)]
    pub coffee: Account<'info, Coffee>,
    pub user: Signer<'info>,
    #[account(mut)]
    pub store: Account<'info, CoffeeStore>,
}

// Buy coffee context
#[derive(Accounts)]
pub struct BuyCoffee<'info> {
    #[account(mut)]
    pub coffee: Account<'info, Coffee>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub admin: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

// Read context
#[derive(Accounts)]
pub struct ReadCoffee<'info> {
    pub coffee: Account<'info, Coffee>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized. Only admin can perform this action.")]
    Unauthorized,
    #[msg("Invalid coffee.")]
    InvalidCoffee,
}

