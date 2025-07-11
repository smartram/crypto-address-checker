# Crypto Address Checker Browser Extension

A Rust + WebAssembly browser extension that identifies cryptocurrency networks from addresses. Supports Bitcoin, Ethereum, Litecoin, Dogecoin, Solana, Cardano, and the entire Polkadot/Substrate ecosystem.

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/crypto-address-checker.git
cd crypto-address-checker

# Make build script executable
chmod +x build.sh

# Build the extension
./build.sh
```

Then load the `extension` folder in your browser as described below.

## ✨ Features

- **Multi-Network Support**: Identifies addresses from 7+ major cryptocurrency networks
- **Real-time Detection**: Automatically highlights crypto addresses on web pages
- **Popup Interface**: Manual address validation through extension popup
- **Tooltips**: Hover over highlighted addresses for network information
- **Context Menu**: Right-click to check selected text
- **Polkadot Ecosystem**: Full SS58 address support for 90+ Substrate-based chains

## 🌐 Supported Networks

| Network | Address Types | Example |
|---------|---------------|---------|
| Bitcoin | Legacy (1...), Script (3...), Bech32 (bc1...) | `1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa` |
| Ethereum | Standard (0x...) | `0x9575a638118E4Ec7aAfdd3A4eb9e1AC4146F4Dd8` |
| Litecoin | Legacy (L.../M...), Bech32 (ltc1...) | `LdP8Qox1VAhCzLJNqrr74YovaWYyNBUWvL` |
| Dogecoin | Standard (D.../A...) | `DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L` |
| Solana | Base58 encoded | `11111111111111111111111111111112` |
| Cardano | Bech32 (addr1...) | `addr1...` |
| Polkadot | SS58 format | `15oF4uVJwmo4TdGW7VfQxNLavjCXp2p9rB2X3DEkiKbvWZ1A` |

## 📋 Prerequisites

Before building the extension, ensure you have:

### 1. Rust and Cargo

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload your shell
source ~/.bashrc  # or ~/.zshrc

# Verify installation
rustc --version
cargo --version
```

### 2. wasm-pack

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Alternative: using cargo
cargo install wasm-pack

# Verify installation
wasm-pack --version
```

### 3. Git (to clone the repository)

```bash
# Most systems have git installed, but if not:
# Ubuntu/Debian: sudo apt install git
# macOS: xcode-select --install
# Windows: Download from https://git-scm.com/
```

## 🔨 Building the Extension

### Option 1: One-Command Build (Recommended)

```bash
# Clone and build
git clone https://github.com/yourusername/crypto-address-checker.git
cd crypto-address-checker
chmod +x build.sh
./build.sh
```

### Option 2: Manual Build Steps

If you prefer to understand each step:

```bash
# 1. Clone the repository
git clone https://github.com/yourusername/crypto-address-checker.git
cd crypto-address-checker

# 2. Check Rust compilation
cargo check

# 3. Build WebAssembly module
wasm-pack build --target web --out-dir pkg

# 4. Create extension directory
mkdir -p extension

# 5. Copy extension files
cp manifest.json popup.html popup.js content.js background.js extension/

# 6. Copy WASM package
cp -r pkg extension/
```

### Build Script Features

The `build.sh` script automatically:
- ✅ Checks for required tools (Rust, wasm-pack)
- 🧹 Cleans previous builds
- 🦀 Runs `cargo check` to verify code
- 🕸️ Builds WebAssembly module with `wasm-pack`
- 📁 Creates extension directory structure
- 📋 Copies all necessary files
- ✅ Verifies build integrity
- 📖 Provides installation instructions

## 🌐 Installing in Browser

### Brave Browser

1. Open Brave and navigate to `brave://extensions/`
2. Enable **"Developer mode"** (toggle in top-right)
3. Click **"Load unpacked"**
4. Select the `extension` folder from your project
5. ✅ Extension should now appear in your toolbar!

### Google Chrome

1. Open Chrome and navigate to `chrome://extensions/`
2. Enable **"Developer mode"**
3. Click **"Load unpacked"**
4. Select the `extension` folder
5. ✅ Done!

