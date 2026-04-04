use dioxus::prelude::*;

#[component]
pub fn ArrowLeftIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-arrow-left-icon lucide-arrow-left",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "m12 19-7-7 7-7" }
            path { d: "M19 12H5" }
        }
    }
}

#[component]
pub fn PlusIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-plus-icon lucide-plus",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M5 12h14" }
            path { d: "M12 5v14" }
        }
    }
}

#[component]
pub fn SearchIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-search-icon lucide-search",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "m21 21-4.34-4.34" }
            circle { cx: "11", cy: "11", r: "8" }
        }
    }
}

#[component]
pub fn CommandIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-command-icon lucide-command",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M15 6v12a3 3 0 1 0 3-3H6a3 3 0 1 0 3 3V6a3 3 0 1 0-3 3h12a3 3 0 1 0-3-3" }
        }
    }
}
#[component]
pub fn EraserIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-eraser-icon lucide-eraser",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M21 21H8a2 2 0 0 1-1.42-.587l-3.994-3.999a2 2 0 0 1 0-2.828l10-10a2 2 0 0 1 2.829 0l5.999 6a2 2 0 0 1 0 2.828L12.834 21" }
            path { d: "m5.082 11.09 8.828 8.828" }
        }
    }
}

#[component]
pub fn XIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-x-icon lucide-x",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M18 6 6 18" }
            path { d: "m6 6 12 12" }
        }
    }
}

#[component]
pub fn TrashIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-trash-icon lucide-trash",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6" }
            path { d: "M3 6h18" }
            path { d: "M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" }
        }
    }
}

#[component]
pub fn WrenchIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-wrench-icon lucide-wrench",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.106-3.105c.32-.322.863-.22.983.218a6 6 0 0 1-8.259 7.057l-7.91 7.91a1 1 0 0 1-2.999-3l7.91-7.91a6 6 0 0 1 7.057-8.259c.438.12.54.662.219.984z" }
        }
    }
}
#[component]
pub fn SaveIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-save-icon lucide-save",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z" }
            path { d: "M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7" }
            path { d: "M7 3v4a1 1 0 0 0 1 1h7" }
        }
    }
}
#[component]
pub fn MenuIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-menu-icon lucide-menu",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            line {
                x1: "4",
                y1: "7",
                x2: "20",
                y2: "7",
            }
            line {
                x1: "8",
                y1: "12",
                x2: "20",
                y2: "12",
            }
            line {
                x1: "4",
                y1: "17",
                x2: "20",
                y2: "17",
            }
        }
    }
}

#[component]
pub fn MoonIcon(
    #[props(default = 24)] size: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-moon-icon lucide-moon",
            fill: "none",
            height: "{size}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M20.985 12.486a9 9 0 1 1-9.473-9.472c.405-.022.617.46.402.803a6 6 0 0 0 8.268 8.268c.344-.215.825-.004.803.401" }
        }
    }
}

#[component]
pub fn ScrollTextIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-scroll-text-icon lucide-scroll-text",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M15 12h-5" }
            path { d: "M15 8h-5" }
            path { d: "M19 17V5a2 2 0 0 0-2-2H4" }
            path { d: "M8 21h12a2 2 0 0 0 2-2v-1a1 1 0 0 0-1-1H11a1 1 0 0 0-1 1v1a2 2 0 1 1-4 0V5a2 2 0 1 0-4 0v2a1 1 0 0 0 1 1h3" }
        }
    }
}

#[component]
pub fn TagsIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-tags-icon lucide-tags",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M13.172 2a2 2 0 0 1 1.414.586l6.71 6.71a2.4 2.4 0 0 1 0 3.408l-4.592 4.592a2.4 2.4 0 0 1-3.408 0l-6.71-6.71A2 2 0 0 1 6 9.172V3a1 1 0 0 1 1-1z" }
            path { d: "M2 7v6.172a2 2 0 0 0 .586 1.414l6.71 6.71a2.4 2.4 0 0 0 3.191.193" }
            circle {
                cx: "10.5",
                cy: "6.5",
                r: ".5",
                fill: "currentColor",
            }
        }
    }
}

#[component]
pub fn GithubIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-github-icon lucide-github",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" }
            path { d: "M9 18c-4.51 2-5-2-7-2" }
        }
    }
}

#[component]
pub fn DiscordIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "discord-icon",
            fill: "currentColor",
            height: "{height}",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M19.27 5.33C17.94 4.71 16.5 4.26 15 4a.1.1 0 0 0-.07.03c-.18.33-.39.76-.53 1.09a16.1 16.1 0 0 0-4.8 0c-.14-.34-.35-.76-.54-1.09c-.01-.02-.04-.03-.07-.03c-1.5.26-2.93.71-4.27 1.33c-.01 0-.02.01-.03.02c-2.72 4.07-3.47 8.03-3.1 11.95c0 .02.01.04.03.05c1.8 1.32 3.53 2.12 5.24 2.65c.03.01.06 0 .07-.02c.4-.55.76-1.13 1.07-1.74c.02-.04 0-.08-.04-.09c-.57-.22-1.11-.48-1.64-.78c-.04-.02-.04-.08-.01-.11c.11-.08.22-.17.33-.25c.02-.02.05-.02.07-.01c3.44 1.57 7.15 1.57 10.55 0c.02-.01.05-.01.07.01c.11.09.22.17.33.26c.04.03.04.09-.01.11c-.52.31-1.07.56-1.64.78c-.04.01-.05.06-.04.09c.32.61.68 1.19 1.07 1.74c.03.01.06.02.09.01c1.72-.53 3.45-1.33 5.25-2.65c.02-.01.03-.03.03-.05c.44-4.53-.73-8.46-3.1-11.95c-.01-.01-.02-.02-.04-.02M8.52 14.91c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.84 2.12-1.89 2.12m6.97 0c-1.03 0-1.89-.95-1.89-2.12s.84-2.12 1.89-2.12c1.06 0 1.9.96 1.89 2.12c0 1.17-.83 2.12-1.89 2.12" }
        }
    }
}

#[component]
pub fn HouseIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-house-icon lucide-house",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M15 21v-8a1 1 0 0 0-1-1h-4a1 1 0 0 0-1 1v8" }
            path { d: "M3 10a2 2 0 0 1 .709-1.528l7-6a2 2 0 0 1 2.582 0l7 6A2 2 0 0 1 21 10v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
        }
    }
}

#[component]
pub fn StarIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-star-icon lucide-star",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M11.525 2.295a.53.53 0 0 1 .95 0l2.31 4.679a2.123 2.123 0 0 0 1.595 1.16l5.166.756a.53.53 0 0 1 .294.904l-3.736 3.638a2.123 2.123 0 0 0-.611 1.878l.882 5.14a.53.53 0 0 1-.771.56l-4.618-2.428a2.122 2.122 0 0 0-1.973 0L6.396 21.01a.53.53 0 0 1-.77-.56l.881-5.139a2.122 2.122 0 0 0-.611-1.879L2.16 9.795a.53.53 0 0 1 .294-.906l5.165-.755a2.122 2.122 0 0 0 1.597-1.16z" }
        }
    }
}

#[component]
pub fn HeartIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-heart-icon lucide-heart",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M2 9.5a5.5 5.5 0 0 1 9.591-3.676.56.56 0 0 0 .818 0A5.49 5.49 0 0 1 22 9.5c0 2.29-1.5 4-3 5.5l-5.492 5.313a2 2 0 0 1-3 .019L5 15c-1.5-1.5-3-3.2-3-5.5" }
        }
    }
}

#[component]
pub fn HeartHandshakeIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-heart-handshake-icon lucide-heart-handshake",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M19.414 14.414C21 12.828 22 11.5 22 9.5a5.5 5.5 0 0 0-9.591-3.676.6.6 0 0 1-.818.001A5.5 5.5 0 0 0 2 9.5c0 2.3 1.5 4 3 5.5l5.535 5.362a2 2 0 0 0 2.879.052 2.12 2.12 0 0 0-.004-3 2.124 2.124 0 1 0 3-3 2.124 2.124 0 0 0 3.004 0 2 2 0 0 0 0-2.828l-1.881-1.882a2.41 2.41 0 0 0-3.409 0l-1.71 1.71a2 2 0 0 1-2.828 0 2 2 0 0 1 0-2.828l2.823-2.762" }
        }
    }
}

#[component]
pub fn BookIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-book-icon lucide-book",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" }
        }
    }
}

#[component]
pub fn ScaleIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-scale-icon lucide-scale",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M12 3v18" }
            path { d: "m19 8 3 8a5 5 0 0 1-6 0zV7" }
            path { d: "M3 7h1a17 17 0 0 0 8-2 17 17 0 0 0 8 2h1" }
            path { d: "m5 8 3 8a5 5 0 0 1-6 0zV7" }
            path { d: "M7 21h10" }
        }
    }
}

#[component]
pub fn CircleDotIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-circle-dot-icon lucide-circle-dot",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            circle { cx: "12", cy: "12", r: "10" }
            circle { cx: "12", cy: "12", r: "1" }
        }
    }
}

#[component]
pub fn GitForkIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-git-fork-icon lucide-git-fork",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            circle { cx: "12", cy: "18", r: "3" }
            circle { cx: "6", cy: "6", r: "3" }
            circle { cx: "18", cy: "6", r: "3" }
            path { d: "M18 9v2c0 .6-.4 1-1 1H7c-.6 0-1-.4-1-1V9" }
            path { d: "M12 12v3" }
        }
    }
}

#[component]
pub fn CalendarDaysIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-calendar-days-icon lucide-calendar-days",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M8 2v4" }
            path { d: "M16 2v4" }
            rect {
                width: "18",
                height: "18",
                x: "3",
                y: "4",
                rx: "2",
            }
            path { d: "M3 10h18" }
            path { d: "M8 14h.01" }
            path { d: "M12 14h.01" }
            path { d: "M16 14h.01" }
            path { d: "M8 18h.01" }
            path { d: "M12 18h.01" }
            path { d: "M16 18h.01" }
        }
    }
}
#[component]
pub fn FishingHookIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-fishing-hook-icon lucide-fishing-hook",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "m17.586 11.414-5.93 5.93a1 1 0 0 1-8-8l3.137-3.137a.707.707 0 0 1 1.207.5V10" }
            path { d: "M20.414 8.586 22 7" }
            circle { cx: "19", cy: "10", r: "2" }
        }
    }
}
#[component]
pub fn UsersRoundIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-users-round-icon lucide-users-round",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M18 21a8 8 0 0 0-16 0" }
            circle { cx: "10", cy: "8", r: "5" }
            path { d: "M22 20c0-3.37-2-6.5-4-8a5 5 0 0 0-.45-8.3" }
        }
    }
}

#[component]
pub fn ArrowRightIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-arrow-right-icon lucide-arrow-right",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M5 12h14" }
            path { d: "m12 5 7 7-7 7" }
        }
    }
}
#[component]
pub fn SunIcon(
    #[props(default = 24)] size: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-sun-icon lucide-sun",
            fill: "none",
            height: "{size}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            circle { cx: "12", cy: "12", r: "4" }
            path { d: "M12 2v2" }
            path { d: "M12 20v2" }
            path { d: "m4.93 4.93 1.41 1.41" }
            path { d: "m17.66 17.66 1.41 1.41" }
            path { d: "M2 12h2" }
            path { d: "M20 12h2" }
            path { d: "m6.34 17.66-1.41 1.41" }
            path { d: "m19.07 4.93-1.41 1.41" }
        }
    }
}

