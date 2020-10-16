pub trait Backend /*: Send + Sync*/ {
    fn hello(&self) {
        println!("ok");
    }
    fn runner<B: Backend, S, R: FnMut(&mut App<B>, &mut S)>(
        &mut self,
    ) -> Box<Fn(App<B>, S, R) -> Result<(), String>>;
}

type BuildCallback<B: Backend, S> = fn(&mut App<B>, &mut S);

pub struct EmptyBackend;

impl Backend for EmptyBackend {
    fn runner<B: Backend, S, R: FnMut(&mut App<B>, &mut S)>(
        &mut self,
    ) -> Box<Fn(App<B>, S, R) -> Result<(), String>> {
        Box::new(|mut app: App<B>, mut state: S, mut cb: R| {
            loop {
                cb(&mut app, &mut state);

                if app.exit_was_requested() {
                    break;
                }
            }
            Ok(())
        })
    }
}

pub trait AppConfig<B: Backend, S> {
    fn apply(&self, builder: &mut AppBuilder<S, B>);
}

pub struct WindowConfig {
    pub cosas: i32,
}

impl<B: Backend, S> AppConfig<B, S> for WindowConfig {
    fn apply(&self, builder: &mut AppBuilder<S, B>) {
        builder.window = "NOP!".to_string();
    }
}

pub struct AppBuilder<S, B>
where
    B: Backend,
{
    state: S,
    backend: B,
    pub window: String,
    init_callback: Option<BuildCallback<B, S>>,
    update_callback: Option<BuildCallback<B, S>>,
    draw_callback: Option<BuildCallback<B, S>>,
    event_callback: Option<BuildCallback<B, S>>,
}

impl<S, B> AppBuilder<S, B>
where
    B: Backend,
{
    pub fn set_config(mut self, config: &AppConfig<B, S>) -> Self {
        config.apply(&mut self);
        self
    }

    pub fn initialize(mut self, cb: BuildCallback<B, S>) -> Self {
        self.init_callback = Some(cb);
        self
    }

    pub fn update(mut self, cb: BuildCallback<B, S>) -> Self {
        self.update_callback = Some(cb);
        self
    }

    pub fn build(mut self) -> Result<(), String> {
        let AppBuilder {
            mut backend,
            mut state,
            window,
            init_callback,
            update_callback,
            draw_callback,
            event_callback,
            ..
        } = self;

        let runner = backend.runner();
        let mut app = App::new(backend);

        if let Some(cb) = init_callback {
            cb(&mut app, &mut state);
        }

        runner(app, state, move |mut app, mut state| {
            app.tick();

            if let Some(cb) = event_callback {
                cb(&mut app, &mut state);
            }

            if let Some(cb) = update_callback {
                cb(&mut app, &mut state);
            }

            //TODO check frame here?
            if let Some(cb) = draw_callback {
                cb(&mut app, &mut state);
            }
        })?;

        Ok(())
    }
}

pub struct App<B: Backend> {
    pub backend: B,
    request_exit: bool,
}

impl<B: Backend> App<B> {
    fn new(backend: B) -> Self {
        Self {
            request_exit: false,
            backend,
        }
    }

    pub fn hello(&self) {
        self.backend.hello();
    }

    pub fn tick(&mut self) {
        //TODO
    }

    pub fn exit(&mut self) {
        self.request_exit = true;
    }

    pub fn exit_was_requested(&self) -> bool {
        self.request_exit
    }
}

pub struct Notan;

impl Notan {
    pub fn init() -> AppBuilder<(), EmptyBackend> {
        Self::init_with_backend((), EmptyBackend {})
    }

    pub fn init_with<S>(state: S) -> AppBuilder<S, EmptyBackend> {
        Self::init_with_backend(state, EmptyBackend {})
    }

    pub fn init_with_backend<S, T: Backend>(state: S, backend: T) -> AppBuilder<S, T> {
        AppBuilder {
            state,
            backend,
            window: "Yeah".to_string(),
            init_callback: None,
            update_callback: None,
            draw_callback: None,
            event_callback: None,
        }
    }
}
