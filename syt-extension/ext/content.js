async function main() {
  try {
    const { default: init, execute } = await import(
      chrome.runtime.getURL('pkg/syt_plugin_web.js')
    );

    await init();
    
    const currentUrl = window.location.href;
    execute(currentUrl);
   
  } catch (e) {
    console.error("Error:", e);
  }
}


setInterval(() => {
  main();
}, 5000);