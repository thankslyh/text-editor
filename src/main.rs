use editor::Location;
use view::View;

mod editor;
mod terminal;
mod view;
mod buffer;

fn main() {
    let mut edi = editor::Editor {
        view: View::default(),
        location: Location::default(),
        quit: false
    };
    edi.run();
    println!("{:?}", edi.view.get_buf());
}
