const { invoke } = window.__TAURI__.core;

export { invoke };

export async function invoke_with_args(cmd, args) {
  return await invoke(cmd, args);
}

export async function setOverlay(enabled) {
  return await invoke("set_overlay", { enabled });
}

export async function startDrag() {
  return await invoke("start_drag");
}

export async function registerShortcut() {
  return await invoke("register_shortcut");
}

export async function unregisterShortcut() {
  return await invoke("unregister_shortcut");
}