#[component]
pub fn DioxusIcon(
    #[props(default = 24.0)] width: f32,
    #[props(default = 24.0)] height: f32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            id: "dioxus",
            version: "1.1",
            view_box: "0 0 32 31.999997",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            defs { id: "defs2" }
            g {
                id: "layer5",
                style: "display:inline",
                transform: "translate(0,76.000006)",
                g {
                    id: "g9268-5",
                    style: "display:inline",
                    transform: "translate(-34.8584,-125.7831)",
                    g {
                        id: "g9116-7-0",
                        style: "display:inline",
                        transform: "translate(24,5.016704)",
                        path {
                            d: "m 33.01563,46.549479 c 0,3.07669 -0.8509,5.481687 -2.21485,7.376727 -1.36394,1.89503 -3.3201,3.55681 -5.44726,5.33007 -2.12717,1.77326 -4.42516,3.65721 -6.25196,6.19532 C 17.27476,67.989706 16,70.966946 16,74.983296 h 4.69922 c 0,-3.07668 0.85285,-5.37671 2.2168,-7.27175 1.36395,-1.89504 3.3201,-3.55682 5.44726,-5.33008 2.12717,-1.77326 4.42516,-3.6572 6.25196,-6.19531 1.8268,-2.53811 3.10156,-5.620317 3.10156,-9.636677 z",
                            id: "path170-3-3-8-5-73-8-9",
                            style: "color:#000000;display:inline;fill:#e96020;fill-opacity:1",
                        }
                        path {
                            d: "m 20.388672,70.693359 c -0.635433,-2.16e-4 -1.020686,0.514958 -1.02047,1.150391 -2.16e-4,0.635433 0.385037,1.150607 1.02047,1.150391 h 12.939453 c 0.635433,2.16e-4 1.150607,-0.514958 1.150391,-1.150391 2.16e-4,-0.635433 -0.514958,-1.150607 -1.150391,-1.150391 z",
                            id: "path4334-0-9-9-4-1",
                            style: "color:#000000;fill:#2d323b;fill-opacity:1;stroke-linecap:round",
                        }
                        path {
                            d: "m 21.75,66.617187 c -0.607971,2.15e-4 -0.954312,0.557976 -0.953448,1.165947 2.16e-4,0.607208 0.34624,1.035009 0.953448,1.035225 h 10.216797 c 0.607208,-2.16e-4 1.099393,-0.492401 1.099609,-1.099609 8.64e-4,-0.607971 -0.491638,-1.101348 -1.099609,-1.101563 z",
                            id: "path4334-9-1-4-5-7",
                            style: "color:#000000;fill:#2d323b;fill-opacity:1;stroke-linecap:round",
                        }
                        path {
                            d: "m 21.75,53.023437 c -0.607971,2.15e-4 -1.100473,0.493592 -1.099609,1.101563 2.16e-4,0.607208 0.492401,1.099393 1.099609,1.099609 h 10.216797 c 0.607208,-2.16e-4 0.953232,-0.597962 0.953448,-1.20517 8.64e-4,-0.607971 -0.345477,-0.995787 -0.953448,-0.996002 z",
                            id: "path4334-0-6-0-7",
                            style: "color:#000000;fill:#2d323b;fill-opacity:1;stroke-linecap:round",
                        }
                        path {
                            d: "m 20.388672,48.787109 c -0.634671,-2.16e-4 -1.149529,0.513768 -1.150391,1.148438 -2.16e-4,0.635432 0.514959,1.150606 1.150391,1.15039 h 12.939453 c 0.635432,2.16e-4 1.020686,-0.497733 1.02047,-1.133165 -8.62e-4,-0.63467 -0.385799,-1.165879 -1.02047,-1.165663 z",
                            id: "path4334-9-3-3-1",
                            style: "color:#000000;fill:#2d323b;fill-opacity:1;stroke-linecap:round;-inkscape-stroke:none",
                        }
                        path {
                            d: "m 16,46.549479 c 0,4.01636 1.27476,7.098567 3.10156,9.636677 1.8268,2.53811 4.12479,4.42205 6.25196,6.19531 2.12716,1.77326 4.08332,3.65601 5.44726,5.55105 1.36395,1.89504 2.21485,3.9741 2.21485,7.05078 h 4.70117 c 0,-4.01635 -1.27476,-7.03779 -3.10156,-9.5759 -1.8268,-2.53811 -4.12479,-4.42206 -6.25196,-6.19532 -2.12716,-1.77326 -4.08331,-3.43504 -5.44726,-5.33007 -1.36395,-1.89504 -2.2168,-4.255837 -2.2168,-7.332527 z",
                            id: "path170-9-6-61-6-61-1",
                            style: "color:#000000;fill:#00a8d6;fill-opacity:1",
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BestOfRSIcon(
    #[props(default = 24.0)] height: f32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let width = height * (170.0 / 90.0);

    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            version: "1.1",
            view_box: "0 0 170 90",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            g {
                stroke: "none",
                transform: "translate(97.67 21.21) rotate(0 10.54 -5.27)",
                path {
                    d: "M -0.58,-2.55 Q -0.58,-2.55 0.46,-2.82 1.52,-3.09 2.56,-3.48 3.59,-3.87 5.00,-4.60 6.41,-5.33 7.33,-6.08 8.25,-6.83 9.63,-7.87 11.01,-8.90 11.98,-9.35 12.96,-9.79 13.88,-10.40 14.80,-11.01 16.02,-11.67 17.24,-12.34 18.49,-13.04 19.75,-13.74 20.29,-13.87 20.84,-14.00 21.40,-13.95 21.96,-13.90 22.47,-13.67 22.98,-13.44 23.40,-13.06 23.81,-12.68 24.08,-12.19 24.35,-11.70 24.45,-11.15 24.55,-10.59 24.47,-10.04 24.39,-9.48 24.13,-8.98 23.88,-8.48 23.48,-8.09 23.08,-7.70 22.57,-7.45 22.06,-7.21 21.51,-7.14 20.95,-7.07 20.40,-7.18 19.85,-7.30 19.36,-7.58 18.88,-7.86 18.51,-8.28 18.14,-8.70 17.92,-9.22 17.71,-9.74 17.67,-10.30 17.63,-10.86 17.77,-11.40 17.91,-11.95 18.22,-12.42 18.53,-12.89 18.97,-13.23 19.41,-13.58 19.94,-13.77 20.47,-13.95 21.03,-13.96 21.59,-13.97 22.13,-13.80 22.66,-13.63 23.11,-13.30 23.56,-12.96 23.89,-12.50 24.21,-12.04 24.37,-11.51 24.53,-10.97 24.50,-10.41 24.48,-9.85 24.28,-9.32 24.08,-8.80 23.73,-8.36 23.37,-7.93 22.89,-7.63 22.42,-7.34 22.42,-7.34 22.42,-7.34 20.64,-6.48 18.87,-5.62 17.59,-5.12 16.32,-4.61 14.82,-3.83 13.32,-3.04 12.23,-2.27 11.14,-1.50 10.06,-0.92 8.99,-0.34 7.20,0.40 5.42,1.16 4.07,1.63 2.73,2.10 1.66,2.33 0.58,2.55 0.27,2.58 -0.03,2.61 -0.35,2.57 -0.66,2.53 -0.95,2.41 -1.25,2.30 -1.50,2.11 -1.76,1.93 -1.97,1.69 -2.17,1.45 -2.32,1.17 -2.46,0.89 -2.53,0.58 -2.60,0.27 -2.60,-0.03 -2.59,-0.35 -2.51,-0.65 -2.43,-0.96 -2.28,-1.24 -2.13,-1.52 -1.92,-1.75 -1.70,-1.98 -1.44,-2.16 -1.18,-2.33 -0.88,-2.44 -0.58,-2.55 -0.58,-2.55 L -0.58,-2.55 Z",
                    fill: "#1aa6a6",
                }
            }
            g {
                stroke: "none",
                transform: "translate(112.41 10.17) rotate(0 6.84 8.98)",
                path {
                    d: "M 2.61,-0.15 Q 2.61,-0.15 2.68,1.23 2.76,2.63 2.89,3.74 3.02,4.85 3.34,6.79 3.65,8.74 4.07,9.93 4.49,11.12 4.95,12.27 5.41,13.43 5.87,14.41 6.32,15.39 7.31,14.94 8.29,14.49 9.48,14.46 10.66,14.42 10.54,14.63 10.41,14.83 10.72,14.35 11.03,13.86 11.48,13.50 11.93,13.14 12.47,12.95 13.01,12.75 13.58,12.73 14.15,12.72 14.70,12.89 15.25,13.06 15.72,13.39 16.19,13.73 16.52,14.19 16.86,14.66 17.03,15.21 17.19,15.76 17.18,16.33 17.16,16.91 16.96,17.45 16.77,17.99 16.41,18.44 16.05,18.88 15.56,19.19 15.08,19.50 14.52,19.64 13.96,19.78 13.39,19.73 12.81,19.69 12.29,19.46 11.76,19.23 11.33,18.85 10.90,18.46 10.62,17.96 10.34,17.46 10.23,16.90 10.12,16.33 10.20,15.76 10.28,15.19 10.53,14.68 10.79,14.16 11.20,13.76 11.60,13.35 12.12,13.10 12.63,12.84 13.20,12.76 13.77,12.69 14.34,12.79 14.90,12.90 15.40,13.19 15.90,13.47 16.29,13.90 16.67,14.33 16.90,14.85 17.12,15.38 17.17,15.96 17.22,16.53 17.08,17.09 16.94,17.64 16.94,17.64 16.94,17.64 14.63,19.37 12.32,21.09 11.19,21.28 10.06,21.46 8.74,21.42 7.41,21.37 6.33,21.18 5.25,20.98 4.00,20.62 2.75,20.27 1.69,19.18 0.63,18.08 0.30,16.99 -0.02,15.90 -0.38,14.87 -0.75,13.83 -1.04,12.67 -1.33,11.51 -1.67,9.54 -2.01,7.58 -2.12,6.48 -2.23,5.37 -2.33,4.15 -2.43,2.93 -2.52,1.54 -2.61,0.15 -2.59,-0.16 -2.57,-0.47 -2.47,-0.77 -2.38,-1.07 -2.22,-1.34 -2.05,-1.61 -1.83,-1.83 -1.61,-2.06 -1.34,-2.22 -1.06,-2.38 -0.76,-2.48 -0.46,-2.57 -0.15,-2.59 0.16,-2.61 0.47,-2.55 0.78,-2.49 1.07,-2.36 1.35,-2.23 1.60,-2.04 1.85,-1.84 2.04,-1.59 2.24,-1.34 2.37,-1.06 2.49,-0.77 2.55,-0.46 2.61,-0.15 2.61,-0.15 L 2.61,-0.15 Z",
                    fill: "#1aa6a6",
                }
            }
            g {
                stroke: "none",
                transform: "translate(135.29 48.19) rotate(0 0.71 -4.68)",
                path {
                    d: "M -0.90,7.35 Q -0.90,7.35 -1.81,7.05 -2.73,6.75 -3.87,6.14 -5.02,5.52 -5.80,4.60 -6.59,3.67 -7.02,2.35 -7.45,1.02 -7.54,-0.78 -7.62,-2.58 -7.63,-4.62 -7.63,-6.66 -7.55,-7.82 -7.46,-8.99 -7.14,-10.38 -6.82,-11.77 -6.19,-13.07 -5.56,-14.37 -4.13,-15.74 -2.71,-17.11 -1.57,-17.37 -0.44,-17.63 0.71,-17.68 1.86,-17.73 3.10,-17.76 4.33,-17.79 6.06,-17.22 7.78,-16.65 8.62,-15.74 9.46,-14.83 9.90,-13.50 10.33,-12.18 10.38,-10.99 10.43,-9.81 10.45,-8.72 10.47,-7.63 10.31,-6.17 10.16,-4.72 9.90,-3.63 9.65,-2.54 9.13,-1.20 8.60,0.12 7.93,1.12 7.26,2.12 6.54,3.10 5.82,4.08 4.83,5.07 3.84,6.06 2.77,6.65 1.71,7.23 0.46,7.37 -0.77,7.52 -2.13,7.32 -3.48,7.13 -6.32,4.40 -9.15,1.67 -9.15,1.06 -9.15,0.45 -8.96,-0.12 -8.76,-0.71 -8.39,-1.20 -8.02,-1.68 -7.52,-2.03 -7.01,-2.38 -6.42,-2.54 -5.83,-2.71 -5.21,-2.67 -4.60,-2.64 -4.03,-2.41 -3.46,-2.19 -2.99,-1.79 -2.52,-1.39 -2.20,-0.87 -1.89,-0.34 -1.75,0.25 -1.62,0.84 -1.69,1.45 -1.75,2.06 -2.01,2.62 -2.27,3.18 -2.69,3.63 -3.11,4.07 -3.65,4.36 -4.19,4.65 -4.80,4.75 -5.40,4.85 -6.01,4.75 -6.62,4.65 -7.16,4.37 -7.70,4.08 -8.12,3.63 -8.55,3.19 -8.80,2.63 -9.06,2.08 -9.13,1.46 -9.20,0.85 -9.07,0.26 -8.94,-0.33 -8.62,-0.86 -8.30,-1.39 -7.84,-1.78 -7.37,-2.18 -6.80,-2.41 -6.23,-2.64 -5.62,-2.67 -5.00,-2.71 -4.41,-2.54 -3.82,-2.38 -3.31,-2.04 -2.80,-1.69 -2.43,-1.20 -2.06,-0.72 -1.86,-0.13 -1.67,0.44 -1.67,0.44 -1.67,0.44 -1.18,0.40 -0.70,0.35 -0.03,-0.55 0.63,-1.45 1.40,-2.51 2.17,-3.56 2.58,-4.78 2.99,-6.00 3.10,-7.24 3.20,-8.49 3.75,-9.60 4.29,-10.70 3.09,-10.73 1.88,-10.75 0.69,-10.32 -0.49,-9.89 -0.89,-8.72 -1.29,-7.55 -1.38,-6.12 -1.47,-4.70 -1.46,-2.85 -1.45,-0.99 -1.16,0.34 -0.87,1.69 0.01,2.05 0.90,2.42 1.18,2.56 1.46,2.70 1.70,2.91 1.94,3.12 2.12,3.38 2.31,3.64 2.42,3.93 2.54,4.22 2.58,4.54 2.62,4.85 2.59,5.17 2.55,5.48 2.44,5.78 2.33,6.08 2.16,6.34 1.98,6.60 1.75,6.81 1.51,7.03 1.23,7.18 0.95,7.33 0.65,7.41 0.34,7.49 0.02,7.49 -0.28,7.49 -0.59,7.42 -0.90,7.35 -0.90,7.35 L -0.90,7.35 Z",
                    fill: "#2f6fd4",
                }
            }
            g {
                stroke: "none",
                transform: "translate(23.01 79.16) rotate(358.86 63.57 0.26)",
                path {
                    d: "M 0,-1.56 Q 0,-1.56 2.35,-1.52 4.70,-1.49 7.33,-1.46 9.96,-1.44 13.13,-1.42 16.29,-1.40 19.81,-1.39 23.33,-1.38 26.88,-1.37 30.42,-1.36 35.42,-1.43 40.41,-1.50 44.23,-1.52 48.05,-1.55 52.39,-1.56 56.73,-1.57 59.66,-1.57 62.59,-1.58 65.47,-1.58 68.34,-1.58 71.01,-1.58 73.68,-1.58 76.26,-1.58 78.84,-1.58 81.27,-1.58 83.69,-1.58 85.69,-1.60 87.68,-1.63 89.32,-1.72 90.95,-1.81 92.91,-1.93 94.86,-2.06 96.00,-2.02 97.14,-1.99 99.19,-1.86 101.24,-1.74 103.03,-1.77 104.82,-1.80 106.39,-1.87 107.97,-1.94 109.34,-2.00 110.72,-2.07 112.27,-2.15 113.81,-2.23 114.88,-2.29 115.95,-2.35 117.16,-2.41 118.37,-2.47 119.52,-2.53 120.68,-2.58 121.82,-2.63 122.97,-2.68 123.56,-2.28 124.16,-1.88 124.54,-2.31 124.92,-2.73 125.41,-3.02 125.91,-3.30 126.47,-3.41 127.03,-3.52 127.60,-3.45 128.16,-3.37 128.67,-3.12 129.19,-2.87 129.59,-2.47 130.00,-2.07 130.25,-1.55 130.51,-1.04 130.59,-0.48 130.67,0.08 130.56,0.64 130.46,1.20 130.18,1.70 129.90,2.19 129.48,2.58 129.05,2.96 128.53,3.19 128.01,3.41 127.44,3.46 126.87,3.51 126.32,3.38 125.76,3.24 125.28,2.94 124.80,2.63 124.44,2.19 124.08,1.74 123.88,1.21 123.68,0.67 123.66,0.10 123.64,-0.46 123.81,-1.00 123.97,-1.55 124.31,-2.01 124.64,-2.48 125.10,-2.81 125.56,-3.15 126.11,-3.32 126.65,-3.49 127.22,-3.48 127.79,-3.46 128.33,-3.27 128.86,-3.07 129.31,-2.72 129.76,-2.36 130.07,-1.88 130.38,-1.40 130.52,-0.85 130.66,-0.30 130.61,0.26 130.57,0.83 130.34,1.36 130.12,1.88 130.12,1.88 130.12,1.88 126.54,3.07 122.97,4.26 121.82,4.21 120.68,4.17 119.52,4.11 118.37,4.05 117.16,3.99 115.95,3.93 114.88,3.87 113.81,3.81 112.26,3.73 110.71,3.65 109.33,3.58 107.94,3.51 106.32,3.41 104.69,3.31 102.66,3.13 100.63,2.94 98.69,2.57 96.74,2.20 95.62,2.01 94.50,1.83 92.72,1.55 90.95,1.28 89.32,1.19 87.68,1.11 85.69,1.08 83.69,1.05 81.27,1.05 78.84,1.05 76.26,1.05 73.68,1.05 71.01,1.06 68.34,1.06 65.47,1.06 62.60,1.07 59.67,1.08 56.74,1.08 52.41,1.10 48.08,1.12 44.26,1.16 40.45,1.19 35.44,1.28 30.42,1.36 26.88,1.37 23.33,1.38 19.81,1.39 16.29,1.40 13.13,1.42 9.96,1.44 7.33,1.46 4.70,1.49 2.35,1.52 0,1.56 -0.18,1.53 -0.37,1.51 -0.54,1.45 -0.72,1.38 -0.88,1.27 -1.03,1.16 -1.16,1.02 -1.28,0.88 -1.37,0.72 -1.46,0.55 -1.50,0.37 -1.55,0.18 -1.55,-0.00 -1.55,-0.18 -1.50,-0.37 -1.46,-0.55 -1.37,-0.72 -1.28,-0.88 -1.16,-1.02 -1.03,-1.16 -0.88,-1.27 -0.72,-1.38 -0.54,-1.45 -0.37,-1.51 -0.18,-1.53 0.00,-1.56 0.00,-1.56 L 0,-1.56 Z",
                    fill: "#e8473c",
                }
            }
            g {
                stroke: "none",
                transform: "translate(144.01 62.34) rotate(0 7.94 3.26)",
                path {
                    d: "M 1.88,-1.88 Q 1.88,-1.88 2.49,-1.32 3.10,-0.76 4.23,-0.02 5.35,0.71 6.47,1.31 7.59,1.92 8.66,2.45 9.72,2.98 11.01,3.09 12.29,3.20 14.09,3.20 15.89,3.21 16.41,3.29 16.94,3.38 17.41,3.63 17.88,3.89 18.25,4.27 18.62,4.66 18.84,5.15 19.06,5.63 19.12,6.17 19.18,6.70 19.06,7.22 18.95,7.74 18.67,8.20 18.39,8.66 17.98,9.00 17.58,9.35 17.08,9.54 16.58,9.74 16.05,9.77 15.51,9.80 15.00,9.65 14.48,9.51 14.04,9.21 13.60,8.91 13.28,8.48 12.96,8.05 12.79,7.55 12.62,7.04 12.62,6.50 12.62,5.97 12.79,5.46 12.96,4.96 13.29,4.53 13.61,4.11 14.06,3.81 14.50,3.51 15.01,3.37 15.53,3.23 16.06,3.26 16.60,3.29 17.09,3.49 17.59,3.68 18.00,4.03 18.40,4.38 18.68,4.84 18.95,5.30 19.07,5.82 19.18,6.34 19.12,6.87 19.06,7.40 18.83,7.89 18.61,8.37 18.24,8.76 17.87,9.15 17.40,9.40 16.93,9.65 16.40,9.73 15.87,9.82 15.87,9.82 15.87,9.82 13.93,9.78 11.99,9.75 10.46,9.44 8.92,9.14 7.62,8.71 6.32,8.29 4.95,7.33 3.58,6.38 2.31,5.59 1.03,4.80 0.13,3.95 -0.76,3.10 -1.32,2.49 -1.88,1.88 -2.08,1.63 -2.27,1.37 -2.41,1.08 -2.54,0.79 -2.60,0.47 -2.65,0.16 -2.63,-0.15 -2.62,-0.48 -2.52,-0.78 -2.42,-1.09 -2.26,-1.36 -2.09,-1.64 -1.86,-1.87 -1.64,-2.09 -1.36,-2.26 -1.09,-2.42 -0.78,-2.52 -0.48,-2.62 -0.15,-2.63 0.16,-2.65 0.47,-2.60 0.79,-2.54 1.08,-2.41 1.37,-2.27 1.63,-2.08 1.88,-1.88 1.88,-1.88 L 1.88,-1.88 Z",
                    fill: "#8756c9",
                }
            }
            g {
                stroke: "none",
                transform: "translate(10 74.92) rotate(0 11.61 -14.87)",
                path {
                    d: "M 0,-2.17 Q 0,-2.17 1.99,-2.09 3.98,-2.01 5.64,-2.02 7.31,-2.03 10.24,-1.93 13.17,-1.84 15.71,-1.77 18.24,-1.70 19.67,-1.77 21.09,-1.85 21.00,-3.60 20.92,-5.35 21.06,-8.14 21.20,-10.92 21.29,-13.99 21.39,-17.05 21.35,-19.13 21.30,-21.20 20.96,-22.42 20.61,-23.65 19.68,-24.96 18.75,-26.27 17.19,-26.60 15.64,-26.93 15.53,-25.43 15.42,-23.93 15.22,-22.54 15.03,-21.14 14.86,-19.21 14.70,-17.28 14.61,-16.09 14.53,-14.89 14.52,-13.20 14.51,-11.52 14.68,-9.50 14.85,-7.47 14.92,-6.96 14.99,-6.45 14.90,-5.95 14.81,-5.44 14.56,-4.99 14.31,-4.54 13.93,-4.19 13.55,-3.84 13.08,-3.63 12.60,-3.42 12.09,-3.38 11.58,-3.33 11.07,-3.45 10.57,-3.57 10.14,-3.85 9.70,-4.12 9.37,-4.52 9.04,-4.92 8.86,-5.40 8.68,-5.88 8.66,-6.40 8.64,-6.91 8.79,-7.41 8.94,-7.90 9.23,-8.32 9.53,-8.75 9.95,-9.05 10.36,-9.36 10.86,-9.51 11.35,-9.67 11.86,-9.66 12.38,-9.65 12.86,-9.47 13.35,-9.30 13.76,-8.98 14.16,-8.66 14.44,-8.23 14.72,-7.80 14.85,-7.30 14.98,-6.80 14.94,-6.28 14.91,-5.77 14.71,-5.29 14.51,-4.82 14.17,-4.43 13.83,-4.04 13.38,-3.79 12.93,-3.53 12.43,-3.43 11.92,-3.33 11.41,-3.39 10.90,-3.46 10.43,-3.68 9.97,-3.91 9.60,-4.27 9.23,-4.63 9.00,-5.09 8.77,-5.55 8.77,-5.55 8.77,-5.55 8.54,-8.56 8.32,-11.58 8.38,-13.36 8.43,-15.14 8.46,-16.63 8.48,-18.12 8.64,-19.87 8.79,-21.61 9.08,-23.36 9.36,-25.12 9.55,-26.21 9.73,-27.30 10.22,-28.58 10.70,-29.87 11.64,-30.74 12.58,-31.62 13.79,-31.95 14.99,-32.27 16.72,-31.90 18.45,-31.53 19.71,-30.79 20.97,-30.06 22.12,-28.43 23.26,-26.81 23.73,-25.61 24.20,-24.42 24.42,-22.90 24.64,-21.38 24.82,-19.21 25.00,-17.04 25.08,-13.97 25.17,-10.89 25.27,-8.06 25.37,-5.22 25.32,-3.40 25.26,-1.57 24.87,-0.55 24.48,0.46 22.79,1.16 21.10,1.85 19.67,1.77 18.24,1.70 15.71,1.77 13.17,1.84 10.24,1.93 7.31,2.03 5.64,2.02 3.98,2.01 1.99,2.09 0,2.17 -0.25,2.14 -0.51,2.10 -0.76,2.01 -1.00,1.92 -1.22,1.77 -1.44,1.62 -1.61,1.43 -1.78,1.23 -1.90,1.00 -2.03,0.77 -2.09,0.51 -2.15,0.26 -2.15,-0.00 -2.15,-0.26 -2.09,-0.51 -2.03,-0.77 -1.90,-1.00 -1.78,-1.23 -1.61,-1.43 -1.44,-1.62 -1.22,-1.77 -1.00,-1.92 -0.76,-2.01 -0.51,-2.10 -0.25,-2.14 0.00,-2.17 0.00,-2.17 L 0,-2.17 Z",
                    fill: "#f28c1b",
                }
            }
            g {
                stroke: "none",
                transform: "translate(160.30 44.82) rotate(0 -6.11 14.87)",
                path {
                    d: "M 0.42,2.53 Q 0.42,2.53 -0.98,2.76 -2.39,2.98 -3.54,3.22 -4.70,3.45 -6.00,3.83 -7.29,4.21 -7.89,6.09 -8.49,7.97 -8.53,9.24 -8.57,10.50 -8.72,11.97 -8.87,13.44 -8.89,14.66 -8.91,15.89 -8.95,17.05 -8.99,18.21 -9.03,19.42 -9.06,20.62 -9.09,21.83 -9.12,23.05 -9.10,26.39 -9.08,29.73 -9.16,30.23 -9.24,30.73 -9.48,31.18 -9.72,31.62 -10.09,31.97 -10.45,32.32 -10.91,32.54 -11.38,32.75 -11.88,32.80 -12.38,32.86 -12.88,32.75 -13.37,32.64 -13.81,32.38 -14.24,32.12 -14.57,31.73 -14.90,31.34 -15.09,30.87 -15.28,30.40 -15.30,29.89 -15.33,29.39 -15.19,28.90 -15.06,28.41 -14.77,27.99 -14.49,27.57 -14.09,27.26 -13.68,26.96 -13.20,26.79 -12.72,26.63 -12.21,26.63 -11.71,26.63 -11.22,26.79 -10.74,26.96 -10.34,27.26 -9.94,27.57 -9.65,27.99 -9.37,28.41 -9.23,28.90 -9.10,29.39 -9.12,29.90 -9.15,30.40 -9.34,30.87 -9.53,31.34 -9.86,31.73 -10.18,32.12 -10.62,32.38 -11.05,32.64 -11.55,32.75 -12.05,32.86 -12.55,32.80 -13.05,32.75 -13.51,32.53 -13.97,32.32 -14.34,31.97 -14.71,31.62 -14.95,31.17 -15.19,30.73 -15.27,30.23 -15.35,29.73 -15.35,29.73 -15.35,29.73 -15.33,26.38 -15.31,23.04 -15.33,21.82 -15.35,20.60 -15.38,19.38 -15.41,18.16 -15.40,15.99 -15.40,13.81 -15.21,12.70 -15.01,11.59 -14.94,10.17 -14.87,8.74 -14.64,7.13 -14.40,5.51 -13.94,3.95 -13.48,2.39 -12.57,1.14 -11.66,-0.09 -9.92,-0.66 -8.19,-1.22 -6.91,-1.43 -5.64,-1.65 -4.43,-1.85 -3.23,-2.05 -1.82,-2.29 -0.42,-2.53 -0.11,-2.54 0.19,-2.56 0.50,-2.50 0.80,-2.44 1.08,-2.30 1.36,-2.17 1.60,-1.98 1.84,-1.78 2.03,-1.54 2.22,-1.29 2.34,-1.00 2.46,-0.72 2.51,-0.41 2.56,-0.11 2.54,0.19 2.51,0.50 2.42,0.79 2.32,1.09 2.16,1.35 1.99,1.61 1.77,1.83 1.55,2.04 1.28,2.20 1.01,2.36 0.71,2.44 0.42,2.53 0.42,2.53 L 0.42,2.53 Z",
                    fill: "#8756c9",
                }
            }
            g {
                stroke: "none",
                transform: "translate(37.94 10.16) rotate(0 -1.67 15.07)",
                path {
                    d: "M 0.96,-2.38 Q 0.96,-2.38 1.82,-2.08 2.67,-1.78 3.84,-1.22 5.02,-0.67 6.03,-0.05 7.04,0.56 7.77,1.61 8.50,2.66 8.93,3.76 9.36,4.85 9.59,5.90 9.81,6.95 9.90,8.13 10.00,9.31 10.26,10.40 10.52,11.49 10.56,12.57 10.60,13.66 10.64,14.84 10.68,16.03 10.70,17.13 10.72,18.23 10.73,19.47 10.75,20.71 10.76,21.78 10.76,22.86 10.25,24.35 9.75,25.84 8.90,27.30 8.05,28.75 6.93,29.77 5.81,30.78 4.98,31.59 4.16,32.39 3.20,32.87 2.24,33.35 0.93,33.63 -0.37,33.91 -1.48,33.92 -2.60,33.94 -4.15,33.72 -5.69,33.50 -7.01,33.19 -8.32,32.89 -9.89,32.32 -11.45,31.75 -12.01,31.49 -12.57,31.23 -13.02,30.80 -13.47,30.38 -13.76,29.83 -14.05,29.28 -14.15,28.67 -14.25,28.06 -14.15,27.45 -14.05,26.84 -13.75,26.29 -13.46,25.74 -13.01,25.32 -12.56,24.89 -12.00,24.63 -11.44,24.37 -10.82,24.31 -10.21,24.24 -9.60,24.38 -9.00,24.51 -8.47,24.83 -7.94,25.15 -7.54,25.62 -7.14,26.10 -6.91,26.67 -6.68,27.25 -6.65,27.87 -6.61,28.49 -6.78,29.08 -6.95,29.68 -7.30,30.19 -7.64,30.70 -8.14,31.08 -8.63,31.45 -9.22,31.65 -9.81,31.85 -10.43,31.84 -11.05,31.84 -11.63,31.64 -12.22,31.45 -12.71,31.07 -13.20,30.69 -13.55,30.18 -13.90,29.67 -14.06,29.07 -14.23,28.47 -14.19,27.85 -14.16,27.24 -13.93,26.66 -13.70,26.09 -13.29,25.61 -12.89,25.14 -12.36,24.82 -11.83,24.51 -11.22,24.37 -10.62,24.24 -10.00,24.31 -9.39,24.38 -9.39,24.38 -9.39,24.38 -7.32,25.11 -5.24,25.84 -4.13,26.08 -3.02,26.33 -1.79,26.42 -0.56,26.52 0.44,25.82 1.46,25.13 2.15,24.22 2.84,23.32 3.13,22.01 3.42,20.71 3.43,19.47 3.45,18.23 3.47,17.13 3.49,16.03 3.53,14.85 3.56,13.66 3.71,12.55 3.86,11.45 3.59,10.31 3.31,9.17 3.20,8.10 3.10,7.03 3.14,5.81 3.19,4.59 1.93,3.86 0.68,3.14 -0.14,2.76 -0.96,2.38 -1.23,2.23 -1.50,2.08 -1.73,1.87 -1.96,1.66 -2.13,1.40 -2.30,1.14 -2.40,0.85 -2.51,0.56 -2.54,0.25 -2.57,-0.05 -2.53,-0.36 -2.48,-0.67 -2.36,-0.95 -2.25,-1.24 -2.07,-1.49 -1.88,-1.75 -1.65,-1.95 -1.41,-2.15 -1.13,-2.28 -0.85,-2.42 -0.55,-2.49 -0.25,-2.56 0.05,-2.55 0.36,-2.54 0.66,-2.46 0.96,-2.38 0.96,-2.38 L 0.96,-2.38 Z",
                    fill: "#d4b100",
                }
            }
            g {
                stroke: "none",
                transform: "translate(43.36 28.88) rotate(0 -5.84 0)",
                path {
                    d: "M 0,2.59 Q 0,2.59 -0.83,2.63 -1.67,2.68 -2.76,2.75 -3.85,2.82 -4.98,2.90 -6.12,2.98 -8.89,3.05 -11.67,3.13 -12.17,3.05 -12.67,2.96 -13.12,2.73 -13.57,2.49 -13.91,2.12 -14.26,1.75 -14.48,1.29 -14.69,0.83 -14.74,0.33 -14.80,-0.16 -14.69,-0.66 -14.58,-1.15 -14.32,-1.59 -14.06,-2.02 -13.67,-2.35 -13.29,-2.68 -12.81,-2.87 -12.34,-3.05 -11.84,-3.08 -11.33,-3.11 -10.84,-2.97 -10.35,-2.84 -9.94,-2.55 -9.52,-2.27 -9.21,-1.87 -8.90,-1.46 -8.74,-0.98 -8.58,-0.50 -8.58,0.00 -8.58,0.50 -8.74,0.98 -8.90,1.46 -9.21,1.87 -9.52,2.27 -9.94,2.55 -10.36,2.84 -10.84,2.97 -11.33,3.11 -11.84,3.08 -12.34,3.05 -12.82,2.87 -13.29,2.68 -13.67,2.35 -14.06,2.02 -14.32,1.59 -14.58,1.15 -14.69,0.66 -14.80,0.16 -14.74,-0.33 -14.69,-0.83 -14.48,-1.29 -14.26,-1.75 -13.91,-2.12 -13.57,-2.49 -13.12,-2.73 -12.67,-2.96 -12.17,-3.05 -11.67,-3.13 -11.67,-3.13 -11.67,-3.13 -8.89,-3.05 -6.12,-2.98 -4.98,-2.90 -3.85,-2.82 -2.76,-2.75 -1.67,-2.68 -0.83,-2.63 0,-2.59 0.31,-2.56 0.62,-2.52 0.91,-2.41 1.20,-2.30 1.46,-2.12 1.72,-1.94 1.93,-1.71 2.13,-1.47 2.28,-1.19 2.42,-0.92 2.50,-0.61 2.57,-0.31 2.57,0.00 2.57,0.31 2.50,0.61 2.42,0.92 2.28,1.19 2.13,1.47 1.93,1.71 1.72,1.94 1.46,2.12 1.20,2.30 0.91,2.41 0.62,2.52 0.31,2.56 -0.00,2.59 -0.00,2.59 L 0,2.59 Z",
                    fill: "#d4b100",
                }
            }
            g {
                stroke: "none",
                transform: "translate(51.42 22.39) rotate(0 18.65 -1.64)",
                path {
                    d: "M 2.46,-0.57 Q 2.46,-0.57 2.86,0.57 3.26,1.72 3.98,2.76 4.71,3.79 5.75,4.41 6.78,5.03 7.86,5.58 8.93,6.13 10.03,5.91 11.13,5.68 11.74,4.72 12.35,3.76 12.98,2.59 13.61,1.41 13.93,0.39 14.25,-0.61 14.62,-1.93 14.99,-3.24 15.36,-4.67 15.72,-6.10 16.29,-7.28 16.85,-8.47 17.27,-9.45 17.69,-10.43 18.51,-11.80 19.32,-13.17 20.37,-14.11 21.42,-15.05 22.60,-15.44 23.79,-15.83 25.25,-15.94 26.72,-16.06 28.47,-15.94 30.22,-15.81 31.40,-15.16 32.58,-14.50 33.66,-13.62 34.73,-12.74 35.79,-11.70 36.86,-10.67 37.71,-9.39 38.57,-8.10 39.33,-6.95 40.10,-5.79 40.55,-4.62 41.00,-3.45 41.14,-3.01 41.27,-2.58 41.30,-2.13 41.32,-1.67 41.24,-1.23 41.16,-0.78 40.97,-0.37 40.79,0.04 40.51,0.39 40.23,0.75 39.87,1.03 39.51,1.31 39.10,1.50 38.68,1.68 38.24,1.76 37.79,1.84 37.34,1.82 36.88,1.79 36.45,1.65 36.02,1.52 35.63,1.28 35.24,1.05 34.92,0.72 34.60,0.40 34.37,0.01 34.13,-0.36 34.07,-1.74 34.00,-3.12 34.29,-3.66 34.58,-4.19 35.02,-4.61 35.46,-5.03 36.02,-5.28 36.57,-5.54 37.18,-5.60 37.78,-5.66 38.38,-5.53 38.97,-5.40 39.49,-5.08 40.01,-4.77 40.41,-4.30 40.80,-3.84 41.02,-3.27 41.25,-2.70 41.28,-2.09 41.31,-1.49 41.14,-0.90 40.98,-0.31 40.63,0.18 40.29,0.68 39.81,1.05 39.32,1.42 38.74,1.61 38.16,1.80 37.55,1.80 36.95,1.80 36.37,1.60 35.79,1.41 35.31,1.03 34.82,0.66 34.48,0.16 34.14,-0.34 33.98,-0.92 33.82,-1.51 33.86,-2.12 33.89,-2.73 34.12,-3.29 34.35,-3.86 34.74,-4.32 35.14,-4.78 35.66,-5.10 36.19,-5.41 36.78,-5.54 37.38,-5.67 37.98,-5.60 38.59,-5.53 39.14,-5.27 39.69,-5.01 40.13,-4.59 40.57,-4.17 40.86,-3.63 41.14,-3.09 41.24,-2.49 41.33,-1.89 41.17,-2.67 41.00,-3.45 41.14,-3.01 41.27,-2.58 41.30,-2.13 41.32,-1.67 41.24,-1.23 41.16,-0.78 40.97,-0.37 40.79,0.04 40.51,0.39 40.23,0.75 39.87,1.03 39.51,1.31 39.10,1.49 38.68,1.68 38.24,1.76 37.79,1.84 37.34,1.82 36.88,1.79 36.45,1.65 36.02,1.52 35.63,1.28 35.24,1.05 34.92,0.72 34.60,0.40 34.37,0.01 34.13,-0.36 33.73,-1.47 33.33,-2.57 32.71,-3.51 32.09,-4.44 31.31,-5.52 30.52,-6.59 29.19,-7.63 27.86,-8.67 26.43,-8.72 25.01,-8.78 24.50,-7.71 23.99,-6.65 23.39,-5.31 22.80,-3.96 22.48,-2.85 22.16,-1.73 21.78,-0.28 21.40,1.16 20.89,2.52 20.38,3.87 19.83,4.95 19.28,6.03 18.70,7.00 18.12,7.97 17.51,8.86 16.91,9.75 16.04,10.50 15.18,11.25 13.75,11.91 12.32,12.56 11.17,12.52 10.03,12.48 8.96,12.41 7.90,12.35 6.68,11.80 5.46,11.25 4.18,10.57 2.89,9.89 1.87,9.00 0.85,8.11 0.09,7.02 -0.66,5.94 -1.19,4.66 -1.72,3.38 -2.09,2.21 -2.46,1.05 -2.52,0.74 -2.58,0.44 -2.57,0.12 -2.55,-0.18 -2.46,-0.48 -2.38,-0.78 -2.22,-1.05 -2.06,-1.32 -1.84,-1.55 -1.63,-1.77 -1.36,-1.94 -1.10,-2.10 -0.80,-2.20 -0.50,-2.30 -0.19,-2.32 0.11,-2.35 0.42,-2.30 0.73,-2.25 1.01,-2.12 1.30,-2.00 1.55,-1.81 1.80,-1.62 2.00,-1.38 2.20,-1.14 2.33,-0.85 2.46,-0.57 2.46,-0.57 L 2.46,-0.57 Z",
                    fill: "#2fa84f",
                }
            }
            g {
                stroke: "none",
                transform: "translate(64.10 72.48) rotate(0 -0.68 -10.34)",
                path {
                    d: "M -4.50,1.50 Q -4.50,1.50 -5.20,-0.06 -5.90,-1.63 -6.53,-3.01 -7.16,-4.38 -7.77,-5.60 -8.38,-6.82 -8.70,-7.93 -9.02,-9.04 -9.27,-10.28 -9.52,-11.53 -9.60,-12.63 -9.68,-13.74 -9.48,-14.92 -9.28,-16.11 -8.80,-17.25 -8.32,-18.40 -7.40,-19.56 -6.48,-20.71 -5.60,-21.73 -4.73,-22.75 -3.49,-23.55 -2.26,-24.35 -0.89,-24.36 0.46,-24.38 1.50,-23.88 2.53,-23.37 3.16,-22.17 3.79,-20.97 4.18,-19.85 4.57,-18.74 4.95,-17.68 5.32,-16.63 5.90,-15.16 6.48,-13.70 6.81,-12.57 7.14,-11.45 7.42,-9.84 7.71,-8.23 7.88,-6.92 8.05,-5.61 8.07,-4.16 8.10,-2.71 7.69,-0.89 7.28,0.92 5.95,1.82 4.61,2.71 3.22,2.74 1.82,2.77 0.73,2.38 -0.36,1.99 -1.69,0.85 -3.02,-0.28 -3.74,-1.22 -4.47,-2.17 -5.07,-3.52 -5.67,-4.87 -5.99,-6.13 -6.31,-7.40 -6.55,-8.82 -6.79,-10.25 -6.88,-11.38 -6.98,-12.51 -6.94,-13.75 -6.90,-15.00 -6.63,-16.38 -6.36,-17.75 -5.91,-19.12 -5.45,-20.49 -4.66,-21.44 -3.88,-22.39 -2.68,-22.76 -1.48,-23.12 -0.09,-22.71 1.30,-22.29 2.03,-21.48 2.75,-20.66 3.28,-19.45 3.81,-18.23 4.02,-17.12 4.23,-16.02 4.46,-14.53 4.68,-13.05 4.85,-11.41 5.01,-9.77 5.14,-8.55 5.27,-7.32 5.32,-6.14 5.37,-4.97 5.13,-3.81 4.89,-2.66 3.28,-1.59 1.67,-0.52 0.49,-0.71 -0.67,-0.89 -1.52,-1.72 -2.37,-2.54 -3.46,-4.70 -4.56,-6.86 -5.04,-8.17 -5.52,-9.49 -5.88,-10.85 -6.25,-12.21 -6.59,-14.07 -6.94,-15.92 -6.50,-17.85 -6.06,-19.78 -4.57,-21.19 -3.09,-22.61 -1.76,-23.19 -0.44,-23.76 0.66,-23.97 1.76,-24.17 3.38,-23.58 4.99,-22.99 5.60,-22.08 6.21,-21.17 6.59,-19.88 6.98,-18.59 7.38,-16.83 7.78,-15.06 8.03,-13.46 8.28,-11.86 8.44,-10.56 8.60,-9.26 8.59,-7.86 8.57,-6.45 8.21,-5.19 7.84,-3.93 7.11,-2.96 6.37,-1.98 5.44,-1.06 4.51,-0.14 3.66,0.66 2.81,1.46 1.62,2.02 0.42,2.58 -1.50,2.02 -3.42,1.45 -3.98,0.49 -4.55,-0.46 -5.03,-1.91 -5.51,-3.36 -5.72,-5.40 -5.92,-7.43 -5.86,-8.98 -5.79,-10.53 -5.72,-11.72 -5.64,-12.92 -5.09,-14.59 -4.54,-16.26 -3.81,-17.90 -3.07,-19.54 -1.67,-20.69 -0.27,-21.84 0.80,-22.05 1.88,-22.26 3.02,-22.15 4.17,-22.04 6.11,-19.37 8.05,-16.70 8.08,-16.15 8.10,-15.59 7.95,-15.06 7.80,-14.53 7.49,-14.07 7.18,-13.61 6.74,-13.28 6.29,-12.94 5.77,-12.77 5.24,-12.59 4.69,-12.59 4.13,-12.60 3.61,-12.78 3.08,-12.95 2.64,-13.29 2.20,-13.63 1.89,-14.09 1.58,-14.55 1.44,-15.08 1.29,-15.62 1.32,-16.17 1.35,-16.72 1.56,-17.24 1.77,-17.75 2.13,-18.17 2.49,-18.60 2.97,-18.88 3.44,-19.16 3.98,-19.28 4.52,-19.40 5.08,-19.34 5.63,-19.28 6.13,-19.04 6.63,-18.81 7.03,-18.42 7.43,-18.04 7.69,-17.55 7.95,-17.06 8.04,-16.51 8.13,-15.96 8.03,-15.42 7.94,-14.87 7.68,-14.38 7.42,-13.89 7.02,-13.51 6.61,-13.13 6.11,-12.90 5.60,-12.67 5.05,-12.61 4.50,-12.56 3.96,-12.68 3.42,-12.80 2.94,-13.09 2.47,-13.37 2.11,-13.80 1.76,-14.22 1.55,-14.74 1.35,-15.25 1.35,-15.25 1.35,-15.25 1.97,-15.77 2.59,-16.28 2.09,-15.23 1.60,-14.19 1.31,-12.41 1.01,-10.63 1.18,-8.51 1.34,-6.40 1.48,-5.02 1.62,-3.64 0.52,-4.42 -0.57,-5.21 0.50,-5.96 1.57,-6.72 1.63,-8.15 1.68,-9.59 1.49,-10.86 1.29,-12.12 0.98,-13.66 0.67,-15.21 0.20,-16.90 -0.25,-18.59 0.79,-17.97 1.85,-17.36 0.71,-17.02 -0.42,-16.68 -0.20,-15.37 0.02,-14.06 0.36,-13.04 0.70,-12.01 1.17,-10.98 1.64,-9.95 2.56,-8.44 3.48,-6.94 2.19,-7.17 0.90,-7.40 -0.48,-6.31 -1.87,-5.22 -1.90,-6.40 -1.93,-7.58 -2.13,-9.05 -2.33,-10.52 -2.54,-11.93 -2.75,-13.34 -2.87,-14.84 -2.98,-16.34 -1.20,-16.14 0.58,-15.94 0.34,-14.81 0.10,-13.69 0.13,-12.59 0.15,-11.50 0.34,-10.35 0.52,-9.20 0.93,-7.93 1.33,-6.66 1.74,-5.48 2.14,-4.30 1.37,-3.02 0.59,-1.75 0.59,-2.99 0.58,-4.24 0.47,-5.38 0.37,-6.52 0.42,-7.93 0.48,-9.35 -0.01,-10.40 -0.52,-11.45 -1.03,-12.70 -1.54,-13.95 -1.88,-15.06 -2.22,-16.17 -2.32,-17.32 -2.42,-18.48 -1.21,-18.15 -0.00,-17.82 -1.02,-16.80 -2.03,-15.77 -2.75,-14.93 -3.47,-14.09 -3.57,-12.90 -3.68,-11.71 -3.61,-10.21 -3.55,-8.71 -3.17,-7.55 -2.79,-6.38 -2.19,-5.05 -1.58,-3.73 -0.78,-2.20 0.00,-0.68 0.10,-0.40 0.20,-0.11 0.23,0.18 0.25,0.48 0.21,0.78 0.16,1.08 0.05,1.36 -0.06,1.64 -0.24,1.88 -0.42,2.12 -0.65,2.32 -0.88,2.51 -1.16,2.64 -1.43,2.77 -1.72,2.84 -2.02,2.90 -2.32,2.89 -2.62,2.88 -2.91,2.80 -3.20,2.72 -3.47,2.57 -3.73,2.42 -3.95,2.21 -4.17,2.00 -4.33,1.75 -4.50,1.50 -4.50,1.50 L -4.50,1.50 Z",
                    fill: "currentColor",
                }
            }
            g {
                stroke: "none",
                transform: "translate(100.54 69.26) rotate(0 1.85 -10.96)",
                path {
                    d: "M -3.77,-1.42 Q -3.77,-1.42 -3.83,-2.68 -3.89,-3.95 -4.08,-5.20 -4.26,-6.45 -4.37,-7.85 -4.48,-9.26 -4.53,-10.34 -4.58,-11.43 -4.28,-13.11 -3.98,-14.78 -2.68,-16.02 -1.38,-17.26 -0.33,-17.52 0.72,-17.79 2.41,-17.56 4.11,-17.33 5.02,-16.49 5.94,-15.65 6.69,-14.66 7.43,-13.67 8.04,-12.68 8.64,-11.70 9.19,-10.33 9.74,-8.97 9.83,-7.71 9.92,-6.46 9.45,-4.69 8.99,-2.93 8.46,-1.90 7.93,-0.86 7.27,0.12 6.61,1.11 5.42,2.06 4.24,3.02 2.41,2.90 0.57,2.78 -0.50,1.61 -1.58,0.45 -2.18,-0.70 -2.78,-1.87 -3.32,-3.38 -3.86,-4.89 -4.00,-7.09 -4.15,-9.30 -3.93,-10.46 -3.71,-11.62 -3.28,-12.63 -2.86,-13.63 -2.28,-14.59 -1.71,-15.55 -0.32,-16.83 1.06,-18.11 2.93,-18.01 4.81,-17.92 5.76,-17.34 6.71,-16.77 7.18,-15.50 7.65,-14.24 7.97,-12.91 8.29,-11.58 8.49,-9.85 8.68,-8.13 8.59,-6.51 8.49,-4.88 8.16,-3.64 7.83,-2.39 7.09,-1.13 6.35,0.12 5.52,0.96 4.68,1.79 3.47,2.48 2.25,3.17 0.56,2.98 -1.12,2.79 -2.12,2.21 -3.12,1.64 -3.95,-0.01 -4.79,-1.67 -5.38,-3.58 -5.96,-5.48 -6.23,-7.13 -6.50,-8.79 -6.54,-9.96 -6.59,-11.12 -6.12,-12.63 -5.65,-14.13 -4.81,-15.09 -3.97,-16.04 -3.04,-16.59 -2.11,-17.13 -0.76,-17.46 0.58,-17.79 1.79,-17.83 3.01,-17.86 4.05,-17.61 5.09,-17.36 6.13,-16.59 7.17,-15.83 8.03,-14.71 8.88,-13.58 9.51,-11.56 10.14,-9.54 10.25,-8.14 10.36,-6.75 10.30,-5.10 10.23,-3.45 9.62,-2.02 9.00,-0.59 8.14,0.26 7.29,1.12 6.30,1.52 5.32,1.93 3.97,1.89 2.62,1.85 1.26,0.86 -0.10,-0.11 -1.05,-1.62 -2.01,-3.14 -2.72,-5.10 -3.43,-7.06 -3.75,-8.94 -4.06,-10.83 -4.16,-12.22 -4.25,-13.61 -4.11,-14.85 -3.97,-16.09 -3.27,-17.42 -2.57,-18.74 -1.69,-19.42 -0.82,-20.09 0.24,-20.43 1.30,-20.78 2.70,-20.46 4.09,-20.14 4.98,-19.25 5.88,-18.35 6.59,-17.02 7.31,-15.69 7.80,-14.46 8.30,-13.22 8.62,-12.09 8.93,-10.96 9.07,-9.56 9.20,-8.17 9.06,-6.51 8.93,-4.85 8.29,-3.56 7.66,-2.26 6.79,-1.32 5.93,-0.38 5.07,0.25 4.22,0.89 3.02,1.14 1.82,1.39 0.21,0.93 -1.39,0.48 -2.27,-0.54 -3.16,-1.56 -4.33,-3.83 -5.50,-6.10 -5.93,-7.11 -6.37,-8.13 -6.75,-9.93 -7.13,-11.72 -6.90,-13.29 -6.67,-14.85 -6.08,-15.74 -5.48,-16.63 -4.34,-17.56 -3.20,-18.50 -1.63,-19.12 -0.06,-19.74 1.32,-19.85 2.71,-19.96 4.35,-19.40 5.99,-18.84 6.78,-17.94 7.57,-17.05 8.11,-15.87 8.64,-14.68 9.07,-13.29 9.49,-11.89 9.64,-10.49 9.79,-9.08 9.81,-7.97 9.83,-6.87 9.49,-5.27 9.15,-3.66 8.61,-2.67 8.07,-1.68 7.19,-0.81 6.31,0.05 4.77,0.61 3.23,1.17 2.07,1.03 0.90,0.89 -0.91,-0.70 -2.72,-2.29 -3.55,-3.88 -4.38,-5.46 -4.98,-7.29 -5.58,-9.11 -5.91,-10.12 -6.24,-11.14 -6.47,-12.97 -6.71,-14.79 -5.95,-16.99 -5.20,-19.18 -3.98,-20.44 -2.77,-21.70 -1.60,-22.08 -0.43,-22.45 0.96,-22.41 2.37,-22.37 3.94,-21.80 5.52,-21.23 6.70,-20.41 7.89,-19.59 8.54,-18.75 9.20,-17.91 9.63,-16.70 10.06,-15.50 10.18,-14.07 10.30,-12.65 10.33,-11.51 10.35,-10.38 10.11,-8.83 9.86,-7.29 9.35,-5.95 8.83,-4.62 8.13,-3.52 7.43,-2.42 5.73,-1.24 4.03,-0.05 2.85,0.05 1.67,0.17 0.37,-0.14 -0.91,-0.46 -1.89,-1.17 -2.88,-1.88 -3.71,-2.81 -4.54,-3.75 -5.31,-4.94 -6.09,-6.13 -6.67,-7.36 -7.25,-8.60 -7.54,-10.08 -7.82,-11.57 -7.86,-12.91 -7.89,-14.26 -7.46,-15.65 -7.04,-17.04 -6.40,-18.04 -5.76,-19.05 -4.46,-19.84 -3.17,-20.64 -2.08,-20.78 -0.99,-20.91 0.38,-20.82 1.76,-20.73 2.99,-20.26 4.22,-19.78 5.34,-19.06 6.46,-18.35 7.30,-17.42 8.15,-16.49 8.84,-15.14 9.54,-13.80 9.83,-12.74 10.12,-11.68 10.28,-10.23 10.44,-8.78 10.55,-7.48 10.66,-6.17 10.57,-4.64 10.47,-3.11 10.00,-1.95 9.53,-0.78 8.79,0.09 8.04,0.98 6.27,1.57 4.50,2.15 2.60,1.66 0.70,1.16 -0.41,0.25 -1.52,-0.65 -2.27,-1.44 -3.02,-2.23 -3.68,-3.16 -4.34,-4.10 -4.95,-5.69 -5.57,-7.28 -5.83,-8.59 -6.10,-9.89 -6.20,-10.99 -6.30,-12.09 -6.32,-13.29 -6.33,-14.49 -6.13,-16.22 -5.93,-17.95 -5.41,-19.10 -4.88,-20.25 -4.13,-21.22 -3.38,-22.19 -2.38,-23.00 -1.38,-23.81 -0.18,-24.35 1.01,-24.89 2.82,-24.89 4.62,-24.89 5.81,-24.25 7.00,-23.62 7.96,-22.69 8.92,-21.75 9.65,-20.85 10.38,-19.94 10.87,-18.67 11.36,-17.40 11.45,-16.04 11.54,-14.68 11.55,-13.46 11.56,-12.23 11.49,-10.88 11.42,-9.53 11.07,-8.47 10.73,-7.41 10.23,-6.23 9.74,-5.05 9.15,-4.00 8.56,-2.96 7.72,-1.89 6.88,-0.82 5.93,-0.23 4.98,0.36 3.40,0.79 1.81,1.22 0.59,1.22 -0.61,1.23 -2.38,0.26 -4.14,-0.70 -4.69,-1.64 -5.24,-2.58 -5.36,-3.42 -5.48,-4.27 -5.47,-4.73 -5.46,-5.18 -5.34,-5.63 -5.21,-6.07 -4.99,-6.47 -4.77,-6.87 -4.46,-7.20 -4.14,-7.54 -3.76,-7.79 -3.38,-8.04 -2.94,-8.19 -2.51,-8.34 -2.06,-8.38 -1.60,-8.43 -1.15,-8.36 -0.69,-8.29 -0.27,-8.12 0.14,-7.94 0.51,-7.67 0.88,-7.40 1.18,-7.05 1.47,-6.70 1.67,-6.29 1.88,-5.88 1.98,-5.43 2.07,-4.99 2.22,-3.58 2.36,-2.17 2.37,-1.59 2.38,-1.02 2.21,-0.47 2.03,0.07 1.69,0.53 1.35,1.00 0.88,1.33 0.41,1.66 -0.13,1.83 -0.68,1.99 -1.26,1.97 -1.84,1.95 -2.37,1.75 -2.91,1.54 -3.36,1.18 -3.80,0.81 -4.11,0.32 -4.42,-0.15 -4.55,-0.71 -4.68,-1.27 -4.63,-1.85 -4.58,-2.42 -4.35,-2.95 -4.11,-3.47 -3.73,-3.90 -3.34,-4.33 -2.83,-4.60 -2.33,-4.88 -1.76,-4.99 -1.20,-5.09 -0.63,-5.01 -0.06,-4.92 0.45,-4.66 0.96,-4.40 1.36,-3.99 1.77,-3.58 2.02,-3.06 2.27,-2.54 2.34,-1.97 2.41,-1.40 2.30,-0.84 2.19,-0.27 1.90,0.22 1.61,0.72 1.18,1.10 0.75,1.48 0.22,1.70 -0.31,1.92 -0.88,1.96 -1.45,2.00 -2.01,1.86 -2.57,1.72 -3.05,1.40 -3.53,1.09 -3.89,0.64 -4.25,0.18 -4.44,-0.35 -4.64,-0.89 -4.63,-0.89 -4.63,-0.89 -5.06,-2.58 -5.48,-4.27 -5.47,-4.73 -5.46,-5.18 -5.34,-5.63 -5.21,-6.07 -4.99,-6.47 -4.77,-6.87 -4.46,-7.20 -4.14,-7.54 -3.76,-7.79 -3.38,-8.04 -2.94,-8.19 -2.51,-8.34 -2.06,-8.38 -1.60,-8.43 -1.15,-8.36 -0.69,-8.29 -0.27,-8.12 0.14,-7.94 0.51,-7.67 0.88,-7.40 1.18,-7.05 1.47,-6.70 1.67,-6.29 1.88,-5.88 1.97,-5.44 2.07,-4.99 1.97,-5.14 1.87,-5.28 0.81,-5.78 -0.24,-6.28 0.88,-6.37 2.00,-6.46 2.73,-7.82 3.47,-9.17 3.86,-10.30 4.24,-11.42 4.25,-12.63 4.25,-13.84 4.26,-15.03 4.26,-16.22 3.15,-16.99 2.04,-17.76 1.22,-16.83 0.40,-15.91 0.42,-14.32 0.44,-12.73 0.65,-11.28 0.87,-9.83 1.53,-8.56 2.18,-7.29 2.69,-6.05 3.19,-4.81 3.19,-6.07 3.18,-7.32 3.02,-8.92 2.86,-10.51 2.27,-11.62 1.68,-12.74 0.83,-13.62 -0.02,-14.51 -0.25,-13.39 -0.48,-12.28 -0.07,-11.05 0.33,-9.82 1.20,-8.60 2.07,-7.39 2.56,-8.48 3.05,-9.58 3.10,-10.97 3.14,-12.36 2.98,-13.57 2.82,-14.78 1.87,-15.60 0.92,-16.43 0.33,-15.27 -0.26,-14.11 0.09,-12.78 0.45,-11.45 1.04,-10.19 1.63,-8.94 2.00,-7.48 2.36,-6.01 2.44,-7.08 2.52,-8.14 2.42,-9.50 2.32,-10.86 1.90,-11.88 1.49,-12.89 0.61,-12.29 -0.26,-11.68 0.17,-10.51 0.61,-9.35 1.71,-7.47 2.80,-5.60 1.59,-5.37 0.38,-5.15 1.16,-5.88 1.94,-6.61 1.92,-7.84 1.90,-9.07 1.51,-10.36 1.12,-11.65 0.67,-12.81 0.23,-13.97 1.56,-14.10 2.90,-14.23 2.91,-13.09 2.93,-11.94 3.19,-10.52 3.46,-9.10 3.96,-7.87 4.46,-6.63 5.09,-5.64 5.72,-4.65 4.41,-5.29 3.11,-5.92 3.07,-7.18 3.02,-8.44 3.01,-9.54 3.00,-10.64 1.76,-10.53 0.52,-10.43 0.71,-8.91 0.90,-7.40 1.33,-6.10 1.76,-4.81 2.23,-3.82 2.70,-2.82 1.11,-3.01 -0.47,-3.20 0.40,-4.18 1.28,-5.17 1.43,-6.32 1.58,-7.48 1.40,-8.78 1.22,-10.08 0.97,-11.37 0.72,-12.66 2.46,-12.11 4.20,-11.56 3.61,-10.39 3.02,-9.23 3.20,-7.70 3.38,-6.17 3.71,-4.94 4.05,-3.71 2.77,-3.97 1.50,-4.23 2.15,-5.50 2.80,-6.76 2.51,-7.87 2.22,-8.97 1.88,-10.21 1.54,-11.44 1.49,-10.36 1.44,-9.28 1.38,-8.04 1.32,-6.80 1.36,-5.49 1.39,-4.18 1.44,-2.91 1.49,-1.65 1.47,-1.33 1.44,-1.01 1.34,-0.71 1.24,-0.41 1.07,-0.14 0.90,0.12 0.68,0.34 0.45,0.56 0.17,0.72 -0.09,0.88 -0.40,0.97 -0.70,1.06 -1.02,1.07 -1.34,1.09 -1.65,1.02 -1.96,0.96 -2.25,0.83 -2.53,0.69 -2.78,0.49 -3.03,0.29 -3.22,0.04 -3.41,-0.21 -3.54,-0.50 -3.66,-0.79 -3.71,-1.10 -3.77,-1.42 -3.77,-1.42 L -3.77,-1.42 Z",
                    fill: "currentColor",
                }
            }
            g {
                stroke: "none",
                transform: "translate(66.28 57.01) rotate(0 0 0)",
                path {
                    d: "M 3.14,-3.14 Q 3.14,-3.14 3.55,-2.55 3.97,-1.97 4.18,-1.28 4.40,-0.59 4.38,0.11 4.36,0.83 4.11,1.51 3.86,2.18 3.41,2.74 2.96,3.30 2.36,3.69 1.75,4.08 1.05,4.25 0.36,4.42 -0.35,4.36 -1.07,4.31 -1.73,4.02 -2.39,3.74 -2.92,3.26 -3.46,2.78 -3.81,2.15 -4.16,1.53 -4.30,0.82 -4.43,0.11 -4.34,-0.59 -4.24,-1.30 -3.92,-1.94 -3.60,-2.59 -3.09,-3.09 -2.59,-3.60 -1.94,-3.92 -1.30,-4.24 -0.59,-4.34 0.12,-4.43 0.82,-4.30 1.53,-4.16 2.15,-3.81 2.78,-3.46 3.26,-2.92 3.74,-2.39 4.02,-1.73 4.31,-1.07 4.36,-0.35 4.42,0.36 4.25,1.05 4.07,1.75 3.69,2.36 3.30,2.96 2.74,3.41 2.18,3.86 1.51,4.11 0.83,4.36 0.11,4.38 -0.60,4.40 -1.28,4.18 -1.97,3.97 -2.55,3.55 -3.14,3.13 -3.14,3.14 -3.14,3.14 -3.47,2.71 -3.80,2.29 -4.02,1.80 -4.24,1.32 -4.33,0.79 -4.43,0.26 -4.40,-0.26 -4.36,-0.80 -4.20,-1.31 -4.04,-1.82 -3.77,-2.28 -3.49,-2.73 -3.11,-3.11 -2.73,-3.49 -2.28,-3.77 -1.82,-4.05 -1.31,-4.20 -0.80,-4.36 -0.26,-4.40 0.26,-4.43 0.79,-4.33 1.32,-4.24 1.80,-4.02 2.29,-3.80 2.71,-3.47 3.14,-3.14 3.14,-3.14 L 3.14,-3.14 Z",
                    fill: "var(--primary-color-2)",
                }
            }
            g {
                stroke: "none",
                transform: "translate(98.86 56.60) rotate(0 0 0)",
                path {
                    d: "M 3.14,-3.14 Q 3.14,-3.14 3.55,-2.55 3.97,-1.97 4.18,-1.28 4.40,-0.59 4.38,0.11 4.36,0.83 4.11,1.51 3.86,2.18 3.41,2.74 2.96,3.30 2.36,3.69 1.75,4.08 1.05,4.25 0.36,4.42 -0.35,4.36 -1.07,4.31 -1.73,4.02 -2.39,3.74 -2.92,3.26 -3.46,2.78 -3.81,2.15 -4.16,1.53 -4.30,0.82 -4.43,0.11 -4.34,-0.59 -4.24,-1.30 -3.92,-1.94 -3.60,-2.59 -3.09,-3.09 -2.59,-3.60 -1.94,-3.92 -1.30,-4.24 -0.59,-4.34 0.12,-4.43 0.82,-4.30 1.53,-4.16 2.15,-3.81 2.78,-3.46 3.26,-2.92 3.74,-2.39 4.02,-1.73 4.31,-1.07 4.36,-0.35 4.42,0.36 4.25,1.05 4.07,1.75 3.69,2.36 3.30,2.96 2.74,3.41 2.18,3.86 1.51,4.11 0.83,4.36 0.11,4.38 -0.60,4.40 -1.28,4.18 -1.97,3.97 -2.55,3.55 -3.14,3.13 -3.14,3.14 -3.14,3.14 -3.47,2.71 -3.80,2.29 -4.02,1.80 -4.24,1.32 -4.33,0.79 -4.43,0.26 -4.40,-0.26 -4.36,-0.80 -4.20,-1.31 -4.04,-1.82 -3.77,-2.28 -3.49,-2.73 -3.11,-3.11 -2.73,-3.49 -2.28,-3.77 -1.82,-4.05 -1.31,-4.20 -0.80,-4.36 -0.26,-4.40 0.26,-4.43 0.79,-4.33 1.32,-4.24 1.80,-4.02 2.29,-3.80 2.71,-3.47 3.14,-3.14 3.14,-3.14 L 3.14,-3.14 Z",
                    fill: "var(--primary-color-2)",
                }
            }
        }
    }
}

#[component]
pub fn FerrisIcon(
    #[props(default = 24.0)] height: f32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let width = height * (1376.0 / 768.0);

    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            version: "1.0",
            view_box: "0 0 1376.000000 768.000000",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            g {
                fill: "currentColor",
                stroke: "none",
                transform: "translate(0.000000,768.000000) scale(0.100000,-0.100000)",
                path {
                    d: "M7288 7211 c-59 -30 -127 -90 -173 -155 -55 -78 -122 -205 -176 -331\n-50 -117 -120 -231 -147 -239 -20 -7 -76 10 -133 39 -19 10 -101 66 -184 125\n-82 59 -173 119 -201 134 -28 15 -78 32 -110 38 -84 15 -133 -1 -200 -63 -68\n-65 -111 -154 -164 -339 -44 -153 -60 -191 -93 -227 -41 -46 -140 -22 -269 63\n-137 90 -303 154 -400 154 -98 0 -178 -53 -228 -149 -25 -49 -47 -140 -54\n-226 -18 -198 -40 -279 -86 -320 -17 -16 -35 -20 -84 -19 -45 1 -94 13 -191\n48 -198 71 -276 89 -385 90 -79 0 -104 -4 -148 -23 -104 -46 -150 -148 -139\n-301 3 -41 20 -136 37 -210 20 -85 32 -165 33 -217 2 -77 0 -84 -24 -109 -34\n-33 -118 -54 -223 -54 -103 0 -190 -16 -246 -44 -63 -32 -109 -99 -120 -176\n-5 -35 -5 -88 1 -127 11 -81 72 -272 130 -414 33 -78 43 -118 44 -159 l0 -55\n-70 1 c-40 1 -129 15 -210 34 -113 26 -163 33 -255 34 -105 1 -120 -2 -171\n-26 -45 -20 -63 -36 -84 -72 -15 -25 -29 -46 -32 -46 -3 0 -48 20 -101 45\n-100 48 -259 156 -351 239 -31 28 -83 82 -117 122 -66 78 -178 233 -172 239 2\n2 37 26 79 53 42 28 108 80 147 117 75 71 174 212 222 319 64 142 97 348 80\n501 -26 240 -112 436 -259 591 -69 72 -116 95 -198 96 -48 0 -75 -6 -111 -24\n-26 -13 -61 -43 -79 -66 -52 -69 -65 -179 -28 -252 7 -14 34 -52 60 -85 88\n-113 125 -214 127 -342 1 -96 -15 -169 -54 -248 -107 -213 -343 -335 -575\n-295 -104 17 -198 66 -277 143 -49 48 -76 85 -106 149 -37 76 -42 96 -50 196\n-10 119 -22 156 -74 210 -61 66 -150 93 -236 72 -72 -17 -123 -58 -156 -127\n-26 -52 -28 -67 -27 -153 1 -52 9 -129 18 -170 19 -84 56 -190 93 -265 116\n-234 358 -435 614 -509 126 -36 244 -48 372 -36 60 6 112 13 116 15 3 2 17\n-17 29 -43 35 -73 137 -218 219 -313 139 -163 345 -320 551 -423 58 -29 143\n-65 188 -80 45 -15 86 -33 91 -39 6 -7 23 -50 40 -97 17 -47 40 -107 52 -135\n28 -62 50 -141 50 -177 0 -16 -8 -36 -19 -47 -26 -26 -131 -48 -306 -66 -112\n-12 -161 -21 -195 -38 -59 -29 -90 -76 -90 -136 1 -88 45 -170 181 -339 110\n-137 263 -283 412 -395 64 -48 117 -92 117 -98 0 -5 -15 -50 -34 -98 -41 -106\n-69 -205 -92 -326 -13 -65 -18 -139 -18 -270 0 -147 5 -199 23 -285 12 -58 35\n-148 52 -200 35 -109 113 -278 158 -343 53 -78 150 -97 224 -45 34 24 66 90\n67 134 0 18 -24 79 -55 143 -59 121 -105 252 -130 370 -8 41 -18 126 -21 190\n-8 144 13 291 63 441 19 58 37 108 41 112 4 4 61 -22 127 -59 66 -36 142 -75\n168 -86 l48 -20 -8 -51 c-4 -28 -8 -116 -8 -196 1 -120 5 -159 23 -225 31\n-106 92 -233 129 -268 76 -71 210 -45 249 49 22 53 17 101 -21 180 -19 40 -42\n101 -51 136 -16 62 -25 210 -13 222 3 4 53 -15 110 -40 57 -26 127 -55 154\n-65 28 -9 97 -36 155 -59 58 -23 218 -80 355 -127 550 -188 988 -297 1535\n-382 610 -95 1303 -132 1885 -101 205 11 552 42 744 65 174 22 575 90 763 129\n452 96 1045 258 1451 398 48 17 91 28 94 24 4 -4 9 -46 11 -94 5 -114 -15\n-199 -78 -328 -57 -117 -63 -162 -27 -214 33 -50 70 -72 124 -77 66 -6 109 19\n155 87 51 75 114 239 133 345 19 111 19 199 0 310 -9 48 -14 89 -13 90 2 1 57\n23 123 49 145 58 436 202 524 259 l64 41 53 -58 c114 -125 203 -281 258 -449\n14 -43 33 -128 42 -190 18 -125 14 -245 -13 -382 -9 -44 -16 -95 -15 -113 4\n-73 63 -136 137 -148 36 -5 54 -2 92 17 79 40 102 109 123 364 17 198 -16 439\n-84 619 -62 163 -190 376 -293 487 -24 25 -43 49 -43 53 0 4 37 49 83 99 235\n264 410 469 453 534 38 57 74 157 74 206 0 16 -10 51 -21 78 -17 38 -32 56\n-66 75 -82 49 -125 56 -349 63 -189 5 -214 8 -233 25 -36 32 -27 73 54 271\n105 257 118 400 49 522 -25 43 -84 85 -137 98 -15 4 -30 12 -33 17 -3 5 8 29\n24 52 52 72 128 235 166 356 20 63 42 157 49 208 l12 93 115 3 c227 7 398 67\n581 202 132 98 260 256 324 400 41 92 73 213 85 322 15 142 -3 213 -72 279\n-45 44 -96 63 -166 63 -43 0 -72 -7 -110 -26 -89 -44 -128 -114 -138 -246 -7\n-102 -34 -185 -81 -258 -77 -117 -192 -202 -318 -235 -85 -22 -208 -19 -291 6\n-171 52 -288 161 -350 326 -31 83 -42 208 -24 284 7 30 30 92 53 139 34 71 40\n95 40 146 -1 71 -22 122 -71 172 -66 65 -154 87 -244 60 -61 -18 -120 -78\n-166 -165 -77 -152 -106 -276 -106 -463 0 -108 4 -152 23 -224 46 -179 127\n-328 248 -456 75 -80 216 -183 316 -231 54 -26 52 -7 16 -196 -36 -183 -139\n-386 -267 -524 -68 -72 -77 -79 -112 -79 -52 0 -70 26 -70 102 0 31 14 114 31\n185 26 104 32 150 33 243 0 106 -2 119 -28 173 -19 39 -40 66 -69 85 -72 48\n-137 63 -318 72 -183 8 -229 21 -254 71 -21 39 -19 109 6 209 34 143 40 181\n40 282 0 75 -5 108 -19 140 -25 57 -87 111 -153 134 -81 28 -229 24 -407 -10\n-144 -27 -199 -28 -233 -3 -9 7 -24 28 -34 47 -15 28 -20 72 -26 230 -4 107\n-13 215 -19 240 -29 123 -78 193 -156 226 -40 17 -128 18 -192 3 -57 -14 -175\n-68 -301 -137 -108 -60 -188 -86 -231 -77 -16 3 -44 23 -63 44 -26 29 -43 67\n-76 172 -23 74 -51 160 -63 192 -24 67 -85 166 -126 204 -64 61 -151 77 -250\n48 -89 -27 -161 -75 -300 -202 -177 -161 -211 -183 -288 -183 -35 0 -58 7 -86\n26 -56 39 -66 54 -253 379 -98 170 -174 264 -254 316 -38 25 -55 29 -110 29\n-43 -1 -79 -7 -103 -19z m-1014 -2666 c37 -8 100 -30 140 -50 228 -114 400\n-354 480 -670 8 -33 20 -115 26 -183 33 -366 -84 -715 -315 -943 -81 -80 -165\n-136 -254 -170 -31 -11 -99 -26 -152 -32 -80 -9 -110 -8 -178 5 -100 20 -192\n62 -282 129 -114 84 -199 191 -276 344 -51 101 -86 212 -108 340 -17 100 -21\n251 -10 355 30 271 134 505 302 677 54 55 165 132 239 164 106 47 262 61 388\n34z m2966 -14 c106 -33 186 -87 295 -196 80 -80 110 -119 152 -195 61 -114 93\n-201 125 -341 20 -90 23 -129 22 -289 -1 -173 -3 -193 -32 -304 -37 -141 -106\n-288 -186 -396 -67 -91 -188 -198 -276 -244 -99 -52 -177 -70 -305 -69 -81 0\n-126 6 -172 21 -78 25 -172 77 -247 139 -66 53 -177 193 -224 282 -44 81 -88\n209 -114 325 -18 81 -22 133 -22 261 0 184 18 297 70 442 87 242 246 432 440\n528 40 20 101 42 136 50 87 20 253 13 338 -14z",
                }
                path { d: "M6096 3985 c-37 -13 -86 -36 -107 -50 -64 -45 -138 -131 -169 -197\n-16 -35 -35 -98 -41 -140 -10 -60 -10 -92 0 -147 7 -38 28 -101 46 -139 65\n-136 213 -243 367 -265 60 -8 86 -7 149 7 98 22 166 56 233 117 109 99 156\n209 156 360 0 147 -50 262 -153 355 -55 50 -124 88 -196 110 -25 8 -84 14\n-131 14 -64 0 -102 -6 -154 -25z" }
                path { d: "M8785 3995 c-112 -35 -215 -112 -275 -204 -51 -80 -72 -155 -74 -256\n0 -69 5 -105 22 -156 13 -36 36 -87 52 -112 44 -70 117 -134 195 -173 122 -59\n235 -67 362 -23 31 10 75 30 97 43 51 30 134 116 168 176 45 77 61 150 56 265\n-3 97 -6 107 -46 188 -31 63 -57 99 -98 137 -99 89 -206 131 -338 129 -45 -1\n-99 -7 -121 -14z" }
            }
        }
    }
}

#[component]
pub fn BORSFerrisIcon(
    #[props(default = 24.0)] width: f32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            width: "{width}",
            version: "1.1",
            view_box: "0 0 137 86",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            g { fill_rule: "evenodd",
                path {
                    fill: "currentColor",
                    transform: "translate(0.000000,86.000000) scale(0.100000,-0.100000)",
                    d: "M653 842 l-10 -7 -17 -32 -17 -33 -7 0 -7 1 -19 14 -20 15 -18 0 -19 0 -11 -31 -10 -30 -17 7 -16 7 -24 5 -23 5 -9 -14 -9 -14 0 -22 0 -23 -4 0 -5 0 -27 10 -27 11 -19 -7 -20 -6 6 -38 6 -38 -31 -6 -32 -6 7 -46 6 -46 -29 7 -29 6 -8 -23 -8 -23 1 -5 1 -5 6 -22 7 -21 -31 -7 -30 -7 0 -13 0 -13 42 -43 43 -44 30 -10 30 -10 -3 -25 -2 -25 -6 -47 -7 -48 17 0 16 0 0 21 0 22 12 38 11 39 14 0 13 0 19 -12 20 -12 -9 -15 -8 -16 -7 -47 -7 -48 15 0 16 0 11 55 11 55 8 5 8 5 52 -13 51 -12 6 -17 5 -17 -10 -20 -11 -20 0 -29 0 -29 23 -21 23 -22 17 0 17 0 0 15 0 14 -19 10 -19 10 -6 17 -6 17 16 25 16 24 40 -6 40 -6 -4 17 -3 18 -45 6 -45 6 78 1 78 2 -11 -21 -12 -21 7 -6 6 -6 19 12 20 12 20 0 20 0 15 -24 16 -25 -10 -19 -10 -20 -16 -6 -15 -5 0 -16 0 -15 18 0 19 0 21 23 22 23 0 28 0 27 -10 19 -10 19 6 15 6 16 32 0 32 0 13 13 12 12 15 -19 14 -19 6 -34 6 -33 9 0 9 0 6 24 5 23 -15 36 -14 36 26 3 27 3 16 -44 15 -45 14 -4 13 -5 -6 39 -6 39 -6 21 -6 20 41 19 42 19 35 43 35 43 -6 15 -5 15 -32 0 -31 0 7 18 8 17 1 25 1 24 -35 11 -35 10 0 35 0 35 -30 5 -30 5 0 35 0 35 -40 5 -40 5 -3 32 -3 31 -15 6 -15 6 -29 -15 -30 -15 -14 35 -15 35 -14 0 -14 0 -28 -22 -28 -22 -23 30 -24 29 0 6 0 7 -13 5 -14 5 -10 -6z m-64 -286 l22 -23 6 -36 6 -37 -13 -30 -13 -31 -18 -9 -18 -10 -24 0 -24 0 -22 24 -22 23 -5 35 -6 35 16 31 16 31 17 10 18 10 21 1 21 0 22 -24z m290 0 l22 -23 6 -36 6 -37 -13 -30 -13 -31 -18 -9 -18 -10 -24 0 -24 0 -21 23 -21 22 -6 27 -7 27 12 35 12 36 21 15 21 15 21 0 22 0 22 -24z m-82 -353 l-7 -6 -7 6 -6 7 13 0 13 0 -6 -7z M528 519 l-18 -10 0 -28 0 -29 16 -12 15 -13 27 9 27 9 3 24 4 23 -18 19 -17 19 -11 0 -11 -1 -17 -10z M790 512 l-23 -19 7 -26 6 -27 27 -6 26 -7 19 23 18 22 0 9 0 9 -20 20 -20 20 -9 0 -9 0 -22 -18z",
                }
                path {
                    fill: "#1aa6a6",
                    transform: "translate(0.000000,86.000000) scale(0.100000,-0.100000)",
                    d: "M750 162 l0 -8 -45 -62 -45 -62 0 -15 0 -15 15 0 15 0 0 13 0 12 50 65 49 65 1 8 0 7 -20 0 -20 0 0 -8z",
                }
                path {
                    fill: "#2fa84f",
                    transform: "translate(0.000000,86.000000) scale(0.100000,-0.100000)",
                    d: "M718 153 l-27 -13 -30 -50 -31 -50 0 -15 0 -15 13 0 12 0 10 20 10 20 39 55 39 56 -4 2 -4 3 -27 -13z",
                }
                path {
                    fill: "#2f6fd4",
                    transform: "translate(0.000000,86.000000) scale(0.100000,-0.100000)",
                    d: "M741 93 l-51 -68 0 -12 0 -13 13 0 12 1 30 42 30 42 19 28 19 28 -10 10 -10 10 -52 -68z",
                }
                path {
                    fill: "#d4b100",
                    transform: "translate(0.000000,86.000000) scale(0.100000,-0.100000)",
                    d: "M648 133 l-14 -18 -17 -27 -17 -27 0 -16 0 -15 13 0 12 0 33 54 32 54 0 6 0 6 -13 0 -14 0 -15 -17z",
                }
                path {
                    fill: "#8756c9",
                    transform: "translate(0.000000,86.000000) scale(0.100000,-0.100000)",
                    d: "M761 69 l-43 -60 6 -7 7 -6 17 10 17 11 3 24 4 24 24 22 24 22 0 11 0 10 -7 -1 -8 0 -44 -60z",
                }
                path {
                    fill: "#e8473c",
                    transform: "translate(0.000000,86.000000) scale(0.100000,-0.100000)",
                    d: "M565 136 l-15 -23 6 -17 6 -16 7 0 6 0 23 33 22 33 0 7 0 7 -20 0 -20 0 -15 -24z",
                }
                path {
                    fill: "#f28c1b",
                    transform: "translate(0.000000,86.000000) scale(0.100000,-0.100000)",
                    d: "M611 136 l-11 -21 -15 -21 -16 -22 15 -5 15 -6 30 44 29 43 -18 5 -18 5 -11 -22z",
                }
            }
        }
    }
}

