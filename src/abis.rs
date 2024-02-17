use ethers::contract::abigen;

abigen!(ERC20, "src/abis/ERC20.json");
abigen!(SwapRouter, "src/abis/SwapRouter.json");
abigen!(Quoter, "src/abis/Quoter.json");
