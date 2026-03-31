pub mod mx;
use crate::mx::morex::Morex;

#[tokio::main]
async fn main() {
    let app = Morex::new();
    app.run().await;
}