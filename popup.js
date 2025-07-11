import init, * as wasm from './pkg/crypto_address_checker.js';

let wasmInitialized = false;

async function initWasm() {
    try {
        await init();
        wasmInitialized = true;
        console.log('WASM module loaded successfully');
    } catch (error) {
        console.error('Failed to load WASM module:', error);
    }
}

async function checkAddress() {
    const addressInput = document.getElementById('address');
    const address = addressInput.value.trim();
    
    if (!address) {
        alert('Please enter a crypto address');
        return;
    }
    
    if (!wasmInitialized) {
        alert('WASM module not loaded. Please try again.');
        return;
    }
    
    const loading = document.getElementById('loading');
    const result = document.getElementById('result');
    
    loading.style.display = 'block';
    result.style.display = 'none';
    
    try {
        const addressInfo = wasm.identify_crypto_address(address);
        
        // Update result display
        document.getElementById('resultNetwork').textContent = 
            addressInfo.valid ? `${addressInfo.network} Address` : 'Invalid Address';
        document.getElementById('resultAddress').textContent = addressInfo.address;
        document.getElementById('resultNetworkName').textContent = addressInfo.network;
        document.getElementById('resultType').textContent = addressInfo.address_type;
        document.getElementById('resultValid').textContent = addressInfo.valid ? 'Yes' : 'No';
        
        // Apply appropriate styling
        result.className = `result ${addressInfo.valid ? 'valid' : 'invalid'}`;
        result.style.display = 'block';
        
    } catch (error) {
        console.error('Error checking address:', error);
        alert('Error checking address. Please try again.');
    } finally {
        loading.style.display = 'none';
    }
}

// Initialize WASM when popup opens
initWasm();

// Event listeners
document.getElementById('checkBtn').addEventListener('click', checkAddress);
document.getElementById('address').addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        checkAddress();
    }
});
