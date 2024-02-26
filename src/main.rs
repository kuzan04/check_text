use slint::{
    Color, ComponentHandle, Model, ModelRc, VecModel
};


slint::include_modules!();

fn single_str(c: String, cl: Color) -> ModelRc<Content> {
    let set: Vec<Content> = vec![Content { content: c.into(), color: cl }];
    ModelRc::new(VecModel::from(set))
}
fn single_line(i: String, n: String) -> ModelRc<Lines> {
    let set: Vec<Lines> = vec![Lines { index: i.into(), number: n.into()}];
    ModelRc::new(VecModel::from(set))
}

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
                    true => {
                        ui.set_lines(single_line("0".to_string(), "1".to_string()));
                        ui.set_t_lines(1);
                        let res = ModelRc::new(single_str("True".to_string(), Color::from_rgb_u8(124,252,0)));
                        ui.set_res_b(res);
                    },
                    false => {
                        let mut res_content_b: Vec<Content> = vec![];
                        let mut res_content_a: Vec<Content> = vec![];
                        let mut res_lines: Vec<Lines> = vec![]; 
                        let mut count: usize = 0;
                        let b: Vec<String> = b_config.clone().lines().map(String::from).collect();
                        let a: Vec<String> = a_config.clone().lines().map(String::from).collect();
                        for i in &b {
                            if !a.contains(i) {
                                res_content_b.push(Content { content: i.into(), color: Color::from_rgb_u8(255,255,51) });
                            }
                        }
                        for (i,j) in a.iter().enumerate() {
                            if !b.contains(j) {
                                res_content_a.push(Content { content: j.to_string().into(), color: Color::from_rgb_u8(255,0,0) });
                                count += 1;
                                res_lines.push(Lines { index: count.to_string().into(), number: (i+1).to_string().into() });
                            }
                        }
                        let model_content_b = ModelRc::new(VecModel::from(res_content_b));
                        let model_content_a = ModelRc::new(VecModel::from(res_content_a));
                        let model_lines = ModelRc::new(VecModel::from(res_lines.clone()));
                        ui.set_res_b(model_content_b.clone());
                        ui.set_res_a(model_content_a.clone());
                        ui.set_res_backup_b(model_content_b);
                        ui.set_res_backup_a(model_content_a);
                        ui.set_lines(model_lines);
                        ui.set_t_lines(res_lines.len().try_into().unwrap());
                    }
                }
            } else {
                let res = single_str("Please add text in input Before and After Thank!".to_string(), Color::from_rgb_u8(255,255,0));
                ui.set_res_b(res);
                ui.set_res_backup_b([].into());
                ui.set_res_backup_a([].into());
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
            let res_b: Vec<Content> = ui.get_res_b().iter().collect();
            let res_a: Vec<Content> = ui.get_res_a().iter().collect();
            let backup_b: Vec<Content> = ui.get_res_backup_b().iter().collect();
            let backup_a: Vec<Content> = ui.get_res_backup_a().iter().collect();
            match (!res_b.is_empty() && !backup_b.is_empty()) && (!res_a.is_empty() && !backup_a.is_empty()) {
                true => {
                    let select_b = single_str(backup_b[select-1].content.to_string(), backup_b[select-1].color);
                    let select_a = single_str(backup_a[select-1].content.to_string(), backup_a[select-1].color);
                    ui.set_res_b(select_b);
                    ui.set_res_a(select_a);
                },
                false => ui.set_res_b(single_str("Not found!".to_string(), Color::from_rgb_u8(0,0,0))),
            }
        }
    });

    ui.on_refresh({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let backup_b: Vec<Content> = ui.get_res_backup_b().iter().collect();
            let backup_a: Vec<Content> = ui.get_res_backup_a().iter().collect();
            match !backup_b.is_empty() && !backup_a.is_empty() {
                true => {
                    ui.set_res_b(ui.get_res_backup_b());
                    ui.set_res_a(ui.get_res_backup_a());
                },
                false => {
                    let mut vec: Vec<Content> = vec![];
                    for i in 0..8 {
                        vec.push(Content { content: format!("Please summit!! {}", (i+1)).to_string().into(), color: Color::from_rgb_u8(255, 0, 0) });
                    }
                    let vec = ModelRc::new(VecModel::from(vec));
                    ui.set_res_b(vec)
                },
            }
        }
    });

    ui.run()
}
