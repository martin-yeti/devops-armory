use vertigo::{component, css, dom, Value};
use vertigo_forms::Input;

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
