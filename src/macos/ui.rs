use std::{process, sync::Arc, thread, time::Duration};

use objc2::rc::Retained;
use objc2_foundation::MainThreadMarker;

use crate::{logger::Logger, macos::appkit::NSMenuItemBadge};

use super::{
    appkit::{
        NSApplication, NSButtonMethods, NSData, NSImage, NSImageResizingMode, NSMenu, NSMenuItem,
        NSSize, NSStatusBar, NSStatusItem,
    },
    service_management::{SMAppService, SMAppServiceStatus},
    Config,
};

pub fn setup_status_bar_item() -> Retained<NSStatusItem> {
    let system_status_bar = NSStatusBar::get_system_status_bar();
    let status_bar_item = system_status_bar.new_status_item(16.0_f64);

    let status_item_button = status_bar_item.get_button();

    let status_item_button_image = NSImage::init_with_data(&NSData::init_with_bytes(
        include_bytes!("../../resources/icon_256x256.png"),
    ));
    status_item_button_image.set_resizing_mode(NSImageResizingMode::Stretch);
    status_item_button_image.set_size(&NSSize::new(20.0, 20.0));

    status_item_button.set_image(&status_item_button_image);

    status_bar_item
}

pub fn setup_toggle_login_item_menu_item(
    mtm: &MainThreadMarker,
    menu: &Retained<NSMenu>,
    logger: Arc<Logger>,
) {
    fn get_status() -> (Result<SMAppServiceStatus, String>, bool) {
        let login_item_status = SMAppService::get_main_app_service().get_status();

        let is_enabled = matches!(login_item_status, Ok(SMAppServiceStatus::Enabled));

        (login_item_status, is_enabled)
    }

    let menu_item = NSMenuItem::init_with_action(
        mtm,
        "Toggle Auto-Start",
        {
            let logger: Arc<Logger> = logger.clone();

            let badge = NSMenuItemBadge::init_with_string(mtm, "Enabled");

            fn error_badge(badge: &Retained<NSMenuItemBadge>) {
                let previous_badge_str = badge.get_string();

                badge.set_string("ERROR");

                thread::sleep(Duration::from_millis(1000));

                badge.set_string(&previous_badge_str);
            }

            Box::new(move |this| {
                this.set_badge(&badge);

                let (login_item_status, _) = get_status();

                match login_item_status {
                    Ok(SMAppServiceStatus::Enabled) => {
                        logger.debug("Unregistering login item...");

                        let (success, error) =
                            SMAppService::get_main_app_service().unregister_and_return_error();

                        if !success {
                            logger.error(&format!(
                                "Encountered error while registering login item: {:#?}",
                                error,
                            ));

                            error_badge(&badge);
                        } else {
                            logger.debug("Login item unregistered!");
                        }
                    }
                    Ok(SMAppServiceStatus::ErrorAndNotFound)
                    | Ok(SMAppServiceStatus::NotRegistered) => {
                        logger.debug("Registering login item...");

                        let (success, error) =
                            SMAppService::get_main_app_service().register_and_return_error();

                        if !success {
                            logger.error(&format!(
                                "Encountered error while registering login item: {:#?}",
                                error,
                            ));

                            error_badge(&badge);
                        } else {
                            logger.debug("Login item registered!");
                        }
                    }
                    Ok(SMAppServiceStatus::RequiresApproval) => {
                        logger.debug("Login item requires approval?");
                        SMAppService::open_system_settings_to_login_items();
                    }
                    Err(_) => {
                        error_badge(&badge);
                    }
                }

                let (_, is_enabled) = get_status();
                badge.set_string(match is_enabled {
                    true => "Enabled",
                    false => "Disabled",
                });
            })
        },
        "",
    );

    let (_, is_enabled) = get_status();
    menu_item.set_badge(&NSMenuItemBadge::init_with_string(
        mtm,
        match is_enabled {
            true => "Enabled",
            false => "Disabled",
        },
    ));

    menu.add_item(&menu_item);
}

pub fn setup_menu(
    mtm: &MainThreadMarker,
    status_bar_item: &Retained<NSStatusItem>,
    config: Arc<Config>,
    logger: Arc<Logger>,
) -> Retained<NSMenu> {
    let menu = NSMenu::init(mtm, "Clipboard Cleanse");

    menu.set_auto_enables_items(true);

    menu.add_item(&NSMenuItem::init_section_header("Clipboard Cleanse"));

    setup_toggle_login_item_menu_item(mtm, &menu, logger.clone());

    menu.add_item(&NSMenuItem::init_with_action(
        mtm,
        "Open Config File",
        {
            let config_path = config.config_path.clone();
            let logger = logger.clone();
            Box::new(move |_| {
                logger.debug("Opening config file...");

                process::Command::new("open")
                    .args([&config_path])
                    .output()
                    .expect("Expected successful opening of config file");

                logger.debug("Config file opened!");
            })
        },
        ",",
    ));

    menu.add_item(&NSMenuItem::init_with_action(
        mtm,
        "Quit",
        Box::new(|_| NSApplication::get_shared().terminate()),
        "q",
    ));

    status_bar_item.set_menu(&menu);

    menu
}
