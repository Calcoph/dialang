use std::env;

pub(crate) struct Options {
    pub(crate) eval: bool,
    pub(crate) class_diag: bool,
    pub(crate) comm_diag: bool,
    pub(crate) seq_diag: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            class_diag: true,
            eval: false,
            comm_diag: false,
            seq_diag: false
        }
    }
}

impl Options {
    pub(crate) fn load() -> Options {
        let args: Vec<String> = env::args().collect();

        let mut options = Options{
            ..Default::default()
        };

        for arg in args {
            match arg.as_str() {
                "--help" => options.help(),
                "--class" => options.class(),
                "--comm" => options.comm(),
                "--seq" => options.seq(),
                a => options.handle_arg(a)
            }
        };

        if !(options.class_diag | options.comm_diag | options.seq_diag) {
            options.class();
            options.comm();
            options.seq();
        }

        options
    }

    fn handle_arg(&mut self, arg: &str) {
        if arg.starts_with('-') {
            if arg.starts_with("--") {
                self.fail(&format!("Unsupported argument \"{arg}\". See dialang -h"))
            } else {
                for c in arg.split_at(1).1.chars() {
                    self.handle_single_letter_arg(c)
                }
            }
        } else {
            self.fail(&format!("Unsupported argument \"{arg}\". See dialang -h"))
        }
    }

    fn handle_single_letter_arg(&mut self, arg: char) {
        match arg {
            'h' => self.help(),
            'c' => self.class(),
            'm' => self.comm(),
            's' => self.seq(),
            a => self.fail(&format!("Unsupported argument \"-{a}\". See dialang -h"))
        }
    }

    fn fail(&mut self, msg: &str) {
        self.eval = false;
        println!("{}", msg);
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
}
