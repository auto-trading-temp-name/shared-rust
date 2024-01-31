use ethers::contract::abigen;

abigen!(SwapRouter, "src/abis/SwapRouter.json");
abigen!(Quoter, "src/abis/Quoter.json");
