use crate::args::parser::{parse_port_range, PortRange};
use clap::Parser;
use std::{net::IpAddr, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about = "Simple and fast port scanner built in Rust.")]
struct CliArgs {
    /// Target ip address(IPv4, IPv6)
    #[arg(default_value = "127.0.0.1")]
    address: IpAddr,

    /// Ports of target ip address(1-1024,3000-4000)
    #[arg(
        short,
        long,
        value_name = "target_ports",
        default_value = "1-1024",
        conflicts_with = "all_ports"
    )]
    ports: String,

    ///All ports scan flag(1~65535)
    #[arg(short, long, default_value_t = false, conflicts_with = "ports")]
    all_ports: bool,

    /// Number of threads used for scanning
    #[arg(
        short = 'n',
        long,
        value_name = "number_of_threads",
        default_value_t = 1
    )]
    threads: usize,

    /// Specifies the timeout in milliseconds for each port scan
    #[arg(long = "timeout", value_name = "timeout_ms", default_value_t = 1000)]
    timeout: u32,

    /// File name to save the scan results
    #[arg(short, long, value_name = "output_file_name")]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub address: IpAddr,
    pub ports: PortRange,
    pub threads: usize,
    pub timeout: u32,
    pub output: Option<PathBuf>,
}

#[allow(clippy::new_without_default)]
impl Args {
    pub fn new() -> Self {
        let cli = CliArgs::parse();

        let target_ports = if cli.all_ports {
            "1-65535".to_string()
        } else {
            cli.ports
        };

        let ports = parse_port_range(target_ports).expect("Failed to parse ports range.");

        Self {
            ports,
            address: cli.address,
            threads: cli.threads,
            timeout: cli.timeout,
            output: cli.output,
        }
    }
}

// Unit tests >------------------------------------------------------------<
// #[cfg(test)]
// mod tests {
//     use std::process::Command;

//     use super::*;

//     #[test]
//     fn test_args_new() {
//         let output = Command::new("target/debug/port_hawk")
//             .arg("127.0.0.1")
//             .arg("--ports")
//             .arg("1-65535")
//             .arg("--threads")
//             .arg("1")
//             .arg("--timeout")
//             .arg("1000")
//             .output()
//             .expect("Failed to execute command");

//         assert!(output.status.success());
//     }
// }
