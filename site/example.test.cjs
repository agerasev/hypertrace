const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');

function fixture(overrides = {}) {
    class Element {
        constructor() { this.listeners = {}; this.dataset = {}; this.hidden = false; this.messages = []; }
        addEventListener(name, callback) { (this.listeners[name] ??= []).push(callback); }
        dispatch(name, event = {}) { for (const fn of this.listeners[name] ?? []) fn(event); }
        postMessage(data, origin) { this.messages.push({ data, origin }); }
        prepend(child) { this.child = child; }
        remove() { this.removed = true; }
    }
    const window = new Element();
    const document = new Element();
    const container = new Element();
    container.dataset.scene = 'fixture';
    const elements = new Map(['.start-viewer', '.viewer-status', '.poster'].map(key => [key, new Element()]));
    container.querySelector = selector => elements.get(selector);
    document.querySelectorAll = () => [];
    document.createElement = () => {
        const frame = new Element();
        frame.contentWindow = new Element();
        return frame;
    };
    let observer, timeout;
    class IntersectionObserver {
        constructor(callback) { observer = callback; }
        observe() {}
    }
    window.IntersectionObserver = IntersectionObserver;
    const scope = { window, document, location: { origin: 'https://example.test' },
        isSecureContext: true, navigator: { gpu: {} }, WebAssembly: {}, IntersectionObserver,
        setTimeout: fn => { timeout = fn; return 1; }, clearTimeout: () => { timeout = undefined; },
        ...overrides };
    const context = vm.createContext(scope);
    vm.runInContext(readFileSync(`${__dirname}/example.js`, 'utf8').replace(/^export /gm, ''), context);
    context.mountViewer(container);
    return { container, scope, context, document, window,
        button: elements.get('.start-viewer'), status: elements.get('.viewer-status'),
        poster: elements.get('.poster'), offscreen: () => observer([{ isIntersecting: false }]),
        expire: () => timeout?.(),
        message(data, changes = {}) {
            window.dispatch('message', { origin: scope.location.origin,
                source: container.child.contentWindow, data: { type: 'hypertrace-status', ...data }, ...changes });
        },
    };
}

test('unsupported hosts give a reason without loading an iframe', () => {
    for (const [overrides, reason] of [[{ isSecureContext: false }, /secure context/],
        [{ navigator: {} }, /does not expose WebGPU/], [{ WebAssembly: undefined }, /WebAssembly/]]) {
        const f = fixture(overrides);
        assert.match(f.status.textContent, reason);
        assert.equal(f.button.hidden, true);
        assert.equal(f.container.child, undefined);
    }
});
test('supported hosts start only on request and preserve the scene URL', () => {
    const f = fixture();
    assert.equal(f.container.child, undefined);
    f.button.dispatch('click');
    assert.equal(f.container.child.src, 'viewer/?scene=fixture&embedded=1');
    assert.equal(f.poster.hidden, true);
    f.message({ state: 'ready' }, { origin: 'https://untrusted.test' });
    assert.equal(f.container.dataset.state, 'loading');
    f.message({ state: 'ready' }, { source: {} });
    assert.equal(f.container.dataset.state, 'loading');
    f.message({ state: 'ready' });
    assert.equal(f.container.dataset.state, 'ready');
    f.expire();
    assert.equal(f.container.dataset.state, 'ready');
});
test('actual initialization failures remove the renderer and allow retry', () => {
    const f = fixture(); f.button.dispatch('click');
    const frame = f.container.child;
    f.message({ state: 'error', message: 'No suitable adapter' });
    assert.match(f.status.textContent, /No suitable adapter/);
    assert.equal(frame.removed, true);
    assert.equal(f.poster.hidden, false);
    assert.equal(f.button.disabled, false);
    f.button.dispatch('click');
    assert.notEqual(f.container.child, frame);
    f.expire();
    assert.match(f.status.textContent, /60 seconds/);
});
test('hidden pages and offscreen embeds tell the renderer to pause', () => {
    const f = fixture(); f.button.dispatch('click'); f.message({ state: 'ready' });
    const frame = f.container.child.contentWindow;
    assert.equal(frame.messages.at(-1).data.hidden, false);
    f.document.hidden = true; f.document.dispatch('visibilitychange');
    assert.equal(frame.messages.at(-1).data.hidden, true);
    f.document.hidden = false; f.document.dispatch('visibilitychange');
    assert.equal(frame.messages.at(-1).data.hidden, false);
    f.offscreen();
    assert.equal(frame.messages.at(-1).data.hidden, true);
    assert.equal(frame.messages.at(-1).origin, f.scope.location.origin);
});
