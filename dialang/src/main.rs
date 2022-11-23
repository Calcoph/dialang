use std::ops::Range;

use crate::class_diag::make_class_diag;

mod class_diag;

type Spanned<T> = (T, Range<usize>);

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let tokens = diaparser::tokenize(&input);

    let class_diag = make_class_diag(tokens);

    let file = format!(include_str!("../../templates/doc.xml"), seq_diag="", comm_diag="", class_diag=class_diag);
    std::fs::write("output.drawio", file).unwrap();
}
