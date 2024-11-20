#![allow(dead_code, non_snake_case)]
use anyhow::{Context, Result};
use chrono::prelude::*;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Record {
  pub id: u32,
  pub tid: String,
  pub description: String,
  pub amount: i64,
  pub cusumBalance: Option<i64>,
  pub when: String,
  pub bookingDate: Option<String>,
  pub bankSubAccId: String,
  pub paymentChannel: String,
  pub virtualAccount: String,
  pub virtualAccountName: String,
  pub corresponsiveName: String,
  pub corresponsiveAccount: String,
  pub corresponsiveBankId: String,
  pub corresponsiveBankName: String,
  pub accountId: u32,
  pub bankCodeName: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Transaction {
  page: u16,
  pageSize: u16,
  nextPage: u16,
  prevPage: u16,
  totalPages: u32,
  totalRecords: u32,
  pub records: Vec<Record>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionResponse {
  error: i8,
  message: String,
  pub data: Transaction,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct MB {
  pub latest_transaction: Record,
  pub transaction: TransactionResponse,
}

impl MB {
  pub fn new() -> Self {
    MB {
      transaction: TransactionResponse {
        error: 0,
        message: String::from("success"),
        data: Transaction {
          page: 0,
          pageSize: 0,
          nextPage: 0,
          prevPage: 0,
          totalPages: 0,
          totalRecords: 0,
          records: Vec::new(),
        },
      },
      latest_transaction: Record::default(),
    }
  }

  pub fn fetch_transaction(&mut self) -> Result<()> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    let apikey = env::var("API_KEY").unwrap_or_else(|_| "nah".to_string());

    headers.insert(
      "Authorization",
      HeaderValue::from_str(&format!("Apikey {}", apikey))?,
    );

    let api_url = format!(
      "https://oauth.casso.vn/v2/transactions?fromDate={}&page=1&pageSize=5&sort=DESC",
      Utc::now().format("%Y-%m-%d").to_string()
    );

    let response = client
      .get(&api_url)
      .headers(headers)
      .send()
      .context("Failed to send request")?;

    if response.status().is_success() {
      let json = match response.json::<TransactionResponse>() {
        Ok(data) => data,
        Err(e) => {
          eprintln!("Failed to parse response as JSON: {}", e);
          return Err(anyhow::anyhow!("Failed to parse JSON response").into());
        }
      };
      self.transaction = json;
    }
    Ok(())
  }

  pub fn compare_transaction(&mut self) -> Result<bool> {
    let latest_transaction = self.latest_transaction.clone();
    let next_transaction = self.transaction.data.records.first().unwrap().clone();

    if latest_transaction.id != 0 && latest_transaction.id < next_transaction.id {
      self.latest_transaction = next_transaction;
      Ok(true)
    } else {
      self.latest_transaction = next_transaction;
      Ok(false)
    }
  }

  pub fn get_latest_transaction(&self) -> Result<Record> {
    Ok(self.latest_transaction.clone())
  }
}
