use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use awc::Client;
use super::models::{
    InputForm,
    PodMetric
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
              <input name="namespace" type="text" placeholder="Namespace" />
              <input name="pod_name" type="text" placeholder="Pod name" />
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
    api_base_url: web::Data<String>
) -> impl Responder {
    let url = format!(
        "{}/{}?project_id={}&region={}&namespace={}&pod_name={}&date_from={}&date_to={}&offset=0",
        api_base_url.get_ref(),
        urlencoding::encode(&form.google_project_id),
        urlencoding::encode(&form.project_id),
        urlencoding::encode(&form.region),
        urlencoding::encode(&form.namespace),
        urlencoding::encode(&form.pod_name),
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

    let metrics: Vec<PodMetric> = match resp.json().limit(64 * 1024 * 1024).await {
        Ok(metrics) => metrics,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Read API body failed: {e}")),
    };

    let rows: String = metrics
        .iter()
        .map(|metric| {
            let row_style = if metric.healthy { "" } else { " style=\"background: #fdd\"" };
            let healthy = if metric.healthy { "&check;" } else { "&cross;" };
            let reason = metric.reason.as_deref().unwrap_or("-");
            let time = metric.time.map(|t| t.to_rfc3339()).unwrap_or_default();

            format!(
                "<tr{}><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{:.3}</td><td>{:.1}</td><td>{:.3}</td><td>{:.1}</td><td>{}</td><td>{}</td><td>{:.3}</td><td>{:.1}</td><td>{}</td></tr>",
                row_style,
                metric.id,
                escape_html(&metric.google_project_id),
                escape_html(&metric.project_id),
                escape_html(&metric.region),
                escape_html(&metric.namespace),
                escape_html(&metric.pod_name),
                metric.cpu_request,
                metric.ram_request,
                metric.cpu_limit,
                metric.ram_limit,
                healthy,
                escape_html(reason),
                metric.cpu_usage,
                metric.ram_usage,
                escape_html(&time),
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
              th:nth-child(1), td:nth-child(1) {{ width: 4%; }}
              th:nth-child(2), td:nth-child(2) {{ width: 7%; }}
              th:nth-child(3), td:nth-child(3) {{ width: 7%; }}
              th:nth-child(4), td:nth-child(4) {{ width: 5%; }}
              th:nth-child(5), td:nth-child(5) {{ width: 7%; }}
              th:nth-child(6), td:nth-child(6) {{ width: 11%; }}
              th:nth-child(7), td:nth-child(7) {{ width: 5%; }}
              th:nth-child(8), td:nth-child(8) {{ width: 6%; }}
              th:nth-child(9), td:nth-child(9) {{ width: 5%; }}
              th:nth-child(10), td:nth-child(10) {{ width: 6%; }}
              th:nth-child(11), td:nth-child(11) {{ width: 5%; }}
              th:nth-child(12), td:nth-child(12) {{
                width: 11%;
                overflow: visible;
                text-overflow: unset;
                white-space: normal;
                word-break: break-word;
              }}
              th:nth-child(13), td:nth-child(13) {{ width: 6%; }}
              th:nth-child(14), td:nth-child(14) {{ width: 6%; }}
              th:nth-child(15), td:nth-child(15) {{ width: 9%; }}
            </style>
          </head>
          <body>
            <form method="post" action="/submit">
              <input name="google_project_id" type="text" placeholder="Google project ID" value="{google_project_id}" />
              <input name="project_id" type="text" placeholder="Project ID" value="{project_id}" />
              <input name="region" type="text" placeholder="Region" value="{region}" />
              <input name="namespace" type="text" placeholder="Namespace" value="{namespace}" />
              <input name="pod_name" type="text" placeholder="Pod name" value="{pod_name}" />
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
                  <th>Namespace</th>
                  <th>Pod Name</th>
                  <th>CPU Request (cores)</th>
                  <th>RAM Request (MiB)</th>
                  <th>CPU Limit (cores)</th>
                  <th>RAM Limit (MiB)</th>
                  <th>Healthy</th>
                  <th>Reason</th>
                  <th>CPU Usage (ratio)</th>
                  <th>RAM Usage (&permil;)</th>
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
        namespace = escape_html(&form.namespace),
        pod_name = escape_html(&form.pod_name),
        date_from = escape_html(&form.date_from),
        date_to = escape_html(&form.date_to),
    );

    HttpResponse::build(status)
        .content_type("text/html")
        .body(html)
}

/// Simple web UI to browse collected pod cpu/ram metrics
/// `api_base_url` must point at the pod-metrics collector API, e.g.
/// "http://127.0.0.1:8889/pod-metrics"
pub async fn collector_fe(api_base_url: String) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(api_base_url.clone()))
            .service(index)
            .service(submit)
    })
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}
