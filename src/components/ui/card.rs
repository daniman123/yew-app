use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CardProps {
    pub children: Html,
}

#[function_component]
pub fn Card(props: &CardProps) -> Html {
    let CardProps { children } = props;
    html! {
        <div class="flex justify-center p-2">
            <div id="introductions" class="max-w-full p-6 rounded-xl bg-white shadow-md">
                {children}
            </div>
        </div>
    }
}
