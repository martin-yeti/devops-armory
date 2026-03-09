use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::path::Path;

use super::models::{
    LogrotateConfig,
    CreateSpec
};

pub type LogrotateFileMap = HashMap<PathBuf, LogrotateConfig>;

/// Custom serializer for logrotate file 
fn format_entry(path: &PathBuf, cfg: &LogrotateConfig) -> String {
    let mut s = String::new();
    s.push_str(&format!("{} {{\n", path.display()));

    if let Some(f) = &cfg.frequency { s.push_str(&format!("    {}\n", f)); }
    if let Some(r) = cfg.rotate { s.push_str(&format!("    rotate {}\n", r)); }
    if cfg.compress { s.push_str("    compress\n"); }
    if cfg.delaycompress { s.push_str("    delaycompress\n"); }
    if cfg.missingok { s.push_str("    missingok\n"); }
    if cfg.notifempty { s.push_str("    notifempty\n"); }
    if let Some(create) = &cfg.create {
        // format mode as octal with leading zeros (e.g. 0640)
        s.push_str(&format!(
            "    create {:04o} {} {}\n",
            create.mode, create.owner, create.group
        ));
    }
    if cfg.sharedscripts { s.push_str("    sharedscripts\n"); }
    if let Some(post) = &cfg.postrotate {
        s.push_str("    postrotate\n");
        for line in post.lines() {
            s.push_str("        ");
            s.push_str(line);
            s.push('\n');
        }
        s.push_str("    endscript\n");
    }

    s.push_str("}\n\n");
    s
}

/// Function to write parsed data into file
fn write_logrotate_file(path: &std::path::Path, map: &LogrotateFileMap) -> io::Result<()> {
    let mut file = File::create(path)?;
    // If ordering matters, iterate in a deterministic order:
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by_key(|(p, _)| p.clone());
    for (p, cfg) in entries {
        let block = format_entry(p, cfg);
        file.write_all(block.as_bytes())?;
    }
    Ok(())
}

/// Logrotate file creator
/// Create logrotate conf file based on provided function parameters.
pub fn logrotate_parser(
    log_file: &str,
    freq: Option<String>,
    rotation: Option<u32>,
    comp: bool,
    delaycomp: bool,
    missok: bool,
    not_if_empty: bool,
    create_mode: u32,
    create_owner: String,
    create_group: String,
    shared_scripts: bool,
    post_rotate: Option<String>, 
    logrotate_file: &Path
) -> io::Result<()> {
    let mut m = LogrotateFileMap::new();
    m.insert(
        PathBuf::from(log_file),
        LogrotateConfig {
            frequency: freq,
            rotate: rotation,
            compress: comp,
            delaycompress: delaycomp,
            missingok: missok,
            notifempty: not_if_empty,
            create: Some(CreateSpec {
                mode: create_mode,
                owner: create_owner,
                group: create_group,
            }),
            sharedscripts: shared_scripts,
            postrotate: post_rotate,
        },
    );

    write_logrotate_file(std::path::Path::new(logrotate_file), &m)
}
