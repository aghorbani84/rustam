use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(default = ButtonVariant::Primary)]
    pub variant: ButtonVariant,

    #[props(default = ButtonSize::Md)]
    pub size: ButtonSize,

    #[props(default = false)]
    pub loading: bool,

    #[props(default = false)]
    pub disabled: bool,

    pub onclick: EventHandler<MouseEvent>,
    pub children: Element,
}

#[derive(Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
}

#[derive(Clone, PartialEq, Default)]
#[allow(dead_code)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_classes = "inline-flex items-center justify-center font-medium rounded-lg transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 dark:focus:ring-offset-gray-900 disabled:opacity-50 disabled:cursor-not-allowed";

    let variant_classes = match props.variant {
        ButtonVariant::Primary => "bg-brand-600 text-white hover:bg-brand-700 focus:ring-brand-500 shadow-sm",
        ButtonVariant::Secondary => "bg-gray-100 text-gray-900 hover:bg-gray-200 dark:bg-gray-800 dark:text-gray-100 dark:hover:bg-gray-700",
        ButtonVariant::Ghost => "text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-800",
        ButtonVariant::Danger => "bg-red-600 text-white hover:bg-red-700 focus:ring-red-500",
    };

    let size_classes = match props.size {
        ButtonSize::Sm => "px-3 py-1.5 text-sm",
        ButtonSize::Md => "px-4 py-2 text-base",
        ButtonSize::Lg => "px-6 py-3 text-lg",
    };

    let is_disabled = props.disabled || props.loading;

    rsx! {
        button {
            class: "{base_classes} {variant_classes} {size_classes}",
            disabled: is_disabled,
            onclick: move |evt| {
                if !is_disabled {
                    props.onclick.call(evt);
                }
            },

            if props.loading {
                svg {
                    class: "animate-spin -ml-1 mr-2 h-4 w-4",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    circle {
                        class: "opacity-25",
                        cx: "12", cy: "12", r: "10",
                        stroke: "currentColor",
                        stroke_width: "4"
                    }
                    path {
                        class: "opacity-75",
                        fill: "currentColor",
                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    }
                }
            }

            {props.children}
        }
    }
}
