slint::include_modules!();
pub mod app_factory;
pub mod errors;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main()
{
    // app_factory::launch();
    let app = (app_factory::create_app("slint-skia")).unwrap();
    app.launch();
}
