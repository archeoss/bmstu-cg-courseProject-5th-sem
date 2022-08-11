#![deny(clippy::all)]
#![deny(elided_lifetimes_in_paths)]
#![forbid(unsafe_code)]

mod app_factory;
use app_factory::create_app;

use crate::app_factory::app::App;
use crate::app_factory::canvas::canvas_pixel::CanvasPixel;

fn main()
{
    let app = create_app("winit-pixel").unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Trace).expect("error initializing logger");
        let app = create_app("winit-pixel").unwrap();
        wasm_bindgen_futures::spawn_local(app.run_wasm());

    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();

        pollster::block_on(app.run_wasm());
    }
}