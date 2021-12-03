use structopt::{
    clap::{App, Arg},
    StructOpt,
};

use cursive::{
    event::{self, EventResult, Key},
    traits::{Boxable, Nameable, Scrollable},
    view::{scroll::Scroller, SizeConstraint},
    views::{Dialog, EditView, OnEventView, Panel, SelectView, TextView},
    With,
};
use cursive::{Cursive, CursiveExt};

mod interpreter;
mod lexer;
mod parser;
mod repl;

fn main() {
    let matches = App::new("example")
        .version("0.2.0")
        .about("An example of StructOpt usage.")
        .arg(
            Arg::with_name("file")
                .help("File containing brainfuck code to be interpreted")
                .short("f")
                .long("file"),
        )
        .arg(
            Arg::with_name("repl")
                .help("Start in repl mode")
                .short("r")
                .long("repl"),
        )
        .get_matches();

    let mut siv = Cursive::new();

    siv.add_fullscreen_layer(Panel::new(
        TextView::new(">++++++++++>+>+[[+++++[>++++++++<-]>.<++++++[>--------<-]+<<<]>.>>[[-]<[>+<-]>>[<<+>+>-]<[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>+<-[>[-]>+>+<<<-[>+<-]]]]]]]]]]]+>>>]<<<]")
            .scrollable()
            .wrap_with(OnEventView::new)
            .on_pre_event_inner(Key::PageUp, |v, _| {
                let scroller = v.get_scroller_mut();
                if scroller.can_scroll_up() {
                    scroller.scroll_up(scroller.last_outer_size().y.saturating_sub(1));
                }
                Some(EventResult::Consumed(None))
            })
            .on_pre_event_inner(Key::PageDown, |v, _| {
                let scroller = v.get_scroller_mut();
                if scroller.can_scroll_down() {
                    scroller.scroll_down(scroller.last_outer_size().y.saturating_sub(1));
                }
                Some(EventResult::Consumed(None))
            }),
    ));

    siv.add_global_callback(event::Key::Esc, |s| {
        s.quit();
    });

    siv.run();
}
