use dioxus::prelude::*;

use crate::{
    components::{common::LottieWebComp, icons::RustGearIcon},
    IO::health::today,
};

const LOTTIE_GEAR_ASSET: Asset = asset!("/assets/ferris-gear.lottie");
const VIEW_W: f32 = 1000.0;
const VIEW_H: f32 = 700.0;

#[derive(Clone, PartialEq)]
struct GearItem {
    x: f32,
    y: f32,
    size: f32,
    left_pct: f32,
    top_pct: f32,
    size_pct: f32,
    rotation_deg: f32,
    spin_seconds: f32,
    clockwise: bool,
    is_lottie: bool,
}

#[component]
pub fn GearMap(
    #[props(default = 7)] count: usize,
    #[props(default = 2026)] seed: u64,
    #[props(default = String::from("text-primary-6/70"))] class: String,
    #[props(default = 180)] height: u32,
) -> Element {
    let today_seed = use_server_future(today).ok();
    let resolved_seed = today_seed
        .as_ref()
        .and_then(|resource| resource().and_then(Result::ok))
        .unwrap_or(seed);
    let gears = use_memo(move || generate_gears(count, resolved_seed));

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "pointer-events-none absolute inset-x-0 bottom-0 overflow-hidden {class}",
            style: "height: {height}px;",
            for (idx, gear) in gears().iter().enumerate() {
                div {
                    key: "{idx}",
                    class: "gear-item",
                    class: if gear.is_lottie {
                        "gear-lottie-drop-in"
                    } else {
                        ""
                    },
                    style: "--gear-left-pct: {gear.left_pct:.2}%; --gear-top-pct: {gear.top_pct:.2}%; --gear-size-pct: {gear.size_pct:.3}%; --gear-rotation-deg: {gear.rotation_deg:.1}deg;",
                    div {
                        class: "animate-spin gear-spin gear-spin-delayed",
                        class: if gear.clockwise {
                            "gear-spin-cw"
                        } else {
                            "gear-spin-ccw"
                        },
                        style: "--gear-spin-seconds: {gear.spin_seconds:.1}s;",
                        div {
                            class: "gear-visual",
                            if gear.is_lottie {
                                LottieWebComp {
                                    src: LOTTIE_GEAR_ASSET.to_string(),
                                }
                            } else {
                                RustGearIcon {
                                    width: gear.size,
                                    style: "width: 100%; height: 100%; display: block;",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn generate_gears(count: usize, seed: u64) -> Vec<GearItem> {
    if count == 0 {
        return Vec::new();
    }

    let mut state = seed ^ 0x9E37_79B9_7F4A_7C15;
    let mut items: Vec<GearItem> = Vec::with_capacity(count);
    let lottie_index = count.saturating_sub(1);

    let lane_pattern: [usize; 10] = [0, 0, 0, 0, 1, 2, 3, 4, 0, 0];
    let lane_phase = (next_u32(&mut state) as usize) % lane_pattern.len();
    let mut lane_counts = [0usize; 5]; // 0: bottom, 1: bottom-left, 2: bottom-right, 3: left, 4: right

    let approx_bottom = ((count as f32) * 0.60).ceil().max(1.0) as usize;
    let approx_corner = ((count as f32) * 0.15).ceil().max(1.0) as usize;
    let approx_side = ((count as f32) * 0.10).ceil().max(1.0) as usize;

    for i in 0..count {
        let size = lerp(140.0, 260.0, next_f32(&mut state));
        let radius = size * 0.5;
        let lane = lane_pattern[(i + lane_phase) % lane_pattern.len()];
        let slot = lane_counts[lane];
        lane_counts[lane] += 1;
        let t = match lane {
            0 => ((slot as f32) + 0.5) / (approx_bottom as f32),
            1 | 2 => ((slot as f32) + 0.5) / (approx_corner as f32),
            _ => ((slot as f32) + 0.5) / (approx_side as f32),
        };
        let lane_jitter = lerp(-0.10, 0.10, next_f32(&mut state));
        let mut along_x = ((t + lane_jitter).clamp(0.04, 0.96)) * VIEW_W;
        let mut along_y = ((t + lane_jitter).clamp(0.06, 0.62)) * VIEW_H;

        let mut placed = None;
        for attempt in 0..180 {
            if attempt > 0 {
                let side_step = (0.15 + 0.03 * (attempt as f32)) * radius;
                let dir = if attempt % 2 == 0 { 1.0 } else { -1.0 };
                if lane == 0 || lane == 1 || lane == 2 {
                    along_x += dir * side_step;
                } else {
                    along_y += dir * side_step;
                }
            }

            let (x, y) = match lane {
                0 => (
                    along_x.clamp(VIEW_W * 0.04, VIEW_W * 0.96),
                    lerp(VIEW_H * 0.72, VIEW_H * 0.90, next_f32(&mut state)),
                ), // bottom band (dominant)
                1 => (
                    lerp(VIEW_W * 0.03, VIEW_W * 0.18, next_f32(&mut state)),
                    lerp(VIEW_H * 0.66, VIEW_H * 0.88, next_f32(&mut state)),
                ), // bottom-left corner area
                2 => (
                    lerp(VIEW_W * 0.82, VIEW_W * 0.97, next_f32(&mut state)),
                    lerp(VIEW_H * 0.66, VIEW_H * 0.88, next_f32(&mut state)),
                ), // bottom-right corner area
                3 => (
                    lerp(VIEW_W * 0.02, VIEW_W * 0.14, next_f32(&mut state)),
                    along_y.clamp(VIEW_H * 0.54, VIEW_H * 0.88),
                ), // left side lower band
                _ => (
                    lerp(VIEW_W * 0.86, VIEW_W * 0.98, next_f32(&mut state)),
                    along_y.clamp(VIEW_H * 0.54, VIEW_H * 0.88),
                ), // right side lower band
            };

            let mut overlaps = false;
            for existing in &items {
                let min_dist = (radius + existing.size * 0.5) * 0.88;
                let dx = x - existing.x;
                let dy = y - existing.y;
                if (dx * dx + dy * dy).sqrt() < min_dist {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                placed = Some((x, y));
                break;
            }
        }

        let is_lottie = i == lottie_index;
        let (mut x, mut y) = placed.unwrap_or_else(|| match lane {
            0 => (
                along_x.clamp(VIEW_W * 0.04, VIEW_W * 0.96),
                lerp(VIEW_H * 0.74, VIEW_H * 0.90, next_f32(&mut state)),
            ),
            1 => (
                lerp(VIEW_W * 0.04, VIEW_W * 0.20, next_f32(&mut state)),
                lerp(VIEW_H * 0.68, VIEW_H * 0.88, next_f32(&mut state)),
            ),
            2 => (
                lerp(VIEW_W * 0.80, VIEW_W * 0.96, next_f32(&mut state)),
                lerp(VIEW_H * 0.68, VIEW_H * 0.88, next_f32(&mut state)),
            ),
            3 => (
                lerp(VIEW_W * 0.03, VIEW_W * 0.16, next_f32(&mut state)),
                along_y.clamp(VIEW_H * 0.56, VIEW_H * 0.88),
            ),
            _ => (
                lerp(VIEW_W * 0.84, VIEW_W * 0.97, next_f32(&mut state)),
                along_y.clamp(VIEW_H * 0.56, VIEW_H * 0.88),
            ),
        });
        let safe = radius * 1.08;
        x = x.clamp(safe, VIEW_W - safe);
        y = y.clamp(safe, VIEW_H - safe);
        let rotation_deg = if is_lottie {
            0.0
        } else {
            lerp(-20.0, 20.0, next_f32(&mut state))
        };
        let clockwise = i % 2 == 0;

        items.push(GearItem {
            x,
            y,
            size,
            left_pct: (x / VIEW_W) * 100.0,
            top_pct: (y / VIEW_H) * 100.0,
            size_pct: (size / VIEW_W) * 100.0,
            rotation_deg,
            spin_seconds: lerp(44.0, 86.0, next_f32(&mut state)),
            clockwise,
            is_lottie,
        });
    }

    items
}

fn next_u32(state: &mut u64) -> u32 {
    let mut x = *state;
    x ^= x >> 12;
    x ^= x << 25;
    x ^= x >> 27;
    *state = x;
    ((x.wrapping_mul(0x2545_F491_4F6C_DD1D)) >> 32) as u32
}

fn next_f32(state: &mut u64) -> f32 {
    (next_u32(state) as f32) / (u32::MAX as f32)
}

fn lerp(min: f32, max: f32, t: f32) -> f32 {
    min + (max - min) * t
}
