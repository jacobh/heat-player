extern crate iui;
extern crate tinyfiledialogs;

use iui::controls::{Button, Group, Label, VerticalBox};
use iui::prelude::*;
use std::path::PathBuf;

struct State {
    music_folder: Option<PathBuf>,
}

fn main() {
    // Initialize the UI library
    let ui = UI::init().expect("Couldn't initialize UI library");
    // Create a window into which controls can be placed
    let mut win = Window::new(&ui, "Heat 🔥", 200, 200, WindowType::NoMenubar);

    // Create a vertical layout to hold the controls
    let mut vbox = VerticalBox::new(&ui);
    vbox.set_padded(&ui, true);

    let mut group_vbox = VerticalBox::new(&ui);
    let mut group = Group::new(&ui, "");

    // Create two buttons to place in the window
    let mut button = Button::new(&ui, "Change Music Directory");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            let res = tinyfiledialogs::select_folder_dialog("Select music directory", "/")
                .map(PathBuf::from);
            println!("{:?}", res);
        }
    });

    let mut quit_button = Button::new(&ui, "Quit");
    quit_button.on_clicked(&ui, {
        let ui = ui.clone();
        move |_| {
            ui.quit();
        }
    });

    group_vbox.append(&ui, button, LayoutStrategy::Compact);
    group_vbox.append(&ui, quit_button, LayoutStrategy::Compact);
    group.set_child(&ui, group_vbox);
    vbox.append(&ui, group, LayoutStrategy::Compact);

    // Actually put the button in the window
    win.set_child(&ui, vbox);
    // Show the window
    win.show(&ui);
    // Run the application
    ui.main();
}
