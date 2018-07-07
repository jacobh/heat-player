use std::path::PathBuf;

use gtk;
use gtk::Orientation::Vertical;
use gtk::{ContainerExt, FileChooserButtonExt, FileChooserExt, Label};
use relm::{Relm, Update, Widget};

#[derive(Debug, PartialEq, Eq)]
pub struct FolderChooserModel {
    folder: Option<PathBuf>,
}

#[derive(Msg)]
pub enum FolderChooserMsg {
    FolderSet(PathBuf),
    FolderUnset,
}

struct Widgets {
    vbox: gtk::Box,
    chooser_button: gtk::FileChooserButton,
}

pub struct FolderChooser {
    model: FolderChooserModel,
    widgets: Widgets,
}

impl Update for FolderChooser {
    type Model = FolderChooserModel;
    type ModelParam = ();
    type Msg = FolderChooserMsg;

    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        FolderChooserModel { folder: None }
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            FolderChooserMsg::FolderSet(folder) => self.model.folder = Some(folder),
            FolderChooserMsg::FolderUnset => self.model.folder = None,
        }
        println!("{:?}", self.model);
    }
}

impl Widget for FolderChooser {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.widgets.vbox.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let label = Label::new("Select music directory");
        vbox.add(&label);

        let chooser_button = gtk::FileChooserButton::new(
            "Select folder to continue",
            gtk::FileChooserAction::SelectFolder,
        );
        vbox.add(&chooser_button);

        {
            let stream = relm.stream().clone();
            chooser_button.connect_file_set(move |widget| {
                if let Some(folder) = widget.get_filename() {
                    stream.emit(FolderChooserMsg::FolderSet(folder))
                } else {
                    stream.emit(FolderChooserMsg::FolderUnset)
                }
            });
        }

        FolderChooser {
            model,
            widgets: Widgets {
                vbox,
                chooser_button,
            },
        }
    }
}
