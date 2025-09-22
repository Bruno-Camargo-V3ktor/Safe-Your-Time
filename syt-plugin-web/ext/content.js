(async () => {
    if (window.blockerExecuted) {
        return;
    }
    window.blockerExecuted = true;

    try {
        const {  default: init, execute } = await import(
            chrome.runtime.getURL("pkg/syt_plugin_web.js")
        );

        await init();

        execute(window.location.href);

    } catch (e) {
        console.error("Error: ", e);
    }
})();