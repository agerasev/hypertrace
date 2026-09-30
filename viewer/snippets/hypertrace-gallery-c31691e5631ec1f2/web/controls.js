const scene = document.querySelector("#scene");
const quality = document.querySelector("#resolution");
const canvas = document.querySelector("#canvas");
const status = document.querySelector("#status");
const stats = document.querySelector("#stats");
const pause = document.querySelector("#pause");
const description = document.querySelector("#example-description");
const mouseLockStatus = document.querySelector("#mouse-lock-status");
const examples = new Map();
const groups = new Map();
let resetRequested = false;
let isPaused = false;
let lastStats = "";
let mouseLockChanged = false;
let wantsMouseLock = false;
let mouseLockPending = false;

const requestedScene = new URLSearchParams(location.search).get("scene");
function describe_example() {
    description.textContent = examples.get(scene.value) ?? "";
}
scene.addEventListener("change", () => {
    describe_example();
    const url = new URL(location.href);
    url.searchParams.set("scene", scene.value);
    location.assign(url);
});
quality.addEventListener("change", () => canvas.focus());
document.querySelector("#reset").addEventListener("click", () => {
    resetRequested = true;
    canvas.focus();
});
pause.addEventListener("click", () => {
    toggle_pause();
    canvas.focus();
});
canvas.addEventListener("pointerdown", () => canvas.focus());
canvas.addEventListener("contextmenu", event => event.preventDefault());

export function mouse_locked() { return document.pointerLockElement === canvas; }
export function take_mouse_lock_change() {
    const changed = mouseLockChanged;
    mouseLockChanged = false;
    return changed;
}
function release_mouse() {
    wantsMouseLock = false;
    mouseLockPending = false;
    if (mouse_locked()) document.exitPointerLock();
}
function mouse_lock_failed() {
    wantsMouseLock = false;
    mouseLockPending = false;
    mouseLockChanged = true;
    mouseLockStatus.textContent = "Mouse lock unavailable · drag to look";
}
// Request synchronously inside the key gesture: deferring this to a render
// frame can lose the browser's user-activation permission. DOM state is the
// authority because requests can fail and Escape can release lock externally.
canvas.addEventListener("keydown", event => {
    if (event.key === "Escape" && (mouse_locked() || mouseLockPending)) {
        event.preventDefault();
        event.stopImmediatePropagation();
        release_mouse();
    }
    if (event.key !== "Tab" || event.shiftKey || event.ctrlKey || event.altKey || event.metaKey) return;
    event.preventDefault();
    event.stopImmediatePropagation();
    if (event.repeat) return;
    if (mouse_locked() || mouseLockPending) {
        release_mouse();
        return;
    }
    wantsMouseLock = true;
    mouseLockPending = true;
    try {
        const request = canvas.requestPointerLock();
        request?.catch(mouse_lock_failed);
    } catch {
        mouse_lock_failed();
    }
}, { capture: true });
document.addEventListener("pointerlockchange", () => {
    mouseLockPending = false;
    mouseLockChanged = true;
    if (mouse_locked() && !wantsMouseLock) {
        // Focus loss or a second Tab can cancel a request before it succeeds.
        document.exitPointerLock();
        return;
    }
    if (!mouse_locked()) wantsMouseLock = false;
    mouseLockStatus.textContent = mouse_locked()
        ? "Mouse locked · Tab / Esc: release" : "Tab: lock mouse";
});
document.addEventListener("pointerlockerror", mouse_lock_failed);
canvas.addEventListener("blur", release_mouse);
window.addEventListener("blur", release_mouse);
document.addEventListener("visibilitychange", () => {
    if (document.hidden) release_mouse();
});

export function supported() { return isSecureContext && !!navigator.gpu; }
export function add_example(id, title, text, group) {
    if (examples.has(id)) return;
    const first = examples.size === 0;
    if (first) {
        scene.replaceChildren();
        scene.disabled = false;
    }
    let options = groups.get(group);
    if (!options) {
        options = document.createElement("optgroup");
        options.label = group;
        groups.set(group, options);
        scene.appendChild(options);
    }
    const option = document.createElement("option");
    option.value = id;
    option.textContent = title;
    options.appendChild(option);
    examples.set(id, text);
    // The catalogue's first entry is the default. Select a requested URL only
    // when Rust supplies a matching entry; unknown IDs keep the first example.
    if (first || id === requestedScene) scene.value = id;
    describe_example();
}
export function scene_name() { return scene.value; }
export function resolution() { return Number(quality.value); }
export function paused() { return isPaused; }
export function take_reset() {
    const requested = resetRequested;
    resetRequested = false;
    return requested;
}
export function toggle_pause() {
    isPaused = !isPaused;
    pause.textContent = isPaused ? "Resume" : "Pause";
    pause.setAttribute("aria-pressed", String(isPaused));
}
export function set_status(message, error) {
    status.textContent = message;
    status.dataset.error = String(error);
    if (error) document.querySelectorAll("button, select").forEach(control => { control.disabled = true; });
}
export function set_stats(width, height, samples) {
    const text = `${width} × ${height} · ${samples} samples${isPaused ? " · paused" : ""}`;
    if (text !== lastStats) {
        stats.textContent = text;
        lastStats = text;
    }
}
