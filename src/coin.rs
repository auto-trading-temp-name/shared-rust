use std::fs;

use ethers::{abi::Address, prelude::*, utils::parse_units};
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::abis::Quoter;

const COINS_PATH: &str = "coins.json";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pair(pub Coin, pub Coin);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Coin {
	pub name: String,
	pub fallback_name: String,
	pub address: Address,
	pub decimals: i32,
}

impl Coin {
	pub async fn get_price(
		&self,
		reference_coin: &Coin,
		quoter: &Quoter<Provider<Http>>,
	) -> Result<u32> {
		Ok(
			quoter
				.quote_exact_input_single(
					self.address,
					reference_coin.address,
					500,
					U256::from(
						parse_units(1.0, self.decimals)
							.wrap_err(format!("1 {} did not parse correctly", reference_coin.name))?,
					),
					U256::zero(),
				)
				.call()
				.await?
				.as_u32(),
		)
	}

	pub fn empty() -> Self {
		Self {
			name: String::from(""),
			fallback_name: String::from(""),
			address: Address::zero(),
			decimals: 0,
		}
	}
}

pub fn load_coins() -> (Coin, Vec<Coin>) {
	let coin_data: Vec<Coin> =
		serde_json::from_str(&fs::read_to_string(COINS_PATH).expect("{COINS_PATH} does not exist"))
			.expect("{COINS_PATH} is not valid");
	(coin_data[0].clone(), coin_data[1..].to_vec())
}
