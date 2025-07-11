use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use regex::Regex;
use base58::FromBase58;
use sha2::{Sha256, Digest};
use blake2::Blake2b512;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Serialize, Deserialize)]
pub struct AddressInfo {
    pub address: String,
    pub network: String,
    pub valid: bool,
    pub address_type: String,
}

#[wasm_bindgen]
pub fn identify_crypto_address(address: &str) -> JsValue {
    let result = check_address(address);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

fn check_address(address: &str) -> AddressInfo {
    let address = address.trim();
    
    // Debug: log the address being checked
    web_sys::console::log_1(&format!("Checking address: {}", address).into());
    
    // Bitcoin addresses
    if is_bitcoin_address(address) {
        web_sys::console::log_1(&"Identified as Bitcoin".into());
        return AddressInfo {
            address: address.to_string(),
            network: "Bitcoin".to_string(),
            valid: true,
            address_type: get_bitcoin_address_type(address),
        };
    }
    
    // Ethereum addresses
    if is_ethereum_address(address) {
        web_sys::console::log_1(&"Identified as Ethereum".into());
        return AddressInfo {
            address: address.to_string(),
            network: "Ethereum".to_string(),
            valid: true,
            address_type: "Standard".to_string(),
        };
    }
    
    // Litecoin addresses
    if is_litecoin_address(address) {
        web_sys::console::log_1(&"Identified as Litecoin".into());
        return AddressInfo {
            address: address.to_string(),
            network: "Litecoin".to_string(),
            valid: true,
            address_type: get_litecoin_address_type(address),
        };
    }
    
    // Dogecoin addresses
    if is_dogecoin_address(address) {
        web_sys::console::log_1(&"Identified as Dogecoin".into());
        return AddressInfo {
            address: address.to_string(),
            network: "Dogecoin".to_string(),
            valid: true,
            address_type: "Standard".to_string(),
        };
    }
    
    // Solana addresses
    if is_solana_address(address) {
        web_sys::console::log_1(&"Identified as Solana".into());
        return AddressInfo {
            address: address.to_string(),
            network: "Solana".to_string(),
            valid: true,
            address_type: "Standard".to_string(),
        };
    }
    
    // Cardano addresses
    if is_cardano_address(address) {
        web_sys::console::log_1(&"Identified as Cardano".into());
        return AddressInfo {
            address: address.to_string(),
            network: "Cardano".to_string(),
            valid: true,
            address_type: "Bech32".to_string(),
        };
    }
    
    // Polkadot addresses (SS58 format)
    if let Some(polkadot_info) = check_polkadot_address(address) {
        web_sys::console::log_1(&"Identified as Polkadot/Substrate".into());
        return polkadot_info;
    }
    
    // Invalid or unknown address
    web_sys::console::log_1(&"Address not recognized".into());
    AddressInfo {
        address: address.to_string(),
        network: "Unknown".to_string(),
        valid: false,
        address_type: "Unknown".to_string(),
    }
}

fn is_bitcoin_address(address: &str) -> bool {
    // Legacy addresses (1...)
    if address.starts_with('1') && address.len() >= 26 && address.len() <= 35 {
        return validate_base58_checksum(address);
    }
    
    // Script addresses (3...)
    if address.starts_with('3') && address.len() >= 26 && address.len() <= 35 {
        return validate_base58_checksum(address);
    }
    
    // Bech32 addresses (bc1...)
    if address.starts_with("bc1") && address.len() >= 42 && address.len() <= 62 {
        return validate_bech32(address);
    }
    
    false
}

fn get_bitcoin_address_type(address: &str) -> String {
    if address.starts_with('1') {
        "Legacy (P2PKH)".to_string()
    } else if address.starts_with('3') {
        "Script (P2SH)".to_string()
    } else if address.starts_with("bc1") {
        "Bech32 (P2WPKH/P2WSH)".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn is_ethereum_address(address: &str) -> bool {
    // Check if it starts with 0x and has exactly 40 hex characters after
    web_sys::console::log_1(&format!("ETH check - address: {}, len: {}", address, address.len()).into());
    
    if !address.starts_with("0x") || address.len() != 42 {
        web_sys::console::log_1(&format!("ETH check failed - starts_with: {}, len: {}", address.starts_with("0x"), address.len()).into());
        return false;
    }
    
    // Check if all characters after 0x are valid hex
    let hex_part = &address[2..];
    let is_valid_hex = hex_part.chars().all(|c| c.is_ascii_hexdigit());
    web_sys::console::log_1(&format!("ETH check - hex_part: {}, is_valid_hex: {}", hex_part, is_valid_hex).into());
    
    is_valid_hex
}

fn is_litecoin_address(address: &str) -> bool {
    // Legacy addresses (L... or M...)
    if (address.starts_with('L') || address.starts_with('M')) && address.len() >= 26 && address.len() <= 35 {
        return validate_base58_checksum(address);
    }
    
    // Bech32 addresses (ltc1...)
    if address.starts_with("ltc1") && address.len() >= 43 && address.len() <= 63 {
        return validate_bech32(address);
    }
    
    false
}

fn get_litecoin_address_type(address: &str) -> String {
    if address.starts_with('L') {
        "Legacy (P2PKH)".to_string()
    } else if address.starts_with('M') {
        "Script (P2SH)".to_string()
    } else if address.starts_with("ltc1") {
        "Bech32".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn is_dogecoin_address(address: &str) -> bool {
    (address.starts_with('D') || address.starts_with('A')) && 
    address.len() >= 26 && address.len() <= 35 && 
    validate_base58_checksum(address)
}

fn is_solana_address(address: &str) -> bool {
    address.len() >= 32 && address.len() <= 44 && 
    address.chars().all(|c| c.is_ascii_alphanumeric()) &&
    !address.starts_with('0') && !address.starts_with('O') &&
    !address.starts_with('I') && !address.starts_with('l')
}

fn is_cardano_address(address: &str) -> bool {
    address.starts_with("addr1") && address.len() >= 100 && address.len() <= 110
}

fn validate_base58_checksum(address: &str) -> bool {
    match address.from_base58() {
        Ok(decoded) => {
            if decoded.len() < 4 {
                return false;
            }
            
            let (payload, checksum) = decoded.split_at(decoded.len() - 4);
            let hash = Sha256::digest(&Sha256::digest(payload));
            &hash[..4] == checksum
        }
        Err(_) => false,
    }
}

fn validate_bech32(address: &str) -> bool {
    // Simplified bech32 validation
    // In a real implementation, you'd use a proper bech32 library
    let re = Regex::new(r"^[a-z0-9]{1,83}$").unwrap();
    let parts: Vec<&str> = address.split('1').collect();
    
    if parts.len() != 2 {
        return false;
    }
    
    let (hrp, data) = (parts[0], parts[1]);
    
    // Check human readable part
    if hrp.is_empty() || hrp.len() > 83 {
        return false;
    }
    
    // Check data part
    if data.len() < 6 || !re.is_match(data) {
        return false;
    }
    
    true
}

// SS58 Network Prefixes for Polkadot ecosystem - using HashMap for O(1) lookup
fn get_ss58_networks() -> &'static HashMap<u16, &'static str> {
    static SS58_NETWORKS: OnceLock<HashMap<u16, &'static str>> = OnceLock::new();
    SS58_NETWORKS.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(0, "Polkadot");
        map.insert(1, "Bare");
        map.insert(2, "Kusama");
        map.insert(3, "Reserved");
        map.insert(4, "Katalchain");
        map.insert(5, "Plasm");
        map.insert(6, "Bifrost");
        map.insert(7, "Edgeware");
        map.insert(8, "Karura");
        map.insert(9, "Reynolds");
        map.insert(10, "Acala");
        map.insert(11, "Laminar");
        map.insert(12, "Polymath");
        map.insert(13, "SubstraTEE");
        map.insert(14, "Totem");
        map.insert(15, "Synesthesia");
        map.insert(16, "Kulupu");
        map.insert(17, "Dark");
        map.insert(18, "Darwinia");
        map.insert(19, "GeekCash");
        map.insert(20, "Stafi");
        map.insert(21, "Dock-Testnet");
        map.insert(22, "Dock-Mainnet");
        map.insert(23, "ShiftNrg");
        map.insert(24, "Zero");
        map.insert(25, "Alphaville");
        map.insert(26, "Jupiter");
        map.insert(27, "Patract");
        map.insert(28, "Subsocial");
        map.insert(29, "Cord");
        map.insert(30, "Phala");
        map.insert(31, "Litentry");
        map.insert(32, "Robonomics");
        map.insert(33, "DataHighway");
        map.insert(34, "Ares");
        map.insert(35, "Valiu");
        map.insert(36, "Bernedette");
        map.insert(37, "SubDAO");
        map.insert(38, "Nodle");
        map.insert(39, "Reserved");
        map.insert(40, "Basilisk");
        map.insert(41, "Kilt");
        map.insert(42, "Picasso");
        map.insert(43, "Composable");
        map.insert(44, "Oak");
        map.insert(45, "KICO");
        map.insert(46, "DICO");
        map.insert(47, "Imbue");
        map.insert(48, "Aventus");
        map.insert(49, "HydraDX");
        map.insert(50, "Astar");
        map.insert(51, "Shiden");
        map.insert(52, "OriginTrail");
        map.insert(53, "Calamari");
        map.insert(54, "Parallel");
        map.insert(55, "Heiko");
        map.insert(56, "Clover");
        map.insert(57, "Bit.Country");
        map.insert(58, "Zeitgeist");
        map.insert(59, "Ajuna");
        map.insert(60, "SubGame");
        map.insert(61, "Ternoa");
        map.insert(62, "Kapex");
        map.insert(63, "Genshiro");
        map.insert(64, "Equilibrium");
        map.insert(65, "Sora");
        map.insert(66, "Social Network");
        map.insert(67, "ChainX");
        map.insert(68, "Crust");
        map.insert(69, "Uniarts");
        map.insert(70, "Reserved");
        map.insert(71, "Unique");
        map.insert(72, "Quartz");
        map.insert(73, "Bit.Country Pioneer");
        map.insert(74, "RMRK");
        map.insert(75, "Litmus");
        map.insert(76, "Kylin");
        map.insert(77, "Bajun");
        map.insert(78, "Integritee");
        map.insert(79, "Centrifuge");
        map.insert(80, "Altair");
        map.insert(81, "InvArch");
        map.insert(82, "Tinkernet");
        map.insert(83, "Brainstorm");
        map.insert(84, "Virto");
        map.insert(85, "Omnibtc");
        map.insert(86, "Hashed");
        map.insert(87, "Frequency");
        map.insert(88, "Manta");
        map.insert(89, "Calamari");
        map.insert(90, "Amplitude");
        map.insert(91, "Pendulum");
        map.insert(1000, "Asset Hub Polkadot"); // Polkadot Asset Hub (Statemint)
        map.insert(1001, "Collectives Polkadot"); // Polkadot Collectives
        map.insert(2000, "Asset Hub Kusama"); // Kusama Asset Hub (Statemine)
        map.insert(2001, "Bridge Hub Kusama"); // Kusama Bridge Hub
        map.insert(2004, "Encointer Kusama"); // Encointer Network
        map.insert(2007, "Crab"); // Darwinia Crab
        map.insert(2011, "Kintsugi"); // Interlay Kintsugi (Kusama)
        map.insert(2012, "Picasso"); // Picasso (Kusama)
        map.insert(2013, "Quartz"); // Quartz (Unique Network on Kusama)
        map.insert(2015, "Moonriver"); // Moonriver (Kusama)
        map.insert(2019, "Karura"); // Karura (Acala on Kusama)
        map.insert(2021, "Altair"); // Altair (Centrifuge on Kusama)
        map.insert(2023, "Khala"); // Khala (Phala on Kusama)
        map.insert(2024, "Kico"); // KICO
        map.insert(2030, "Basilisk"); // Basilisk
        map.insert(2032, "Interlay"); // Interlay (Bitcoin bridge)
        map.insert(2034, "HydraDX"); // HydraDX
        map.insert(2035, "Phala"); // Phala Network
        map.insert(2037, "Unique"); // Unique Network
        map.insert(2040, "Polkadex"); // Polkadex
        map.insert(2043, "OriginTrail"); // OriginTrail Parachain
        map.insert(2046, "Darwinia"); // Darwinia Network
        map.insert(2048, "Robonomics"); // Robonomics
        map.insert(2051, "Ajuna"); // Ajuna Network
        map.insert(2092, "Kilt"); // KILT Protocol
        map.insert(2094, "Calamari"); // Calamari (Manta on Kusama)
        map
    })
}

fn check_polkadot_address(address: &str) -> Option<AddressInfo> {
    // Check if it's a valid SS58 address
    if let Some((_, network_name)) = validate_ss58_address(address) {
        return Some(AddressInfo {
            address: address.to_string(),
            network: network_name.to_string(),
            valid: true,
            address_type: "SS58".to_string(),
        });
    }
    None
}

fn validate_ss58_address(address: &str) -> Option<(u16, &'static str)> {
    // Decode base58
    let decoded = match address.from_base58() {
        Ok(bytes) => bytes,
        Err(_) => return None,
    };
    
    // SS58 addresses must be at least 35 bytes (1 + 32 + 2 checksum)
    if decoded.len() < 35 {
        return None;
    }
    
    // Extract network prefix
    let (network_id, payload_start) = if decoded[0] & 0b01000000 == 0 {
        // Simple format: network ID is the first byte with MSB cleared
        (decoded[0] as u16, 1)
    } else if decoded[0] & 0b10000000 == 0 {
        // Full format: network ID is in first two bytes
        if decoded.len() < 36 {
            return None;
        }
        let network_id = ((decoded[0] & 0b00111111) as u16) << 8 | decoded[1] as u16;
        (network_id, 2)
    } else {
        // Reserved format
        return None;
    };
    
    // Check if we have enough bytes for address + checksum
    if decoded.len() < payload_start + 32 + 2 {
        return None;
    }
    
    // Validate checksum using Blake2b
    let payload_end = decoded.len() - 2;
    let payload = &decoded[..payload_end];
    let checksum = &decoded[payload_end..];
    
    // Create Blake2b hash with SS58 prefix
    let mut hasher = Blake2b512::new();
    hasher.update(b"SS58PRE");
    hasher.update(payload);
    let hash = hasher.finalize();
    
    // Check if first 2 bytes of hash match the checksum
    if &hash[0..2] != checksum {
        return None;
    }
    
    // Find network name using O(1) HashMap lookup
    let ss58_networks = get_ss58_networks();
    let network_name = ss58_networks
        .get(&network_id)
        .copied()
        .unwrap_or("Substrate");
    
    Some((network_id, network_name))
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