#[component]
pub fn FerrisFisherIcon(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            width: "100%",
            version: "1.1",
            view_box: "0 0 640 402",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            g { fill_rule: "evenodd",
                path {
                    fill: "currentColor",
                    fill_opacity: "0.2",
                    d: "M1588 993 l-48 -4 0 -115 0 -114 20 0 19 0 4 93 3 92 2 -92 2 -93 23 2 22 1 60 -7 60 -7 15 -4 15 -5 0 90 0 91 -15 9 -15 10 14 9 14 9 -6 16 -6 16 -48 0 -48 0 -20 4 -20 3 -47 -4z m29 -185 l-4 -33 0 60 0 60 4 -27 4 -28 -4 -32z M3715 961 l-40 -6 -3 -97 -3 -97 127 -3 126 -3 -6 70 -7 70 -9 3 -10 4 6 22 5 22 -26 12 -26 12 -47 -1 -47 -2 -40 -6z m127 -99 l3 -52 -7 0 -8 0 1 73 2 72 3 -20 3 -20 3 -53z m36 -62 l-1 -35 -7 50 -8 50 0 40 1 40 7 -55 8 -55 0 -35z M1804 835 l0 -95 33 0 33 0 0 81 0 82 -17 13 -18 13 -15 1 -15 0 -1 -95z M3931 893 l0 -28 9 -45 9 -45 -3 65 -4 65 8 -20 9 -20 0 -42 1 -41 6 -16 6 -16 12 0 12 0 -4 73 -4 72 -10 13 -10 12 -19 0 -19 0 1 -27z M3945 760 l-6 -10 10 0 11 0 0 10 0 10 -4 0 -5 0 -6 -10z M0 489 l0 -142 633 7 632 7 135 7 135 7 6 -9 7 -8 158 6 159 6 174 6 173 6 107 -11 106 -11 455 -2 455 -1 130 7 130 7 200 5 200 5 118 -1 119 -1 14 -6 14 -5 0 131 0 131 -114 0 -115 0 -28 -11 -27 -10 -422 0 -422 1 -5 -13 -5 -12 -10 14 -10 14 -366 -9 -366 -9 0 8 0 8 -225 -2 -224 -2 -4 -16 -5 -16 -1 18 -1 17 -22 1 -23 0 -400 4 -400 5 -62 5 -63 6 0 -11 0 -11 -21 6 -21 7 -219 3 -218 2 -5 -11 -4 -11 -1 13 -1 12 -225 0 -225 0 0 -141z m2306 38 l-11 -7 -365 2 -365 3 35 5 35 5 341 0 342 0 -12 -8z m1319 2 l-140 -7 -140 1 -140 1 160 6 160 6 120 0 120 1 -140 -8z m-2600 -9 l-400 -7 -310 6 -310 6 710 1 710 0 -400 -6z m1833 3 l-88 -4 -87 4 -88 3 175 0 175 0 -87 -3z m1270 -10 l-68 -4 -67 4 -68 3 135 0 135 0 -67 -3z m-3348 -64 l15 -10 -25 5 -25 5 -175 2 -175 2 23 6 23 6 162 -3 161 -3 16 -10z m2028 14 l-188 -3 -187 3 -188 2 375 0 375 0 -187 -2z m-1000 -30 l-118 -4 -117 4 -118 3 235 0 235 0 -117 -3z m2010 0 l-227 -3 -233 3 -233 2 460 0 460 0 -227 -2z m-3483 -12 l50 -7 -130 1 -130 0 50 6 50 6 30 1 30 0 50 -7z m2007 -8 l-93 -4 -87 4 -87 3 180 0 180 0 -93 -3z M3838 353 l-47 -4 -4 -47 -3 -47 -2 48 -2 47 -26 0 -26 0 5 -37 5 -38 -11 38 -11 37 -18 0 -18 0 0 -175 0 -175 130 0 130 0 0 149 0 148 -6 32 -6 31 -22 -2 -21 -2 -47 -3z m69 -80 l-4 -28 0 55 0 55 4 -27 4 -28 -4 -27z m-60 0 l-4 -28 0 50 0 50 4 -22 4 -23 -4 -27z M3958 193 l3 -168 4 -12 4 -13 22 0 21 0 -6 23 -6 22 0 158 0 157 -9 0 -10 0 -4 -42 -4 -43 -1 43 -2 42 -7 0 -8 0 3 -167z M1813 343 l-13 -5 0 -169 0 -169 40 0 40 0 0 175 0 175 -27 -1 -28 -1 -12 -5z M1540 170 l0 -170 120 0 120 0 0 170 0 170 -24 0 -25 0 -4 -42 -4 -43 -1 43 -2 42 -15 0 -14 0 -4 -77 -3 -78 -2 78 -2 77 -45 0 -45 0 -1 -77 -1 -78 -7 -25 -8 -25 5 103 5 102 -22 0 -21 0 0 -170z m97 -17 l-3 -58 0 115 0 115 3 -57 4 -58 -4 -57z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#9c6a33",
                    d: "M2460 1855 l-12 -15 11 0 12 0 84 -85 83 -85 11 0 11 0 0 20 0 20 -7 0 -8 0 -35 31 -35 30 -42 39 -43 40 0 10 0 10 -9 0 -9 0 -12 -15z M1902 1828 l-12 -12 0 -17 0 -16 123 -121 122 -121 60 -56 60 -56 18 -10 17 -10 0 16 0 15 15 0 15 0 0 -2 0 -3 -15 -80 -15 -80 -16 -55 -15 -55 -10 -31 -9 -30 0 -13 0 -13 -13 -31 -14 -32 -11 -35 -11 -35 -13 -57 -13 -58 -12 0 -13 0 0 -10 0 -10 -21 0 -21 0 -29 -21 -29 -21 5 -9 6 -9 17 0 17 0 90 31 90 31 90 29 90 30 100 39 100 38 50 17 50 17 17 9 18 9 10 0 10 0 0 -14 0 -13 45 -42 45 -41 0 -8 0 -8 -32 -13 -33 -13 -45 -13 -45 -13 -75 -31 -75 -30 65 -1 65 -1 100 33 100 33 3 8 4 7 12 0 12 0 64 -45 64 -45 40 0 41 0 -59 54 -59 55 6 14 5 14 104 33 103 33 53 25 52 25 23 -20 22 -20 45 -39 44 -39 28 -26 27 -26 50 -37 51 -36 0 44 0 44 -62 53 -63 54 -5 5 -5 6 -42 30 -43 31 -47 -8 -48 -8 -55 -10 -55 -9 -125 -42 -124 -41 -39 40 -38 41 3 17 3 17 35 12 35 12 73 21 72 22 -5 9 -5 8 -19 -5 -18 -5 -6 21 -5 20 -28 -9 -29 -9 -93 -31 -92 -30 -14 0 -14 0 -6 15 -5 15 -21 0 -20 0 0 14 0 15 -15 13 -14 13 -41 30 -41 30 -72 60 -72 60 -47 39 -48 38 0 -11 0 -11 -15 0 -15 0 0 15 0 15 -11 0 -11 0 5 20 5 21 -23 15 -24 16 -181 179 -181 179 -12 0 -13 0 -12 -12z m504 -518 l29 -30 95 -74 95 -75 44 -39 45 -38 -6 -16 -6 -15 -49 -13 -48 -13 -38 -18 -37 -17 -88 -27 -87 -26 -53 -25 -53 -24 -9 0 -10 0 0 11 0 10 25 102 26 102 15 55 15 55 9 30 8 30 7 35 7 35 5 8 4 7 13 0 13 0 29 -30z M3575 1199 l-20 -9 43 6 42 6 0 -41 0 -41 -12 0 -13 0 -80 -40 -80 -39 5 -1 6 0 51 18 51 18 14 11 13 11 28 7 27 7 0 49 0 49 -27 -1 -28 0 -20 -10z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#e8473c",
                    d: "M2453 1812 l-22 -3 46 -67 45 -67 41 -48 40 -48 26 3 26 3 -3 30 -3 30 -87 85 -87 85 -22 -3z M3598 1173 l-18 -4 0 -15 0 -14 20 0 20 0 0 20 0 20 -2 -1 -3 -1 -17 -5z M4888 409 l-26 -10 -7 -27 -6 -27 -6 -20 -6 -20 51 49 51 49 -5 9 -5 8 -7 -1 -7 -1 -27 -9z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#f28c1b",
                    d: "M2369 1783 l22 -28 55 -85 56 -85 41 -42 41 -43 13 0 13 0 -18 37 -17 36 12 4 13 5 -54 66 -53 67 -29 45 -30 45 -43 3 -44 3 22 -28z M3526 1148 l6 -23 5 -5 4 -4 17 10 17 11 3 17 3 16 -31 0 -31 0 7 -22z M3003 1143 l7 -6 7 6 6 7 -13 0 -13 0 6 -7z M3460 1075 l-45 -24 15 0 15 -1 40 25 40 24 -10 0 -10 0 -45 -24z M3400 1045 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M4898 364 l-48 -46 0 -11 1 -12 14 -19 15 -20 3 5 2 4 54 58 53 58 -18 14 -19 14 -5 1 -5 0 -47 -46z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#d4b100",
                    d: "M2275 1783 l24 -28 58 -72 58 -72 40 -52 40 -51 20 -25 20 -25 31 18 30 17 -43 40 -42 39 -60 92 -60 91 -21 28 -22 27 -48 0 -48 0 23 -27z M2585 1390 l6 -10 5 0 5 0 -6 10 -6 10 -5 0 -5 0 6 -10z M3457 1135 l6 -35 -19 0 -20 0 -19 -13 -20 -14 -45 -17 -45 -18 -70 -14 -70 -14 70 6 69 6 24 9 23 9 24 0 25 0 55 27 55 27 0 13 0 13 15 0 15 0 0 9 0 9 -6 16 -6 16 -33 0 -34 0 6 -35z M2940 1140 l-25 -8 25 0 25 -1 20 9 20 9 -20 -1 -20 0 -25 -8z M4937 325 l-49 -54 6 -19 7 -20 15 -6 16 -6 26 28 27 27 29 38 28 38 -3 2 -4 2 -22 13 -23 12 -2 -1 -3 -1 -48 -53z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#2fa84f",
                    d: "M2173 1803 l17 -4 0 -11 0 -10 32 -42 33 -41 52 -55 52 -55 70 -88 71 -88 35 -36 36 -37 16 17 17 17 -32 33 -32 33 0 7 0 7 -47 60 -48 61 -23 31 -23 32 -57 70 -57 69 -17 19 -18 18 -47 -1 -48 -1 18 -5z M3380 1144 l10 -25 0 -8 0 -8 -30 -6 -30 -6 -62 -29 -63 -29 -15 -6 -15 -5 20 -1 21 -1 57 15 56 15 46 21 45 21 0 14 0 14 20 0 20 0 0 9 0 9 -6 16 -6 16 -39 0 -39 0 10 -26z M2810 1145 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M2833 1133 l7 -6 7 6 6 7 -13 0 -13 0 6 -7z M2877 1133 l-7 -6 0 -13 0 -12 35 11 36 11 -13 8 -13 8 -16 0 -16 0 -6 -7z M3123 1013 l17 -5 13 5 12 5 -30 0 -30 0 18 -5z M5062 385 l-22 -25 0 -6 0 -5 -27 -37 -28 -37 -22 -22 -23 -22 0 -16 0 -15 9 0 9 0 4 9 3 9 60 68 60 67 29 29 29 28 -30 0 -29 0 -22 -25z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#1aa6a6",
                    d: "M5100 368 l-30 -33 -55 -60 -55 -60 0 -12 0 -13 8 0 7 0 60 66 60 66 28 30 27 31 0 8 0 9 -10 0 -11 0 -29 -32z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#2f6fd4",
                    d: "M2077 1800 l22 -9 11 -21 12 -22 16 -17 17 -18 46 -59 46 -59 13 -25 12 -25 47 -43 46 -42 43 -38 44 -37 26 -23 26 -23 34 -27 35 -27 13 -13 14 -13 11 7 12 7 -27 14 -27 14 3 17 3 17 -36 35 -36 35 -72 90 -72 90 -52 55 -52 55 -33 41 -32 42 0 10 0 10 -22 6 -23 6 -45 -1 -45 0 22 -9z M2620 1235 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M2690 1185 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M2710 1175 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M2735 1159 l6 -9 15 0 14 0 0 -14 0 -14 25 -16 24 -16 16 0 15 0 0 19 0 19 -27 11 -28 11 -33 9 -32 10 5 -10z M3280 1165 l0 -5 13 -20 12 -21 -15 -9 -15 -9 -100 -37 -100 -37 -52 -1 -53 -1 0 -7 0 -8 58 0 57 0 56 9 57 10 66 30 65 31 10 0 10 0 8 14 8 13 11 4 12 4 -9 23 -8 22 -46 0 -45 0 0 -5z M5120 350 l-25 -30 -31 -33 -30 -32 -33 -37 -33 -38 15 -6 16 -6 36 29 36 28 45 66 44 67 0 11 0 11 -7 0 -8 0 -25 -30z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#8756c9",
                    d: "M2030 1784 l25 -29 34 -30 35 -30 25 -29 26 -30 16 -15 16 -16 22 -18 23 -18 4 4 5 5 -51 63 -50 64 -20 25 -19 25 -18 23 -18 23 -40 6 -40 6 25 -29z M2260 1555 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M2384 1283 l16 -31 67 -57 68 -56 38 -20 38 -20 -12 19 -12 19 -50 34 -51 34 -25 23 -26 22 -34 33 -33 32 16 -32z M3236 1144 l6 -33 -28 -7 -29 -7 -44 -20 -45 -20 -60 -18 -61 -19 53 4 53 5 100 37 99 36 0 14 0 14 10 0 10 0 -12 19 -12 19 -23 5 -23 4 6 -33z M2620 1095 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M5121 300 l-44 -69 -39 -32 -40 -31 11 -11 11 -11 36 35 36 34 44 60 44 59 0 18 0 18 -7 0 -8 -1 -44 -69z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#7a4b17",
                    d: "M4910 2185 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M4853 2173 l7 -6 7 6 6 7 -13 0 -13 0 6 -7z M4820 2165 l0 -5 5 0 5 0 0 5 0 5 -5 0 -5 0 0 -5z M4778 2153 l12 -5 13 5 12 5 -25 0 -25 0 13 -5z M4660 2125 l-75 -23 -95 -34 -95 -35 -20 -11 -20 -11 -150 -74 -150 -75 -65 -39 -65 -38 -43 -30 -44 -30 -61 -44 -62 -45 -71 -53 -71 -53 -23 -25 -24 -25 11 -11 10 -10 83 63 82 63 52 36 51 37 73 48 74 49 41 26 42 26 70 41 70 40 50 28 50 27 60 32 60 31 110 43 110 43 40 17 40 18 25 11 25 11 -10 0 -10 0 -75 -24z M3380 1374 l-105 -85 -34 -27 -35 -27 -38 -39 -39 -38 4 -12 4 -11 14 -3 14 -3 57 51 56 50 7 0 6 0 52 48 52 47 58 57 59 57 -7 10 -6 11 -7 0 -7 0 -105 -86z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                g {
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                    path {
                        fill: "#ffffff",
                        d: "M3186 1545 l-16 -16 0 -24 0 -24 11 -25 12 -25 16 -6 16 -6 22 26 23 26 0 28 0 29 -12 10 -13 10 -21 6 -22 7 -16 -16z m64 -60 l0 -25 -15 0 -15 0 0 25 0 25 15 0 15 0 0 -25z M3015 1515 l-25 -24 0 -29 0 -28 28 -27 28 -27 17 0 16 0 21 22 20 22 0 36 0 37 -23 21 -23 22 -18 0 -17 0 -24 -25z m93 -58 l3 -27 -21 0 -21 0 -5 15 -6 15 6 16 6 15 17 -3 18 -3 3 -28z",
                    }
                    g {
                        transform: "translate(3120,1538) rotate(200)",
                        rect {
                            x: "-140",
                            y: "-132",
                            width: "360",
                            height: "98",
                            rx: "0",
                            fill: "#000000",
                            fill_opacity: "1",
                            animate {
                                attribute_name: "height",
                                values: "98;98;200;200;282;98",
                                key_times: "0;0.8333;0.9000;0.9583;0.9792;1",
                                calc_mode: "spline",
                                key_splines: "0 0 1 1;0.25 0 0.35 1;0 0 1 1;0.2 0 0.2 1;0.25 0 0.35 1",
                                dur: "10s",
                                repeat_count: "indefinite",
                            }
                        }
                    }
                }
                path {
                    fill: "#9a9b98",
                    d: "M4983 1830 l-1 -375 8 -120 9 -120 10 -409 10 -409 5 -4 5 -5 -5 343 -5 344 -11 105 -11 105 -7 460 -6 460 -1 -375z M5032 88 l-2 -46 -6 -19 -7 -18 12 11 12 11 -4 54 -3 54 -2 -47z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "#000",
                    d: "M4865 2190 l-40 -8 -80 -22 -80 -22 -30 -9 -30 -8 -100 -35 -100 -35 -185 -92 -185 -92 -40 -26 -40 -26 -26 -13 -26 -14 -25 -23 -25 -23 -32 -17 -33 -16 -61 -47 -62 -47 -5 -6 -5 -5 -45 -31 -45 -31 -19 -21 -19 -21 -24 0 -23 0 0 -18 0 -17 -20 -13 -20 -12 0 15 0 15 -15 0 -15 0 0 -27 0 -28 -52 -43 -51 -43 -3 4 -3 3 24 61 25 62 6 30 7 31 -17 0 -16 0 0 46 0 47 -28 -6 -28 -6 2 46 3 45 -33 -6 -33 -6 -17 39 -18 39 -31 -9 -32 -9 -23 30 -23 30 -8 0 -8 0 -23 -20 -23 -20 -5 0 -5 0 -44 26 -44 26 -14 -12 -14 -11 -3 -22 -3 -22 -17 -3 -18 -3 -32 15 -31 16 -9 0 -8 0 0 -45 0 -45 -30 0 -30 0 -10 25 -10 25 -8 0 -9 0 -76 65 -77 66 0 15 0 15 -15 5 -15 6 -24 -16 -24 -15 -7 -21 -6 -20 -198 0 -198 0 -44 15 -43 16 -7 11 -7 12 -17 -14 -16 -14 -15 -27 -14 -28 0 -26 0 -26 9 5 9 6 118 -116 119 -115 55 -54 55 -54 28 -32 27 -32 0 -16 0 -16 -16 -55 -16 -55 -8 -35 -9 -35 -10 -43 -11 -43 -9 -17 -9 -17 -12 -45 -13 -45 -14 -41 -15 -42 -41 -18 -41 -19 -13 3 -12 3 -6 -23 -5 -23 -3 -25 -2 -25 45 -3 44 -3 45 12 45 13 -14 4 -14 5 49 18 50 18 95 31 95 31 80 30 80 31 20 8 20 9 60 20 60 20 43 -42 42 -41 0 -6 0 -5 -27 -11 -28 -12 -45 -15 -45 -15 -77 -28 -78 -27 0 -19 0 -18 93 1 92 1 40 13 40 13 -30 1 -30 2 16 9 15 9 70 19 71 20 26 -16 27 -17 0 -7 0 -8 1 -7 1 -8 27 -17 26 -18 68 0 67 0 0 23 0 22 -60 50 -60 50 0 7 0 7 55 20 56 21 14 0 13 0 80 34 80 33 99 -86 99 -86 35 -27 34 -27 -19 -1 -18 0 12 -15 12 -15 33 0 32 0 -4 73 -5 72 -5 15 -5 15 -7 -18 -7 -18 -44 39 -45 39 -47 36 -48 36 53 16 52 17 20 13 20 12 27 6 27 7 5 18 5 17 6 46 7 46 -18 17 -17 17 -6 -9 -6 -10 -47 -7 -48 -7 0 -14 0 -14 -140 0 -140 0 0 3 0 4 60 46 60 47 29 0 30 0 20 26 21 27 0 12 0 13 19 5 19 5 19 36 18 35 114 86 113 86 79 50 79 49 44 33 44 32 53 30 54 30 75 43 75 42 65 34 65 34 167 66 166 67 74 18 73 19 28 6 27 6 0 10 0 10 -22 -1 -23 -2 -40 -7z m55 -5 l0 -5 -5 0 -5 0 0 5 0 5 5 0 5 0 0 -5z m-53 -12 l-7 -6 -7 6 -6 7 13 0 13 0 -6 -7z m-37 -8 l0 -5 -5 0 -5 0 0 5 0 5 5 0 5 0 0 -5z m-27 -12 l-13 -5 -12 5 -13 5 25 0 25 0 -12 -5z m-73 -15 l-25 -11 -40 -18 -40 -17 -110 -43 -110 -43 -60 -31 -60 -32 -50 -27 -50 -28 -70 -40 -70 -41 -42 -26 -41 -26 -74 -49 -73 -48 -51 -37 -52 -36 -82 -63 -83 -63 -10 10 -11 11 24 25 23 25 71 53 71 53 62 45 61 44 44 30 43 30 65 38 65 39 150 75 150 74 20 11 20 11 95 35 95 34 75 23 75 24 10 0 10 0 -25 -11z m-2240 -278 l0 -10 43 -40 42 -39 35 -30 35 -31 8 0 7 0 0 -20 0 -20 -11 0 -11 0 -83 85 -84 85 -12 0 -11 0 12 15 12 15 9 0 9 0 0 -10z m-365 -202 l185 -183 23 -18 22 -18 -28 6 -27 5 -8 -21 -8 -22 -55 48 -54 48 -16 16 -17 16 -126 124 -126 124 0 16 0 17 12 12 12 12 13 0 12 0 186 -182z m487 25 l36 -31 4 -34 3 -33 -37 -3 -38 -3 0 -10 0 -10 15 -29 15 -28 -35 -21 -35 -20 0 -15 0 -15 32 -25 33 -25 -18 -30 -17 -29 0 -11 0 -10 26 -14 27 -14 -12 -7 -11 -7 -14 13 -13 13 -35 27 -34 27 -26 23 -26 23 -29 24 -28 23 -94 87 -94 86 -31 34 -31 34 -29 26 -29 26 -39 40 -38 40 232 5 233 5 50 -50 50 -50 37 -32z m646 -145 l12 -10 0 -29 0 -28 -23 -26 -22 -26 -16 6 -16 6 -12 25 -11 25 0 24 0 24 16 16 16 16 22 -7 21 -6 13 -10z m-161 -20 l23 -21 0 -37 0 -36 -20 -22 -21 -22 -16 0 -17 0 -28 27 -28 27 0 28 0 29 25 24 24 25 17 0 18 0 23 -22z m408 -67 l5 -10 -30 -34 -30 -34 5 -7 5 -6 -6 5 -7 4 -31 -25 -31 -26 -47 -44 -47 -44 -6 0 -7 0 -56 -50 -57 -51 -14 3 -14 3 -4 11 -4 12 39 38 38 39 35 27 34 27 105 85 105 86 7 0 7 0 6 -9z m-1157 -39 l13 -9 -21 -89 -21 -89 -9 -35 -8 -35 -10 -30 -10 -30 -33 -137 -32 -138 -11 0 -11 0 -15 10 -14 9 13 53 13 53 10 35 11 35 14 32 13 31 0 13 0 13 9 30 10 31 15 55 16 55 10 60 11 60 4 13 4 12 8 0 8 0 13 -8z m209 -147 l72 -60 41 -30 41 -30 14 -13 15 -13 0 -15 0 -14 19 0 20 0 6 -10 6 -10 -16 0 -15 0 -19 -12 -19 -12 -64 55 -63 55 -85 67 -85 67 -29 29 -30 30 14 25 13 26 46 -37 46 -38 72 -60z m878 65 l-18 -20 -10 0 -10 0 23 20 23 20 5 0 5 0 -18 -20z m-977 -101 l17 -16 45 -30 45 -31 32 -31 33 -31 -5 0 -6 0 -34 14 -35 15 -37 28 -38 28 -36 35 -36 35 -18 35 -17 35 36 -35 37 -35 17 -16z m-3 -69 l40 -32 42 -24 41 -25 53 -18 53 -19 -52 -15 -51 -16 -33 -15 -33 -16 -97 -29 -97 -30 -30 -16 -31 -16 0 10 0 10 20 83 21 83 15 55 15 55 10 31 9 30 0 19 0 19 33 -47 33 -46 39 -31z m175 75 l0 -5 -5 0 -5 0 0 5 0 5 5 0 5 0 0 -5z m1020 -74 l0 -49 -27 -7 -28 -7 -13 -11 -14 -11 -51 -18 -51 -18 -6 0 -5 1 80 39 80 40 13 0 12 0 0 41 0 41 -42 -6 -43 -6 20 9 20 10 28 0 27 1 0 -49z m-950 24 l0 -5 -5 0 -5 0 0 5 0 5 5 0 5 0 0 -5z m20 -10 l0 -5 -5 0 -5 0 0 5 0 5 5 0 5 0 0 -5z m900 -17 l0 -18 -24 0 -25 0 -7 -11 -7 -12 -156 7 -156 8 -7 11 -8 12 0 10 0 10 195 0 195 0 0 -17z m-875 2 l6 -10 -5 0 -5 0 -6 10 -6 10 5 0 5 0 6 -10z m105 -46 l0 -24 -15 0 -16 0 -24 16 -25 16 0 18 0 18 40 -10 40 -10 0 -24z m125 21 l-50 -13 -28 -11 -28 -11 3 17 3 18 45 6 45 6 30 1 30 1 -50 -14z m95 -22 l0 -15 20 5 20 5 5 -8 5 -9 -72 -22 -73 -21 -40 -13 -40 -13 -55 -12 -55 -12 -20 -8 -20 -8 -50 -17 -50 -17 -100 -38 -100 -39 -90 -30 -90 -29 -90 -31 -90 -31 -17 0 -17 0 -6 9 -5 9 29 21 29 21 21 0 21 0 0 11 0 10 16 -15 15 -16 30 0 30 0 11 31 10 30 52 24 51 24 87 26 88 27 37 17 38 18 65 17 65 18 20 8 20 8 33 11 33 10 14 -5 15 -6 0 10 0 11 73 22 72 21 20 8 20 8 8 -5 7 -4 0 -16z m359 -12 l94 -6 -40 -22 -41 -23 -63 -10 -64 -10 -100 -15 -100 -15 -77 0 -77 0 -6 10 -6 9 28 7 28 6 65 20 64 20 30 15 30 16 4 -12 4 -11 9 0 9 0 0 15 0 15 58 -1 57 -1 94 -7z m51 -110 l35 -26 5 -5 5 -5 63 -54 62 -53 0 -44 0 -44 -51 36 -50 37 -27 26 -27 26 -74 65 -74 65 39 5 39 6 10 -5 10 -5 35 -25z m-140 10 l0 -9 -34 -16 -34 -16 -119 -39 -118 -40 -18 18 -18 18 91 30 92 31 -14 4 -13 5 30 0 30 1 50 10 50 10 13 1 12 1 0 -9z m-431 -19 l16 -19 30 -34 30 -34 45 -35 45 -34 39 -38 38 -38 -39 0 -39 0 -65 45 -64 46 -74 67 -74 67 44 11 44 12 4 1 4 1 16 -18z m181 -23 l-55 -21 -26 -5 -26 -5 -21 29 -20 28 101 -3 102 -3 -55 -20z m-190 -109 l25 -19 -5 -3 -5 -3 -100 -33 -100 -33 -65 1 -65 1 75 30 75 31 46 13 46 12 19 11 19 11 5 0 5 1 25 -20z M3227 1513 l-7 -6 0 -23 0 -23 9 -6 10 -6 10 20 11 20 -6 15 -6 16 -7 0 -8 0 -6 -7z M3064 1476 l-6 -16 6 -15 5 -15 21 0 21 0 -3 27 -3 28 -18 3 -17 3 -6 -15z M3590 930 l0 -10 16 0 15 0 -6 10 -6 10 -10 0 -9 0 0 -10z M5120 413 l15 -6 23 -24 22 -24 0 -13 0 -12 -44 -59 -44 -60 -35 -34 -36 -34 -36 27 -35 27 1 -18 0 -18 29 -23 30 -23 18 6 19 5 69 70 69 70 7 17 7 18 0 42 1 42 -16 15 -15 16 -32 -1 -32 -1 15 -5z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
                path {
                    fill: "currentColor",
                    d: "M4950 2190 l0 -10 15 0 15 0 0 10 0 10 -15 0 -15 0 0 -10z M1851 1764 l1 -19 5 13 5 13 -6 6 -6 6 1 -19z M2260 1370 l0 -13 7 6 6 7 -6 7 -7 6 0 -13z M2560 1320 l0 -13 7 6 6 7 -6 7 -7 6 0 -13z M3653 1243 l7 -6 7 6 6 7 -13 0 -13 0 6 -7z M1520 1029 l0 -31 -20 -5 -20 -5 0 -9 0 -9 20 0 20 0 0 -104 0 -104 -735 2 -735 1 0 -8 0 -7 -25 6 -25 6 0 -66 0 -66 224 0 224 0 5 -13 5 -12 1 11 1 11 220 -2 219 -3 3 56 3 55 11 4 11 4 5 -51 5 -50 7 -8 8 -7 149 -4 149 -3 0 59 0 60 16 0 16 0 -4 -59 -3 -58 20 0 20 0 225 -4 225 -4 72 -1 72 -1 4 -17 5 -18 1 16 1 16 593 0 592 0 8 -11 9 -11 5 12 5 13 422 -1 422 0 27 10 28 11 115 0 114 0 0 -131 0 -131 -14 5 -14 6 -119 1 -118 1 -200 -5 -200 -5 -130 -7 -130 -7 -455 1 -455 2 -106 11 -107 11 -173 -6 -174 -6 -159 -6 -158 -6 -7 8 -6 9 -135 -7 -135 -7 -632 -7 -633 -7 0 -10 0 -10 633 7 632 6 128 6 127 6 0 -176 0 -176 10 0 10 0 0 170 0 170 21 0 22 0 -5 -102 -5 -103 8 25 7 25 1 78 1 77 45 0 44 0 4 -77 3 -78 2 78 2 77 14 0 15 0 4 -43 4 -42 1 43 2 42 25 0 25 0 0 -170 0 -170 10 0 10 0 0 169 0 169 16 6 16 6 24 0 24 0 0 -175 0 -175 10 0 10 0 0 174 0 174 16 6 16 6 156 0 157 0 65 -10 65 -9 460 -2 460 -3 183 8 182 9 0 -177 0 -176 10 0 10 0 0 175 0 175 18 0 18 0 11 -37 11 -38 -5 38 -5 37 25 0 26 0 4 -47 3 -48 2 46 2 46 67 6 67 7 5 -6 6 -6 3 -174 4 -174 11 0 12 0 -2 57 -3 58 -2 123 -3 122 7 0 7 0 4 -43 4 -42 1 43 2 42 10 0 10 0 0 -157 0 -158 6 -22 6 -23 7 0 6 0 -4 181 -3 180 108 -4 109 -4 25 -7 25 -7 0 211 0 211 -132 -3 -132 -3 -6 35 -5 35 2 62 2 63 -9 7 -8 8 -4 -12 -5 -13 -22 0 -23 0 -13 25 -13 25 -9 0 -8 0 0 -10 0 -11 -22 12 -22 12 -77 -6 -76 -5 -22 -11 -21 -12 0 16 0 15 -10 0 -10 0 0 -15 0 -16 -22 -8 -23 -9 17 -7 17 -7 11 7 10 6 0 -20 0 -20 10 -6 10 -6 0 30 0 30 8 5 8 5 77 7 76 7 31 -13 31 -13 -5 -23 -6 -22 10 -4 9 -3 7 -70 6 -70 -121 3 -121 3 0 -15 0 -16 -32 0 -31 0 -18 16 -17 15 -188 -11 -188 -12 -4 11 -4 11 -9 0 -9 0 0 -20 0 -20 -19 0 -20 0 -4 -52 -3 -53 -2 53 -2 52 -42 0 -43 0 -30 20 -30 20 -76 0 -76 0 -32 -11 -31 -11 0 -59 0 -59 -10 0 -10 0 0 55 0 55 -95 0 -95 0 0 17 0 16 -178 -6 -179 -7 -43 -15 -43 -15 -49 0 -49 0 -5 14 -5 14 -60 -4 -59 -3 0 124 0 125 -10 0 -10 0 0 -32 0 -32 -12 12 -12 12 -23 0 -23 0 0 9 0 9 13 5 12 5 -12 13 -13 13 0 23 0 23 -10 0 -10 0 0 -19 0 -20 -120 1 -120 1 0 23 0 24 -10 0 -10 0 0 -31z m203 -39 l48 0 6 -16 6 -16 -14 -9 -14 -9 15 -10 15 -9 0 -91 0 -90 -15 5 -15 4 -60 7 -60 7 -22 -1 -23 -2 -2 93 -2 92 -3 -92 -4 -93 -19 0 -20 0 0 114 0 115 48 4 47 4 20 -3 20 -4 48 0z m130 -74 l17 -13 0 -82 0 -81 -33 0 -33 0 0 95 1 95 15 0 15 -1 18 -13z m2125 -8 l10 -13 4 -72 4 -73 -12 0 -12 0 -6 16 -6 16 -1 41 0 42 -9 20 -8 20 4 -65 3 -65 -9 45 -9 45 0 28 -1 27 19 0 19 0 10 -12z m-18 -148 l0 -10 -11 0 -10 0 6 10 6 10 5 0 4 0 0 -10z m-3762 -17 l-28 -4 -27 4 -28 4 55 0 55 0 -27 -4z m1462 -60 l0 -56 -4 4 -4 4 -10 53 -9 52 13 0 14 0 0 -57z m740 -3 l0 -60 -16 0 -16 0 6 23 6 22 0 38 0 37 10 0 10 0 0 -60z m1482 13 l-3 -48 -3 -15 -4 -15 -1 63 -1 62 8 0 7 0 -3 -47z m-370 -18 l3 -55 -12 0 -13 0 0 55 0 55 9 0 9 0 4 -55z m-3057 10 l0 -28 -12 -4 -13 -5 0 37 0 37 13 -4 12 -5 0 -28z m1513 -18 l3 -47 -20 0 -21 0 0 51 0 50 18 -3 17 -3 3 -48z m-845 -34 l-42 -4 -48 4 -48 3 90 0 90 1 -42 -4z M1613 835 l0 -60 4 33 4 32 -4 28 -4 27 0 -60z M3831 883 l-1 -73 8 0 7 0 -3 52 -3 53 -3 20 -3 20 -2 -72z M3862 905 l0 -40 8 -50 7 -50 1 35 0 35 -8 55 -7 55 -1 -40z M920 626 l0 -16 10 0 10 0 0 9 0 10 -10 6 -10 6 0 -15z M1600 530 l-35 -5 365 -3 365 -2 11 7 12 8 -342 0 -341 0 -35 -5z M3365 530 l-160 -6 140 -1 140 -1 140 7 140 8 -120 -1 -120 0 -160 -6z M315 519 l310 -6 400 7 400 6 -710 0 -710 -1 310 -6z M2683 523 l87 -4 88 4 87 3 -175 0 -175 0 88 -3z M3993 513 l67 -4 68 4 67 3 -135 0 -135 0 68 -3z M418 459 l-23 -6 175 -2 175 -2 25 -5 25 -5 -15 10 -16 10 -161 3 -162 3 -23 -6z M2433 463 l187 -3 188 3 187 2 -375 0 -375 0 188 -2z M1573 433 l117 -4 118 4 117 3 -235 0 -235 0 118 -3z M3358 433 l233 -3 227 3 227 2 -460 0 -460 0 233 -2z M4864 432 l-11 -7 -17 -32 -16 -32 0 -31 0 -31 14 -27 15 -27 40 -37 41 -38 10 0 10 0 0 13 0 14 -38 34 -39 34 -16 25 -16 25 11 42 10 42 30 10 30 11 34 -16 33 -16 12 -14 11 -14 14 0 14 0 22 25 22 25 38 6 38 6 -6 9 -5 9 -34 0 -33 0 -31 -26 -31 -26 -31 26 -31 26 -41 0 -42 0 -11 -8z M175 421 l-50 -6 130 0 130 -1 -50 7 -50 7 -30 0 -30 -1 -50 -6z M2162 413 l87 -4 93 4 93 3 -180 0 -180 0 87 -3z M3903 300 l0 -55 4 28 4 27 -4 28 -4 27 0 -55z M3843 295 l0 -50 4 28 4 27 -4 23 -4 22 0 -50z M1634 210 l0 -115 3 58 4 57 -4 58 -3 57 0 -115z",
                    transform: "translate(0.000000,402.000000) scale(0.100000,-0.100000)",
                }
            }
        }
    }
}

