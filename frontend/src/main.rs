use stylist::yew::styled_component;
use yew::prelude::*;

static BACKEND_URL: &str = "http://192.168.195.124:5000";

#[derive(Properties, PartialEq)]
struct ImageProperties {
    url: String,
}

#[function_component]
fn Image(props: &ImageProperties) -> Html {
    html! {
        <img
            src={props.url.clone()}
            style="height:100%;width:100%;object-fit:inherit"
        />
    }
}

#[derive(Properties, PartialEq)]
struct BookCoverProperties {
    name: String,
}

#[function_component]
fn BookCover(props: &BookCoverProperties) -> Html {
    let url = format!("{}/books/{}/thumbnail", BACKEND_URL, props.name);
    html! {
        <Image url={url}/>
    }
}

#[derive(Properties, PartialEq)]
struct BookProperties {
    name: String,
}

#[function_component]
fn Book(props: &BookProperties) -> Html {
    html! {
    <>
        <div>
            <BookCover name={props.name.clone()} />
        </div>
        <div>{ format!("{}", props.name) }</div>
    </>
    }
}

#[styled_component]
fn BookShelf() -> Html {
    let books = use_state(|| vec![]);
    {
        let books = books.clone();
        use_effect_with((), move |_| {
            let books = books.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("{}/books", BACKEND_URL);
                let fetched_books: Vec<String> =
                    reqwest::get(&url).await.unwrap().json().await.unwrap();
                books.set(fetched_books);
            });
        })
    }

    html! {
        <>
            {
                books.chunks(2).map(|books| html! {
                    <div class={css!("display:flex;")}>
                        {
                            books.iter().map(|book| html! {
                                <div class={css!("flex:50%;")}>
                                    <Book name={book.clone()}/>
                                </div>
                            }).collect::<Html>()
                        }
                    </div>
                }).collect::<Html>()
            }
        </>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <h1>{ "Books" }</h1>
            <BookShelf />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
