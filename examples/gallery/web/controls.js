const scene = document.querySelector("#scene");
const quality = document.querySelector("#resolution");
const canvas = document.querySelector("#canvas");
const status = document.querySelector("#status");
const stats = document.querySelector("#stats");
const pause = document.querySelector("#pause");
const description = document.querySelector("#example-description");
const examples = new Map();
const groups = new Map();
let resetRequested = false;
let isPaused = false;
let lastStats = "";

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
