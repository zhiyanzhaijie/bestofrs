export async function redirect_to(url) {
  window.location.assign(url);
}

export async function blur_active_element() {
  const activeElement = document.activeElement;
  if (activeElement && typeof activeElement.blur === "function") {
    activeElement.blur();
  }
}

function ensureFuzzySearchHotkeyStore() {
  const key = "__bestofrs_fuzzy_hotkey_store";
  const store = window[key];
  if (store) {
    return store;
  }

  const nextStore = {
    triggerIdCounts: new Map(),
    keyHandler: null,
  };
  window[key] = nextStore;
  return nextStore;
}

function normalizeTriggerIds(triggerIdsOrId) {
  if (Array.isArray(triggerIdsOrId)) {
    return triggerIdsOrId.filter(Boolean);
  }
  if (typeof triggerIdsOrId === "string" && triggerIdsOrId.length > 0) {
    return [triggerIdsOrId];
  }
  return [];
}

function isVisible(element) {
  if (!element) {
    return false;
  }
  const style = window.getComputedStyle(element);
  if (style.display === "none" || style.visibility === "hidden") {
    return false;
  }
  return element.getClientRects().length > 0;
}

function pickTrigger(store) {
  for (const id of store.triggerIdCounts.keys()) {
    const triggers = document.querySelectorAll(`[id="${id}"]`);
    for (const trigger of triggers) {
      if (isVisible(trigger)) {
        return trigger;
      }
    }
  }
  for (const id of store.triggerIdCounts.keys()) {
    const triggers = document.querySelectorAll(`[id="${id}"]`);
    for (const trigger of triggers) {
      return trigger;
    }
  }
  return null;
}

export async function mount_fuzzy_search_hotkey(trigger_ids_or_id, drop) {
  const store = ensureFuzzySearchHotkeyStore();
  const triggerIds = normalizeTriggerIds(trigger_ids_or_id);

  for (const id of triggerIds) {
    const prev = store.triggerIdCounts.get(id) || 0;
    store.triggerIdCounts.set(id, prev + 1);
  }

  if (!store.keyHandler) {
    store.keyHandler = function fuzzySearchKeyHandler(event) {
      const key = (event.key || "").toLowerCase();
      if (key !== "k" || (!event.metaKey && !event.ctrlKey)) {
        return;
      }

      event.preventDefault();
      const trigger = pickTrigger(store);
      if (trigger && typeof trigger.click === "function") {
        trigger.click();
      }
    };

    window.addEventListener("keydown", store.keyHandler);
  }

  drop.then(() => {
    for (const id of triggerIds) {
      const prev = store.triggerIdCounts.get(id) || 0;
      if (prev <= 1) {
        store.triggerIdCounts.delete(id);
      } else {
        store.triggerIdCounts.set(id, prev - 1);
      }
    }

    if (store.triggerIdCounts.size === 0 && store.keyHandler) {
      window.removeEventListener("keydown", store.keyHandler);
      store.keyHandler = null;
    }
  });
}

export async function rewrite_markdown_links(root_id) {
  const root = document.getElementById(root_id);
  if (!root) {
    return;
  }

  const linkBase = root.getAttribute("data-md-link-base-url") || "";
  const imageBase = root.getAttribute("data-md-image-base-url") || "";
  const isAbsolute = (value) => /^(?:[a-zA-Z][a-zA-Z\d+\-.]*:|\/\/)/.test(value);

  for (const element of root.querySelectorAll("[href], [src]")) {
    if (element.hasAttribute("href")) {
      const raw = element.getAttribute("href") || "";
      if (raw && linkBase) {
        if (raw.startsWith("#") || !isAbsolute(raw)) {
          try {
            element.setAttribute("href", new URL(raw, linkBase).toString());
          } catch {
            // ignore
          }
        }
      }
    }

    if (element.hasAttribute("src")) {
      const raw = element.getAttribute("src") || "";
      const base = imageBase || linkBase;
      if (raw && base && !isAbsolute(raw)) {
        try {
          element.setAttribute("src", new URL(raw, base).toString());
        } catch {
          // ignore
        }
      }
    }
  }
}
