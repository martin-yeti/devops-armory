use gcp_auth::{CustomServiceAccount, TokenProvider};

/// Below function uses Service Account credentials to generate token
pub async fn gcp_get_credentials_token_sa(project: Option<String>, credentials: Option<String>) -> Result<String, std::io::Error> {

    let token: String = match project {

        Some(p) if !p.is_empty() => {
            let cred = credentials.unwrap_or_default();
            let service_account = CustomServiceAccount::from_file(cred);
            let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
            let token = service_account.expect("No token available").token(scopes).await.expect("Missing scopes for existing token");
            let tok = token.as_str();

            tok.to_owned()
        },
        Some(_) => {
            let unknown_str = "Unknown project selected".to_string();
            unknown_str
        },
        None => {
            let error_msg = "No project provided".to_string();
            error_msg
        }
    };

    Ok(token)

}


/// Below function uses current system log in credentials obtained via gcloud auth login command
pub async fn gcp_get_credentials_token_iam() -> Result<String, std::io::Error> {

    let provider = gcp_auth::provider().await.unwrap();
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = provider.token(scopes).await.unwrap();

    let t = token.as_str();

    Ok(t.to_string())

}

/// Below function uses type of authentication to GCP account you want to use - you can choose either
pub async fn gcp_get_authentication_method(auth_method: String) -> Result<(), std::io::Error> {

    match auth_method.trim() {

        "Service Account" => {
            gcp_get_credentials_token_sa(None, None).await.unwrap_or_default();
        },
        "IAM" => {
            gcp_get_credentials_token_iam().await.unwrap_or_default();
        },
        _ => {
            eprintln!("Unkown authentication method to GCP")
        }
        
    }

    Ok(())

}

