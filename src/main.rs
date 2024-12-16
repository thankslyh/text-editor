mod buffer;
mod documentstatus;
mod editor;
mod editorcommand;
mod fileinfo;
mod line;
mod statusbar;
mod terminal;
mod view;

fn main() {
    let mut edi = editor::Editor::new().unwrap();
    edi.run();
}