#[component]
pub fn FisherWaveIcon(#[props(extends = GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        svg {
            fill: "currentColor",
            width: "100%",
            view_box: "0 24 150 28",
            preserve_aspect_ratio: "none",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            g {
                g {
                    opacity: 1,
                    animateTransform {
                        attribute_name: "transform",
                        attribute_type: "XML",
                        r#type: "translate",
                        values: "-352,0;0,0",
                        dur: "6s",
                        repeat_count: "indefinite",
                    }
                    path { d: "M-160 44c30 0 58-18 88-18s58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z" }
                    path {
                        d: "M-160 44c30 0 58-18 88-18s58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z",
                        transform: "translate(352, 0)",
                    }
                }
                g {
                    opacity: 0.5,
                    animateTransform {
                        attribute_name: "transform",
                        attribute_type: "XML",
                        r#type: "translate",
                        values: "-352,3;0,3",
                        dur: "8s",
                        repeat_count: "indefinite",
                    }
                    path { d: "M-160 44c30 0 58-18 88-18s58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z" }
                    path {
                        d: "M-160 44c30 0 58-18 88-18s58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z",
                        transform: "translate(352, 0)",
                    }
                }
                g {
                    opacity: 0.3,
                    animateTransform {
                        attribute_name: "transform",
                        attribute_type: "XML",
                        r#type: "translate",
                        values: "-352,5;0,5",
                        dur: "10s",
                        repeat_count: "indefinite",
                    }
                    path { d: "M-160 44c30 0 58-18 88-18s58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z" }
                    path {
                        d: "M-160 44c30 0 58-18 88-18s58 18 88 18 58-18 88-18 58 18 88 18 v44h-352z",
                        transform: "translate(352, 0)",
                    }
                }
            }
        }
    }
}

