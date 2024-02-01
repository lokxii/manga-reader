use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
use gloo_net::http::{Request, Response};
use web_sys::{AbortController, AbortSignal};
use yew::prelude::*;
use yew_router::prelude::*;

static BACKEND_URL: &str = "http://192.168.195.95:5000";

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/book/:name")]
    BookContent { name: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq)]
struct ImageProperties {
    url: String,
    done: Option<UseStateHandle<bool>>,
}

async fn fetch(url: &str, signal: Option<&AbortSignal>) -> Option<Response> {
    loop {
        match Request::get(url).abort_signal(signal).send().await {
            Ok(res) => return Some(res),
            Err(gloo_net::Error::JsError(j)) => {
                if j.name == "AbortError" {
                    return None;
                }
                continue;
            }
            _ => continue,
        }
    }
}

#[function_component]
fn Image(props: &ImageProperties) -> Html {
    let image = use_state(|| String::new());
    {
        let image = image.clone();
        let url = props.url.clone();
        let done_state = props.done.clone();
        let controller = AbortController::new().unwrap();
        let signal = controller.signal();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = match fetch(&url, Some(&signal)).await {
                    Some(res) => res,
                    None => return,
                };
                let type_ = res.headers().get("Content-Type").unwrap();
                let bytes = res.binary().await.unwrap();
                let data = b64.encode(bytes);
                image.set(format!("data:{};base64,{}", type_, data));

                if let Some(done) = done_state {
                    done.set(true)
                }
            });

            move || {
                controller.abort();
            }
        });
    }

    html! {
        <img
            src={(*image).clone()}
            alt={"loading"}
            style="height:auto;width:100%;object-fit:inherit"
        />
    }
}

#[derive(Properties, PartialEq)]
struct BookPagesProperties {
    pages: Vec<String>,
}

#[function_component]
fn BookPages(props: &BookPagesProperties) -> Html {
    let pages = &props.pages;
    if pages.len() == 0 {
        return html! {};
    }

    let done_signal = use_state(|| false);
    html! {
        <>
            <Image key={pages[0].clone()} url={pages[0].clone()} done={Some(done_signal.clone())} />
            if *done_signal {
                <BookPages pages={pages[1..].to_owned()} />
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct BookProperties {
    name: String,
}

#[function_component]
fn BookContent(props: &BookProperties) -> Html {
    let pages = use_state(|| vec![]);
    {
        let pages = pages.clone();
        let book = props.name.clone();
        use_effect_with((), move |_| {
            let pages = pages.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{}/books/{}/pages", BACKEND_URL, book);
                let index: Vec<String> =
                    fetch(&url, None).await.unwrap().json().await.unwrap();
                pages.set(index);
            });
        })
    }

    let book = &props.name;
    html! {
        <BookPages pages={
            pages.iter().map(|page|
                format!("{}/books/{}/pages/{}", BACKEND_URL, book, page)
            ).collect::<Vec<String>>()
        }/>
    }
}

#[function_component]
fn BookCover(props: &BookProperties) -> Html {
    let url = format!("{}/books/{}/thumbnail", BACKEND_URL, props.name);
    html! {
        <Image url={url} done={None}/>
    }
}

#[function_component]
fn Book(props: &BookProperties) -> Html {
    html! {
        <>
            <Link<Route> to={Route::BookContent {name: props.name.clone()}}>
                <BookCover name={props.name.clone()} />
            </Link<Route>>
            <div>{ format!("{}", props.name) }</div>
        </>
    }
}

#[function_component]
fn BookShelf() -> Html {
    let books = use_state(|| vec![]);
    {
        let books = books.clone();
        use_effect_with((), move |_| {
            let books = books.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{}/books", BACKEND_URL);
                let fetched_books: Vec<String> =
                    fetch(&url, None).await.unwrap().json().await.unwrap();
                books.set(fetched_books);
            });
        })
    }

    html! {
        <>
            {
                for books.chunks(2).map(|books| html! {
                    <div style="display:flex;">
                        {
                            books.iter().map(|book| html! {
                                <div style="flex:50%;">
                                    <Book name={book.clone()}/>
                                </div>
                            }).collect::<Html>()
                        }
                    </div>
                })
            }
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <>
                <h1>{ "Books" }</h1>
                <BookShelf />
            </>
        },
        Route::BookContent { name } => html! {
            <div>
                <BookContent name={name} />
            </div>
        },
        Route::NotFound => html! { <div>{ "You sure that book exist?" }</div> },
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
