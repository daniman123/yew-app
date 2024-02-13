use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TitleBannerProps {
    pub title: String,
}

#[function_component]
pub fn TitleBanner(props: &TitleBannerProps) -> Html {
    let TitleBannerProps { title } = props;
    html! {
        <div class="w-full border flex justify-center p-2">
            <h3 class="text-3xl font-semibold">{title}</h3>
        </div>
    }
}
