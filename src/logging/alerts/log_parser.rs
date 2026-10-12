use std::collections::{HashSet, VecDeque};
use std::thread::sleep;
use std::time::{self, Duration, Instant};

use awc::*;
use futures::stream::select_all;
use futures::{StreamExt, TryStreamExt};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use tuple_conv::*;

use super::models::{
    Pod,
    Log,
    Notification
};

/// Function which parse logs from k8s and send notifications to Slack channel on every match
/// Token, project, k8s data and slack data need to be provided
pub async fn gke_log_parser(
    token: String,
    project_name: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_region: String,
    gke_k8s_hostname: String,
    gke_log_message: Vec<String>,
    slack_webhook_url: String,
    slack_channel: String,
    slack_username: String,
    slack_message_text: String,
    slack_notified_users: Vec<String>,
    slack_icon_emoji: String
) -> Result<(), Box<dyn std::error::Error>> {
    gke_log_parser_inner(
        token,
        project_name,
        gke_cluster_endpoint,
        gke_cluster_namespace,
        gke_cluster_region,
        gke_k8s_hostname,
        gke_log_message,
        slack_webhook_url,
        slack_channel,
        slack_username,
        slack_message_text,
        slack_notified_users,
        slack_icon_emoji,
        None
    ).await
}

/// Function which parse logs from k8s and send notification to Slack channel
/// only when matched messages appear more than `req_count` times within `time_window`
/// Token, project, k8s data and slack data need to be provided
pub async fn gke_log_parser_threshold(
    token: String,
    project_name: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_region: String,
    gke_k8s_hostname: String,
    gke_log_message: Vec<String>,
    req_count: usize,
    time_window: Duration,
    slack_webhook_url: String,
    slack_channel: String,
    slack_username: String,
    slack_message_text: String,
    slack_notified_users: Vec<String>,
    slack_icon_emoji: String
) -> Result<(), Box<dyn std::error::Error>> {
    gke_log_parser_inner(
        token,
        project_name,
        gke_cluster_endpoint,
        gke_cluster_namespace,
        gke_cluster_region,
        gke_k8s_hostname,
        gke_log_message,
        slack_webhook_url,
        slack_channel,
        slack_username,
        slack_message_text,
        slack_notified_users,
        slack_icon_emoji,
        Some((req_count, time_window))
    ).await
}

async fn send_slack_notification(slack_webhook_url: &str, notification: &Notification) {
    let mut builder2 = SslConnector::builder(SslMethod::tls()).unwrap();
    builder2.set_verify(SslVerifyMode::NONE);

    let myconnector2 = builder2.build();

    let client_not = Client::builder()
        .connector(Connector::new().openssl(myconnector2))
        .finish();

    let slack_notification_request = client_not
        .post(format!("{slack_webhook_url}"))
        .timeout(Duration::from_secs(120))
        .send_json(notification)
        .await
        .expect("Failed send request");
    let req_status = slack_notification_request.status();
    match req_status.as_u16() {
        200 => {
            println!("Request status OK")
        },
        _ => {
            eprintln!("Request status unknown: {}", req_status)
        }
    };
}

