use crate::components::icons::DiscordIcon;
use dioxus::prelude::*;

const BESTOFRS_DISCORD_INVITE_URL: &str = "https://discord.gg/VhZgJYQRDn";

#[component]
pub fn DiscordLink() -> Element {
    rsx! {
        Link {
            class: "inline-flex h-[1.6rem] w-[1.6rem] items-center justify-center rounded-full p-0 text-secondary-5 shadow-none transition-colors hover:text-secondary-4 hover:cursor-pointer",
            to: BESTOFRS_DISCORD_INVITE_URL,
            new_tab: true,
            rel: "noopener noreferrer",
            aria_label: "Bestofrs Discord",
            title: "Bestofrs Discord",
            DiscordIcon { width: 22, height: 22 }
        }
    }
}
