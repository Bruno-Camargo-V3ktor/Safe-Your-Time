(async () => {
  try {
    const { default: init, execute } = await import(
      chrome.runtime.getURL('pkg/syt_plugin_web.js')
    );

    await init();

    window.addEventListener('DOMContentLoaded', () => {
        console.log("SYT Blocker: Página carregada, verificando URL.");

        const currentUrl = window.location.href;
        block_page(currentUrl);
    });

  } catch (e) {
    console.error("Error:", e);
  }
})();