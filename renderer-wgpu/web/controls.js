const scene = document.querySelector("#scene");
const quality = document.querySelector("#resolution");
const canvas = document.querySelector("#canvas");
const status = document.querySelector("#status");
const stats = document.querySelector("#stats");
const pause = document.querySelector("#pause");
let resetRequested = false;
let isPaused = false;
let lastStats = "";

const requestedScene = new URLSearchParams(location.search).get("scene");
if (["eu", "hy", "sp"].includes(requestedScene)) scene.value = requestedScene;
scene.addEventListener("change", () => {
    const url = new URL(location.href);
    url.searchParams.set("scene", scene.value);
    history.replaceState(null, "", url);
    canvas.focus();
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
