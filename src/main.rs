use std::collections::HashSet;
use std::env;
use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
struct AliasEntry {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Source")]
    source: Option<String>,
}

fn ps_single_quote(value: &str) -> String {
    value.replace('\'', "''")
}

fn main() {
    let keep_aliases: HashSet<String> = env::args().skip(1).collect();

    let output = Command::new("pwsh")
        .args([
            "-NonInteractive",
            "-NoProfile",
            "-NoLogo",
            "-Command",
            "Import-Module Microsoft.PowerShell.Management -ErrorAction SilentlyContinue; $a = Get-Alias | Select-Object Name, Source; @($a) | ConvertTo-Json -Compress",
        ])
        .output()
        .unwrap_or_else(|err| {
            eprintln!("failed to run pwsh: {err}");
            std::process::exit(1);
        });

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("failed to read aliases from pwsh: {stderr}");
        std::process::exit(1);
    }

    let aliases_json = String::from_utf8_lossy(&output.stdout);
    let aliases: Vec<AliasEntry> =
        serde_json::from_str(aliases_json.trim()).unwrap_or_else(|err| {
            eprintln!("failed to parse aliases json: {err}");
            std::process::exit(1);
        });

    let remove_list = aliases
        .iter()
        .map(|alias| alias.name.as_str())
        .filter(|name| !keep_aliases.contains(*name))
        .map(|name| format!("'{}'", ps_single_quote(name)))
        .collect::<Vec<String>>()
        .join(",");

    let sourced_remove_list = aliases
        .iter()
        .filter(|alias| {
            alias
                .source
                .as_ref()
                .is_some_and(|value| !value.trim().is_empty())
        })
        .map(|alias| alias.name.as_str())
        .filter(|name| !keep_aliases.contains(*name))
        .map(|name| format!("'{}'", ps_single_quote(name)))
        .collect::<Vec<String>>()
        .join(",");

    println!("Remove-Alias -Name @({remove_list}) -Force -ErrorAction SilentlyContinue");
    println!("Remove-Alias -Name @({sourced_remove_list}) -Force -ErrorAction SilentlyContinue");
}
