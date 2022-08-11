use async_trait::async_trait;

#[async_trait(?Send)]
pub(crate) trait App
{
    fn new(width: u32, height: u32) -> Self where Self: Sized;
    async fn run(&self);
    async fn run_wasm(self: Box<Self>);
}
