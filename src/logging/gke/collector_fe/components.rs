use vertigo::{component, css, dom, Value, Resource};
use vertigo_forms::{Input, InputWithButton, InputWithButtonParams};
use std::rc::Rc;
use super::state::ApiData;

#[derive(Default)]
pub struct FilterLogs {
    pub gcp_idd: Value<String>,
    pub project_idd: Value<String>,
}

#[component]
pub fn Filters(gcp_idd: Value<String>, project_idd: Value<String>) {

    let menu_layout = css! {"
        display: flex;
        flex-wrap: wrap;
        gap: 10px 50px;
    "};

    dom! {
        <div>
            <div css={menu_layout}>
                    <p>
                    <Input value={&gcp_idd}/><Input value={&project_idd}/>
                    </p>
            </div>
        </div>
    }
}

#[component]
pub fn Dashboard(api_data: Rc<ApiData>) {
    let config_view = &api_data.log_view;
    //let route = route.clone();

    let api_data_clone = api_data.clone();
    let config_view_clone = config_view.clone();
    let html_clients = api_data.log_view.render_value(move |logs| {
        let html_client_list = match logs {
            Resource::Ready(log) => log
                .into_iter()
                .map(|c| {
                    let row_css = css! {"
                        border: 1px solid yellow;
                        border-collapse: collapse;
                        width: auto;
                    "};
                    dom! {                 
                        <tr css={row_css}><td>{c.id}</td><td>{c.google_project_id}</td><td>{c.project_id}</td><td>{c.message}</td></tr>
                    }
                })
                .collect(),
            _ => vec![],
        };

        let table_css = css! {"
            border: 1px solid gray;
            border-collapse: collapse;
            width: auto;
        "};


        dom! {
            <table css={table_css}>
            <tr><th>"ID"</th><th>"GCP Project ID"</th><th>"Project Name"</th><th>"Message"</th></tr>
                {..html_client_list}
            </table>

        }
    });

    let filter_state = &api_data.filter_state;
    let iframe_view = &filter_state.gcp_idd;
    let filters = {
        dom! {
            <div>
                <div>
                        <p>
                        <InputWithButton value={&filter_state.gcp_idd}
                        params={InputWithButtonParams {
                            //input_css: css!("width: 300px;"),
                            button_label: "Load".to_string(),
                            ..Default::default()
                        }}
                        />
                        </p>
                </div>
            </div>
        }
    };

    let h4 = css! {"
        text-transform: uppercase;
        font-weight: bold;
        margin-top: 15px;
    "};

    dom! {
        <div>
            <h4 css={h4.clone()}>"Logs View"</h4>
            <p>{filters}</p>
            {html_clients}
            //<h4 css={h4}>"Config View"</h4>
            //{html_configs}
        </div>
    }
}

