// Background script for the crypto address checker extension

chrome.runtime.onInstalled.addListener(() => {
    console.log('Crypto Address Checker extension installed');
});

// Listen for messages from content script or popup
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    if (request.action === 'checkAddress') {
        // This could be used for additional background processing if needed
        sendResponse({ status: 'received' });
    }
});

// Optional: Add context menu item
chrome.runtime.onInstalled.addListener(() => {
    chrome.contextMenus.create({
        id: 'checkCryptoAddress',
        title: 'Check Crypto Address',
        contexts: ['selection']
    });
});

chrome.contextMenus.onClicked.addListener((info, tab) => {
    if (info.menuItemId === 'checkCryptoAddress') {
        // Send the selected text to content script for processing
        chrome.tabs.sendMessage(tab.id, {
            action: 'checkSelectedAddress',
            address: info.selectionText
        });
    }
});
