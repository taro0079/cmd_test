mod commands;
use clap::Parser;
use colored::*;
use std::{thread, time};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    #[clap(short, long)]
    name: String,
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    let one_seconds = time::Duration::from_secs(1);
    commands::first_clean();
    thread::sleep(one_seconds);
    print_title();
    thread::sleep(one_seconds);
    print_selection();
    // commands::execute_shell();
}

fn print_title() {
    let text =
" MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMYYYWMMMMMM#>>>JMMMMMMMM:
 MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMN>>>JMMMMMM#>>>JMMMMMMMM:
 MM8C>>>1TWMMMM@111dBC>>>?HMBC>>>?TMMMMMMMM8C>>>>?TMMMMMMMMM#51>>>+7WMMMMM$1>>>>11zMMM#>?>J#6>>>zTM:
 #>>>u&&+>jMMMMb>>>>>>>>>>>>>>>?>>>JMMMMM6>>>>++>>>>?MMMMM#<>>>+++>>>>dMMMI>>>?>>>jMMM#?>>>>>?>>>>J:
 @>>>TMMMMMMMMMb>>>uMMMR>>>>dMMN+>>>MMMMC>>+dMMMMN+>>?MMM#>>>uMMMMNe>>>dMMMN>>>JMMMMMM#>>>jMMMN+>>>`
 Me>?>>>>7HMMMMb>>?dMMMN>?>jMMMMP>>>MMM#>>>jMMMMMMb>>>dMM$>>?MMMMMMM+>>jMMMN>>>JMMMMMM#>>>jMMMMP>?>
 MMMNgxx>>>JMMMb>>>dMMMN>>>jMMMMP>>>MMMN>>>?MMMMMM5>>>dMMP>>>HMMMMM#?>>jMMMN>>>JMMMMMM#>?>JMMMMP>>>
 #1?WMMM$>>jMMMb>>>dMMMN>>>jMMMMP>>?MMMMm>>>?TMMBC>>>uMMMNx>>>?HMM5?>>jMMMMN>>>?TH9MMM#>>>JMMMMP>>>
 gx>?>>>>>jdMMMb>>>dMMMN>>>jMMMMP>>>MMMMMNx>>>>>>>>udMMMMMMmx>>>>>>>+uMMMMMMp>>>>>jMMM#>>>JMMMMP>?>";
    println!("{}", text.cyan())
}

fn print_selection() {
    let first_text = "This is the installer for smooth develop.";
    let first_selection = "1) Install settings";
    let second_selection = "2) Quit setting";
    println!("");
    println!("{}", first_text);
    println!("");

    println!("{}", first_selection);
    println!("{}", second_selection);
}
