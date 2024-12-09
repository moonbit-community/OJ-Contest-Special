// moon: The build system and package manager for MoonBit.
// Copyright (C) 2024 International Digital Economy Academy
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// For inquiries, you can contact us via e-mail at jichuruanjian@idea.edu.cn.

use std::path::PathBuf;

use anyhow::Context;

#[derive(Debug, clap::Parser)]
pub struct Embed {
    #[clap(long, conflicts_with = "text")]
    binary: bool,
    #[clap(long, conflicts_with = "binary")]
    text: bool,
    #[clap(long, short)]
    input: PathBuf,
    #[clap(long, short)]
    output: PathBuf,
    #[clap(long)]
    name: Option<String>,
    #[clap(long)]
    timestamp: bool,
}

pub fn run_embed_text(cmd: Embed) -> anyhow::Result<i32> {
    let input = std::fs::read_to_string(&cmd.input)?;
    let name = cmd.name.unwrap_or_else(|| "resource".to_string());
    let mut content = format!(
        r#"// Generated by `moon tool embed --text`{}, do not edit.

///|
let {} : String =
"#,
        if cmd.timestamp {
            format!(" on {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))
        } else {
            String::new()
        },
        name
    );
    for line in input.split('\n') {
        content.push_str(&format!("  #|{}\n", line));
    }
    std::fs::write(cmd.output, content).context("write output file")?;
    Ok(0)
}

pub fn run_embed_bin(cmd: Embed) -> anyhow::Result<i32> {
    let input = std::fs::read(&cmd.input)?;
    let name = cmd.name.unwrap_or_else(|| "resource".to_string());
    let mut content = format!(
        r#"// Generated by `moon tool embed --binary`{}, do not edit.

///|
let {} : Bytes = Bytes::of([
"#,
        if cmd.timestamp {
            format!(" on {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))
        } else {
            String::new()
        },
        name
    );

    for (i, byte) in input.iter().enumerate() {
        if i % 12 == 0 {
            if i > 0 {
                content.push('\n');
            }
            content.push_str("  ");
        }
        content.push_str(&format!("0x{:02x}, ", byte));
    }
    content.push_str("\n])\n");
    std::fs::write(cmd.output, content).context("write output file")?;
    Ok(0)
}

pub fn run_embed(cmd: Embed) -> anyhow::Result<i32> {
    if cmd.binary {
        run_embed_bin(cmd)
    } else {
        run_embed_text(cmd)
    }
}
