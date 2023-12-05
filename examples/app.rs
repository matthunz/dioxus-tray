use dioxus::prelude::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello World!" }
    })
}

fn main() {
    dioxus_tray::launch(app, load_icon());
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