async fn gke_log_parser_inner(
    token: String,
    project_name: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_region: String,
    gke_k8s_hostname: String,
    gke_log_message: Vec<String>,
    slack_webhook_url: String,
    slack_channel: String,
    slack_username: String,
    slack_message_text: String,
    slack_notified_users: Vec<String>,
    slack_icon_emoji: String,
    threshold: Option<(usize, Duration)>
) -> Result<(), Box<dyn std::error::Error>> {

    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);

    let myconnector = builder.build();

    let client = Client::builder()
        .connector(Connector::new().openssl(myconnector))
        .finish();

    let pod_list = client
        .get(format!("https://{gke_cluster_endpoint}/api/v1/namespaces/{gke_cluster_namespace}/pods"))
        .bearer_auth(&token)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Failed to get pods in current namespace")
        .json::<Pod>()
        .await;

    let mut streams = Vec::new();

    let filtered_k8s_hostname = gke_k8s_hostname.as_str();

    for pod in &pod_list {
        for hostname in &pod.hostname {
            if hostname.contains(filtered_k8s_hostname) {
                let res = client
                    .get(&format!("https://{gke_cluster_endpoint}/api/v1/namespaces/{gke_cluster_namespace}/pods/{hostname}/log?&tailLines=10&follow&timestamps=true"))   // <- Create request builder
                    .bearer_auth(&token.clone())
                    .insert_header(("Content-Type", "application/json"))
                    .send()
                    .await
                    .expect("Fail to connect to stream")
                    .into_stream();

                streams.push(res);
            }
        }
    }

    let mut combined_stream = select_all(streams);

    // Timestamps of matched log lines within the current time window
    let mut matches: VecDeque<Instant> = VecDeque::new();

    loop {
        match combined_stream.next().await {
            Some(chunk) => match chunk {
                Ok(chunk_bytes) => {
                    if let Some((req_count, time_window)) = threshold {
                        let now = Instant::now();
                        let chunk_string = String::from_utf8_lossy(&chunk_bytes);
                        let mut last_message = None;

                        // Chunk may contain multiple lines in format: "<timestamp> <message>"
                        for line in chunk_string.lines() {
                            let log_message = line
                                .split_once(char::is_whitespace)
                                .map(|(_, message)| message)
                                .unwrap_or_default();

                            if gke_log_message.iter().any(|s| log_message.contains(s)) {
                                matches.push_back(now);
                                last_message = Some(log_message.to_string());
                            }
                        }

                        while matches.front().is_some_and(|t| now.duration_since(*t) > time_window) {
                            matches.pop_front();
                        }

                        if let Some(log_message) = last_message.filter(|_| matches.len() > req_count) {
                            let not = Notification {
                                channel: slack_channel.clone(),
                                username: slack_username.clone(),
                                text: format!(
                                    "{slack_message_text} {:?} {} matches within {}s, last: {log_message}",
                                    slack_notified_users,
                                    matches.len(),
                                    time_window.as_secs()
                                ),
                                icon_emoji: slack_icon_emoji.to_string(),
                            };

                            send_slack_notification(&slack_webhook_url, &not).await;

                            // Start counting from scratch, so one burst produces one alert
                            matches.clear();
                        }

                        continue;
                    }

                    let mut host_name = "".to_string();

                    let mut unique_hash: HashSet<Notification> = HashSet::new();

                    for pod in &pod_list {
                        for hostname in &pod.hostname {
                            if hostname.contains(filtered_k8s_hostname) {
                                let gcp_id = project_name.as_str();
                                let gcp_region = gke_cluster_region.as_str();
                                let project_name = project_name.as_str();
                                host_name = hostname.to_owned();
                                let chunk_string =
                                    std::str::from_utf8(&chunk_bytes).expect("Non-UTF8 bytes");
                                let iter = chunk_string.split_once(char::is_whitespace);

                                let vec = iter.unwrap_or_default().to_vec();

                                let log = Log {
                                    time: vec.get(0).map(|x| x.to_string()).unwrap_or_default(),
                                    message: vec.get(1).map(|x| x.to_string()).unwrap_or_default(),
                                    host: host_name.to_string(),
                                    google_project_id: gcp_id.to_string(),
                                    region: gcp_region.to_string(),
                                    project_id: project_name.to_string(),
                                };

                                let log_message = log.message;

                                let msg_pattern = gke_log_message.iter().any(|s| log_message.contains(s));

                                match msg_pattern {
                                    true => {
                                        let not = Notification {
                                            channel: slack_channel.clone(),
                                            username: slack_username.clone(),
                                            text: format!("{slack_message_text} {:?} {log_message}", slack_notified_users),
                                            icon_emoji: slack_icon_emoji.to_string(),
                                        };

                                        unique_hash.insert(not.clone());

                                    },
                                    false => {
                                        // If there is no match on message, action below. 
                                        // At the momment, there is no point of putting missed match on STDOUT
                                        //println!("No message pattern found: {:?}", gke_log_message);
                                    }
                                }

                            }

                        }

                    }

                    match !unique_hash.is_empty() {
                        true => {
                            let current_hash: Vec<Notification> = unique_hash.into_iter().collect();
                            let x = &current_hash[0];

                            send_slack_notification(&slack_webhook_url, x).await;
                        },
                        false => {
                            // If there is empty hash - no match on message, then print below
                            // In high traffic environments, there's no need to print below.
                            // Can be improved in further development
                            //println!("Unique hash is empty. Nothing to alert");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Failed to read stream chunk: {}", err)
                }
            },
            None => {
                break Err("Reached end of stream".into());
            }
        }
    }
}

/// Log parser loop with error handling and reconnecting
pub async fn gke_log_parser_loop(
    token: String,
    project_name: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_region: String,
    gke_k8s_hostname: String,
    gke_log_message: Vec<String>,
    slack_webhook_url: String,
    slack_channel: String,
    slack_username: String,
    slack_message_text: String,
    slack_notified_users: Vec<String>,
    slack_icon_emoji: String
    ) {
    let dur = time::Duration::from_secs(60);
    loop {
        match gke_log_parser(
            token.clone(),
            project_name.clone(),
            gke_cluster_endpoint.clone(),
            gke_cluster_namespace.clone(),
            gke_cluster_region.clone(),
            gke_k8s_hostname.clone(),
            gke_log_message.clone(),
            slack_webhook_url.clone(),
            slack_channel.clone(),
            slack_username.clone(),
            slack_message_text.clone(),
            slack_notified_users.clone(),
            slack_icon_emoji.clone(),
            ).await {
                Ok(()) => {
                    println!("Stream working as expected. Proceeding...");
                }
                Err(e) => {
                    eprintln!("Unexpected end of stream - {e}. Retrying...");
                    sleep(dur);
                    break;
                }
            }
    }
}

/// Log parser loop with threshold (`req_count` matches within `time_window`), error handling and reconnecting
pub async fn gke_log_parser_threshold_loop(
    token: String,
    project_name: String,
    gke_cluster_endpoint: String,
    gke_cluster_namespace: String,
    gke_cluster_region: String,
    gke_k8s_hostname: String,
    gke_log_message: Vec<String>,
    req_count: usize,
    time_window: Duration,
    slack_webhook_url: String,
    slack_channel: String,
    slack_username: String,
    slack_message_text: String,
    slack_notified_users: Vec<String>,
    slack_icon_emoji: String
    ) {
    let dur = time::Duration::from_secs(60);
    loop {
        match gke_log_parser_threshold(
            token.clone(),
            project_name.clone(),
            gke_cluster_endpoint.clone(),
            gke_cluster_namespace.clone(),
            gke_cluster_region.clone(),
            gke_k8s_hostname.clone(),
            gke_log_message.clone(),
            req_count,
            time_window,
            slack_webhook_url.clone(),
            slack_channel.clone(),
            slack_username.clone(),
            slack_message_text.clone(),
            slack_notified_users.clone(),
            slack_icon_emoji.clone(),
            ).await {
                Ok(()) => {
                    println!("Stream working as expected. Proceeding...");
                }
                Err(e) => {
                    eprintln!("Unexpected end of stream - {e}. Retrying...");
                    sleep(dur);
                    break;
                }
            }
    }
}

