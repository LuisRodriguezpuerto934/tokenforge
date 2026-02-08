# 🔨 TokenForge

**One-Command Token Launcher on Solana**

[![Solana](https://img.shields.io/badge/Solana-Devnet-purple)](https://solana.com)
[![Anchor](https://img.shields.io/badge/Anchor-0.32.1-blue)](https://anchor-lang.com)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## 🚀 Overview

TokenForge is a streamlined Solana token launcher that creates SPL tokens with a single command. Built with Anchor framework for security and reliability.

## ✨ Features

- ✅ **One-Command Launch** - Create token in seconds
- ✅ **Custom Metadata** - Name, symbol, supply
- ✅ **Mint Authority** - Automatic configuration
- ✅ **Liquidity Ready** - Prepared for DEX integration
- ✅ **Low Fees** - Minimal SOL for deployment

## 📦 Installation

```bash
# Clone repository
git clone https://github.com/LuisRodriguezpuerto934/tokenforge.git
cd tokenforge

# Install dependencies
npm install

# Build program
anchor build
```

## 🔧 Setup

```bash
# Configure Solana CLI
solana config set --url devnet

# Set deployment wallet
export ANCHOR_WALLET=~/.config/solana/id.json
```

## 📝 Usage

### CLI Tool
```bash
# Create new token
tokenforge create "MyToken" MTK 1000000

# Output:
# ✅ Token created: 7xK9...mN3p
# 📊 Supply: 1,000,000 MTK
# 🔗 Explorer: https://explorer.solana.com/address/...
```

### Program Integration
```rust
use tokenforge::create_token;

let token = create_token(
    "MyToken",           // name
    "MTK",               // symbol
    1_000_000,           // supply
    &payer,              // payer
    &program_id
)?;
```

## 🏗️ Architecture

```rust
// Core instruction
pub fn create_token(
    ctx: Context<CreateToken>,
    name: String,
    symbol: String,
    supply: u64,
) -> Result<Pubkey> {
    // 1. Create mint account
    // 2. Initialize token metadata
    // 3. Mint initial supply
    // 4. Return token address
}
```

## 📊 Token Structure

| Field | Type | Description |
|-------|------|-------------|
| name | String | Token name |
| symbol | String | Token symbol |
| supply | u64 | Total supply |
| decimals | u8 | Decimal places |
| mint_authority | Pubkey | Mint controller |

## 🎯 Use Cases

- **Community Tokens** - Launch for DAOs
- **Reward Points** - In-app currencies
- **Test Tokens** - Devnet experimentation
- **Meme Coins** - Quick launches

## 🔐 Security

- PDA-based mint accounts
- Authority validation
- Supply cap enforcement
- No reentrancy risks

## 📈 Cost

| Action | Cost (SOL) |
|--------|-----------|
| Create Token | ~0.001 |
| Mint Supply | ~0.0005 |
| Total | ~0.0015 |

## 🛠️ Tech Stack

- **Framework:** Anchor 0.32.1
- **Language:** Rust
- **Network:** Solana Devnet/Mainnet
- **Token Standard:** SPL Token

## 🧪 Testing

```bash
# Run tests
anchor test

# Test on devnet
anchor deploy --provider.cluster devnet
```

## 🚀 Deployment

```bash
# Deploy to devnet
anchor deploy --provider.cluster devnet

# Deploy to mainnet
anchor deploy --provider.cluster mainnet
```

## 📄 Smart Contract

**Location:** `programs/tokenforge/src/lib.rs`
**Lines:** 175
**Instructions:** 1 (create_token)
**Accounts:** 4

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 License

MIT License

## 👤 Author

**Luis Rodriguez Puerto**
- X: [@BrainTease870](https://x.com/BrainTease870)
- GitHub: [@LuisRodriguezpuerto934](https://github.com/LuisRodriguezpuerto934)

## 🏆 Hackathon

Submitted to **USDC Agent Hackathon 2026**
- Track: Agentic Commerce
- Prize Pool: $30,000 USDC

---

**Built with ⚓ Anchor on Solana**

## Quickstart

```bash
# TODO: agrega pasos de instalacion/ejecucion
```

