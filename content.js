// Content script for crypto address detection
(function() {
    'use strict';

    let wasmModule = null;

    // Initialize WASM module
    async function initWasm() {
        try {
            const wasmUrl = chrome.runtime.getURL('pkg/crypto_address_checker.js');
            const wasm = await import(wasmUrl);
            await wasm.default();
            wasmModule = wasm;
            console.log('WASM module loaded in content script');
        } catch (error) {
            console.error('Failed to load WASM module in content script:', error);
        }
    }

    // Crypto address patterns
    const patterns = {
        bitcoin: /\b[13][a-km-zA-HJ-NP-Z1-9]{25,34}\b|bc1[a-z0-9]{39,59}/g,
        ethereum: /0x[a-fA-F0-9]{40}/g,
        litecoin: /\b[LM][a-km-zA-HJ-NP-Z1-9]{26,33}\b|ltc1[a-z0-9]{39,59}/g,
        dogecoin: /\b[DA][a-km-zA-HJ-NP-Z1-9]{25,34}\b/g,
        solana: /\b[1-9A-HJ-NP-Za-km-z]{32,44}\b/g,
        cardano: /addr1[a-z0-9]{98,108}/g,
        polkadot: /\b[1-9A-HJ-NP-Za-km-z]{47,48}\b/g
    };

    // Create tooltip element
    function createTooltip() {
        const tooltip = document.createElement('div');
        tooltip.id = 'crypto-address-tooltip';
        tooltip.style.cssText = `
            position: absolute;
            background: #333;
            color: white;
            padding: 8px 12px;
            border-radius: 4px;
            font-size: 12px;
            font-family: Arial, sans-serif;
            z-index: 10000;
            pointer-events: none;
            opacity: 0;
            transition: opacity 0.3s;
            max-width: 300px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.2);
        `;
        document.body.appendChild(tooltip);
        return tooltip;
    }

    // Show tooltip
    function showTooltip(element, addressInfo, x, y) {
        const tooltip = document.getElementById('crypto-address-tooltip') || createTooltip();
        
        const validText = addressInfo.valid ? 'Valid' : 'Invalid';
        const color = addressInfo.valid ? '#4CAF50' : '#f44336';
        
        tooltip.innerHTML = `
            <div style="font-weight: bold; color: ${color};">${validText} ${addressInfo.network} Address</div>
            <div style="margin-top: 4px;">Type: ${addressInfo.address_type}</div>
        `;
        
        tooltip.style.left = x + 'px';
        tooltip.style.top = (y - 60) + 'px';
        tooltip.style.opacity = '1';
    }

    // Hide tooltip
    function hideTooltip() {
        const tooltip = document.getElementById('crypto-address-tooltip');
        if (tooltip) {
            tooltip.style.opacity = '0';
        }
    }

    // Detect and highlight crypto addresses
    function detectCryptoAddresses() {
        if (!wasmModule) return;

        const textNodes = [];
        const walker = document.createTreeWalker(
            document.body,
            NodeFilter.SHOW_TEXT,
            null,
            false
        );

        let node;
        while (node = walker.nextNode()) {
            if (node.parentNode && 
                node.parentNode.tagName !== 'SCRIPT' && 
                node.parentNode.tagName !== 'STYLE' &&
                !node.parentNode.classList.contains('crypto-address-highlight')) {
                textNodes.push(node);
            }
        }

        textNodes.forEach(textNode => {
            const text = textNode.textContent;
            let hasMatches = false;
            let newHTML = text;

            // Check all patterns
            Object.values(patterns).forEach(pattern => {
                const matches = text.match(pattern);
                if (matches) {
                    matches.forEach(match => {
                        try {
                            const addressInfo = wasmModule.identify_crypto_address(match);
                            if (addressInfo.valid) {
                                const color = '#4CAF50';
                                const highlightedAddress = `<span class="crypto-address-highlight" style="background-color: ${color}20; border-bottom: 2px solid ${color}; cursor: pointer;" data-address="${match}">${match}</span>`;
                                newHTML = newHTML.replace(match, highlightedAddress);
                                hasMatches = true;
                            }
                        } catch (error) {
                            console.error('Error checking address:', error);
                        }
                    });
                }
            });

            if (hasMatches) {
                const wrapper = document.createElement('span');
                wrapper.innerHTML = newHTML;
                textNode.parentNode.replaceChild(wrapper, textNode);
            }
        });
    }

    // Event delegation for highlighted addresses
    document.addEventListener('mouseover', (e) => {
        if (e.target.classList.contains('crypto-address-highlight')) {
            const address = e.target.getAttribute('data-address');
            if (address && wasmModule) {
                try {
                    const addressInfo = wasmModule.identify_crypto_address(address);
                    showTooltip(e.target, addressInfo, e.pageX, e.pageY);
                } catch (error) {
                    console.error('Error showing tooltip:', error);
                }
            }
        }
    });

    document.addEventListener('mouseout', (e) => {
        if (e.target.classList.contains('crypto-address-highlight')) {
            hideTooltip();
        }
    });

    // Initialize and run detection
    initWasm().then(() => {
        detectCryptoAddresses();
        
        // Re-run detection when DOM changes
        const observer = new MutationObserver((mutations) => {
            let shouldRerun = false;
            mutations.forEach((mutation) => {
                if (mutation.type === 'childList' && mutation.addedNodes.length > 0) {
                    shouldRerun = true;
                }
            });
            
            if (shouldRerun) {
                setTimeout(detectCryptoAddresses, 500);
            }
        });
        
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    });
})();