#[component]
pub fn RustDEVIcon(
    #[props(default = 24.0)] width: f32,
    #[props(default = 24.0)] height: f32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            width: "{width}",
            height: "{height}",
            view_box: "0 0 128 128",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path {
                fill: "currentColor",
                d: "M62.96.242c-.232.135-1.203 1.528-2.16 3.097c-2.4 3.94-2.426 3.942-5.65.55c-2.098-2.208-2.605-2.612-3.28-2.607c-.44.002-.995.152-1.235.332s-.916 1.612-1.504 3.183c-1.346 3.6-1.41 3.715-2.156 3.86c-.46.086-1.343-.407-3.463-1.929c-1.565-1.125-3.1-2.045-3.411-2.045c-1.291 0-1.655.706-2.27 4.4c-.78 4.697-.754 4.681-4.988 2.758c-1.71-.776-3.33-1.41-3.603-1.41s-.792.293-1.15.652c-.652.652-.653.655-.475 4.246l.178 3.595l-.68.364c-.602.322-1.017.283-3.684-.348c-3.48-.822-4.216-.8-4.92.15l-.516.693l.692 2.964c.38 1.63.745 3.2.814 3.487c.067.287-.05.746-.26 1.02c-.348.448-.717.49-3.94.44c-5.452-.086-5.761.382-3.51 5.3c.718 1.56 1.305 2.98 1.305 3.15c0 .898-.717 1.224-3.794 1.727c-1.722.28-3.218.51-3.326.51c-.107 0-.43.235-.717.522c-.937.936-.671 1.816 1.453 4.814c2.646 3.735 2.642 3.75-1.73 5.421c-4.971 1.902-5.072 2.37-1.287 5.96c3.525 3.344 3.53 3.295-.461 5.804C.208 62.8.162 62.846.085 63.876c-.093 1.253-.071 1.275 3.538 3.48c3.57 2.18 3.57 2.246.067 5.56C-.078 76.48.038 77 5.013 78.877c4.347 1.64 4.353 1.66 1.702 5.394c-1.502 2.117-1.981 3-1.981 3.653c0 1.223.637 1.535 4.44 2.174c3.206.54 3.92.857 3.92 1.741c0 .182-.588 1.612-1.307 3.177c-2.236 4.87-1.981 5.275 3.31 5.275c4.93 0 4.799-.15 3.737 4.294c-.8 3.35-.813 3.992-.088 4.715c.554.556 1.6.494 4.87-.289c2.499-.596 2.937-.637 3.516-.328l.66.354l-.177 3.594c-.178 3.593-.177 3.595.475 4.248c.358.36.884.652 1.165.652s1.903-.63 3.604-1.404c4.22-1.916 4.194-1.932 4.973 2.75c.617 3.711.977 4.4 2.294 4.4c.327 0 1.83-.88 3.34-1.958c2.654-1.893 3.342-2.19 4.049-1.74c.182.115.89 1.67 1.572 3.455c1.003 2.625 1.37 3.31 1.929 3.576c1.062.51 1.72.1 4.218-2.62c3.016-3.286 3.14-3.27 5.602.72c2.72 4.406 3.424 4.396 6.212-.089c2.402-3.864 2.374-3.862 5.621-.47c2.157 2.25 2.616 2.61 3.343 2.61c.464 0 1.019-.175 1.23-.388c.214-.213.92-1.786 1.568-3.496c.649-1.71 1.321-3.2 1.495-3.31c.687-.436 1.398-.13 4.048 1.752c1.56 1.108 3.028 1.96 3.377 1.96c1.296 0 1.764-.92 2.302-4.535c.46-3.082.554-3.378 1.16-3.685c.596-.302.954-.2 3.75 1.07c1.701.77 3.323 1.402 3.604 1.402s.816-.302 1.184-.672l.672-.67l-.184-3.448c-.177-3.29-.16-3.468.364-3.943c.54-.488.596-.486 3.615.204c3.656.835 4.338.857 5.025.17c.671-.67.664-.818-.254-4.69c-1.03-4.346-1.168-4.19 3.78-4.19c3.374 0 3.75-.049 4.18-.523c.718-.793.547-1.702-.896-4.779c-.729-1.55-1.32-2.96-1.315-3.135c.024-.914.743-1.227 4.065-1.767c2.033-.329 3.553-.71 3.829-.96c.923-.833.584-1.918-1.523-4.873c-2.642-3.703-2.63-3.738 1.599-5.297c5.064-1.866 5.209-2.488 1.419-6.09c-3.51-3.335-3.512-3.317.333-5.677c4.648-2.853 4.655-3.496.082-6.335c-3.933-2.44-3.93-2.406-.405-5.753c3.78-3.593 3.678-4.063-1.295-5.965c-4.388-1.679-4.402-1.72-1.735-5.38c1.588-2.18 1.982-2.903 1.982-3.65c0-1.306-.586-1.598-4.436-2.22c-3.216-.52-3.924-.835-3.924-1.75c0-.174.588-1.574 1.307-3.113c1.406-3.013 1.604-4.22.808-4.94c-.428-.387-1-.443-4.067-.392c-3.208.054-3.618.008-4.063-.439c-.486-.488-.48-.557.278-3.725c.931-3.88.935-3.975.17-4.694c-.777-.73-1.262-.718-4.826.121c-2.597.612-3.027.653-3.617.337l-.67-.36l.185-3.582l.186-3.58l-.67-.67c-.369-.37-.891-.67-1.163-.67c-.27 0-1.884.64-3.583 1.421c-2.838 1.306-3.143 1.393-3.757 1.072c-.612-.32-.714-.637-1.237-3.829c-.603-3.693-.977-4.412-2.288-4.412c-.311 0-1.853.925-3.426 2.055c-2.584 1.856-2.93 2.032-3.574 1.807c-.533-.186-.843-.59-1.221-1.599c-.28-.742-.817-2.172-1.194-3.177c-.762-2.028-1.187-2.482-2.328-2.482c-.637 0-1.213.458-3.28 2.604c-3.25 3.375-3.261 3.374-5.65-.545C66.073 1.78 65.075.382 64.81.24c-.597-.32-1.3-.32-1.85.002m2.96 11.798c2.83 2.014 1.326 6.75-2.144 6.75c-3.368 0-5.064-4.057-2.66-6.36c1.358-1.3 3.304-1.459 4.805-.39m-3.558 12.507c1.855.705 2.616.282 6.852-3.8l3.182-3.07l1.347.18c4.225.56 12.627 4.25 17.455 7.666c4.436 3.14 10.332 9.534 12.845 13.93l.537.942l-2.38 5.364c-1.31 2.95-2.382 5.673-2.382 6.053c0 .878.576 2.267 1.13 2.726c.234.195 2.457 1.265 4.939 2.378l4.51 2.025l.178 1.148c.23 1.495.26 5.167.052 6.21l-.163.816h-2.575c-2.987 0-2.756-.267-2.918 3.396c-.118 2.656-.76 4.124-2.22 5.075c-2.377 1.551-6.304 1.27-7.97-.57c-.255-.284-.752-1.705-1.105-3.16c-1.03-4.254-2.413-6.64-5.193-8.965c-.878-.733-1.595-1.418-1.595-1.522c0-.102.965-.915 2.145-1.803c4.298-3.24 6.77-7.012 7.04-10.747c.519-7.126-5.158-13.767-13.602-15.92c-2.002-.51-2.857-.526-27.624-.526c-14.057 0-25.56-.092-25.56-.204c0-.263 3.125-3.295 4.965-4.816c5.054-4.178 11.618-7.465 18.417-9.22l2.35-.61l3.34 3.387c1.839 1.863 3.64 3.5 4.003 3.637M20.3 46.34c1.539 1.008 2.17 3.54 1.26 5.062c-1.405 2.356-4.966 2.455-6.373.178c-2.046-3.309 1.895-7.349 5.113-5.24m90.672.13c4.026 2.454.906 8.493-3.404 6.586c-2.877-1.273-2.97-5.206-.155-6.64c1.174-.6 2.523-.579 3.56.053M32.163 61.5v15.02h-13.28l-.526-2.285c-1.036-4.5-1.472-9.156-1.211-12.969l.182-2.679l4.565-2.047c2.864-1.283 4.706-2.262 4.943-2.625c1.038-1.584.94-2.715-.518-5.933l-.68-1.502h6.523V61.5M70.39 47.132c2.843.74 4.345 2.245 4.349 4.355c.002 1.55-.765 2.52-2.67 3.38c-1.348.61-1.562.625-10.063.708l-8.686.084v-8.92h7.782c6.078 0 8.112.086 9.288.393m-2.934 21.554c1.41.392 3.076 1.616 3.93 2.888c.898 1.337 1.423 3.076 2.667 8.836c1.05 4.87 1.727 6.46 3.62 8.532c2.345 2.566 1.8 2.466 13.514 2.466c5.61 0 10.198.09 10.198.2c0 .197-3.863 4.764-4.03 4.764c-.048 0-2.066-.422-4.484-.939c-6.829-1.458-7.075-1.287-8.642 6.032l-1.008 4.702l-.91.448c-1.518.75-6.453 2.292-9.01 2.82c-4.228.87-8.828 1.162-12.871.821c-6.893-.585-16.02-3.259-16.377-4.8c-.075-.327-.535-2.443-1.018-4.704c-.485-2.26-1.074-4.404-1.31-4.764c-1.13-1.724-2.318-1.83-7.547-.674c-1.98.44-3.708.796-3.84.796c-.248 0-3.923-4.249-3.923-4.535c0-.09 8.728-.194 19.396-.23l19.395-.066l.07-6.89c.05-4.865-.018-6.997-.23-7.25c-.234-.284-1.485-.358-6.011-.358H53.32v-8.36l6.597.001c3.626.002 7.02.12 7.539.264M37.57 100.02c3.084 1.88 1.605 6.804-2.043 6.8c-3.74 0-5.127-4.88-1.94-6.826c1.055-.643 2.908-.63 3.983.026m56.48.206c1.512 1.108 2.015 3.413 1.079 4.95c-2.46 4.034-8.612.827-6.557-3.419c1.01-2.085 3.695-2.837 5.478-1.53",
            }
        }
    }
}

