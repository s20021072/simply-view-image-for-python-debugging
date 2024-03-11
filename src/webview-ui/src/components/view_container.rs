use stylist::yew::use_style;
use yew::prelude::*;
use yewdux::functional::use_selector;

use crate::{
    app_state::app_state::AppState,
    common::{ImageAvailability, ViewId, Viewable},
    components::spinner::Spinner,
};

#[derive(PartialEq, Properties)]
pub(crate) struct ViewContainerProps {
    #[prop_or_default]
    pub class: Classes,
    pub node_ref: NodeRef,
    pub view_id: ViewId,
}

#[function_component]
pub(crate) fn ViewContainer(props: &ViewContainerProps) -> Html {
    let ViewContainerProps {
        node_ref,
        class,
        view_id,
    } = props;

    let plotly_div = use_style!(
        r#"
        display: flex;
        height: 100%;
        width: 100%;
        justify-content: center;
        align-items: center;
        "#,
    );

    let current_image_availability = {
        let view_id = *view_id;
        use_selector(move |state: &AppState| -> Option<ImageAvailability> {
            let viewable = state.image_views.borrow().get_viewable(view_id)?;
            match viewable {
                Viewable::Image(image_id) => {
                    let availability = state.viewables_cache.borrow().get(&image_id);
                    Some(availability)
                }
                Viewable::Plotly(_) => Some(ImageAvailability::PlotlyAvailable(())),
            }
        })
    };

    let inner_element = if let Some(availability) = current_image_availability.as_ref() {
        match availability {
            ImageAvailability::NotAvailable => Some(html! {
                <div>{"No Data"}</div>
            }),
            ImageAvailability::Pending => Some(html! {
                <Spinner />
            }),
            ImageAvailability::ImageAvailable(_) => None,
            ImageAvailability::PlotlyAvailable(_) => Some(html! {
                <div id="my-plot" class={plotly_div} />
            }),
        }
    } else {
        None
    };

    let style = use_style!(
        r#"
        display: flex;
        height: 100%;
        width: 100%;
        justify-content: center;
        align-items: center;
        "#,
    );

    html! {
        <div ref={node_ref.clone()} class={classes!(class.clone(), style)}>
            {inner_element}
        </div>
    }
}
