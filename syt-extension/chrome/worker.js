const REDIRECT_URL = "http://localhost:4321/block";
const API_URL = "http://localhost:4321";
const UPDATE_TIME = 5000;
const timeblocks = [];

function init() {
  setInterval(async () => {
    let newTimeBlocks = await request();

    if (newTimeBlocks != timeblocks) {
      newTimeBlocks.forEach((tb, index) => {
        timeblocks[index] = tb;
      });

      const newRules = createRules();
      const oldRules = await chrome.declarativeNetRequest.getDynamicRules();
      const oldRuleIds = oldRules.map((rule) => rule.id);

      await updateDynamicRules(oldRuleIds, newRules);
    }
  }, UPDATE_TIME);
}

async function request() {
  //...
  return [{ name: "", message: "", denied_acess: ["youtube.com"] }];
}

function createRules() {
  let rules = [];
  let id = 1;

  for (let i = 0; i < timeblocks.length; i++) {
    for (let j = 0; j < timeblocks[i].denied_acess.length; j++) {
      rules.push({
        id: id,
        priority: 1,
        action: { type: "redirect", redirect: { url: REDIRECT_URL } },
        condition: {
          urlFilter: `https://youtube.com/*`,
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