#[component]
pub fn LogOutIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            class: "lucide lucide-log-out-icon lucide-log-out",
            fill: "none",
            height: "{height}",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "2",
            view_box: "0 0 24 24",
            width: "{width}",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "m16 17 5-5-5-5" }
            path { d: "M21 12H9" }
            path { d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" }
        }
    }
}

#[component]
pub fn RustGearIcon(
    #[props(default = 24.0)] width: f32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            width: "{width}",
            height: "{width}",
            preserve_aspect_ratio: "xMidYMid meet",
            version: "1.0",
            view_box: "0 0 1920.000000 1920.000000",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            g {
                fill: "currentColor",
                stroke: "none",
                transform: "translate(0.000000,1920.000000) scale(0.100000,-0.100000)",
                path {
                    d: "M9504 19097 c-42 -16 -94 -54 -120 -86 -11 -14 -125 -201 -254 -416\n-128 -214 -238 -396 -245 -403 -7 -8 -61 -18 -132 -25 l-121 -12 -317 350\nc-174 192 -337 364 -361 382 -129 92 -325 47 -396 -91 -11 -21 -91 -237 -177\n-479 l-158 -440 -122 -37 -121 -37 -378 279 c-207 154 -395 289 -416 300 -49\n25 -138 34 -193 19 -66 -18 -133 -70 -166 -129 -27 -48 -34 -88 -101 -535\nl-72 -482 -104 -57 c-58 -32 -111 -57 -119 -58 -8 0 -206 90 -439 200 -233\n110 -445 205 -471 211 -82 18 -179 -9 -244 -70 -38 -34 -76 -111 -82 -164 -3\n-23 5 -253 18 -512 l24 -470 -26 -20 c-14 -11 -57 -46 -95 -77 -37 -32 -75\n-58 -82 -58 -7 0 -221 52 -474 115 -254 63 -476 115 -494 115 -144 0 -276\n-132 -276 -276 0 -18 52 -240 115 -494 63 -253 115 -467 115 -474 0 -7 -26\n-45 -57 -82 -32 -38 -67 -81 -78 -95 l-21 -26 -474 24 c-261 13 -492 21 -513\n18 -50 -7 -125 -46 -158 -82 -61 -65 -88 -162 -70 -245 6 -25 101 -237 211\n-470 110 -232 200 -430 199 -438 0 -8 -26 -62 -57 -120 l-57 -104 -482 -72\nc-447 -67 -487 -74 -535 -101 -59 -33 -111 -100 -129 -166 -16 -59 -6 -148 23\n-199 13 -24 148 -210 300 -415 l275 -372 -37 -122 -37 -122 -440 -158 c-242\n-86 -458 -166 -480 -177 -137 -70 -182 -268 -90 -396 18 -24 190 -187 382\n-361 l350 -316 -12 -121 c-7 -72 -17 -126 -25 -133 -7 -7 -192 -119 -410 -250\n-218 -130 -413 -253 -432 -272 -56 -57 -71 -96 -71 -193 0 -97 15 -136 71\n-193 19 -19 214 -142 432 -272 218 -131 403 -243 410 -250 8 -7 18 -61 25\n-133 l12 -121 -350 -316 c-192 -174 -364 -337 -382 -361 -92 -128 -47 -326 90\n-396 22 -11 238 -91 480 -177 l440 -158 37 -122 37 -122 -275 -372 c-152 -205\n-287 -391 -300 -415 -29 -51 -39 -140 -23 -199 18 -66 70 -133 129 -166 48\n-27 88 -34 535 -101 l482 -72 57 -104 c32 -58 57 -112 58 -120 0 -8 -90 -206\n-200 -438 -110 -233 -205 -445 -211 -470 -18 -83 9 -180 70 -245 33 -36 108\n-75 158 -82 21 -3 252 5 513 18 l474 24 21 -26 c11 -14 46 -57 78 -95 31 -37\n57 -75 57 -82 0 -7 -52 -221 -115 -474 -63 -254 -115 -476 -115 -494 0 -144\n132 -276 276 -276 18 0 241 52 496 116 254 63 470 113 479 111 9 -3 57 -38\n107 -78 l90 -74 -4 -60 c-16 -232 -44 -919 -38 -942 10 -43 43 -105 73 -136\n58 -64 167 -97 252 -78 26 6 238 101 471 211 233 110 431 200 439 200 8 -1 61\n-26 119 -58 l104 -57 72 -482 c67 -447 74 -487 101 -535 33 -59 100 -111 166\n-129 55 -15 144 -6 193 19 21 11 209 146 416 300 l377 279 122 -36 121 -37\n158 -441 c87 -242 167 -458 178 -479 71 -138 267 -183 396 -91 24 18 187 190\n361 382 l317 350 121 -12 c71 -7 125 -17 132 -25 7 -7 119 -192 250 -410 130\n-218 253 -413 272 -432 57 -56 96 -71 193 -71 95 0 136 15 192 69 18 18 140\n212 272 432 131 219 244 405 251 412 7 8 61 18 133 25 l121 12 316 -350 c174\n-192 337 -364 361 -382 129 -93 325 -47 396 92 12 22 92 237 178 479 l157 439\n122 37 122 37 377 -279 c207 -154 395 -289 416 -300 49 -25 138 -34 193 -19\n66 18 133 70 166 129 27 48 34 88 101 535 l72 482 104 57 c58 31 112 57 121\n57 9 1 128 -52 265 -118 624 -297 621 -295 699 -295 56 0 81 6 121 27 66 34\n117 93 138 161 16 50 16 78 -3 482 -11 236 -22 456 -25 489 l-4 60 90 74 c50\n40 98 75 107 78 9 2 225 -48 479 -111 255 -64 478 -116 496 -116 144 0 276\n132 276 276 0 18 -52 240 -115 494 -63 253 -115 467 -115 474 0 8 27 46 60 86\n33 39 68 82 78 94 l18 23 474 -24 c261 -13 492 -21 513 -18 50 7 125 46 158\n82 61 65 88 162 70 245 -6 25 -101 237 -211 470 -110 232 -200 430 -200 438 1\n8 26 62 58 120 l57 104 482 72 c447 66 486 74 535 101 59 33 111 100 129 166\n15 55 6 144 -19 193 -11 21 -146 209 -300 416 l-279 378 37 121 37 122 440\n157 c241 87 454 165 473 175 144 72 191 268 97 399 -18 24 -190 187 -382 361\nl-350 316 12 121 c7 72 17 126 25 133 7 7 192 119 410 250 218 130 413 253\n432 272 56 57 71 96 71 193 0 97 -15 136 -71 193 -19 19 -214 142 -432 272\n-218 131 -403 243 -410 250 -8 7 -18 61 -25 133 l-12 121 350 316 c192 174\n364 337 382 361 94 131 47 327 -97 399 -19 10 -232 88 -473 175 l-440 157 -37\n122 -37 121 279 378 c154 207 289 395 300 416 25 49 34 138 19 193 -18 66 -70\n133 -129 166 -49 27 -88 35 -535 101 l-482 72 -57 104 c-32 58 -57 112 -58\n120 0 8 90 206 200 438 110 233 205 445 211 470 18 83 -9 180 -70 245 -33 36\n-108 75 -158 82 -21 3 -252 -5 -513 -18 l-474 -24 -21 26 c-11 14 -46 57 -77\n95 -32 37 -58 75 -58 82 0 7 52 221 115 474 63 254 115 476 115 494 0 144\n-132 276 -276 276 -18 0 -240 -52 -494 -115 -253 -63 -467 -115 -474 -115 -7\n0 -45 26 -82 58 -38 31 -81 66 -95 77 l-26 20 24 489 c23 462 23 492 7 542\n-21 68 -72 128 -138 161 -40 21 -65 27 -121 27 -78 0 -75 2 -699 -295 -137\n-66 -256 -119 -265 -118 -9 0 -63 26 -121 57 l-104 57 -72 482 c-67 447 -74\n487 -101 535 -33 59 -100 111 -166 129 -55 15 -144 6 -193 -19 -21 -11 -209\n-146 -416 -300 l-377 -279 -122 37 -122 37 -157 439 c-86 242 -166 457 -178\n479 -71 139 -267 185 -396 92 -24 -18 -187 -190 -361 -382 l-316 -350 -121 12\nc-72 7 -126 17 -133 25 -7 7 -120 193 -251 412 -132 220 -254 414 -272 432\n-18 18 -50 40 -70 50 -45 22 -174 29 -218 11z m231 -1732 c91 -24 174 -73 245\n-145 76 -76 124 -163 145 -265 18 -88 18 -132 0 -218 -42 -198 -197 -361 -391\n-411 -294 -76 -596 113 -659 411 -18 86 -18 130 0 218 41 193 197 358 386 409\n72 19 201 20 274 1z m-935 -1399 c434 -433 492 -487 550 -515 71 -34 180 -61\n250 -61 70 0 179 27 250 61 58 28 116 82 550 515 l486 484 119 -24 c980 -200\n1917 -618 2725 -1214 564 -417 1065 -918 1485 -1486 174 -236 495 -745 495\n-786 0 -7 -133 -274 -296 -593 -310 -610 -334 -665 -334 -787 0 -73 32 -197\n67 -263 34 -61 111 -148 167 -186 23 -15 300 -160 616 -321 316 -161 576 -294\n578 -296 2 -1 8 -35 13 -76 61 -484 59 -1180 -6 -1685 -165 -1284 -667 -2474\n-1470 -3478 -113 -142 -311 -370 -359 -415 l-27 -24 -652 102 c-359 57 -670\n103 -692 102 -168 -2 -345 -99 -443 -242 -64 -94 -75 -140 -183 -825 l-103\n-652 -146 -65 c-1808 -805 -3864 -806 -5675 -2 l-151 67 -103 652 c-112 706\n-118 733 -195 841 -94 132 -269 224 -431 226 -22 1 -333 -45 -692 -102 l-652\n-102 -27 24 c-48 45 -246 273 -359 415 -804 1006 -1303 2188 -1470 3478 -65\n502 -67 1201 -6 1685 5 41 11 75 13 76 2 2 161 83 353 181 851 432 833 422\n910 500 39 38 83 93 98 122 35 66 67 191 67 263 0 122 -24 177 -334 787 -163\n319 -296 586 -296 593 0 39 317 544 480 765 801 1088 1859 1902 3105 2390 307\n121 694 239 995 304 80 17 163 35 185 40 22 5 44 9 50 10 5 1 228 -217 495\n-483z m-5985 -3601 c108 -23 190 -68 271 -149 218 -218 215 -547 -7 -761 -85\n-83 -181 -130 -297 -147 -286 -42 -560 165 -604 455 -41 272 148 542 422 601\n85 19 130 19 215 1z m13780 0 c279 -59 469 -327 427 -602 -35 -229 -220 -417\n-450 -455 -250 -42 -506 116 -592 366 -100 292 91 623 400 690 85 19 130 19\n215 1z m-11090 -8116 c129 -40 259 -153 319 -279 149 -310 -28 -675 -366 -755\n-183 -43 -370 15 -503 156 -106 113 -149 222 -148 374 1 141 44 250 143 358\n141 154 350 209 555 146z m8528 -4 c88 -30 155 -74 217 -142 99 -108 142 -217\n143 -358 2 -158 -56 -290 -175 -402 -297 -278 -788 -123 -883 279 -51 216 44\n442 238 569 136 89 303 108 460 54z",
                }
            }
        }
    }
}

