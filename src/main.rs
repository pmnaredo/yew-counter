use yew::prelude::*;

// Represents the type of messages our component can receive
//
enum Msg {
    AddOne,
}

// Counter component
// Fields in this struct represent the state of the component
//
struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    // Associated types
    //
    type Message = Msg;
    type Properties = ();

    // Called when component is created
    //
    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    // Called when a new message is sent to the component via it's scope
    //
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true // re-render component
            }
        }
    }

    // Called after create() and update() (because it returns true)
    //
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<CounterComponent>();
}
