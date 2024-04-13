use std::{process::exit, collections::HashMap};

use options::Options;

use diaparser::Annotation;

use crate::class_diag::make_class_diag;
use crate::clean_ast::{clean_parser_top_lvl_statement, TopLevelStatement};
use crate::seq_diag::make_seq_diag;

mod seq_diag;
mod class_diag;
mod options;
mod style;
mod clean_ast;

fn main() {
    let opt = Options::load();
    if !opt.eval {
        exit(0)
    }

    println!("Reading from {:?}", opt.input_path);
    let input = std::fs::read_to_string(opt.input_path).unwrap();
    let (tokens, _) = diaparser::tokenize(&input);

    let tokens = match tokens {
        Ok(t) => t,
        Err(_) => panic!("Error while parsing the file, check that the syntax is correct.\nThis message is temporary, there will be better ones in the future."),
    };

    let mut classes = HashMap::new();
    let mut annotated_blocks = HashMap::new();

    tokens.into_iter()
        .map(|(class, _)| clean_parser_top_lvl_statement(class))
        .for_each(|stmnt| {
            match stmnt {
                TopLevelStatement::Class(class) => {
                    classes.insert(class.name.clone(), class);
                },
                TopLevelStatement::AnnotatedBlock(ablock) => {
                    annotated_blocks.insert(ablock.annotation, ablock.elements);
                },
            }
        });

    let class_diag = if opt.class_diag {
        let classes = make_class_diag(&classes);
        classes.into_iter()
            .collect::<String>()
    } else {
        "".to_string()
    };

    let seq_diag = if opt.seq_diag {
        if let Some(entry_points) = annotated_blocks.get(&Annotation::SequenceEntrypoint) {
            dbg!(entry_points);
            make_seq_diag(&classes).into_iter()
                .collect()
        } else {
            eprintln!("Cannot create a seqence diagram without @SequenceEntrypoint");
            "".to_string()
        }
    } else {
        "".to_string()
    };

    let comm_diag = if opt.comm_diag {
        "".to_string()
    } else {
        "".to_string()
    };

    let file = format!(include_str!("../../templates/doc.xml"), seq_diag=seq_diag, comm_diag=comm_diag, class_diag=class_diag);
    println!("Results written to {:?}", opt.output_path);
    std::fs::write(opt.output_path, file).unwrap();
}