#[component]
pub fn LetterEnglishAIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            fill: "currentColor",
            width: "{width}",
            height: "{height}",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M18.97 19.757L15.35 5.272A2.996 2.996 0 0 0 12.437 3h-.877a2.996 2.996 0 0 0-2.91 2.272L5.03 19.757a1 1 0 0 0 1.94.486L8.28 15h7.44l1.31 5.243a1 1 0 0 0 1.94-.486M8.78 13l1.811-7.242a1 1 0 0 1 .97-.758h.878a1 1 0 0 1 .97.758L15.219 13Z" }
        }
    }
}

#[component]
pub fn LetterChineseAIcon(
    #[props(default = 24)] width: u32,
    #[props(default = 24)] height: u32,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        svg {
            fill: "currentColor",
            width: "{width}",
            height: "{height}",
            view_box: "0 0 24 24",
            xmlns: "http://www.w3.org/2000/svg",
            ..attributes,
            path { d: "M20 5h-7V4a1 1 0 0 0-2 0v1H4a1 1 0 0 0 0 2h11.882a14.5 14.5 0 0 1-3.94 7.952A14.4 14.4 0 0 1 8.663 9.67a1 1 0 0 0-1.889.66a16.4 16.4 0 0 0 3.68 5.958a14.3 14.3 0 0 1-5.768 2.735A1 1 0 0 0 4.899 21a1 1 0 0 0 .215-.023a16.3 16.3 0 0 0 6.831-3.319a16.4 16.4 0 0 0 6.842 3.319a1 1 0 0 0 .426-1.954a14.4 14.4 0 0 1-5.79-2.733A16.48 16.48 0 0 0 17.893 7H20a1 1 0 0 0 0-2" }
        }
    }
}
