/// struct used to parse argument provided to the functions by the user
use clap::Parser;


///the different commands do not have the same number of arguments,
/// most are therefore optional and the verification of the validity of the input
/// is checked in the main file when launching the associated functions.
#[derive(Parser)]
#[derive(Debug)]
pub struct Cli {
    pub command: String,
    pub arg2: Option<String>,
    pub arg3: Option<String>,
    pub arg4: Option<String>
}