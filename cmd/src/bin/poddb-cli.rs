/*
 Copyright 2022 kklinan <kklinang@gmail.com>.

 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at

      https://www.apache.org/licenses/LICENSE-2.0

 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
 */

use clap::{Parser, Subcommand};
 
/// PodDB
#[derive(Parser)]
#[clap(name = "poddb-cli")]
#[clap(version = "0.1.0", about = None)]
#[warn(unreachable_patterns)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    /// Server hostname (default: 127.0.0.1).
    #[clap(short, default_value = "127.0.0.1")]
    hostname: String,

    /// Server port (default: 6379).
    #[clap(short, default_value_t = 6379)]
    port: u16,
}

#[derive(Subcommand)]
enum Commands {
     /// Set a key.
     #[clap(arg_required_else_help = true)]
     Set {
         /// The name of key.
         key: String,
         value: String,
     },
     
    /// Get a key.
    #[clap(arg_required_else_help = true)]
    Get {
        /// The name of key.
        key: String,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Set { key, value } => {
            println!("set {} = {}", key, value);
        }
        Commands::Get { key } => {
            println!("get {}", key);
        }
    }
}
