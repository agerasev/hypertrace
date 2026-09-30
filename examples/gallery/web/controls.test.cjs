const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');

function fixture() {
    class Element {
        constructor() { this.listeners = new Map(); this.textContent = ''; this.dataset = {}; }
        setAttribute() {}
        addEventListener(name, callback) {
            const listeners = this.listeners.get(name) ?? [];
            listeners.push(callback);
            this.listeners.set(name, listeners);
        }
        dispatch(name, values = {}) {
            const event = {
                repeat: false, defaultPrevented: false, stopped: false,
                preventDefault() { this.defaultPrevented = true; },
                stopImmediatePropagation() { this.stopped = true; },
                ...values,
            };
            for (const listener of this.listeners.get(name) ?? []) listener(event);
            return event;
        }
        focus() {}
    }
    const elements = new Map();
    const document = new Element();
    document.pointerLockElement = null;
    document.querySelector = id => {
        if (!elements.has(id)) elements.set(id, new Element());
        return elements.get(id);
    };
    document.querySelectorAll = () => [];
    const window = new Element();
    window.parent = { messages: [], postMessage(data, origin) { this.messages.push({ data, origin }); } };
    const canvas = document.querySelector('#canvas');
    let requests = 0;
    canvas.requestPointerLock = () => { requests++; };
    document.exitPointerLock = () => {
        document.pointerLockElement = null;
        document.dispatch('pointerlockchange');
    };
    const context = vm.createContext({ document, window, URLSearchParams, location: { search: '', origin: 'https://example.test' } });
    const source = readFileSync(`${__dirname}/controls.js`, 'utf8').replace(/^export /gm, '');
    vm.runInContext(source, context);
    return {
        context, document, window, canvas,
        get requests() { return requests; },
        tab: values => canvas.dispatch('keydown', { key: 'Tab', ...values }),
        grant() { document.pointerLockElement = canvas; document.dispatch('pointerlockchange'); },
        hint: () => document.querySelector('#mouse-lock-status').textContent,
    };
}

test('Tab requests inside the gesture; actual browser state controls capture', () => {
    const f = fixture();
    const event = f.tab();
    assert.equal(f.requests, 1);
    assert.ok(event.defaultPrevented && event.stopped);
    assert.equal(f.context.mouse_locked(), false);
    f.tab({ repeat: true });
    assert.equal(f.requests, 1);
    f.grant();
    assert.equal(f.context.mouse_locked(), true);
    assert.equal(f.context.take_mouse_lock_change(), true);
    assert.equal(f.context.take_mouse_lock_change(), false);
    f.tab();
    assert.equal(f.context.mouse_locked(), false);
    assert.equal(f.context.take_mouse_lock_change(), true);
});

test('Escape releases capture before reaching the renderer pause shortcut', () => {
    const f = fixture();
    f.tab(); f.grant();
    const event = f.canvas.dispatch('keydown', { key: 'Escape' });
    assert.ok(event.defaultPrevented && event.stopped);
    assert.equal(f.context.mouse_locked(), false);
    const unlocked = f.canvas.dispatch('keydown', { key: 'Escape' });
    assert.equal(unlocked.stopped, false);
});

test('focus loss releases capture and cancels a still-pending request', () => {
    for (const pending of [false, true]) {
        const f = fixture();
        f.tab();
        if (!pending) f.grant();
        f.window.dispatch('blur');
        if (pending) f.grant();
        assert.equal(f.context.mouse_locked(), false);
    }
});

test('Tab can cancel a pending request without reacquiring it on completion', () => {
    const f = fixture();
    f.tab(); f.tab(); f.grant();
    assert.equal(f.context.mouse_locked(), false);
});

test('external browser unlock is authoritative and permits the next request', () => {
    const f = fixture();
    f.tab(); f.grant();
    f.document.exitPointerLock();
    assert.equal(f.context.mouse_locked(), false);
    assert.equal(f.hint(), 'Tab: lock mouse');
    f.tab();
    assert.equal(f.requests, 2);
});

test('denied requests remain unlocked and provide a recoverable hint', async () => {
    for (const failure of ['throw', 'reject', 'event']) {
        const f = fixture();
        if (failure === 'throw') f.canvas.requestPointerLock = () => { throw Error('denied'); };
        if (failure === 'reject') f.canvas.requestPointerLock = () => Promise.reject(Error('denied'));
        f.tab();
        if (failure === 'event') f.document.dispatch('pointerlockerror');
        await Promise.resolve();
        assert.equal(f.context.mouse_locked(), false);
        assert.match(f.hint(), /unavailable/);
    }
});

test('Shift-Tab keeps keyboard navigation available; hidden pages release capture', () => {
    const f = fixture();
    assert.equal(f.tab({ shiftKey: true }).defaultPrevented, false);
    assert.equal(f.requests, 0);
    f.tab(); f.grant();
    f.document.hidden = true;
    f.document.dispatch('visibilitychange');
    assert.equal(f.context.mouse_locked(), false);
});


test('only the embedding parent can pause the renderer; hidden documents stay paused', () => {
    const f = fixture();
    const event = { origin: 'https://example.test', source: f.window.parent,
        data: { type: 'hypertrace-visibility', hidden: true } };
    assert.equal(f.context.paused(), false);
    f.window.dispatch('message', { ...event, origin: 'https://untrusted.test' });
    f.window.dispatch('message', { ...event, source: {} });
    assert.equal(f.context.paused(), false);
    f.tab(); f.grant();
    f.window.dispatch('message', event);
    assert.equal(f.context.paused(), true);
    assert.equal(f.context.mouse_locked(), false);
    f.window.dispatch('message', { ...event, data: { ...event.data, hidden: false } });
    assert.equal(f.context.paused(), false);
    f.document.hidden = true;
    assert.equal(f.context.paused(), true);
});

test('ready and failure states propagate to the embedding page', () => {
    const f = fixture();
    f.context.set_ready();
    assert.equal(f.window.parent.messages.at(-1).data.state, 'ready');
    f.context.set_status('No compatible adapter', true);
    const message = f.window.parent.messages.at(-1);
    assert.equal(message.data.state, 'error');
    assert.equal(message.data.message, 'No compatible adapter');
    assert.equal(message.origin, 'https://example.test');
});
