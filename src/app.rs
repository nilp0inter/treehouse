use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html, TargetCast};
use yew::events::KeyboardEvent;
use web_sys::HtmlInputElement as InputElement;

// Define the possible messages which can be sent to the component
pub enum Msg {
    Set { value: i64 },
    Increment,
    Decrement,
    Reset,
    Refresh
}

pub struct App {
    value: i64, // This will store the counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Set { value: n } => {
                self.value = n;
                console::log!("set to {:?}", n); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
            Msg::Reset => {
                self.value = 0;
                console::log!("zero");
                true
            }
            Msg::Refresh => {
                console::log!("refresh");
                true
            }

        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                match input.value().parse::<i64>() {
                    Ok(n) => Some(Msg::Set { value: n }),
                    Err(_) => Some(Msg::Refresh),
                }
            } else {
                None
            }
        });
        html! {
            <div>
                <div class="panel">
                    // An input bot to type the amount to be set
                    <input class="text" value={ self.value.to_string() } { onkeypress } />

                    // A button to send the Increment message
                    <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
                        { "+1" }
                    </button>

                    // A button to reset the counter
                    <button onclick={ctx.link().callback(|_| Msg::Reset)}>
                        { "Reset" }
                    </button>

                    // A button to send the Decrement message
                    <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
                        { "-1" }
                    </button>

                </div>

                // Display the current value of the counter
                <p class="counter">
                    { self.value }
                </p>

                // Display the current date and time the page was rendered
                <p class="footer">
                    { "Rendered: " }
                    { String::from(Date::new_0().to_string()) }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
