use crate::server::model::{CurrencyEnum, Transaction};
use chrono::{Utc};
use uuid::uuid;

pub fn db() -> Vec<Transaction> {
    vec![
        Transaction {
            id: uuid!("6bc8ed59-96bc-4885-8839-4e234462cf05"),
            description: "Cappuchino".to_string(),
            merchant_name: "Starbucks".to_string(),
            value: 5.50,
            currency: CurrencyEnum::DOLLAR, 
            date_time: Utc::now(), 
        },
        Transaction {
            id: uuid!("74526606-4305-4388-92a3-f4cf2c203e6b"),
            description: "Coke".to_string(),
            merchant_name: "Wallmart".to_string(),
            value: 7.80,
            currency: CurrencyEnum::DOLLAR, 
            date_time: Utc::now(), 
        },
        Transaction {
            id: uuid!("3dbfd7a8-db68-4e47-b5fa-ffa40ca7510e"),
            description: "Ice Crean".to_string(),
            merchant_name: "Wallmart".to_string(),
            value: 2.50,
            currency: CurrencyEnum::EURO, 
            date_time: Utc::now(), 
        },
        Transaction {
            id: uuid!("00160798-742e-4248-b626-e6a595302fbf"),
            description: "Cookie".to_string(),
            merchant_name: "Starbucks".to_string(),
            value: 4.00,
            currency: CurrencyEnum::DOLLAR, 
            date_time: Utc::now(), 
        },
    ]
}