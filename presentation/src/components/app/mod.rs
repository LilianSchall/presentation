use leptos::logging;
use leptos::prelude::*;
use shell::lexer::Token;
use shell::lexer::Lexer;

stylance::import_crate_style!(style, "src/components/app/app.module.css");

#[component]
pub fn App() -> impl IntoView {
    let (tokens, setTokens): (ReadSignal<Vec<Token>>, WriteSignal<Vec<Token>>) = signal(Vec::new());
    let input_element: NodeRef<leptos::html::Input> = NodeRef::default();

    view! {
        <div 
            class=style::app 
            on:click:target=move |_ev| {
                let _ = input_element.get().unwrap().focus();
            }>
            <ul>
                <For
                    each=move || tokens.get()
                    key=move |token| token.representation.clone()
                    children=move |token: Token| {
                        view! {
                            <li>{token.representation}</li>
                        }
                    }
                />
            </ul>
            <input 
                id="entry"
                type="text" 
                node_ref=input_element
                on:input:target=move |ev| {
                    let lexer = Lexer::new(ev.target().value());
                    let t: Vec<Token> = lexer.collect();

                    logging::log!("{:?}", t);
                    setTokens.set(t)
                }
            />
        </div>
    }
}
