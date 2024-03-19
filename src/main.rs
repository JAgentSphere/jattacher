use clap::{Parser, Subcommand};

/// A tool for attaching to and diagnosing JVM processes
#[derive(Parser, Debug)]
#[command(name = "jattacher", version, about, long_about = None)]
struct JAttacher {
    /// Process ID of target JVM
    pid: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List Java processes
    Jps {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
        /// Enable dashboard mode
        #[arg(long)]
        dashboard: bool,
    },
    /// Print system properties of the JVM
    Properties,
}

fn main() {
    let args = JAttacher::parse();

    match args.command {
        Commands::Jps { json, dashboard } => {
            if json {
                println!("Listing Java processes in JSON format");
            } else if dashboard {
                println!("Displaying Java process dashboard");
            } else {
                println!("Listing Java processes");
            }
        }
        Commands::Properties => {
            if let Some(pid) = args.pid {
                println!("Printing system properties, PID {pid}");
            } else {
                eprintln!("PID cannot be empty")
            }
        }
    }
}
