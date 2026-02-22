export async function js_seed_theme(cookie_name, channel_name, seeded_key, channel_key) {
  if (window[seeded_key]) {
    return;
  }
  window[seeded_key] = true;

  function getCookie(name) {
    const prefix = `${name}=`;
    const parts = document.cookie.split(";");
    for (let part of parts) {
      part = part.trim();
      if (part.startsWith(prefix)) {
        return decodeURIComponent(part.slice(prefix.length));
      }
    }
    return null;
  }

  function apply(theme) {
    if (theme === "dark" || theme === "light") {
      document.documentElement.setAttribute("data-theme", theme);
    } else {
      document.documentElement.removeAttribute("data-theme");
    }
  }

  apply(getCookie(cookie_name));

  try {
    const channel = new BroadcastChannel(channel_name);
    channel.addEventListener("message", (event) => {
      const data = event.data;
      apply(data && data.theme);
    });
    window[channel_key] = channel;
  } catch {
    // ignore
  }
}

export async function js_seed_grid_theme(cookie_name) {
  const theme = read_cookie(cookie_name);
  if (theme) {
    document.documentElement.setAttribute("data-grid-theme", theme);
  }
}

export async function js_is_dark_mode() {
  const theme = document.documentElement.getAttribute("data-theme");
  if (theme === "dark") {
    return true;
  }
  if (theme === "light") {
    return false;
  }
  return !!(
    window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: dark)").matches
  );
}

function read_cookie(name) {
  const prefix = `${name}=`;
  const parts = document.cookie.split(";");
  for (let part of parts) {
    part = part.trim();
    if (part.startsWith(prefix)) {
      return decodeURIComponent(part.slice(prefix.length));
    }
  }
  return null;
}

function broadcast_theme(channel_name, channel_key, theme) {
  try {
    const channel = window[channel_key];
    if (channel && typeof channel.postMessage === "function") {
      channel.postMessage({ theme });
      return;
    }
    const temp = new BroadcastChannel(channel_name);
    temp.postMessage({ theme });
    temp.close();
  } catch {
    // ignore
  }
}

export async function js_set_theme(cookie_name, channel_name, channel_key, theme) {
  document.documentElement.setAttribute("data-theme", theme);
  if (read_cookie(cookie_name) !== theme) {
    document.cookie = `${cookie_name}=${theme}; path=/; max-age=31536000; samesite=lax`;
  }
  broadcast_theme(channel_name, channel_key, theme);
}

export async function js_toggle_theme(cookie_name, channel_name, channel_key) {
  const current = document.documentElement.getAttribute("data-theme");
  const next = current === "dark" ? "light" : "dark";
  await js_set_theme(cookie_name, channel_name, channel_key, next);
}

export async function js_set_grid_theme(cookie_name, theme) {
  if (!theme) {
    document.documentElement.removeAttribute("data-grid-theme");
    document.cookie = `${cookie_name}=; path=/; max-age=0; samesite=lax`;
    return;
  }
  document.documentElement.setAttribute("data-grid-theme", theme);
  if (read_cookie(cookie_name) !== theme) {
    document.cookie = `${cookie_name}=${theme}; path=/; max-age=31536000; samesite=lax`;
  }
}
