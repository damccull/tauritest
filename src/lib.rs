// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { counter: 0 }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let imgval = format!(
        "https://via.placeholder.com/250/AAAAAA/000000?text=Clicks+{}",
        model.counter
    );
    let button = button![
        "Click me!",
        ev(Ev::Click, |_| Msg::Increment),
        attrs! {At::Type=>"button"},
        C!["px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border"],
        C!["border-purple-200 hover:text-white hover:bg-purple-600"],
        C!["hover:border-transparent focus:outline-none focus:ring-2"],
        C!["focus:ring-purple-600 focus:ring-offset-2"],
    ];

    div![
        C!["py-8 px-8 max-w-sm mx-auto bg-white rounded-xl shadow-lg space-y-2 sm:py-4 sm:flex sm:items-center sm:space-y-0 sm:space-x-6"],
        img![
            attrs!{At::Src => imgval},
            C!["block mx-auto h-24 rounded-full sm:mx-0 sm:shrink-0"],
        ],
        div![
            C!["text-center space-y-2 sm:text-left"],
            div![C!["space-y-0.5"],
                p!["Counter", C!["text-lg text-black font-semibold"]],
                p!["Increase Counter", C!["text-slate-500 font-medium"]],
            ],
            button,
        ],
    ]

    // div![
    //     "This is a counter: ",
    //     C!["counter"],
    //     C!["counter bg-blue-500/[.75] hover:bg-green-600/[.5]"],
    //     button![model.counter, ev(Ev::Click, |_| Msg::Increment),],
    // ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}

fn btn<F: FnOnce(seed::prelude::web_sys::Event) + std::clone::Clone>(
    text: &str,
    click_handler: F,
) -> seed::virtual_dom::Node<_> {
    button![
        "Click me!",
        ev(Ev::Click, click_handler),
        attrs! {At::Type=>"button"},
        C!["px-4 py-1 text-sm text-purple-600 font-semibold rounded-full border"],
        C!["border-purple-200 hover:text-white hover:bg-purple-600"],
        C!["hover:border-transparent focus:outline-none focus:ring-2"],
        C!["focus:ring-purple-600 focus:ring-offset-2"],
    ]
}
