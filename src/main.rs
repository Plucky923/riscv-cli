use anyhow::Result;
use clap::Parser;
use riscv_decode::decode;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "riscv cli", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "disassemble", about = "Disassemble riscv instructions")]
    Disassemble(DisassembleOpts),
}

#[derive(Parser, Debug)]
pub struct DisassembleOpts {
    #[arg(short, long)]
    input: String,
}

pub fn disassemble(input: &str) -> Result<String> {
    let instr = input_to_u32(input)?;
    let instr = decode(instr);
    Ok(format!("{:?}", instr))
}

fn input_to_u32(hex_str: &str) -> Result<u32, std::num::ParseIntError> {
    // Remove the 0x prefix if it exists
    let trimmed_str = if hex_str.starts_with("0x") || hex_str.starts_with("0X") {
        &hex_str[2..]
    } else {
        hex_str
    };

    u32::from_str_radix(trimmed_str, 16)
}

fn main() {
    let args = Cli::parse();
    match args.cmd {
        SubCommand::Disassemble(opts) => {
            let disassembled = disassemble(&opts.input);
            println!("{:?}", disassembled);
        }
    }
}
