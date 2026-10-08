use parse_clargs::{ValueKind, Value, Arg, Parser};

fn main() -> Result<(), String> {
    let arg = Arg::new("in")
        .help("the input file")
        .value_kind(ValueKind::Str)
        .takes_value().required();
        
    let cli_parser = Parser::new("program").description("does some random shit")
        .add(arg)?
        .add(Arg::new("bs").help("the block size").takes_value().value_kind(ValueKind::Num))?;

    let args: Vec<String> = std::env::args().skip(1).collect();
    
    let matches = cli_parser.parse(&args)?;

    let input_path = matches.get_str("in").unwrap();
    let block_size = matches.get_num("bs").unwrap_or(432);

    println!("{}", input_path);
    Ok(())
}
