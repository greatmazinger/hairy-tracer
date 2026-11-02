use hairy_tracer_core::render_scene_json;
use image::{ImageBuffer, Rgb};
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

fn print_usage(program: &str) {
    println!(
        "Usage: {program} --scene <FILE.json|FILE.dsl> [options]

Options:
  -s, --scene <FILE>       Path to a JSON or DSL scene configuration file
      --size <WxH>         Image resolution as <width>x<height> (default: 640x480)
  -o, --outfile <FILE>     Output image filename, e.g. out.bmp or out.png (default: output.bmp)
  -d, --depth <N>          Maximum ray bounce depth (default: 5)
      --emit-json <FILE>   When rendering a .dsl file, also write the compiled JSON to <FILE>
  -h, --help               Print this help message"
    );
}

fn parse_size(size_str: &str) -> Result<(usize, usize), String> {
    let lower = size_str.to_lowercase();
    let parts: Vec<&str> = lower.split('x').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid size '{size_str}'. Expected <width>x<height>, e.g. 800x800"
        ));
    }
    let width = parts[0]
        .parse::<usize>()
        .map_err(|_| format!("Invalid width in '{size_str}'"))?;
    let height = parts[1]
        .parse::<usize>()
        .map_err(|_| format!("Invalid height in '{size_str}'"))?;
    if width == 0 || height == 0 {
        return Err("Width and height must be greater than 0".to_string());
    }
    Ok((width, height))
}

fn next_arg(args: &[String], idx: &mut usize, flag: &str) -> String {
    *idx += 1;
    if *idx >= args.len() {
        eprintln!("Error: Missing value for {flag}");
        process::exit(1);
    }
    args[*idx].clone()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map(|s| s.as_str()).unwrap_or("htrace");

    let mut scene_path: Option<String> = None;
    let mut size_str = "640x480".to_string();
    let mut outfile = "output.bmp".to_string();
    let mut max_depth: u32 = 5;
    let mut emit_json: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-h" || arg == "--help" {
            print_usage(program);
            process::exit(0);
        } else if arg == "-s" || arg == "--scene" {
            scene_path = Some(next_arg(&args, &mut i, arg));
        } else if let Some(val) = arg.strip_prefix("--scene=") {
            scene_path = Some(val.to_string());
        } else if arg == "--size" {
            size_str = next_arg(&args, &mut i, arg);
        } else if let Some(val) = arg.strip_prefix("--size=") {
            size_str = val.to_string();
        } else if arg == "-o" || arg == "--outfile" {
            outfile = next_arg(&args, &mut i, arg);
        } else if let Some(val) = arg.strip_prefix("--outfile=") {
            outfile = val.to_string();
        } else if arg == "-d" || arg == "--depth" {
            let val = next_arg(&args, &mut i, arg);
            max_depth = val.parse::<u32>().unwrap_or_else(|_| {
                eprintln!("Error: Invalid depth '{val}'");
                process::exit(1);
            });
        } else if let Some(val) = arg.strip_prefix("--depth=") {
            max_depth = val.parse::<u32>().unwrap_or_else(|_| {
                eprintln!("Error: Invalid depth '{val}'");
                process::exit(1);
            });
        } else if arg == "--emit-json" {
            emit_json = Some(next_arg(&args, &mut i, arg));
        } else if let Some(val) = arg.strip_prefix("--emit-json=") {
            emit_json = Some(val.to_string());
        } else {
            eprintln!("Error: Unknown argument '{arg}'");
            print_usage(program);
            process::exit(1);
        }
        i += 1;
    }

    let scene_file = match scene_path {
        Some(p) => p,
        None => {
            eprintln!("Error: You must specify a scene file using --scene");
            print_usage(program);
            process::exit(1);
        }
    };

    let (width, height) = match parse_size(&size_str) {
        Ok(dims) => dims,
        Err(e) => {
            eprintln!("Error: {e}");
            print_usage(program);
            process::exit(1);
        }
    };

    let raw_content = match fs::read_to_string(&scene_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read scene file '{scene_file}': {e}");
            process::exit(1);
        }
    };

    let is_dsl = Path::new(&scene_file)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("dsl"))
        .unwrap_or(false);

    let scene_json = if is_dsl {
        println!("Compiling Scene DSL: {scene_file}");
        let compiled = match scene_dsl::compile_dsl(&raw_content) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error compiling DSL '{scene_file}': {e}");
                process::exit(1);
            }
        };
        if let Some(ref json_out) = emit_json {
            if let Err(e) = fs::write(json_out, &compiled) {
                eprintln!("Error writing compiled JSON to '{json_out}': {e}");
                process::exit(1);
            }
            println!("Wrote compiled JSON to: {json_out}");
        }
        compiled
    } else {
        raw_content
    };

    println!(
        "Rendering image: {width} x {height} using Rust CLI (max_depth: {max_depth})"
    );
    let start = Instant::now();

    let pixels = match render_scene_json(&scene_json, width, height, max_depth) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Render error: {e}");
            process::exit(1);
        }
    };

    let elapsed = start.elapsed().as_secs_f64();
    println!("Rust render time: {elapsed:.4} seconds");

    let img = ImageBuffer::<Rgb<u8>, _>::from_raw(width as u32, height as u32, pixels)
        .expect("Pixel buffer size mismatch");

    if let Err(e) = img.save(&outfile) {
        eprintln!("Error saving image to '{outfile}': {e}");
        process::exit(1);
    }
    println!("Saved image to: {outfile}");
}
