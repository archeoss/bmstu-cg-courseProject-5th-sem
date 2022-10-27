slint::include_modules!();
pub mod app_factory;
pub mod custom_loader;
pub mod errors;
pub mod managers;
pub mod models;
pub mod objects;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main()
{
    let factory = (app_factory::create_app("slint-skia")).unwrap();
    let app = factory.make(1300, 1000);
    app.launch();
}
