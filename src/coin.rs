use ethers::{prelude::*, types::Address, utils::parse_units};
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::abis::Quoter;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pair(pub Coin, pub Coin);

impl Pair {
	pub fn usdc_weth() -> Self {
		Self(Coin::usdc(), Coin::weth())
	}
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Coin {
	pub name: String,
	pub fallback_name: String,
	pub address: Address,
	pub decimals: i32,
}

impl Default for Coin {
	fn default() -> Self {
		Self::empty()
	}
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

	pub fn get_coin(name: &str) -> Option<Self> {
		match name {
			"USDC" => Some(Self::usdc()),
			"WETH" => Some(Self::weth()),
			_ => None,
		}
	}

	pub fn empty() -> Self {
		Self {
			name: String::from(""),
			fallback_name: String::from(""),
			address: Address::zero(),
			decimals: 0,
		}
	}

	pub fn usdc() -> Self {
		Self {
			name: String::from("USDC"),
			fallback_name: String::from(""),
			address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
				.parse()
				.expect("USDC address should parse into Address"),
			decimals: 6,
		}
	}

	pub fn weth() -> Self {
		Self {
			name: String::from("WETH"),
			fallback_name: String::from("ETH/USD"),
			address: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
				.parse()
				.expect("USDC address should parse into Address"),
			decimals: 18,
		}
	}
}
