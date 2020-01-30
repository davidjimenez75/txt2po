use std::fs::File;
use std::path::Path;
use std::env;
use std::io::{self, prelude::*, BufReader};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            //println!("1 argument.");          
            version();            
            eprintln!("Error: Need a filename.txt as parameter. ");   
            eprintln!("");
            help();
            process::exit(1);         
        },
        // one argument passed
        2 => {
            //println!("2 arguments: {}",&args[1]);

            if  &args[1]=="-v"
            {
                version();
                process::exit(1);             
            }
        },
        // Handle the rest of cases
        _ => {
            //println!("Ain't special");
        }
    }
    
    let input_file = &args[1];
    if Path::new(input_file).exists() == false
    {
        version();        
        eprintln!("Error: Could not find the filename: \"{}\" ", input_file);
        eprintln!("");        
        help();
        process::exit(1);    
    }

    let file = File::open(input_file)?;
    let reader = BufReader::new(file);

    po_header();
    for line in reader.lines() {
        println!("msgid \"{0}\"\r\nmsgstr \"{0}\"\r\n", line?);
    }

    Ok(())
}

fn help() {
    println!("Usage:
txt2po input.txt > output.po
    ");
}

fn version() {
    println!("txt2po v{}",env!("CARGO_PKG_VERSION"));
}

fn po_header()
{
    println!("msgid \"\"
msgstr \"\"
\"Project-Id-Version: \\n\"
\"POT-Creation-Date: 2020-01-08 10:26+0100\\n\"
\"PO-Revision-Date: 2020-01-08 10:34+0100\\n\"
\"Language-Team: \\n\"
\"MIME-Version: 1.0\\n\"
\"Content-Type: text/plain; charset=UTF-8\\n\"
\"Content-Transfer-Encoding: 8bit\\n\"
\"X-Generator: Poedit 2.2.4\\n\"
\"X-Poedit-Basepath: .\\n\"
\"Plural-Forms: nplurals=2; plural=(n != 1);\\n\"
\"Last-Translator: \\n\"
\"Language: es_ES\\n\"
");
}