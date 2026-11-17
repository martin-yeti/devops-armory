# devops-armory

Rust toolset to improve DevOps work. Based on `russh`, `toml`, and `actix-web`.

## Repository Structure

- **`src/`** — Library modules organized by domain
  - `cloud/gcp/` — Google Cloud Platform integration
  - `cloud/hetzner/` — Hetzner Cloud integration
  - `crypt/` — Sodium-based encrypt/decrypt
  - `ini_parser/` — INI file parser (for SystemD)
  - `logrotate/` — logrotate.conf parser
  - `logging/` — Log collector with PostgreSQL storage + web UI
  - `load_balancer/` — Simple iptables-based load balancer/reverse proxy
  - `monitoring/` — Metrics collector (CPU/RAM/health for GKE pods)
  - `rustible/` — Remote SSH command execution (Rustible)
  - `toml_parser/` — TOML config parser with static typing
  - `cloud/mod.rs` — Re-exports `gcp` and `hetzner` modules

- **`examples/`** — Practical usage examples (17 files)
  - GCP: auth, project, billing, compute VM, DNS, GKE, IAM, SQL, SSL, VPC
  - Rustible: remote SSH command, CLI
  - Log collector DB + FE
  - Pod metrics collector (DB + FE)
  - Simple load balancer

- **`toml/`** — Configuration files
  - `config.toml` — Primary config template (all fields required, statically typed)
  - `example.json` — JSON representation of config for readability

- **`Cargo.toml`** — Root crate configuration, version `0.7.9`
- **`Cargo.lock`** — Locked dependencies

## Getting Started

As of Rust stable 1.91, add to your `Cargo.toml`:

```toml
[dependencies]
actix-web = "4.12.1"
devops-armory = "0.7.9"
rustls = { version = "0.23", default-features = false, features = ["aws-lc-rs"] }
tokio = "1"
```

At the beginning of your `main()` function, install the Rustls crypto provider:

```rust
rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
```

Then explore the `examples/` directory — each example demonstrates a specific feature and should get you going.

## How It Works

The library uses TOML files as configuration data, parsed via a static-typed TOML parser. If a TOML file is malformed, the parser reports exactly where the problem occurs.

You must provide:
1. Path to a TOML config file
2. SSH username
3. SSH private key path

### Caveats

- **Commands are arrays** — Rust arrays begin at index 0. For a single command, use index `[0]`.
- **IP addresses are arrays** — Same indexing rule applies.
- **GKE endpoint addresses are arrays** — Index from 0.

## Current Features (v0.7.9)

1. **Rustible** — Execute commands on remote servers via SSH. Commands are executed in separate sessions; use `&&` in TOML to chain commands.

2. **TOML parser** — Parse TOML config files with exact error reporting. Can be used as static configuration storage.

3. **Cloud GCP** — Full Google Cloud Platform integration:
   - Auth: current system login via `gcloud` or ServiceAccount
   - Project creation and billing
   - User/role management
   - VM creation
   - GKE clusters, deployments, services, ingress, routes, gateways
   - Namespace, secrets, PVCs, ConfigMaps management
   - Node/pool info
   - VPCs: networks, subnets, IPs, peering, private service connections
   - SQL: instances, databases, users, passwords
   - DNS management
   - SSL: Google Managed
   - API services enable/disable
   - ServiceAccount creation/modification/deletion
   - GKE Roles/ClusterRoles/RolesBindings
   - Firewall Rules creation/deletion/modification

4. **GKE logger alert** — Send Slack notifications when specific phrases appear in GKE logs.

5. **INI parser** — Create INI files required for SystemD (Ubuntu, Debian, etc.).

6. **Logrotate parser** — Create `logrotate.conf` files for Linux log rotation.

7. **Crypt/Decrypt** — libsodium-based encryption/decryption. Requires server keypair, client keypair, and nonce key (generatable via `keystore()`).

8. **Cloud Hetzner** — Hetzner Cloud integration:
   - Create/Modify/Get/Delete instances/servers
   - Create/Modify/Get/Delete volumes
   - Create/Modify/Get/Delete SSH keys

9. **Simple Load Balancer** — Run load balancing/reverse proxy with custom ports/upstreams/blocked paths and IP filtering. Uses `iptables` wrapped in a bash script.

10. **Log collector** — PostgreSQL-backed log storage with REST API for filtering and a simple web frontend to browse logs.
    - `collector_stdout` — Forward GKE logs to STDOUT
    - `collector_db` — Store GKE logs in PostgreSQL (run migrations via `diesel_logs.toml`)
    - `collector_fe` — Simple HTML web interface for browsing collected logs

11. **Metrics collector** — GKE metrics collection:
    - CPU metrics for pods
    - RAM metrics for pods
    - Pod health (shown regardless of requests/limits)

## Coming Soon

1. Creating GCS
2. Sync GCS to S3
3. GCS cleaning based on date
4. Crypt/Decrypt in runtime
5. Gitlab CI example usage

## Bug Reports / Feature Requests

Use the following formatting:

```
[Rustible] Name of your issue/bug/feature
```

or

```
[Cloud][Gcp] Name of your issue/bug/feature
```

## Documentation

- [Google Cloud Compute APIs](https://docs.cloud.google.com/compute/docs/apis)
- [Google Cloud SQL for Postgres](https://docs.cloud.google.com/sql/docs/postgres)
- [Google Cloud DNS](https://docs.cloud.google.com/dns/docs/reference/rest/v1)
- [Google VPC APIs](https://docs.cloud.google.com/vpc/docs/apis)
- [Google Kubernetes Engine reference](https://docs.cloud.google.com/kubernetes-engine/docs/reference)
- [Hetzner Cloud reference](https://docs.hetzner.cloud/reference/cloud)

## Files Summary

| Directory | Description |
|---|---|
| `examples/` | 17 practical usage examples |
| `src/` | 11 library modules organized by domain |
| `toml/` | 2 config files (`config.toml`, `example.json`) |
| `.gitignore` | Standard git ignore |
| `Cargo.toml` / `Cargo.lock` | Rust project metadata |
| `LICENSE` | MIT License |
| `README.md` | This file |

## Quick Example

```rust
use devops_armory::{
    rustible::vm::vm_remote::vm_remote::exec_command_on_remote,
    toml_parser::parser::toml_parser,
};
use actix_web;

#[actix_web::main]
async fn main() -> std::io::Error {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    let ssh_user = "user".to_string();
    let ssh_key_location = "path_to_your_private_ssh_key".to_string();
    let toml_file = "path_to_your_config.toml".to_string();
    let toml_data = toml_parser(toml_file);
    let f = toml_data.unwrap_or_default();
    let g = &f.rustible[0];
    let slack_vm_address_list = &g.vm[0].slackware.ip_address_list;
    let slack_vm_commands = &g.vm[0].slackware.commands;
    exec_command_on_remote(ssh_user, ssh_key_location, slack_vm_address_list.to_vec(), slack_vm_commands.to_vec()).await
}
```
