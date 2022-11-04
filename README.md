# rubble_tea
A rust TUI framework similar to golang's [bubbletea](https://github.com/charmbracelet/bubbletea/).
Unlike bubbletea which offers widgets and styling in different packages rubble_tea offers everything in a single package.
## Tutorial
A rubble_tea program generally consists of two structures:
- The event structure is used describe events and should implement the Event trait
- The model structure is used to store, modify and show the program's data. It should implement the Model trait
### Events
The Event trait consists of 2 methods:
```rust
fn from_system_event(se: SystemEvent) -> Self;
fn to_system_event(&self) -> Option<SystemEvent>;
```
`from_system_event` is used to encode system events. Since system events should always
be encoded, this function should always return a valid event.
`to_system_event` is used to decode a system event. Since not all events contain
system events this function is allowed to return `None`. An event should also be
cloneable and equatable. It is worth mentioning that the `SystemEvent` enum implements
the Event trait. That means that if a program only wants to handle system events
it does not have to create it's own event structure.
#### Example
An event generally looks like this
```rust
#[derive(Eq, PartialEq, Clone)]
pub enum MyEvent {
    Se(SystemEvent),
    MyCustomEvent,
}

impl Event for MyEvent {
    fn from_system_event(se: SystemEvent) -> Self {
        MyEvent::Se(se)
    }
    fn to_system_event(&self) -> Option<SystemEvent> {
        match self {
            MyEvent::Se(x) => Some(x.clone()),
            _ => None,
        }
    }
}
```
### Models
The `Model` trait consists of 2 methods:
```rust
pub trait Model<E: Event> {
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>>;
    fn view(&self) -> String;
}
```
The update method is responsible for modifying the model based on the event that
it recives and returning a closure that returns a new event.
the view method is responsible for turning the given model to a string. The same
string will be displayed.
#### Example
A model generally looks like this
```rust
struct MyModel(i32);
impl Model<SystemEvent> for MyModel {
    fn update(
        &mut self,
        e: &SystemEvent,
    ) -> Vec<Box<dyn FnOnce() -> SystemEvent + Send + 'static>> {
        match e {
            SystemEvent::KeyPress(Key::Char('+')) => self.0 += 1,
            SystemEvent::KeyPress(Key::Char('-')) => self.0 -= 1,
            _ => (),
        };
        if self.0 < 0 || self.0 > 100 {
            vec![Box::new(|| SystemEvent::Quit)]
        } else {
            vec![]
        }
    }
    fn view(&self) -> String {
        if self.0 < 0 || self.0 > 100 {
            "Out of range!".to_string()
        } else {
            format!("{}", self.0)
        }
    }
}
```
