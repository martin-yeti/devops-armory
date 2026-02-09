use std::{
    collections::HashSet, 
    time::Duration
};
use super::models::{
    SetPolicy,
    PolicyBindings,
    GcpProjectIam
};

/// Get IAM GCP project etag
/// Required for IAM policy modification
async fn get_iam_gcp_project_etag(
    token: String,
    project: String
) -> Result<String, std::io::Error> {

    let client = awc::Client::default();
    let mut request = client
        .post(format!("https://cloudresourcemanager.googleapis.com/v3/projects/{project}:getIamPolicy"))
        .bearer_auth(token)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request: GET Project IAM E-Tag could not been sent");

    let body: SetPolicy = request.json::<SetPolicy>().await.unwrap_or_default();

    let e_tag = body.etag;

    Ok(e_tag)

}

/// Get Policy bindings for existing users
async fn get_iam_gcp_project_policy(
    token: String,
    project: String
) -> Result<Vec<PolicyBindings>, std::io::Error> {

    let client = awc::Client::default();
    let mut request = client
        .post(format!("https://cloudresourcemanager.googleapis.com/v3/projects/{project}:getIamPolicy"))
        .bearer_auth(token)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("Request: GET Project IAM List could not been sent");

    let body: SetPolicy = request.json::<SetPolicy>().await.unwrap_or_default();

    let policy_bindings_vec = body.bindings;

    Ok(policy_bindings_vec)

}

/// Update IAM policy
/// Add roles/members to existing set
pub async fn update_iam_gcp_project_policy(
    token: String,
    project: String,
    iam_role: String,
    iam_members: Vec<String>
) {

    let etag = get_iam_gcp_project_etag(
        token.clone(),
        project.clone()
    ).await.unwrap_or_default();

    let current_policy = get_iam_gcp_project_policy(
        token.clone(),
        project.clone()
    ).await.unwrap_or_default();

    let mut new_policy: HashSet<PolicyBindings> = HashSet::from_iter(current_policy);
    
    let iam_data: PolicyBindings = PolicyBindings { 
        role: iam_role, 
        members: iam_members 
    };

    let modified_hashset = new_policy.insert(iam_data);

    let modified_vec: Vec<PolicyBindings> = new_policy.into_iter().collect();
    

    let project_policy: GcpProjectIam = GcpProjectIam { 
        policy: SetPolicy { 
            bindings: modified_vec, 
            etag: etag,
            version: 3
        } 
    };

    let client = awc::Client::default();
    let request = client
        .post(format!("https://cloudresourcemanager.googleapis.com/v3/projects/{project}:setIamPolicy"))
        .bearer_auth(token)
        .timeout(Duration::from_secs(30))
        .send_json(&project_policy)
        .await
        .expect("Request: POST Project IAM List could not been sent");
    
    let mut req = request;
    let req_status = req.status().as_u16();
    let respone = req.body().await.unwrap_or_default();

    match req_status {

        200 => {
            println!("Request has been successfull: Status: {:?}, {:?}", req_status, respone);
        },
        400 => {
            println!("Bad Request. Check URL parameters or body: {:?}", respone);
        },
        403 => {
            println!("You don't have access to perform such request: {:?}", respone);
        }
        404 => {
            println!("Requested resource does not exists: {:?}", respone);
        },
        409 => {
            println!("Requested resource already exists! {:?}", respone)
        }
        _ => {
            println!("Request status mismatch. Check response: {:?}", respone);
        }

    }

}

