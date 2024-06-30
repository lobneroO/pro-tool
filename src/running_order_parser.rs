// Tim Lobner

use core::fmt;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use chrono::NaiveDateTime;

use crate::band::Band;

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
pub fn parse_running_order(input_path: &Path) { //-> Result<(), UnimplementedError>{
    let parser = RunningOrderParser{input_path};
    parser.parse_file();


    // Err(UnimplementedError)    
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

        let mut bands: Vec<Band> = vec!();
       
        let mut ctr = 0;
        for line_read in reader.lines() {
            // make sure the line was read once, no need to do it on every op later on
            // remove leading white space, it will never be useful to us
            let mut line = line_read
                .unwrap()
                .to_owned();
            line = line.trim_start().to_string();
            
            // file is csv, i.e. comma separated! split at comma
            let test = line.split(',');
            let elements: Vec<&str> = test.collect();
            // first line should contain the info order. 
            // if this doesn't fit, the file may be broken
            if ctr == 0 {
                let mut problem = false;
                let expected: [&str; 5] = ["Band", "Date", "Start", "End", "Stage"];
                if elements.len() == expected.len() {
                    for i in 0..5 {
                        if expected[i] != elements[i] {
                             problem = true;
                        }
                    }
                }
                else {
                    problem = true;
                }

                if problem {
                    println!("The first line does not list the information the file will contain.");
                    println!("Expected \"Band,Date,Start,End,Stage\"");
                    println!("There may be problems parsing the input file!");
                }
                else {
                    // first line is the descriptive line, as expected
                    // TODO: allow csv files without descriptive line?
                    ctr += 1;
                    continue;
                }
            }

            // skip empty lines
            if elements.is_empty() {
                continue
            }

            // skip lines that start with a '#' char, they are comments.
            // note that leading white space has been removed in the line already.
            if elements[0].starts_with('#') {
                continue
            }

            // this is a data line, so it should contain 5 data points
            if elements.len() != 5 {
                println!("Line {} with data does not contain 5 comma separated data points! skipping {}", ctr, line);
                continue
            }

            // at this point we can assume the line contains data.
            // we assume the given structure of band,date,start,...
            let name = elements[0];
            let date = elements[1];
            let start = elements[2];
            let end = elements[3];
            // make start date parseable:
            // date is german format, e.g. "31.03.2024",
            // time is also german format, e.g. "23:50"
            // bring it into format "31.03.2024_23:50"
            let start_date: String = date.to_string() + "_" + start;
            let end_date: String = date.to_string() + "_" + end;
            let stage = elements[4].trim_end();

            let naive_start_date = NaiveDateTime::parse_from_str(&start_date, "%d.%m.%Y_%H:%M")
                .expect("Should have been able to parse start date");
            let naive_end_date = NaiveDateTime::parse_from_str(&end_date, "%d.%m.%Y_%H:%M")
                .expect("Should have been able to parse start date");
            bands.push(Band{
                name: name.to_string(),
                start_dt: naive_start_date,
                end_dt: naive_end_date,
                stage: stage.to_string()
            });
            println!("{}", line);
            ctr += 1;
        }
    }
}

