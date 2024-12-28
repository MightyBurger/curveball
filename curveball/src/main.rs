use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use curveball_lib::curve::{Bank, Curve, CurveResult, Rayto};
use curveball_lib::map::{Brush, QEntity, QMap, SimpleWorldspawn};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate a curve with rays from a circle segment to a single point")]
    Rayto(RaytoArgs),
    #[command(about = "Generate a banked curve")]
    Bank(BankArgs),
}

#[derive(Args)]
struct RaytoArgs {
    #[arg(long, help = "Number of segments")]
    n: u32,
    #[arg(long, help = "Starting radius")]
    r0: f64,
    #[arg(long, help = "Ending radius")]
    r1: f64,
    #[arg(long, help = "Starting angle (deg)")]
    theta0: f64,
    #[arg(long, help = "Ending angle (deg)")]
    theta1: f64,
    #[arg(long, help = "X-coordinate of corner")]
    x: f64,
    #[arg(long, help = "Y-coordinate of corner")]
    y: f64,
    #[arg(long, help = "Height/thickness of curve")]
    h: f64,
}

#[derive(Args)]
pub struct BankArgs {
    #[arg(long, help = "Number of segments")]
    pub n: u32,
    #[arg(long, help = "Starting inner radius")]
    pub ri0: f64,
    #[arg(long, help = "Starting outer radius")]
    pub ro0: f64,
    #[arg(long, help = "Ending inner radius")]
    pub ri1: f64,
    #[arg(long, help = "Ending outer radius")]
    pub ro1: f64,
    #[arg(long, help = "Starting angle (deg)")]
    pub theta0: f64,
    #[arg(long, help = "Ending angle (deg)")]
    pub theta1: f64,
    #[arg(long, help = "Cone height")]
    pub h: f64,
    #[arg(long, help = "Thickness")]
    pub t: f64,
    #[arg(long, help = "Filled bank")]
    pub fill: bool,
}

fn main() {
    let cli = Cli::parse();
    let map = map(cli.command).unwrap_or_else(|err| {
        eprintln!("{} {err}", "error:".red());
        std::process::exit(1);
    });
    match cli.file {
        None => println!("{}", map.to_string()),
        Some(filename) => std::fs::write(filename, map.to_string()).unwrap_or_else(|err| {
            eprintln!("{} {err}", "error:".red());
            std::process::exit(1);
        }),
    }
}

fn map(command: Commands) -> CurveResult<QMap> {
    let brushes: Vec<Brush> = match command {
        Commands::Rayto(args) => Rayto {
            n: args.n,
            r0: args.r0,
            r1: args.r1,
            theta0: args.theta0,
            theta1: args.theta1,
            x: args.x,
            y: args.y,
            h: args.h,
        }
        .bake()?,
        Commands::Bank(args) => Bank {
            n: args.n,
            ri0: args.ri0,
            ri1: args.ri1,
            ro0: args.ro0,
            ro1: args.ro1,
            theta0: args.theta0,
            theta1: args.theta1,
            h: args.h,
            t: args.t,
            fill: args.fill,
        }
        .bake()?,
    };
    let simple_worldspawn = SimpleWorldspawn::new(brushes);
    let entity = QEntity::from(simple_worldspawn);
    Ok(QMap::new(vec![entity]).with_tb_neverball_metadata())
}
