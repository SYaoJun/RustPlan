use clap::{App, Arg};

fn main() {
    // 创建一个新的命令行应用
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Your Name")
        .about("A simple Rust application using clap")
        // 添加一个命令行参数
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Sets the input file to use")
            .takes_value(true))
        // 添加一个标志
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Sets the level of verbosity"))
        .get_matches();

    // 处理命令行参数
    if let Some(input_file) = matches.value_of("input") {
        println!("Input file: {}", input_file);
    }

    if matches.is_present("verbose") {
        println!("Verbose mode is enabled");
    }
}
