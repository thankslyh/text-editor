mod buffer;
mod command;
mod commandbar;
mod documentstatus;
mod editor;
mod fileinfo;
mod line;
mod messagebar;
mod position;
mod size;
mod statusbar;
mod terminal;
mod uicomponent;
mod view;

fn main() {
    let mut edi = editor::Editor::new().unwrap();
    edi.run();
}
