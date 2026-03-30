use dioxus::prelude::*;

use crate::components::common::{
    GradientDirection, GridBackground, GridLineType, GridPadding, GridPattern, GridType,
    GridWrapper,
};
use crate::components::skeleton::Skeleton;

#[component]
pub(crate) fn MetaSectionSkeleton() -> Element {
    rsx! {
        section { class: "relative min-h-80 overflow-hidden",
            div { class: "relative z-10 flex flex-col gap-6 md:gap-10",
                div { class: "grid grid-cols-1 lg:grid-cols-12",
                    GridWrapper {
                        class: "lg:col-span-9".to_string(),
                        padding: GridPadding::Sm,
                        is_dot_on: false,
                        div { class: "flex min-w-0 flex-col gap-5 md:gap-8",
                            div { class: "flex flex-col items-start gap-5 md:flex-row md:gap-8",
                                div { class: "relative h-24 w-24 shrink-0 self-center md:h-40 md:w-40 md:self-auto",
                                    div { class: "absolute left-2 top-2 h-full w-full border-2 border-primary-6 bg-screentone" }
                                    Skeleton { class: "skeleton relative z-10 h-24 w-24 border-4 border-primary-6 md:h-40 md:w-40" }
                                }
                                div { class: "flex min-w-0 w-full flex-1 flex-col items-center gap-4 text-center md:items-start md:text-left",
                                    div { class: "flex flex-col items-center md:items-start",
                                        Skeleton { class: "skeleton h-12 w-40 border border-primary-6 md:h-20 md:w-72" }
                                        Skeleton { class: "skeleton mt-1.5 h-3 w-36 border border-primary-6 md:mt-2 md:h-4 md:w-48" }
                                    }
                                    Skeleton { class: "skeleton h-6 w-full max-w-3xl border border-primary-6" }
                                }
                            }

                            Skeleton { class: "skeleton h-3 w-44 self-center border border-primary-6 md:w-56 md:self-start" }

                            GridWrapper {
                                grid_type: GridType::Inner,
                                line_type: GridLineType::Dashed,
                                is_dot_on: false,
                                padding: GridPadding::None,
                                background: GridBackground {
                                    pattern: GridPattern::Slash,
                                    gradient: GradientDirection::None,
                                },
                                div { class: "grid grid-cols-2 gap-2 px-2 py-2 md:grid-cols-4",
                                    Skeleton { class: "skeleton h-12 border border-primary-6 md:h-14" }
                                    Skeleton { class: "skeleton h-12 border border-primary-6 md:h-14" }
                                    Skeleton { class: "skeleton h-12 border border-primary-6 md:h-14" }
                                    Skeleton { class: "skeleton h-12 border border-primary-6 md:h-14" }
                                }
                            }
                        }
                    }

                    GridWrapper {
                        class: "lg:col-span-3".to_string(),
                        padding: GridPadding::Sm,
                        line_type: GridLineType::None,
                        div { class: "flex flex-col gap-4 md:gap-6",
                            Skeleton { class: "skeleton h-4 w-28 border border-primary-6" }

                            div { class: "flex flex-col gap-3 md:gap-5",
                                Skeleton { class: "skeleton h-[44px] w-[calc(100%-8px)] rounded-full border-2 border-primary-6 md:h-[58px] md:w-[calc(100%-10px)] md:border-4" }
                                Skeleton { class: "skeleton h-[44px] w-[calc(100%-8px)] rounded-full border-2 border-primary-6 md:h-[58px] md:w-[calc(100%-10px)] md:border-4" }
                            }

                            div { class: "flex flex-wrap gap-1.5 pt-1 md:gap-2",
                                Skeleton { class: "skeleton h-6 w-16 border border-primary-6 md:h-7 md:w-20" }
                                Skeleton { class: "skeleton h-6 w-16 border border-primary-6 md:h-7 md:w-20" }
                                Skeleton { class: "skeleton h-6 w-20 border border-primary-6 md:h-7 md:w-24" }
                                Skeleton { class: "skeleton h-6 w-14 border border-primary-6 md:h-7 md:w-16" }
                            }
                        }
                    }
                }
            }
        }
    }
}
