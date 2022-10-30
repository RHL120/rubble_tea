pub trait Widget<E> {
    fn init(&mut self) -> Option<Box<dyn FnOnce() -> E + Send + 'static>>;
    fn update(&mut self, e: E) -> Option<Box<dyn FnOnce() -> E + Send + 'static>>;
    fn view(&self) -> String;
}
