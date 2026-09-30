// The native application is primary; the embedded renderer starts on request.
export function unsupportedReason(scope = globalThis) {
    if (!scope.isSecureContext) return 'WebGPU requires HTTPS or localhost. This page is not in a secure context.';
    if (!scope.navigator.gpu) return 'This browser does not expose WebGPU. It may be unsupported or disabled by its graphics configuration.';
    if (!scope.WebAssembly) return 'This browser does not support WebAssembly.';
    return '';
}

export function mountViewer(container) {
    const button = container.querySelector('.start-viewer');
    const status = container.querySelector('.viewer-status');
    const poster = container.querySelector('.poster');
    let frame;
    let timeout;
    let visible = true;
    function fail(message) {
        clearTimeout(timeout);
        container.dataset.state = 'error';
        status.textContent = message;
        frame?.remove();
        frame = undefined;
        poster.hidden = false;
        button.disabled = false;
        button.textContent = 'Retry browser viewer';
    }
    function visibility() {
        frame?.contentWindow.postMessage({type: 'hypertrace-visibility', hidden: document.hidden || !visible}, location.origin);
    }
    const unsupported = unsupportedReason();
    if (unsupported) {
        fail(unsupported);
        button.hidden = true;
        return;
    }
    button.addEventListener('click', () => {
        if (frame) return;
        button.disabled = true;
        status.textContent = 'Loading the renderer and requesting a WebGPU device…';
        container.dataset.state = 'loading';
        frame = document.createElement('iframe');
        frame.title = `${container.dataset.scene} interactive renderer`;
        frame.allow = 'fullscreen';
        frame.src = `viewer/?scene=${encodeURIComponent(container.dataset.scene)}&embedded=1`;
        frame.addEventListener('load', visibility);
        container.prepend(frame);
        poster.hidden = true;
        timeout = setTimeout(() => fail('The renderer did not finish starting within 60 seconds. A download or graphics initialization may have stalled. Retry, or use the native application.'), 60000);
    });
    window.addEventListener('message', event => {
        if (!frame || event.origin !== location.origin || event.source !== frame.contentWindow || event.data?.type !== 'hypertrace-status') return;
        if (event.data.state === 'error') fail(`Browser renderer unavailable: ${event.data.message}`);
        else if (event.data.state === 'ready') {
            clearTimeout(timeout);
            container.dataset.state = 'ready';
            visibility();
        } else if (typeof event.data.message === 'string') status.textContent = event.data.message;
    });
    document.addEventListener('visibilitychange', visibility);
    if ('IntersectionObserver' in window) {
        new IntersectionObserver(entries => { visible = entries[0].isIntersecting; visibility(); }).observe(container);
    }
}
if (typeof document !== 'undefined') document.querySelectorAll('.viewer').forEach(mountViewer);
