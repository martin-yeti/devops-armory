# devops-armory
Rust toolset to improve DevOps work
It is based on russh, toml and actix-web library.

## How to get started ?
As of end of November 2025, you can use Rust stable channel - 1.91.
Install Rust, create project, then add to Cargo.toml in your Rust Project below line:
```
[dependencies]
actix-web = "4.12.1"
devops-armory = "0.3.1"
```

After that, check examples directory and that should get you going. 

## How does it work ? 
It uses toml file as the source data. It is parsed via toml_parser - in case toml file is malformed it will throw error where exactly problem occurs. \
You will have to provide function with location of TOML config file, SSH username and SSH private key path. 

Caveat 1 - Commands are arrays, so if you want to execute single command on single host just put proper index in it (arrays in Rust begin with 0) \
Caveat 2 - IP addresses are also arrays, so again - if you want to use single address just put proper index in it (see above) \
Caveat 3 - GKE endpoint addresses are also arrays, so again keep in mind that [] starts with 0 

In toml directory there are 2 example files: 
1. example toml config - as a basis for further use (keep in mind, that all fields in the file are required - right now it's statically typed, not yet Option-alized) \
2. example json file - JSON representation of toml file, so toml file will be more readable (in case someone is not familiar with TOML) 

Examples can be found in examples directory.

### Current features:
1 - Rustible - tool to execute command on remote server. These are are executed in separate session - in case you need faciliate work you can use "&&" between commands in toml file. \
2 - Cloud GCP - set of functions to interact with Google Cloud Platform. Already implemented: \
    - Auth method (IAM current system login via gcloud or ServiceAccount) \
    - Creating a project \
    - Adding billing to the project \
    - Adding users/modifying roles \
    - Creating a VM \
    - Creating/updating a k8s cluster (selected options), deployments, services, ingress, routes (selected options), and gateways (selected options), creating namespace, creating/modifying/viewing secrets \
    - Creating/updating VPCs - networks, subnets, IPs, network peering, private service connections \
    - Creating/updating SQL instances, dbs, users, passwords \
    - Creating/updating DNS \
    - Creating SSL - Google Managed 

### Coming soon: 
1 - Creating GCS \
2 - Sync GCS to S3 \
3 - GCS cleaning based on date \
4 - GKE logger alert - send notifications to Slack channel if specific phrase appear in the logs 

### Bug reports/Feature requests:  
Please use below formatting for creating issues/bug report/feature requests: 
```
[Rustible] Name of your issue/bug/feature
```
or
```
[Cloud][Gcp] Name of your issue/bug/feature
```

### Docs:
https://docs.cloud.google.com/compute/docs/apis \
https://docs.cloud.google.com/sql/docs/postgres \
https://docs.cloud.google.com/dns/docs/reference/rest/v1 \
https://docs.cloud.google.com/vpc/docs/apis \
https://docs.cloud.google.com/kubernetes-engine/docs/reference 
