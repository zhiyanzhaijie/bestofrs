use dioxus::prelude::*;

use crate::IO::repos::get_repo_readme;
use crate::components::common::CommonMarkdown;

#[component]
pub fn ReadmeSection(owner: String, name: String, refresh_tick: Signal<u32>) -> Element {
    let readme_fut = use_server_future({
        let owner = owner.clone();
        let name = name.clone();
        move || {
            let _ = refresh_tick();
            get_repo_readme(owner.clone(), name.clone())
        }
    })?;

    rsx! {
        section { class: "space-y-4 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                h2 { class: "text-lg font-semibold", "README" }
                p { class: "text-sm text-secondary-5", "Rendered from GitHub README" }
            }

            match readme_fut() {
                Some(Ok(Some(readme))) => rsx! {
                    div { class: "rounded-md border border-primary-6 bg-primary-1 p-4",
                        CommonMarkdown {
                            src: readme.content,
                            link_base_url: readme.html_url,
                            image_base_url: readme.download_url,
                        }
                    }
                },
                Some(Ok(None)) => rsx! { div { class: "text-sm text-secondary-5", "README not found" } },
                Some(Err(e)) => Err(e)?,
                None => rsx! { div { class: "text-sm text-secondary-5", "Loading README..." } },
            }
        }
    }
}
