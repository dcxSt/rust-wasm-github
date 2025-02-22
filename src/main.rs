use yew::prelude::*;

struct Index;
impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    { "Hello, World!" }
                </div>
                <br/>
                <div>
                    { "Please please work github actions!" }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Index>();
}
