// Install before the WASM module so load, panic and device errors remain visible.
(() => {
    let failed = false;
    // Winit's web EventLoop::run exits the synchronous WASM entry point using
    // this sentinel; the scheduled event loop continues normally afterwards.
    const controlFlow = "Using exceptions for control flow, don't mind me. This isn't actually an error!";
    function handle(message, event) {
        if (message === controlFlow || message === `Uncaught Error: ${controlFlow}`) {
            event.preventDefault();
            return;
        }
        report(message);
    }
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
    window.addEventListener('error', event => handle(event.error?.message || event.message || 'A renderer script could not be loaded.', event));
    window.addEventListener('unhandledrejection', event => handle(String(event.reason?.message ?? event.reason ?? 'Renderer initialization failed.'), event));
})();
