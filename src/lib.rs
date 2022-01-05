use sand::run_app, Widget, StatelessWidget, BuildContext};

#[derive(Debug)]
struct MyApp;

impl Widget for MyApp {
    fn build(&self, ctx: BuildContext) -> Box<dyn Widget> {
        unimplemented!()
    }
}

impl StatelessWidget for MyApp {}

#[cfg_attr(target_os = "android", sand::main(backtrace = "on"))]
fn main() {
    sand::run_app(MyApp);
}
