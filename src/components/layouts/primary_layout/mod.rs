use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct PrimaryLayoutProps {
    pub children: Html,
}

#[function_component]
pub fn PrimaryLayout(props: &PrimaryLayoutProps) -> Html {
    let PrimaryLayoutProps { children } = props;
    html! {
        <div class="h-[844px] w-[390px] max-h:h-[844px] max-w:w-[390px] border">
            {children}
        </div>
    }
}
