async function wait_canvas(canvas_id, attempts = 400) {
  for (let i = 0; i < attempts; i += 1) {
    const el = document.getElementById(canvas_id);
    if (el instanceof HTMLCanvasElement) return el;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

async function wait_chart(attempts = 400) {
  for (let i = 0; i < attempts; i += 1) {
    const Chart = window.Chart;
    if (Chart) return Chart;
    await new Promise((resolve) => requestAnimationFrame(resolve));
  }
  return null;
}

const chart_by_id = new Map();
const upsert_version_by_id = new Map();

function begin_upsert(canvas_id) {
  const next = (upsert_version_by_id.get(canvas_id) ?? 0) + 1;
  upsert_version_by_id.set(canvas_id, next);
  return next;
}

function is_current_upsert(canvas_id, version) {
  return upsert_version_by_id.get(canvas_id) === version;
}

function safe_destroy(chart) {
  if (!chart) return;
  try {
    chart.destroy();
  } catch {
    // ignore
  }
}

function resolve_chart_for_canvas(canvas, config, Chart) {
  let chart = Chart.getChart(canvas) ?? null;

  if (chart && chart.config.type !== config.type) {
    safe_destroy(chart);
    chart = null;
  }

  return chart;
}

function upsert_chart_instance(chart, context, config, Chart) {
  if (!chart) return new Chart(context, config);
  chart.data = config.data;
  chart.options = config.options;
  chart.update("none");
  return chart;
}

export async function upsert_chart(canvas_id, config) {
  const version = begin_upsert(canvas_id);

  const Chart = await wait_chart();
  if (!Chart) return null;
  if (!is_current_upsert(canvas_id, version)) return null;

  const canvas = await wait_canvas(canvas_id);
  if (!canvas) return null;
  if (!is_current_upsert(canvas_id, version)) return null;
  if (canvas.id !== canvas_id) return null;

  const context = canvas.getContext("2d");
  if (!context) return null;
  if (!is_current_upsert(canvas_id, version)) return null;
  if (canvas.id !== canvas_id) return null;

  const mapped_chart = chart_by_id.get(canvas_id) ?? null;
  if (mapped_chart && mapped_chart.canvas !== canvas) {
    safe_destroy(mapped_chart);
    chart_by_id.delete(canvas_id);
  }

  let chart = chart_by_id.get(canvas_id) ?? resolve_chart_for_canvas(canvas, config, Chart);
  if (!is_current_upsert(canvas_id, version)) return null;
  if (canvas.id !== canvas_id) return null;

  chart = upsert_chart_instance(chart, context, config, Chart);
  chart_by_id.set(canvas_id, chart);
  if (!is_current_upsert(canvas_id, version)) return null;
  if (canvas.id !== canvas_id) return null;

  chart.resize();
  return chart;
}
