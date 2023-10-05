use std::{fs, path::Path};
use colorful::Colorful;
use colorful::core::color_string::CString;
use tree_sitter_highlight::{HighlightConfiguration,Highlighter, HighlightEvent};
use colorful::RGB;

fn main() {
    let path_arg: Vec<String> = std::env::args().collect();

    if path_arg.len()>1 {
        let path =Path::new(&path_arg[1]);
        if path.exists(){
           let md = std::fs::metadata(&path).unwrap();
           if md.is_file(){
               let contents = fs::read_to_string(path).unwrap();
               highlite(contents.as_bytes());
           }else{
               println!("{} is not a file",path_arg[1]);
               return;
           }
        }
    }
}

fn highlite(source_code:&[u8]){
    let highlight_names = [
        "attribute",
        "property",
        "identifier",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "punctuation",
        "punctuation.delimiter.keyword",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
        "import_specifier",
        "import_statement",
        "comment",
        "conditional",
        "variable.parameter",
        "parameter.variable",
        "type_identifier",
    ];

    let mut highlighter = Highlighter::new();

    let javascript_language = tree_sitter_javascript::language();

    let mut javascript_config = HighlightConfiguration::new(
        javascript_language,
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTION_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    ).unwrap();

    javascript_config.configure(&highlight_names);

    let highlights = highlighter.highlight(
        &javascript_config,
        source_code,
        None,
        |_| None
    ).unwrap();

    let mut color=1;
    let mut code_line:String="".to_string();

    for event in highlights {
        match event.unwrap() {
            HighlightEvent::HighlightStart(s) => {
                color=s.0;
                eprintln!("highlight style started: {:?} {:?}", highlight_names[s.0],s);
            },
            HighlightEvent::Source {start, end} => {
                eprintln!("source: {}-{}", start, end);
                let mut text:String="".to_string();
                    if let Ok(subslice) = std::str::from_utf8(&source_code[start..end]) {
                        text= format!("{}{}",text, subslice.to_string());
                    }
                let temp_ref=format!("{}{}",code_line,get_colored_text(color,text));
                code_line=temp_ref;
            },
            HighlightEvent::HighlightEnd => {
                eprintln!("highlight style ended");
            },
        }
    }
    println!("{}",code_line);
}

fn get_colored_text(color:usize,text:String)->CString {
    match color{
        1=>return text.color(RGB::new(41, 127, 207)),
        2=>return text.color(RGB::new(41, 127, 207)),
        3=>return text.color(RGB::new(19, 113, 168)),
        4=>return text.color(RGB::new(222, 230, 71)),
        5=>return text.color(RGB::new(217, 232, 2)),
        6=>return text.color(RGB::new(154, 88, 173)),
        7=>return text.color(RGB::new(255, 255, 255)),
        8=>return text.color(RGB::new(191, 167, 11)),
        9=>return text.color(RGB::new(255, 186, 36)),
        10=>return text.color(RGB::new(161, 111, 11)),
        11=>return text.color(RGB::new(25, 29, 230)),
        12=>return text.color(RGB::new(161, 111, 11)),
        13=>return text.color(RGB::new(193, 69, 255)),
        14=>return text.color(RGB::new(238, 134, 247)),
        15=>return text.color(RGB::new(3, 129, 161)),
        16=>return text.color(RGB::new(184, 51, 113)),
        17=>return text.color(RGB::new(19, 113, 168)),
        18=>return text.color(RGB::new(255, 0, 0)),
        19=>return text.color(RGB::new(255, 0, 0)),
        20=>return text.color(RGB::new(29, 94, 29)),
        21=>return text.color(RGB::new(255, 0, 0)),
        22=>return text.color(RGB::new(18, 115, 30)),
        _=>return text.white(),
    };
}
