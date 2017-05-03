#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

docopt!(Args, "
Usage: wc [options] [<file>]

Options:
    -c, --bytes  print the byte counts
    -m, --chars  print the character counts
    -l, --lines  print the newline counts
    -w, --words  print the word counts
    -L, --max-line-length  print the length of the longest line
    -h, --help  display this help and exit
    -v, --version  output version information and exit
", arg_file: Option<String>);

fn main() {
    println!("24 days of Rust - docopt (day 4)");
    let docopt = Args::docopt();
    println!("{:?}", docopt);
    let args: Args = docopt.decode().unwrap_or_else(|e| e.exit());
    println!("Counting stuff in {}", args.arg_file.unwrap_or("standard input".to_string()));
    if args.flag_bytes {
        println!("Counting bytes!");
    }
    if args.flag_chars {
        println!("Counting characters!");
    }
    if args.flag_lines {
        println!("Counting lines!");
    }
    if args.flag_words {
        println!("Counting words!");
    }
    if args.flag_max_line_length {
        println!("Measuring the longest line!");
    }
}
