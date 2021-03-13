#[macro_use]
extern crate log;
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use druid::im::{vector, Vector};
use druid::widget::{Align, Button, CrossAxisAlignment, Flex, Label, List, Scroll, TextBox};
use druid::{
    AppLauncher, Color, Data, Env, Lens, LocalizedString, UnitPoint, Widget, WidgetExt, WindowDesc,
};
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use uuid::Uuid;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const SEARCH_BOX_WIDTH: f64 = 200.0;

#[derive(Clone, Data, Lens, PartialEq)]
struct Tag {
    #[data(same_fn = "PartialEq::eq")]
    id: Uuid,
    name: String,
}

#[derive(Clone, Data, Lens, PartialEq)]
struct Note {
    #[data(same_fn = "PartialEq::eq")]
    id: Uuid,
    #[data(same_fn = "PartialEq::eq")]
    created_dt: DateTime<Utc>,
    content: String,
    #[data(same_fn = "PartialEq::eq")]
    tags: Vector<Tag>,
}

#[derive(Clone, Data, Lens)]
struct AppState {
    search_keywords: String,
    #[data(same_fn = "PartialEq::eq")]
    tags: Vector<Tag>,
    #[data(same_fn = "PartialEq::eq")]
    notes: Vector<Note>,
}

fn main() {
    dotenv().ok();
    SimpleLogger::new().init().unwrap();
    let window_title = dotenv::var("WINDOW_TITLE").expect("Window title must be set.");

    // describe the main window
    let main_window = WindowDesc::new(build_app)
        .title(window_title)
        .window_size((400.0, 400.0));

    // demo data
    let test_tag = Tag {
        id: Uuid::new_v4(),
        name: "test_tag".into(),
    };
    let tags = vector![
        Tag {
            id: Uuid::new_v4(),
            name: "All".into(),
        },
        Tag {
            id: Uuid::new_v4(),
            name: "Untagged".into(),
        },
        test_tag.clone(),
    ];
    let notes = vector![
        Note {
            id: Uuid::new_v4(),
            created_dt: Utc::now(),
            content: "I am Untagged note".into(),
            tags: vector![],
        },
        Note {
            id: Uuid::new_v4(),
            created_dt: Utc::now(),
            content: "I am #test_tag note".into(),
            tags: vector![test_tag.clone()],
        },
    ];
    let initial_state = AppState {
        search_keywords: "".into(),
        tags,
        notes,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_app() -> impl Widget<AppState> {
    let mut root = Flex::row();
    let mut tags_wrap = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let mut main_wrap = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
    let mut notes_wrap = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);

    tags_wrap.add_flex_child(
        Scroll::new(List::new(|| {
            Button::new(|tag: &Tag, _env: &_| tag.name.clone())
                .padding((0.0, 0.0, 0.0, 5.0))
                .align_vertical(UnitPoint::LEFT)
        }))
        .vertical()
        .lens(AppState::tags),
        1.0,
    );

    let search_box = TextBox::new()
        .with_placeholder("Search")
        .fix_width(SEARCH_BOX_WIDTH)
        .lens(AppState::search_keywords);

    notes_wrap.add_flex_child(
        Scroll::new(List::new(|| {
            Flex::row().with_flex_child(
                Flex::column()
                    .with_child(
                        Flex::row()
                            .with_child(Label::new("note").with_text_size(15.0))
                            .with_child(
                                Button::from_label(Label::new("A").with_text_size(14.0))
                                    .fix_size(16.0, 16.0),
                            )
                            .with_child(
                                Button::from_label(Label::new("D").with_text_size(14.0))
                                    .fix_size(16.0, 16.0),
                            )
                            .align_left(),
                    )
                    .with_child(
                        TextBox::multiline()
                            .padding((0.0, 0.0, 0.0, 5.0))
                            .expand_width()
                            .lens(Note::content),
                    ),
                1.0,
            )
        }))
        .vertical()
        .lens(AppState::notes),
        1.0,
    );

    main_wrap.add_child(search_box);
    main_wrap.add_flex_child(notes_wrap, 1.0);

    root.add_child(tags_wrap);
    root.add_flex_child(main_wrap.align_vertical(UnitPoint::TOP), 1.0);

    // center the two widgets in the available space
    Align::left(root)
}
