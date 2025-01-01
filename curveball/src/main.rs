use clap::{Args, Parser, Subcommand};
use colored::Colorize;

use curveball_lib::curve::serpentine::SerpentineOffsetMode;
use curveball_lib::curve::{Bank, Catenary, Curve, CurveResult, Rayto, Serpentine};
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
    #[command(about = "Generate a catenary curve")]
    Catenary(CatenaryArgs),
    #[command(about = "Generate a serpentine curve")]
    Serpentine(SerpentineArgs),
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
    #[arg(long, help = "Inner radius")]
    pub ri: f64,
    #[arg(long, help = "Outer radius")]
    pub ro: f64,
    #[arg(long, help = "Starting angle (deg)")]
    pub theta0: f64,
    #[arg(long, help = "Ending angle (deg)")]
    pub theta1: f64,
    #[arg(long, help = "Cone height")]
    pub h: f64,
    #[arg(long, help = "Thickness of the bank")]
    pub t: f64,
    #[arg(long, help = "Filled bank")]
    pub fill: bool,
}

#[derive(Args)]
pub struct CatenaryArgs {
    #[arg(long, help = "Number of segments")]
    pub n: u32,
    #[arg(long, help = "Starting horizontal position of the curve")]
    pub x0: f64,
    #[arg(long, help = "Starting height of the curve")]
    pub z0: f64,
    #[arg(long, help = "Ending horizontal position of the curve")]
    pub x1: f64,
    #[arg(long, help = "Ending height of the curve")]
    pub z1: f64,
    #[arg(long, help = "Length of the curve (i.e. how long your rope is)")]
    pub s: f64,
    #[arg(long, help = "Width of the curve")]
    pub w: f64,
    #[arg(long, help = "Thickness of the curve")]
    pub t: f64,
    #[arg(
        long,
        help = "The initial guess for the catenary parameter 'a'; used for Newton's method"
    )]
    pub initial_guess: Option<f64>,
}

#[derive(Args)]
pub struct SerpentineArgs {
    #[arg(
        long,
        help = "Number of segments; will be rounded up to the nearest multiple of 2"
    )]
    pub n: u32,
    #[arg(long, help = "Ending horizontal position of curve")]
    pub x: f64,
    #[arg(long, help = "Ending height of the curve")]
    pub z: f64,
    #[arg(long, help = "Width of the curve")]
    pub w: f64,
    #[arg(long, help = "Thickness of the curve")]
    pub t: f64,
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
fn div_up(a: u32, b: u32) -> u32 {
    (a + (b - 1)) / b
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
            ro: args.ro,
            ri: args.ri,
            theta0: args.theta0,
            theta1: args.theta1,
            h: args.h,
            t: args.t,
            fill: args.fill,
        }
        .bake()?,
        Commands::Catenary(args) => Catenary {
            n: args.n,
            x0: args.x0,
            z0: args.z0,
            x1: args.x1,
            z1: args.z1,
            s: args.s,
            w: args.w,
            t: args.t,
            initial_guess: args.initial_guess,
        }
        .bake()?,
        Commands::Serpentine(args) => Serpentine {
            n_each: div_up(args.n, 2),
            x: args.x,
            z: args.z,
            w: args.w,
            t: args.t,
            offset: SerpentineOffsetMode::Middle,
        }
        .bake()?,
    };
    let simple_worldspawn = SimpleWorldspawn::new(brushes);
    let entity = QEntity::from(simple_worldspawn);
    Ok(QMap::new(vec![entity]).with_tb_neverball_metadata())
}