### Microsoft Edge

1. Navigate to `edge://extensions/`
2. Enable **"Developer mode"**
3. Click **"Load unpacked"**
4. Select the `extension` folder
5. ✅ Ready to use!

## 🧪 Testing the Extension

### 1. Popup Test
- Click the extension icon in your browser toolbar
- Enter a crypto address (try: `0x9575a638118E4Ec7aAfdd3A4eb9e1AC4146F4Dd8`)
- Verify it shows "Valid Ethereum Address"

### 2. Auto-Detection Test
- Visit a crypto block explorer (like etherscan.io)
- Crypto addresses should be automatically highlighted
- Hover over them to see network information

### 3. Context Menu Test
- Select any crypto address on a webpage
- Right-click and choose "Check Crypto Address"

## 🔄 Development Workflow

### Making Changes to Rust Code

```bash
# After modifying src/lib.rs
./build.sh

# Then reload extension in browser:
# Go to browser extensions page → Click reload button for this extension
```

### Making Changes to JavaScript/HTML

```bash
# Copy updated files
cp popup.html popup.js content.js background.js extension/

# Reload extension in browser
```

## 🛠️ Troubleshooting

### Common Issues

**❌ "cargo: command not found"**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc
```

**❌ "wasm-pack: command not found"**
```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

**❌ "Build failed" or compilation errors**
```bash
# Check detailed error messages
cargo check --verbose

# Common fix: update Rust
rustup update
```

**❌ "Extension won't load in browser"**
- Ensure you selected the `extension` folder (not the root project folder)
- Check browser console for errors (F12 → Console)
- Verify all files exist in `extension/` directory

**❌ "CSP violation errors"**
- This usually means files weren't copied correctly
- Re-run `./build.sh` to ensure proper file structure

### Debug Mode

Enable detailed logging:
1. Right-click extension icon → "Inspect popup"
2. Check console for debug messages
3. Test various crypto addresses

## 📊 Project Structure

```
crypto-address-checker/
├── src/
│   └── lib.rs              # Main Rust library
├── extension/              # Built extension (generated)
│   ├── manifest.json
│   ├── popup.html
│   ├── popup.js
│   ├── content.js
│   ├── background.js
│   └── pkg/               # WASM files
├── pkg/                   # WASM build output (generated)
├── Cargo.toml             # Rust dependencies
├── build.sh               # Build script
└── README.md              # This file
```

## 🔗 Polkadot Ecosystem Support

The extension supports 90+ Substrate-based networks including:

### Major Networks
- **Polkadot** (prefix 0) - Main relay chain
- **Kusama** (prefix 2) - Canary network
- **Asset Hub Polkadot** (prefix 1000) - Asset management parachain
- **Asset Hub Kusama** (prefix 2000) - Kusama asset hub

### Popular Parachains
- **Acala** (10), **Karura** (8) - DeFi platforms
- **Moonbeam** (1284), **Moonriver** (1285) - Ethereum compatibility
- **Astar** (50), **Shiden** (51) - Smart contract platforms
- **HydraDX** (49) - DEX and liquidity
- **Interlay** (2032) - Bitcoin bridge
- **Phala** (30) - Privacy computing
- And 80+ more networks...

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Test thoroughly with `./build.sh`
5. Submit a pull request

### Adding New Networks

To add support for new cryptocurrencies:
1. Update validation logic in `src/lib.rs`
2. Add address patterns to `content.js`
3. Test with real addresses
4. Update documentation

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🆘 Support

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/smartram/crypto-address-checker/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/smartram/crypto-address-checker/discussions)
- 📧 **Questions**: Create an issue with the "question" label

## 🙏 Acknowledgments

- Built with [wasm-pack](https://rustwasm.github.io/wasm-pack/) and [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
- SS58 address format specification from [Substrate](https://docs.substrate.io/reference/address-formats/)
- Crypto address validation patterns from various network specifications

---

**⭐ Star this repo if you find it useful!**
