const charts_by_canvas_id = new Map();

function resolve_canvas(canvas_id) {
  const element = document.getElementById(canvas_id);
  if (!element) return null;
  if (!(element instanceof HTMLCanvasElement)) {
    throw new Error(`Element is not a canvas: ${canvas_id}`);
  }
  return element;
}
async function wait_for_canvas(canvas_id, max_attempts = 120) {
  for (let attempt = 0; attempt < max_attempts; attempt += 1) {
    const canvas = resolve_canvas(canvas_id);
    if (canvas) return canvas;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

async function wait_for_chart(max_attempts = 240) {
  for (let attempt = 0; attempt < max_attempts; attempt += 1) {
    if (window.Chart) {
      return window.Chart;
    }
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  throw new Error("Chart.js did not become available in time");
}

function apply_chart_config(chart, config) {
  chart.config.type = config.type;
  chart.config.data = config.data;
  chart.config.options = config.options;
}

export async function upsert_chart(canvas_id, config, active) {
  if (!active) {
    const existing = charts_by_canvas_id.get(canvas_id);
    if (existing) {
      try {
        existing.resize();
      } catch {
        // ignore
      }
    }
    return null;
  }

  const Chart = await wait_for_chart();
  const canvas = await wait_for_canvas(canvas_id);
  if (!canvas) {
    return null;
  }
  const context = canvas.getContext("2d");
  if (!context) {
    return null;
  }

  let chart = charts_by_canvas_id.get(canvas_id);
  if (chart && chart.canvas !== canvas) {
    try {
      chart.destroy();
    } catch {
      // ignore
    }
    charts_by_canvas_id.delete(canvas_id);
    chart = null;
  }

  if (!chart) {
    chart = Chart.getChart(canvas) ?? null;
  }

  const chart_type_changed = chart && chart.config.type !== config.type;
  if (chart_type_changed) {
    try {
      chart.destroy();
    } catch {
      // ignore
    }
    chart = null;
  }

  if (!chart) {
    chart = new Chart(context, config);
    charts_by_canvas_id.set(canvas_id, chart);
    return chart;
  }

  apply_chart_config(chart, config);
  chart.resize();
  chart.update("none");
  charts_by_canvas_id.set(canvas_id, chart);
  return chart;
}
