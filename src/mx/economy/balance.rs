use serenity::all::UserId;
use sqlx::PgPool;
use crate::mx::economy::accounts::Accounts;

pub struct BankAccount {
   pub wallet: i64,
   pub bank: i64
}

pub struct Balance;
impl Balance {
   pub async fn get_bank_account(user_id: &UserId, pool: &PgPool) -> BankAccount {
      let user = Accounts::fetch_account(&user_id, &pool).await;

      BankAccount {
         wallet: user.wallet,
         bank: user.bank
      }
   }
}