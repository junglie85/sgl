pub fn main() {
    let app = MyApp {};
    let engine = Engine::new();
    engine.launch(app);
}

struct MyState {
    some_state: u32,
}

struct MyApp {}

impl App for MyApp {
    type State = MyState;

    fn init(&self) -> Self::State {
        MyState { some_state: 0 }
    }

    fn update(&mut self, state: &mut Self::State) -> bool {
        println!("state: {}", state.some_state);
        state.some_state += 1;
        state.some_state < 10
    }
}

trait App {
    type State;

    fn init(&self) -> Self::State;
    fn update(&mut self, state: &mut Self::State) -> bool;
}

struct Engine {}

impl Engine {
    fn new() -> Self {
        Self {}
    }

    fn launch(&self, mut app: impl App) {
        let mut state = app.init();

        let mut running = true;
        while running {
            running = app.update(&mut state);
        }
    }
}
