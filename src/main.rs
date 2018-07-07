#![feature(proc_macro)]

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::WidgetExt;

use relm::{Relm, Update, Widget};

struct Model;

#[derive(Msg)]
enum Msg {
    Quit,
}

struct Win {
    window: gtk::Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}
impl Widget for Win {
    type Root = gtk::Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), gtk::Inhibit(false))
        );

        window.show_all();

        Win { window }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
