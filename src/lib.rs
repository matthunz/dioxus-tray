use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi::PhysicalPosition, use_window, Config, PhysicalSize, WindowBuilder};
use dioxus_signals::use_signal;
use futures::channel::mpsc;
use futures::StreamExt;
use std::thread;
use tray_icon::{menu::Menu, Icon, TrayIconBuilder, TrayIconEvent};

pub fn launch(app: Component, icon: Icon) {
    let config = Config::new().with_window(
        WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(400., 400.))
            .with_decorations(false)
            .with_visible(false)
            .with_always_on_top(true),
    );
    dioxus_desktop::launch_with_props(root, RootProps { app, icon: icon }, config);
}

struct RootProps {
    app: Component,
    icon: Icon,
}

fn root(cx: Scope<RootProps>) -> Element {
    let RootProps { app, icon } = cx.props;

    let window = use_window(cx);

    let channel = use_signal(cx, || {
        let (tx, rx) = mpsc::unbounded();
        (tx, RefCell::new(Some(rx)))
    });
    let (tx, rx) = &*channel();

    to_owned![window];
    use_future(cx, (), move |_| {
        let mut rx = rx.borrow_mut().take().unwrap();
        async move {
            while let Some(x) = rx.next().await {
                let size = window.outer_size();
                window.set_visible(!window.is_visible());
                window.set_outer_position(PhysicalPosition::new(x - size.width as f64 / 2., 0.));
            }
        }
    });

    to_owned![tx];
    let _ = use_signal(cx, || {
        let menu = Menu::new();
        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("system-tray - tray icon library!")
            .with_menu(Box::new(menu))
            .with_icon(icon.clone())
            .build()
            .unwrap();

        let tray_channel = TrayIconEvent::receiver();
        thread::spawn(move || loop {
            if let Ok(event) = tray_channel.try_recv() {
                tx.unbounded_send(
                    event.icon_rect.left + (event.icon_rect.right - event.icon_rect.left) / 2.,
                )
                .unwrap();
            }
        });

        tray_icon
    });

    #[allow(non_snake_case)]
    let App = *app;
    render! { App {} }
}
