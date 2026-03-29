use serenity::all::UserId;
use sqlx::PgPool;

use crate::morex::account;

pub struct BankAccount {
    pub wallet: i64,
    pub bank: i64
}

pub async fn get_bank_account(user_id: &UserId, pool: &PgPool) -> BankAccount {
    let user = account::fetch_account(&user_id, &pool).await;

    BankAccount {
        wallet: user.wallet,
        bank: user.bank
    }
}
