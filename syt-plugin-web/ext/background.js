import(chrome.runtime.getURL("pkg/blocker_lib.js")).then(module => {
    async function checkActiveTabAndBlock() {
        await module.default();
        
        const { is_url_blocked } = module;

        const [activeTab] = await chrome.tabs.query({ active: true, currentWindow: true });

        if (!activeTab || !activeTab.url) {
            return;
        }

        if (is_url_blocked(activeTab.url)) {
            console.log(`ALARME: A aba ativa (${activeTab.url}) é um site bloqueado. Injetando script...`);

            
            chrome.scripting.insertCSS({
                target: { tabId: activeTab.id },
                files: ['style.css']
            });
            chrome.scripting.executeScript({
                target: { tabId: activeTab.id },
                files: ['content.js']
            });
        } else {
            console.log(`ALARME: A aba ativa (${activeTab.url}) está OK.`);
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
});