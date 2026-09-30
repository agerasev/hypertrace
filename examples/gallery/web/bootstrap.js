// Install before the WASM module so load, panic and device errors remain visible.
(() => {
    let failed = false;
    function report(message) {
        if (failed) return;
        failed = true;
        const status = document.querySelector('#status');
        if (status) {
            status.textContent = `Unable to render: ${message}`;
            status.dataset.error = 'true';
        }
        if (window.parent !== window) window.parent.postMessage({type: 'hypertrace-status', state: 'error', message}, location.origin);
    }
    window.addEventListener('error', event => report(event.message || 'A renderer script could not be loaded.'));
    window.addEventListener('unhandledrejection', event => report(String(event.reason?.message ?? event.reason ?? 'Renderer initialization failed.')));
})();
