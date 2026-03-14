use crate::components::skeleton::Skeleton;
use dioxus::prelude::*;

#[component]
pub(crate) fn MetaSectionSkeleton() -> Element {
    rsx! {
        section { class: "relative min-h-80 overflow-hidden",
            div { class: "relative z-10 flex flex-col gap-10",
                div { class: "grid grid-cols-1 gap-10 lg:grid-cols-12",
                    div { class: "lg:col-span-8 flex min-w-0 flex-col gap-8",
                        div { class: "flex flex-col items-start gap-8 md:flex-row",
                            Skeleton { class: "h-32 w-32 shrink-0 border-2 border-primary-6 md:h-40 md:w-40" }
                            div { class: "flex min-w-0 flex-1 flex-col gap-4",
                                Skeleton { class: "h-16 w-56 border border-primary-6 md:h-20 md:w-72" }
                                Skeleton { class: "h-4 w-48 border border-primary-6" }
                                Skeleton { class: "h-6 w-full max-w-2xl border border-primary-6" }
                            }
                        }
                        Skeleton { class: "h-3 w-56 border border-primary-6" }
                        div { class: "grid grid-cols-2 gap-4 sm:grid-cols-4",
                            Skeleton { class: "h-20 border border-primary-6" }
                            Skeleton { class: "h-20 border border-primary-6" }
                            Skeleton { class: "h-20 border border-primary-6" }
                            Skeleton { class: "h-20 border border-primary-6" }
                        }
                    }
                    div { class: "lg:col-span-4 flex flex-col gap-3 lg:border-l-2 lg:border-dashed lg:border-primary-6 lg:pl-8",
                        Skeleton { class: "h-4 w-28 border border-primary-6" }
                        Skeleton { class: "h-11 w-full border border-primary-6" }
                        Skeleton { class: "h-11 w-full border border-primary-6" }
                        div { class: "flex flex-wrap gap-2 pt-1",
                            Skeleton { class: "h-7 w-20 border border-primary-6" }
                            Skeleton { class: "h-7 w-20 border border-primary-6" }
                            Skeleton { class: "h-7 w-24 border border-primary-6" }
                        }
                    }
                }
            }
        }
    }
}
