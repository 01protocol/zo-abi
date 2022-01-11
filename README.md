# 01 abi 

The abi is a repository for interfacing with the 01 program either through a rust client or through CPIs from another Solana program.

### Program Accounts
|       | Cluster | Pubkey                                       |
| ----- |---------|----------------------------------------------|
| State | Devnet  | HAdeMzG1ZuzhWnt26iyggLhYUen3YosXiD5sgDXJoNDY |

### Collaterals
| Symbol      | Cluster | Mint                                         | Decimals |
| ----------- | ------- | -------------------------------------------- | -------- |
| USDC        | Devnet  | 7UT1javY6X1M9R2UrPGrwcZ78SX3huaXyETff5hm5YdX | 6        |
| BTC         | Devnet  | 3n3sMJMnZhgNDaxp6cfywvjHLrV1s34ndfa6xAaYvpRs | 6        |
| SOL         | Devnet  | So11111111111111111111111111111111111111112  | 9        |

### Devnet token faucet
Replace `<WALLET>`, `<MINT>`, and `<AMOUNT>`
```bash 
curl -XPOST 'https://devnet-faucet.01.xyz?owner=<WALLET>&mint=<MINT>&amount=<AMOUNT>'
```
SOL can be deposited directly using native lamports. You can get SOL either through Solana cli airdrop or at any airdrop faucet.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.
