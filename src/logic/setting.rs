use super::tr::tr;
use crate::{
    config,
    slint_generatedAppWindow::{AppWindow, Logic, Store, Theme},
};
use slint::ComponentHandle;

pub fn init(ui: &AppWindow) {
    init_setting(ui);

    ui.global::<Store>()
        .set_is_first_run(config::is_first_run());

    ui.global::<Store>()
        .set_is_show_landing_page(config::is_first_run());

    ui.global::<Logic>()
        .on_inner_tr(move |_is_cn, text| tr(text.as_str()).into());

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_get_setting_preference(move || {
        let ui = ui_handle.unwrap();
        ui.global::<Store>().get_setting_preference()
    });

    let ui_handle = ui.as_weak();
    ui.global::<Logic>()
        .on_set_setting_preference(move |mut setting| {
            let ui = ui_handle.unwrap();

            let font_size = u32::min(50, u32::max(10, setting.font_size.parse().unwrap_or(16)));
            setting.font_size = slint::format!("{}", font_size);

            let mut all = config::all();
            all.preference.win_width =
                u32::max(480, setting.win_width.to_string().parse().unwrap_or(480));
            all.preference.win_height =
                u32::max(640, setting.win_height.to_string().parse().unwrap_or(640));
            all.preference.font_size = font_size;
            all.preference.font_family = setting.font_family.into();
            all.preference.language = setting.language.into();
            all.preference.always_on_top = setting.always_on_top;
            all.preference.no_frame = setting.no_frame;
            all.preference.is_dark = setting.is_dark;
            _ = config::save(all);

            if cfg!(feature = "desktop") {
                ui.global::<crate::Util>().invoke_update_window_size();
            } else {
                _ = ui;
            }
        });
}

fn init_setting(ui: &AppWindow) {
    let config = config::preference();
    let mut setting = ui.global::<Store>().get_setting_preference();

    let font_size = u32::min(50, u32::max(10, config.font_size));
    setting.win_width = slint::format!("{}", u32::max(480, config.win_width));
    setting.win_height = slint::format!("{}", u32::max(640, config.win_height));
    setting.font_size = slint::format!("{}", font_size);
    setting.font_family = config.font_family.into();
    setting.language = config.language.into();
    setting.always_on_top = config.always_on_top;
    setting.no_frame = config.no_frame;
    setting.is_dark = config.is_dark;

    ui.global::<Theme>().invoke_set_dark(config.is_dark);
    ui.global::<Store>().set_setting_preference(setting);
}
