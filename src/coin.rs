use ethers::{prelude::*, types::Address, utils::parse_units};
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::abis::Quoter;

const DEFAULT_CHAIN: u16 = 0x1;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Pair(pub Coin, pub Coin, pub Option<String>);

impl Default for Pair {
	fn default() -> Self {
		Self::empty()
	}
}

impl Pair {
	pub fn empty() -> Self {
		Self(Coin::empty(), Coin::empty(), None)
	}

	pub fn usdc_weth(chain: Option<u16>) -> Self {
		Self(
			Coin::usdc(chain),
			Coin::weth(chain),
			Some(String::from("ETH/USD")),
		)
	}
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Coin {
	pub name: String,
	pub address: Address,
	pub decimals: i32,
}

impl Default for Coin {
	fn default() -> Self {
		Self::empty()
	}
}

impl Coin {
	pub fn get_coin(name: &str, chain: Option<u16>) -> Option<Self> {
		match name {
			"USDC" => Some(Self::usdc(chain)),
			"WETH" => Some(Self::weth(chain)),
			_ => None,
		}
	}

	pub fn empty() -> Self {
		Self {
			name: String::from(""),
			address: Address::zero(),
			decimals: 0,
		}
	}

	pub fn usdc(chain: Option<u16>) -> Self {
		let chain = match chain {
			Some(chain) => chain,
			None => DEFAULT_CHAIN,
		};

		Self {
			name: String::from("USDC"),
			address: match chain {
				0x1 => "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", // mainnet
				0x5 => "0x07865c6E87B9F70255377e024ace6630C1Eaa37F", // goerli
				0xa4b1 => "0xaf88d065e77c8cc2239327c5edb3a432268e5831", // arbitrum one (native)
				_ => panic!("unsupported chain {} for USDC", chain),
			}
			.parse()
			.expect("USDC address should parse into Address"),
			decimals: 6,
		}
	}

	pub fn weth(chain: Option<u16>) -> Self {
		let chain = match chain {
			Some(chain) => chain,
			None => DEFAULT_CHAIN,
		};

		Self {
			name: String::from("WETH"),
			address: match chain {
				0x1 => "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
				0x5 => "0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6",
				0xa4b1 => "0x82af49447d8a07e3bd95bd0d56f35241523fbab1",
				_ => panic!("unsupported chain {} for WETH", chain),
			}
			.parse()
			.expect("WETH address should parse into Address"),
			decimals: 18,
		}
	}
}
