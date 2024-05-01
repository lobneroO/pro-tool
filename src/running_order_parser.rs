// Tim Lobner

use core::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

/// custom error type for running order parser
#[derive(Clone)]
pub struct UnimplementedError;

impl fmt::Display for UnimplementedError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        // user facing output
        write!(f, "Called an unimplemented function!")
    }
}

impl fmt::Debug for UnimplementedError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        // programmer-facing output
        write!(f, "Unimplemented: {{ file: {}, line: {} }}", file!(), line!())
    }
}

/// static call, user does not need to know about underlying struct
pub fn parse_running_order(input_path: &Path) -> Result<(), UnimplementedError>{
    let parser = RunningOrderParser{input_path: &input_path};
    parser.parse_file();


    Err(UnimplementedError)    
}

/// the parser for an input running order csv file
struct RunningOrderParser<'a>{
    input_path: &'a Path,
}

impl RunningOrderParser<'_>{
    fn parse_file(&self){
        println!("parsing file {}", &self.input_path.display());
        let file = File::open(self.input_path)
            .expect("Should have been able to open the file");
        let reader = io::BufReader::new(file);
        
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
}

