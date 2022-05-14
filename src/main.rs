use clap::Parser;
use colored::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    #[clap(short, long)]
    name: String,
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    print_title();
}

fn print_title() {
    let text = "MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMYYYWMMMMMM#>>>JMMMMMMMM:
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
