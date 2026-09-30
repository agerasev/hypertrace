const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');

function fixture() {
    const handlers = {};
    const messages = [];
    const status = { dataset: {}, textContent: '' };
    const window = { addEventListener: (name, fn) => { handlers[name] = fn; },
        parent: { postMessage: data => messages.push(data) } };
    vm.runInNewContext(readFileSync(`${__dirname}/bootstrap.js`, 'utf8'), {
        window, document: { querySelector: () => status }, location: { origin: 'https://example.test' },
    });
    return { handlers, messages, status };
}

test('Winit control-flow signals do not hide later real failures', () => {
    const signal = "Using exceptions for control flow, don't mind me. This isn't actually an error!";
    for (const name of ['error', 'unhandledrejection']) {
        const f = fixture();
        let prevented = false;
        f.handlers[name]({ error: Error(signal), reason: Error(signal), preventDefault() { prevented = true; } });
        assert.equal(prevented, true);
        assert.equal(f.messages.length, 0);
        f.handlers[name]({ error: Error('Device lost'), reason: Error('Device lost') });
        assert.equal(f.messages[0].message, 'Device lost');
        assert.match(f.status.textContent, /Device lost/);
    }
});
test('missing scripts and rejected startup promises report a concrete failure once', () => {
    const f = fixture();
    f.handlers.error({ message: 'Unable to load renderer module' });
    f.handlers.unhandledrejection({ reason: Error('follow-on failure') });
    assert.equal(f.messages.length, 1);
    assert.equal(f.messages[0].state, 'error');
    assert.match(f.status.textContent, /Unable to load renderer module/);
});
