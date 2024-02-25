use slint::{ComponentHandle, Model, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {

    let ui = AppWindow::new()?;

    ui.on_summit({
        let ui_handle = ui.as_weak();
        move |b, a| {
            let ui = ui_handle.unwrap();
            let b_config: String = b.parse().unwrap();
            let a_config: String = a.parse().unwrap();
            if !b_config.is_empty() && !a_config.is_empty() {
                match b_config.contains(&a_config) {
                    true => ui.set_res("True".into()),
                    false => {
                        let mut res_content: Vec<String> = vec![];
                        let mut res_lines: Vec<Lines> = Default::default(); 
                        let mut count: usize = 0;
                        let b: Vec<String> = b_config.clone().lines().map(|s| s.to_string()).collect();
                        let a: Vec<String> = a_config.clone().lines().map(|s| s.to_string()).collect();
                        for (i,j) in a.iter().enumerate() {
                            if !b.contains(j) {
                                res_content.push(j.to_string());
                                count += 1;
                                res_lines.push(Lines { index: count.to_string().into(), number: (i+1).to_string().into() });
                            }
                        }
                        let model_lines = slint::ModelRc::new(VecModel::from(res_lines.clone()));
                        let backup: Vec<slint::SharedString> = res_content.clone().into_iter().map(Into::into).collect();
                        let model_backup = slint::ModelRc::new(VecModel::from(backup));
                        ui.set_res(res_content.join("\n").into());
                        ui.set_res_backup(model_backup);
                        ui.set_lines(model_lines);
                        ui.set_t_lines(res_lines.len().try_into().unwrap());
                    }
                }
            } else {
                ui.set_res("Please summit...".into());
                ui.set_res_backup([].into());
                ui.set_lines([].into());
                ui.set_t_lines(0);
            }
        }
    });

    ui.on_select({
        let ui_handle = ui.as_weak();
        move |s| {
            let ui = ui_handle.unwrap();
            let select: usize = s.parse().unwrap();
            let backup: Vec<String> = ui.get_res_backup().iter().map(|s| s.to_string()).collect();
            match !ui.get_res().is_empty() && !backup.is_empty() {
                true => {
                    ui.set_res(backup[select - 1].to_owned().into())
                },
                false => ui.set_res("Not found!".into())
            }
        }
    });

    ui.on_refresh({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let backup: Vec<String> = ui.get_res_backup().iter().map(|s| s.to_string()).collect();
            match !backup.is_empty() {
                true => ui.set_res(backup.join("\n").into()),
                false => ui.set_res("Please Summit Again!".into()),
            }
        }
    });

    ui.run()
}
