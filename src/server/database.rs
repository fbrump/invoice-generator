use crate::server::model::{CurrencyEnum, Transaction};
use chrono::Utc;
use uuid::uuid;

pub fn db() -> Vec<Transaction> {
    vec![
        Transaction::new(
            uuid!("6bc8ed59-96bc-4885-8839-4e234462cf05"),
            "Cappuchino".to_string(),
            "Starbucks".to_string(),
            5.50,
            CurrencyEnum::DOLLAR,
            Utc::now(),
        ),
        Transaction::new(
            uuid!("74526606-4305-4388-92a3-f4cf2c203e6b"),
            "Coke".to_string(),
            "Wallmart".to_string(),
            7.80,
            CurrencyEnum::DOLLAR,
            Utc::now(),
        ),
        Transaction::new(
            uuid!("3dbfd7a8-db68-4e47-b5fa-ffa40ca7510e"),
            "Ice Crean".to_string(),
            "Wallmart".to_string(),
            2.50,
            CurrencyEnum::EURO,
            Utc::now(),
        ),
        Transaction::new(
            uuid!("00160798-742e-4248-b626-e6a595302fbf"),
            "Cookie".to_string(),
            "Starbucks".to_string(),
            4.00,
            CurrencyEnum::DOLLAR,
            Utc::now(),
        ),
    ]
}
