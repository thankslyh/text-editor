mod buffer;
mod documentstatus;
mod editor;
mod editorcommand;
mod fileinfo;
mod line;
mod messagebar;
mod statusbar;
mod terminal;
mod uicomponent;
mod view;
fn main() {
    let mut edi = editor::Editor::new().unwrap();
    edi.run();
}
