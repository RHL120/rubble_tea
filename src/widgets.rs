///The trait that all widgets must implement
pub trait Widget<E: crate::Event> {
    ///Sets up the widget and returns an initial event
    fn init(&mut self) -> Option<Box<dyn FnOnce() -> E + Send + 'static>>;
    ///Handles an event and returns another one
    fn update(&mut self, e: E) -> Option<Box<dyn FnOnce() -> E + Send + 'static>>;
    ///Returns the string representation of a widget
    fn view(&self) -> String;
}
