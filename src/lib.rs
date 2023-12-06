use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_signals::{use_effect, Signal};
use slotmap::{DefaultKey, SlotMap};
use tray_icon::{
    menu::{accelerator::Accelerator, IsMenuItem, Menu, MenuItemBuilder, NativeIcon, IconMenuItemBuilder, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

#[derive(Clone)]
struct TrayIconContext {
    tray_icon: TrayIcon,
    items: Signal<SlotMap<DefaultKey, Box<dyn IsMenuItem>>>,
}

#[component]
pub fn TrayIcon<'a>(cx: Scope<'a>, icon: Signal<Icon>, children: Element<'a>) -> Element<'a> {
    let tray_icon_cx = use_context_provider(cx, || {
        let tray_icon = TrayIconBuilder::new()
            .with_icon(icon().clone())
            .with_menu(Box::new(Menu::new()))
            .build()
            .unwrap();
        TrayIconContext {
            tray_icon,
            items: Signal::new(SlotMap::new()),
        }
    })
    .clone();

    use_effect(cx, move || {
        let menu = Menu::new();
        for item in tray_icon_cx.items.read().values() {
            menu.append(&**item).unwrap();
        }
        tray_icon_cx.tray_icon.set_menu(Some(Box::new(menu)));
    });

    cx.render(rsx! {
        children
    })
}

#[component]
pub fn MenuItem<'a>(
    cx: Scope<'a>,
    text: &'a str,
    enabled: Option<bool>,
    accelerator: Option<Accelerator>,
) -> Element<'a> {
    let tray_icon_cx = use_context::<TrayIconContext>(cx).unwrap().clone();

    let text = text.to_string();
    let accelerator = accelerator.clone();
    let enabled = enabled.unwrap_or_default();
    use_effect(cx, move || {
        let menu_item = MenuItemBuilder::new().text(&*text).enabled(enabled).build();
        menu_item.set_accelerator(accelerator).unwrap();
        tray_icon_cx.items.write().insert(Box::new(menu_item));
    });

    cx.render(rsx! {
        {}
    })
}

#[component]
pub fn IconMenuItem<'a>(
    cx: Scope<'a>,
    text: &'a str,
    native_icon: Option<NativeIcon>,
    enabled: Option<bool>,
    accelerator: Option<Accelerator>,
) -> Element<'a> {
    let tray_icon_cx = use_context::<TrayIconContext>(cx).unwrap().clone();

    let text = text.to_string();
    let enabled = enabled.unwrap_or_default();
    to_owned![accelerator, native_icon];
    use_effect(cx, move || {
        let menu_item = IconMenuItemBuilder::new().text(&*text).enabled(enabled).native_icon(native_icon).build();
        menu_item.set_accelerator(accelerator).unwrap();
        tray_icon_cx.items.write().insert(Box::new(menu_item));
    });

    cx.render(rsx! {
        {}
    })
}

#[component]
pub fn Seperator<'a>(
    cx: Scope<'a>,
) -> Element<'a> {
    let tray_icon_cx = use_context::<TrayIconContext>(cx).unwrap().clone();

    use_effect(cx, move || {
        let menu_item = PredefinedMenuItem::separator();
        tray_icon_cx.items.write().insert(Box::new(menu_item));
    });

    cx.render(rsx! {
        {}
    })
}
