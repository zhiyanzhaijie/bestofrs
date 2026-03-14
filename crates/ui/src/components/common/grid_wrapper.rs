use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Default)]
pub enum GridPadding {
    None,
    Sm,
    #[default]
    Md,
    Lg,
}

impl GridPadding {
    fn class(self) -> &'static str {
        match self {
            GridPadding::None => "p-0",
            GridPadding::Sm => "p-4 md:p-6",
            GridPadding::Md => "px-6 py-6 md:px-8 md:py-8",
            GridPadding::Lg => "px-8 py-8 md:px-12 md:py-12",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum GridType {
    #[default]
    Default,
    Inner,
    End,
}
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum GridLineType {
    None,
    #[default]
    Solid,
    Dashed,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum GridPattern {
    #[default]
    None,
    Slash,
    Grid,
    Dot,
}

impl GridPattern {
    fn class(self) -> Option<&'static str> {
        match self {
            GridPattern::None => None,
            GridPattern::Slash => Some("bg-hatch"),
            GridPattern::Grid => Some("pixel-matrix"),
            GridPattern::Dot => Some("bg-screentone"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum GradientDirection {
    #[default]
    None,
    ToTop,
    ToRight,
    ToBottom,
    ToLeft,
    ToTopRight,
    ToTopLeft,
    ToBottomRight,
    ToBottomLeft,
}

impl GradientDirection {
    fn class(self) -> Option<&'static str> {
        match self {
            GradientDirection::None => None,
            GradientDirection::ToTop => Some("bg-gradient-to-t"),
            GradientDirection::ToRight => Some("bg-gradient-to-r"),
            GradientDirection::ToBottom => Some("bg-gradient-to-b"),
            GradientDirection::ToLeft => Some("bg-gradient-to-l"),
            GradientDirection::ToTopRight => Some("bg-gradient-to-tr"),
            GradientDirection::ToTopLeft => Some("bg-gradient-to-tl"),
            GradientDirection::ToBottomRight => Some("bg-gradient-to-br"),
            GradientDirection::ToBottomLeft => Some("bg-gradient-to-bl"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct GridBackground {
    pub pattern: GridPattern,
    pub gradient: GradientDirection,
}

impl GridBackground {
    fn is_empty(self) -> bool {
        matches!(self.pattern, GridPattern::None)
            && matches!(self.gradient, GradientDirection::None)
    }
}

#[component]
pub fn GridWrapper(
    children: Element,
    #[props(default)] is_pixel: bool,
    #[props(default = true)] is_dot_on: bool,
    #[props(default)] line_type: GridLineType,
    #[props(default)] background: GridBackground,
    #[props(default)] bg_class: Option<String>,
    #[props(default = GridType::Default)] grid_type: GridType,
    #[props(default)] class: Option<String>,
    #[props(default = GridPadding::Md)] padding: GridPadding,
) -> Element {
    let wrapper_class = class.unwrap_or_default();
    let padding_class = padding.class();
    let pattern_class = background.pattern.class();
    let background_overlay_class = bg_class.unwrap_or_default();
    let gradient_class = background.gradient.class();

    rsx! {
        div { class: "relative w-full {wrapper_class}",
            if !background.is_empty() {
                div { class: "pointer-events-none absolute inset-0 overflow-hidden rounded-[inherit] {background_overlay_class}",
                    if matches!(background.pattern, GridPattern::Slash) {
                        svg {
                            class: "absolute inset-0 h-full w-full select-none text-[color:var(--grid-hatch-color)] opacity-40",
                            defs {
                                pattern {
                                    id: "grid-slash-pattern",
                                    width: "8",
                                    height: "8",
                                    pattern_units: "userSpaceOnUse",
                                    path {
                                        d: "M -2 2 L 2 -2 M 0 8 L 8 0 M 6 10 L 10 6",
                                        stroke: "currentColor",
                                        stroke_width: "1",
                                        fill: "none",
                                        vector_effect: "non-scaling-stroke",
                                    }
                                }
                            }
                            rect {
                                width: "100%",
                                height: "100%",
                                fill: "url(#grid-slash-pattern)",
                            }
                        }
                    } else if matches!(background.pattern, GridPattern::Grid) {
                        svg {
                            class: "absolute inset-0 h-full w-full select-none text-[color:var(--grid-matrix-color)] opacity-35",
                            defs {
                                pattern {
                                    id: "grid-pattern",
                                    width: "10",
                                    height: "10",
                                    pattern_units: "userSpaceOnUse",
                                    x: "-1",
                                    y: "-1",
                                    path {
                                        d: "M.5 10V.5H10",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_dasharray: "0",
                                        stroke_width: "1",
                                        vector_effect: "non-scaling-stroke",
                                    }
                                }
                            }
                            rect {
                                width: "100%",
                                height: "100%",
                                fill: "url(#grid-pattern)",
                            }
                        }
                    } else if matches!(background.pattern, GridPattern::Dot) {
                        svg {
                            class: "absolute inset-0 h-full w-full select-none text-[color:var(--grid-screentone-color)] opacity-35",
                            defs {
                                pattern {
                                    id: "dot-pattern",
                                    width: "12",
                                    height: "12",
                                    pattern_units: "userSpaceOnUse",
                                    x: "0",
                                    y: "0",
                                    circle {
                                        cx: "1",
                                        cy: "1",
                                        r: "1",
                                        fill: "currentColor",
                                    }
                                }
                            }
                            rect {
                                width: "100%",
                                height: "100%",
                                fill: "url(#dot-pattern)",
                            }
                        }
                    } else if let Some(background_class) = pattern_class {
                        div { class: "absolute inset-0 {background_class}" }
                    }
                    if let Some(gradient_class) = gradient_class {
                        div { class: "absolute inset-0 {gradient_class} grid-gradient-overlay" }
                    }
                }
            }
            if matches!(line_type, GridLineType::Solid) {
                div { class: "pointer-events-none absolute inset-y-0 left-0 z-10", style: "border-left:1px solid var(--grid-line-color);" }
                div { class: "pointer-events-none absolute inset-y-0 right-0 z-10", style: "border-right:1px solid var(--grid-line-color);" }
                if matches!(grid_type, GridType::Default) {
                    div { class: "pointer-events-none absolute bottom-0 left-1/2 z-10 w-screen -translate-x-1/2", style: "border-bottom:1px solid var(--grid-line-color);" }
                } else if matches!(grid_type, GridType::Inner) {
                    div { class: "pointer-events-none absolute inset-x-0 top-0 z-10", style: "border-top:1px solid var(--grid-line-color);" }
                    div { class: "pointer-events-none absolute inset-x-0 bottom-0 z-10", style: "border-bottom:1px solid var(--grid-line-color);" }
                }
            } else if matches!(line_type, GridLineType::Dashed) {
                div { class: "pointer-events-none absolute inset-y-0 left-0 z-10", style: "border-left:2px dashed var(--grid-line-color);" }
                div { class: "pointer-events-none absolute inset-y-0 right-0 z-10", style: "border-right:2px dashed var(--grid-line-color);" }
                if matches!(grid_type, GridType::Default) {
                    div { class: "pointer-events-none absolute bottom-0 left-1/2 z-10 w-screen -translate-x-1/2", style: "border-bottom:2px dashed var(--grid-line-color);" }
                } else if matches!(grid_type, GridType::Inner) {
                    div { class: "pointer-events-none absolute inset-x-0 top-0 z-10", style: "border-top:2px dashed var(--grid-line-color);" }
                    div { class: "pointer-events-none absolute inset-x-0 bottom-0 z-10", style: "border-bottom:2px dashed var(--grid-line-color);" }
                }
            }
            if is_dot_on {
                if matches!(grid_type, GridType::Default) {
                    div { class: "absolute bottom-0 left-0 z-20 h-1.5 w-1.5 -translate-x-1/2 translate-y-1/2 rounded-full", style: "background-color: var(--grid-dot-color);" }
                    div { class: "absolute bottom-0 right-0 z-20 h-1.5 w-1.5 translate-x-1/2 translate-y-1/2 rounded-full", style: "background-color: var(--grid-dot-color);" }
                } else if matches!(grid_type, GridType::Inner) {
                    div { class: "absolute top-0 left-0 z-20 h-1.5 w-1.5 -translate-x-1/2 -translate-y-1/2 rounded-full", style: "background-color: var(--grid-dot-color);" }
                    div { class: "absolute top-0 right-0 z-20 h-1.5 w-1.5 translate-x-1/2 -translate-y-1/2 rounded-full", style: "background-color: var(--grid-dot-color);" }
                    div { class: "absolute bottom-0 left-0 z-20 h-1.5 w-1.5 -translate-x-1/2 translate-y-1/2 rounded-full", style: "background-color: var(--grid-dot-color);" }
                    div { class: "absolute bottom-0 right-0 z-20 h-1.5 w-1.5 translate-x-1/2 translate-y-1/2 rounded-full", style: "background-color: var(--grid-dot-color);" }
                }
            }

            if is_pixel {
                div { class: "pointer-events-none absolute inset-0 z-10 select-none overflow-hidden rounded-[inherit]",
                    div { class: "scanlines absolute inset-0 opacity-50 mix-blend-multiply" }
                    div { class: "pixel-matrix absolute inset-0 opacity-30 mix-blend-multiply" }
                    div { class: "absolute inset-0 bg-gradient-to-br from-transparent via-transparent to-black/5" }
                }
            }

            div { class: "relative h-full w-full {padding_class}",
                {children}
            }
        }
    }
}
