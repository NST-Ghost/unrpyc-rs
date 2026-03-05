use clap::{Parser, Subcommand};
use std::path::{PathBuf, Path};
use anyhow::{Context, Result};
use unrpyc_rs::reader::{read_rpyc_file, decompress_data};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input file (.rpyc) or directory
    input: Option<String>,

    /// Output directory (optional)
    #[arg(short, long)]
    output: Option<String>,

    /// Dump internal structure
    #[arg(long)]
    dump: bool,

    /// Recursive processing for directories
    #[arg(short, long, default_value_t = false)]
    recursive: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Extract RPA archive
    Extract {
        /// Archive file (.rpa)
        archive: String,
        /// Output directory (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn process_single_file(input_path: &Path, output_dir: Option<&Path>, dump: bool) -> Result<()> {
    println!("Processing file: {:?}", input_path);

    // 1. Read file
    let raw_data = read_rpyc_file(&PathBuf::from(input_path)).context("Failed to read rpyc file")?;
    
    // 2. Decompress
    let decompressed = decompress_data(&raw_data).context("Failed to decompress data")?;
    
    // 3. Unpickle
    let options = serde_pickle::DeOptions::new().replace_unresolved_globals();
    let decoded: serde_pickle::Value = serde_pickle::from_slice(&decompressed, options)
        .context("Failed to unpickle data")?;

    if dump {
        println!("{:#?}", decoded);
    } else {
        if let Some(stmts) = unrpyc_rs::ast::extract_statements(&decoded) {
             println!("Found {} statements in {:?}", stmts.len(), input_path);
             // Currently just parsing to confirm structure
             for (i, stmt) in stmts.iter().enumerate().take(5) { // Show first 5 for brevity
                 match unrpyc_rs::ast::parse_statement(stmt) {
                     Ok(parsed) => println!("  Stmt {}: {:?}", i, parsed),
                     Err(e) => println!("  Stmt {}: Failed: {}", i, e),
                 }
             }
             if stmts.len() > 5 {
                 println!("  ... and {} more", stmts.len() - 5);
             }
        } else {
             println!("Could not extract statements from {:?}", input_path);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(Commands::Extract { archive, output }) = args.command {
        let rpa = unrpyc_rs::rpa::RpaArchive::open(&archive)?;
        println!("Opened archive: {}", archive);
        let files = rpa.list_files();
        println!("Found {} files.", files.len());
        
        let out_dir = output.unwrap_or_else(|| {
            let path = Path::new(&archive);
            path.file_stem().unwrap().to_string_lossy().to_string() + "_extracted"
        });
        
        std::fs::create_dir_all(&out_dir)?;
        
        let mut rpa = rpa; // Make mutable for extraction
        for file in files {
            println!("Extracting {}", file);
            if let Some(data) = rpa.extract_file(&file)? {
                let out_path = Path::new(&out_dir).join(&file);
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(out_path, data)?;
            }
        }
        println!("Extraction complete to {}", out_dir);
        return Ok(());
    }

    // Default behavior: Decompile .rpyc
    let input_path_str = match args.input {
        Some(p) => p,
        None => {
             use clap::CommandFactory;
             Args::command().print_help()?;
             std::process::exit(1);
        }
    };
    
    let input_path = Path::new(&input_path_str);
    let output_dir = args.output.as_ref().map(|s| Path::new(s));

    if input_path.is_dir() {
        println!("Scanning directory: {:?}", input_path);
        let walker = if args.recursive {
            WalkDir::new(input_path)
        } else {
            WalkDir::new(input_path).max_depth(1)
        };

        let mut count = 0;
        for entry in walker.into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "rpyc") {
                if let Err(e) = process_single_file(entry.path(), output_dir, args.dump) {
                    eprintln!("Error processing {:?}: {}", entry.path(), e);
                }
                count += 1;
            }
        }
        println!("Finished processing {} files.", count);
    } else {
        process_single_file(input_path, output_dir, args.dump)?;
    }

    Ok(())
}
