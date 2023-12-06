use dioxus::prelude::*;
use dioxus_signals::use_signal;
use dioxus_tray::{MenuItem, TrayIcon, IconMenuItem, Seperator};
use tray_icon::menu::{ NativeIcon};

fn app(cx: Scope) -> Element {
    let icon = use_signal(cx, || load_icon());
    cx.render(rsx! {
        TrayIcon {
            icon: icon,
            IconMenuItem {
                text: "Icon",
                native_icon: NativeIcon::StatusAvailable,
                enabled: true,
            }
            MenuItem {
                text: "Open",
                enabled: true,
                accelerator: "CMD+O".parse().unwrap()
            }
            MenuItem {
                text: "Save",
                accelerator: "CMD+S".parse().unwrap()
            }
            MenuItem {
                text: "Quit",
                enabled: true,
                accelerator: "CMD+Q".parse().unwrap()
            }
        }
    })
}

fn main() {
    dioxus_desktop::launch(app);
}

fn load_icon() -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
