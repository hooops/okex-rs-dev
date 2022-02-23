pub mod utils;
pub mod rest;
pub mod ws;

#[cfg(test)]
mod tests {    use std::collections::HashMap;
    use crate::rest::{account, Client};

    #[tokio::test]
    async fn it_works() {
        let mut a = Client::new("x", "y", "z").unwrap();

        let mut map = HashMap::new();
        map.insert("ccy", "BTC");
        let b = account::balance(&mut a, Some(map)).await.unwrap();
        println!("{}", b);
    }
}
