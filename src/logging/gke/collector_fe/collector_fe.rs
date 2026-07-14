use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use awc::Client;
use super::models::{
    InputForm,
    Log
};

fn date_from_rfc3339(date: &str) -> String {
    if date.is_empty() {
        "1970-01-01T00:00:00.000000000Z".to_string()
    } else {
        format!("{date}T00:00:00.000000000Z")
    }
}

fn date_to_rfc3339(date: &str) -> String {
    if date.is_empty() {
        "9999-12-31T23:59:59.999999999Z".to_string()
    } else {
        format!("{date}T23:59:59.999999999Z")
    }
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[get("/")]
async fn index() -> impl Responder {
    let html = r#"
        <!doctype html>
        <html>
          <body>
            <form method="post" action="/submit">
              <input name="google_project_id" type="text" placeholder="Google project ID" />
              <input name="project_id" type="text" placeholder="Project ID" />
              <input name="region" type="text" placeholder="Region" />
              <input name="host" type="text" placeholder="Host" />
              <input name="message" type="text" placeholder="Message" />
              <input name="date_from" type="date" placeholder="Date from" />
              <input name="date_to" type="date" placeholder="Date to" />
              <button type="submit">Send</button>
            </form>
          </body>
        </html>
    "#;

    HttpResponse::Ok().content_type("text/html").body(html)
}

#[post("/submit")]
async fn submit(
    form: web::Form<InputForm>,
    api_base_url: String
) -> impl Responder {
    let api_base = api_base_url;
    let url = format!(
        "{}/{}?project_id={}&region={}&host={}&message={}&date_from={}&date_to={}&offset=0",
        api_base,
        urlencoding::encode(&form.google_project_id),
        urlencoding::encode(&form.project_id),
        urlencoding::encode(&form.region),
        urlencoding::encode(&form.host),
        urlencoding::encode(&form.message),
        urlencoding::encode(&date_from_rfc3339(&form.date_from)),
        urlencoding::encode(&date_to_rfc3339(&form.date_to)),
    );

    let client = Client::default();

    let result = client
        .get(url)
        .send()
        .await;

    let mut resp = match result {
        Ok(r) => r,
        Err(e) => return HttpResponse::InternalServerError().body(format!("API request failed: {e}")),
    };

    let status = resp.status();

    let logs: Vec<Log> = match resp.json().limit(64 * 1024 * 1024).await {
        Ok(logs) => logs,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Read API body failed: {e}")),
    };

    let rows: String = logs
        .iter()
        .map(|log| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                log.id,
                escape_html(&log.google_project_id),
                escape_html(&log.project_id),
                escape_html(&log.region),
                escape_html(&log.host),
                escape_html(&log.message),
                escape_html(&log.time),
            )
        })
        .collect();

    let html = format!(
        r#"
        <!doctype html>
        <html>
          <head>
            <style>
              table {{ border-collapse: collapse; width: 100%; table-layout: fixed; }}
              th, td {{
                border: 1px solid #ccc;
                padding: 4px 8px;
                text-align: left;
                overflow: hidden;
                text-overflow: ellipsis;
                white-space: nowrap;
              }}
              td:nth-child(6) {{
                overflow: visible;
                text-overflow: unset;
                white-space: normal;
                word-break: break-word;
              }}
              th:nth-child(1), td:nth-child(1) {{ width: 5%; }}
              th:nth-child(2), td:nth-child(2) {{ width: 7.5%; }}
              th:nth-child(3), td:nth-child(3) {{ width: 7.5%; }}
              th:nth-child(4), td:nth-child(4) {{ width: 10%; }}
              th:nth-child(5), td:nth-child(5) {{ width: 15%; }}
              th:nth-child(6), td:nth-child(6) {{ width: 45%; }}
              th:nth-child(7), td:nth-child(7) {{ width: 10%; }}
            </style>
          </head>
          <body>
            <form method="post" action="/submit">
              <input name="google_project_id" type="text" placeholder="Google project ID" value="{google_project_id}" />
              <input name="project_id" type="text" placeholder="Project ID" value="{project_id}" />
              <input name="region" type="text" placeholder="Region" value="{region}" />
              <input name="host" type="text" placeholder="Host" value="{host}" />
              <input name="message" type="text" placeholder="Message" value="{message}" />
              <input name="date_from" type="date" placeholder="Date from" value="{date_from}" />
              <input name="date_to" type="date" placeholder="Date to" value="{date_to}" />
              <button type="submit">Send</button>
            </form>
            <table>
              <thead>
                <tr>
                  <th>ID</th>
                  <th>Google Project ID</th>
                  <th>Project ID</th>
                  <th>Region</th>
                  <th>Host</th>
                  <th>Message</th>
                  <th>Time</th>
                </tr>
              </thead>
              <tbody>
                {rows}
              </tbody>
            </table>
          </body>
        </html>
        "#,
        google_project_id = escape_html(&form.google_project_id),
        project_id = escape_html(&form.project_id),
        region = escape_html(&form.region),
        host = escape_html(&form.host),
        message = escape_html(&form.message),
        date_from = escape_html(&form.date_from),
        date_to = escape_html(&form.date_to),
    );

    HttpResponse::build(status)
        .content_type("text/html")
        .body(html)
}

pub async fn collector_fe() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(submit))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
