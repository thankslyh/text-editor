mod editor;
mod terminal;
mod view;

fn main() {
    let mut edi = editor::Editor::new();
    edi.run();
    println!("Hello, world!");
}
