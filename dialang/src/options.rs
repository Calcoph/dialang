use std::{env, path::PathBuf, vec::IntoIter};

type ArgIter = IntoIter<String>;

pub(crate) struct Options {
    pub(crate) eval: bool,
    pub(crate) class_diag: bool,
    pub(crate) comm_diag: bool,
    pub(crate) seq_diag: bool,
    pub(crate) input_path: PathBuf,
    pub(crate) output_path: PathBuf
}

impl Default for Options {
    fn default() -> Options {
        Options {
            class_diag: true,
            eval: false,
            comm_diag: false,
            seq_diag: false,
            input_path: PathBuf::from("input.txt"),
            output_path: PathBuf::from("output.drawio"),
        }
    }
}

impl Options {
    pub(crate) fn load() -> Options {
        let args: Vec<String> = env::args().collect();

        let mut options = Options{
            ..Default::default()
        };

        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--help" => options.help(),
                "--class" => options.class(),
                "--comm" => options.comm(),
                "--seq" => options.seq(),
                "--input" => options.input(&mut iter),
                "--output" => options.output(&mut iter),
                a => options.handle_arg(a, &mut iter)
            }
        }

        if !(options.class_diag | options.comm_diag | options.seq_diag) {
            options.class();
            options.comm();
            options.seq();
        }

        options
    }

    fn handle_arg(&mut self, arg: &str, arg_iter: &mut ArgIter) {
        if arg.starts_with('-') {
            if arg.starts_with("--") {
                self.fail(&format!("Unsupported argument \"{arg}\"."))
            } else {
                for c in arg.split_at(1).1.chars() {
                    self.handle_single_letter_arg(c, arg_iter)
                }
            }
        } else {
            self.fail(&format!("Unsupported argument \"{arg}\"."))
        }
    }

    fn handle_single_letter_arg(&mut self, arg: char, arg_iter: &mut ArgIter) {
        match arg {
            'h' => self.help(),
            'c' => self.class(),
            'm' => self.comm(),
            's' => self.seq(),
            'i' => self.input(arg_iter),
            'o' => self.output(arg_iter),
            a => self.fail(&format!("Unsupported argument \"-{a}\"."))
        }
    }

    fn fail(&mut self, msg: &str) {
        self.eval = false;
        println!("{}", msg);
        println!("See dialang -h.")
    }

    fn help(&mut self) {
        self.eval = false;
        println!(include_str!("../../documentation/help.txt"))
    }

    fn class(&mut self) {
        self.class_diag = true;
    }

    fn comm(&mut self) {
        self.comm_diag = true;
    }

    fn seq(&mut self) {
        self.seq_diag = true;
    }

    fn input(&mut self, arg_iter: &mut ArgIter) {
        if let Some(arg) = arg_iter.next() {
            if arg.starts_with('-') {
                self.fail("Please provide a file name after \"-i\" or \"--input\" (file cannot start with \"-\").")
            } else {
                self.input_path = PathBuf::from(arg)
            }
        } else {
            self.fail("Please provide a file name after \"-i\" or \"--input\".")
        }
    }

    fn output(&mut self, arg_iter: &mut ArgIter) {
        if let Some(arg) = arg_iter.next() {
            if arg.starts_with('-') {
                self.fail("Please provide a file name after \"-o\" or \"--output\" (file cannot start with \"-\").")
            } else {
                self.output_path = PathBuf::from(arg)
            }
        } else {
            self.fail("Please provide a file name after \"-o\" or \"--output\".")
        }
    }
}
