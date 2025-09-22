try {
    importScripts('./pkg-sw/syt_plugin_web.js');
} catch (e) {
    console.error("Error:", e);
}


const wasmPromise = wasm_bindgen('./pkg-sw/syt_plugin_web.wasm');

async function checkActiveTabAndBlock() {
    const wasm = await wasmPromise;

    const [activeTab] = await chrome.tabs.query({ active: true, currentWindow: true });

    if (!activeTab || !activeTab.url) {
        return;
    }
    
    
    if (wasm.is_url_blocked(activeTab.url)) {
        chrome.scripting.insertCSS({
            target: { tabId: activeTab.id },
            files: ['style.css']
        });
        chrome.scripting.executeScript({
            target: { tabId: activeTab.id },
            files: ['content.js']
        });
    }
}


const ALARM_NAME = 'site_check_alarm';

chrome.alarms.onAlarm.addListener((alarm) => {
    if (alarm.name === ALARM_NAME) {
        checkActiveTabAndBlock();
    }
});

chrome.runtime.onInstalled.addListener(() => {
    chrome.alarms.create(ALARM_NAME, {
        delayInMinutes: 1,
        periodInMinutes: 1
    });
});