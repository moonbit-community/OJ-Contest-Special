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

use clap;
use clap::Parser;
use std::path::PathBuf;

mod cmdtest;
mod sync_docs;

#[derive(Debug, clap::Parser)]
struct Cli {
    #[clap(subcommand)]
    pub subcommand: XSubcommands,
}

#[derive(Debug, clap::Parser)]
enum XSubcommands {
    #[command(name = "cmdtest")]
    CmdTest(CmdTest),

    #[command(name = "sync-docs")]
    SyncDocs(SyncDocs),
}

#[derive(Debug, clap::Parser)]
struct CmdTest {
    file: PathBuf,

    #[arg(short, long)]
    update: bool,
}

#[derive(Debug, clap::Parser)]
struct SyncDocs {
    #[arg(long)]
    moonbit_docs_dir: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let code = match cli.subcommand {
        XSubcommands::CmdTest(t) => cmdtest::run::t(&t.file, t.update),
        XSubcommands::SyncDocs(t) => sync_docs::run(&t.moonbit_docs_dir).map_or(1, |_| 0),
    };
    std::process::exit(code);
}
