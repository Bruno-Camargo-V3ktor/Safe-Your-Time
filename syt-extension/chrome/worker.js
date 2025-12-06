const REDIRECT_URL = "http://localhost:4321/block";
const API_URL = "http://localhost:4321";
const UPDATE_TIME = 5000;
let lastTimeBlocksJson = "";

function init() {
  setTimeout(async () => {
    const oldRules = await chrome.declarativeNetRequest.getDynamicRules();
    const oldRuleIds = oldRules.map((rule) => rule.id);
    await updateDynamicRules(oldRuleIds, []);
  }, 1000);

  setInterval(async () => {
    let newTimeBlocks = await request();

    const currentJson = JSON.stringify(newTimeBlocks);
    if (currentJson != lastTimeBlocksJson) {
      lastTimeBlocksJson = currentJson;

      const newRules = createRules(newTimeBlocks);
      const oldRules = await chrome.declarativeNetRequest.getDynamicRules();
      const oldRuleIds = oldRules.map((rule) => rule.id);

      await updateDynamicRules(oldRuleIds, newRules);
    }
  }, UPDATE_TIME);
}

async function request() {
  //...
  return [
    {
      name: "Sem Youtube",
      message: "Foco garoto",
      denied_acess: ["youtube.com"],
    },
  ];
}

function createRules(timeblocks) {
  let rules = [];
  let id = 1;

  for (let i = 0; i < timeblocks.length; i++) {
    for (let j = 0; j < timeblocks[i].denied_acess.length; j++) {
      const url = new URL(REDIRECT_URL);
      url.searchParams.set("name", timeblocks[i].name);
      url.searchParams.set("message", timeblocks[i].message);

      rules.push({
        id: id,
        priority: 1,
        action: {
          type: "redirect",
          redirect: {
            url: `${url.toString()}`,
          },
        },
        condition: {
          urlFilter: `||${timeblocks[i].denied_acess[j]}^`,
          resourceTypes: ["main_frame"],
        },
      });

      id += 1;
    }
  }

  return rules;
}

async function updateDynamicRules(oldIds, newRules) {
  await chrome.declarativeNetRequest.updateDynamicRules({
    removeRuleIds: oldIds,
    addRules: newRules,
  });
}

init();
