use sand::{run_app, Widget, StatelessWidget, BuildContext};

#[derive(Debug)]
struct MyApp;

impl Widget for MyApp {
    fn build(&self, ctx: BuildContext) -> Box<dyn Widget> {
        unimplemented!()
    }
}

impl StatelessWidget for MyApp {}

fn main() {
    sand::run_app(MyApp);
}
