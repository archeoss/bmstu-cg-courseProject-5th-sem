// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial
use app::App;
use slint::Model;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
// use pixels::{Pixels, SurfaceTexture};
use crate::app_factory::app;
use crate::app_factory::create_app;

pub struct AppSlint
{
    width: u32,
    height: u32,
    // canvas: Option<Arc<Mutex<Box<dyn Canvas>>>>,
    // pixels: Option<Arc<Mutex<Pixels>>>,
    // drawer: Option<Arc<Mutex<Box<dyn Drawer>>>>,
}

#[async_trait(?Send)]
impl App for AppSlint
{
    fn new(width: u32, height: u32) -> Self
    {
        AppSlint {
            width,
            height,
            // canvas: None,
            // pixels: None,
            // drawer: None,
        }
    }
    async fn run(&self)
    {
        // let app = create_app("slint-pixel").unwrap();
        // app.run();
        self.apprun();
    }
    async fn run_wasm(self: Box<Self>)
    {
        // let app = create_app("slint-pixel").unwrap();
        // app.run_wasm();
        self.apprun();
    }
}
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

impl AppSlint
{
    pub fn apprun(&self)
    {
        // This provides better error messages in debug mode.
        // It's disabled in release mode so it doesn't bloat up the file size.
        #[cfg(all(debug_assertions, target_arch = "wasm32"))]
        console_error_panic_hook::set_once();

        let todo_model = Rc::new(slint::VecModel::<TodoItem>::from(vec![
            TodoItem { checked: true, title: "Implement the .slint file".into() },
            TodoItem { checked: true, title: "Do the Rust part".into() },
            TodoItem { checked: false, title: "Make the C++ code".into() },
            TodoItem { checked: false, title: "Write some JavaScript code".into() },
            TodoItem { checked: false, title: "Test the application".into() },
            TodoItem { checked: false, title: "Ship to customer".into() },
            TodoItem { checked: false, title: "???".into() },
            TodoItem { checked: false, title: "Profit".into() },
        ]));

        let main_window = MainWindow::new();
        main_window.on_todo_added({
            let todo_model = todo_model.clone();
            move |text| todo_model.push(TodoItem { checked: false, title: text })
        });
        main_window.on_remove_done({
            let todo_model = todo_model.clone();
            move || {
                let mut offset = 0;
                for i in 0..todo_model.row_count() {
                    if todo_model.row_data(i - offset).unwrap().checked {
                        todo_model.remove(i - offset);
                        offset += 1;
                    }
                }
            }
        });

        let weak_window = main_window.as_weak();
        main_window.on_popup_confirmed(move || {
            let window = weak_window.unwrap();
            window.hide();
        });

        {
            let weak_window = main_window.as_weak();
            let todo_model = todo_model.clone();
            main_window.window().on_close_requested(move || {
                let window = weak_window.unwrap();

                if todo_model.iter().any(|t| !t.checked) {
                    window.invoke_show_confirm_popup();
                    slint::CloseRequestResponse::KeepWindowShown
                } else {
                    slint::CloseRequestResponse::HideWindow
                }
            });
        }

        main_window.set_todo_model(todo_model.into());
        // let app = create_app("slint-pixel").unwrap();
        // // pollster::block_on(app.run_wasm());

        main_window.run();
    }
}
// pub fn main() {
//
// }