const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

let profiles = [];

async function loadProfiles() {
  try {
    profiles = await invoke("load_profiles_cmd");
    renderProfiles();
  } catch (e) {
    console.error("Failed to load profiles:", e);
  }
}

function renderProfiles() {
  const listEl = document.getElementById("profilesList");
  if (profiles.length === 0) {
    listEl.innerHTML = '<p class="empty">No profiles imported yet</p>';
    return;
  }
  
  listEl.innerHTML = profiles
    .map(
      (p) => `
    <div class="profile-item">
      <div class="profile-info">
        <p class="profile-name">${escapeHtml(p.name)}</p>
        <p class="profile-hotkey">${p.hotkey ? "Hotkey: " + escapeHtml(p.hotkey) : "No hotkey assigned"}</p>
      </div>
      <div class="profile-actions">
        <button class="btn-small" onclick="applyProfile('${escapeHtml(p.id)}')">Apply</button>
        <button class="btn-small" onclick="setHotkey('${escapeHtml(p.id)}')">Set Hotkey</button>
      </div>
    </div>
  `
    )
    .join("");
}

function escapeHtml(text) {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}

window.applyProfile = async (id) => {
  try {
    const result = await invoke("apply_profile", { id });
    console.log(result);
    alert(result);
  } catch (e) {
    alert("Error: " + e);
  }
};

window.setHotkey = async (id) => {
  const hotkey = prompt("Enter hotkey (e.g., Ctrl+Shift+1):");
  if (hotkey) {
    // TODO: Register global hotkey
    alert("Hotkey registration coming soon: " + hotkey);
  }
};

document.getElementById("importBtn").addEventListener("click", async () => {
  try {
    const file = await open({
      multiple: false,
      filters: [
        {
          name: "JSON",
          extensions: ["json"],
        },
      ],
    });
    
    if (file) {
      const result = await invoke("import_profile", { path: file });
      console.log("Imported:", result);
      await loadProfiles();
    }
  } catch (e) {
    alert("Error: " + e);
  }
});

loadProfiles();
