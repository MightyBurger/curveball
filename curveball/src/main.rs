use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use curveball_lib::{
    brush::Brush,
    curve::rayto,
    curve::Curve,
    curve::CurveResult,
    entity::SimpleWorldspawn,
    qmap::{QEntity, QMap},
};

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
    Rayto(RaytoArgs),
}

#[derive(Args)]
struct RaytoArgs {
    #[arg(long, help = "Number of segments")]
    n: u32,
    #[arg(long, help = "Starting radius")]
    r0: f64,
    #[arg(long, help = "Ending radius")]
    r1: f64,
    #[arg(long, help = "Starting angle")]
    theta0: f64,
    #[arg(long, help = "Ending angle")]
    theta1: f64,
    #[arg(long, help = "X-coordinate of corner")]
    x: f64,
    #[arg(long, help = "Y-coordinate of corner")]
    y: f64,
    #[arg(long, help = "Height/thickness of curve")]
    h: f64,
}

fn main() {
    let cli = Cli::parse();
    let map = map(cli.command).unwrap_or_else(|err| {
        eprintln!(
            "{} could not create curve, caused by: \n{err}",
            "error:".red(),
        );
        std::process::exit(1);
    });
    match cli.file {
        None => println!("{}", map.to_string()),
        Some(filename) => std::fs::write(filename, map.to_string()).unwrap_or_else(|err| {
            eprintln!(
                "{} could not write to specified file, caused by: {err}",
                "error:".red(),
            );
            std::process::exit(1);
        }),
    }
}

fn map(command: Commands) -> CurveResult<QMap> {
    let brushes: Vec<Brush> = match command {
        Commands::Rayto(args) => rayto(args)?,
    };
    let simple_worldspawn = SimpleWorldspawn::new(brushes);
    let entity = QEntity::from(simple_worldspawn);
    Ok(QMap::new(vec![entity]).with_tb_neverball_metadata())
}

fn rayto(args: RaytoArgs) -> CurveResult<Vec<Brush>> {
    rayto::Rayto {
        n: args.n,
        r0: args.r0,
        r1: args.r1,
        theta0: args.theta0,
        theta1: args.theta1,
        x: args.x,
        y: args.y,
        h: args.h,
    }
    .bake()
}
