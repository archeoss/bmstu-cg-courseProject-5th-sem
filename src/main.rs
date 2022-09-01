slint::include_modules!();
pub mod app_factory;
pub mod errors;
pub mod objects;
pub mod managers;
pub mod models;
pub mod custom_loader;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main()
{
    // app_factory::launch();
    let mut factory = (app_factory::create_app("slint-skia")).unwrap();
    let mut app = factory.make(750, 800);
    app.launch();
}
