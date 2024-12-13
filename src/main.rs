mod buffer;
mod editor;
mod editorcommand;
mod line;
mod statusbar;
mod terminal;
mod view;

fn main() {
    let mut edi = editor::Editor::new().unwrap();
    edi.run();
}
