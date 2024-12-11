mod editor;
mod terminal;
mod view;
mod buffer;
mod editorcommand;
mod line;

fn main() {
    let mut edi = editor::Editor::new().unwrap();
    edi.run();
}