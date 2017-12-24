extern crate argparse;

use argparse::{ArgumentParser, Store, StoreOption};


enum KnownAlgo {
    BinaryTree,
    SideWinder
}


impl KnownAlgo {
    fn all() -> Vec<KnownAlgo> {
        vec![
            KnownAlgo::BinaryTree,
            KnownAlgo::SideWinder
        ]
    }

    fn name(&self) -> &str {
        match *self {
            KnownAlgo::BinaryTree => "BinaryTree",
            KnownAlgo::SideWinder => "SideWinder"
        }
    }

    fn from(name: &String) -> Option<KnownAlgo> {
        match name.as_str() {
            "BinaryTree" => Some(KnownAlgo::BinaryTree),
            "SideWinder" => Some(KnownAlgo::SideWinder),
            _ => None
        }
    }
}


struct CmdArgs {
    size: [usize; 2],
    algo: Option<KnownAlgo>
}


fn parse_args(cmd_args: &mut CmdArgs) {
    let (mut width, mut height) = (cmd_args.size[0], cmd_args.size[1]);
    let mut algo_name : Option<String> = None;

    {
        
        let mut ap = ArgumentParser::new();
        ap.stop_on_first_argument(true);
        ap.set_description("Maze generator.");
        ap.refer(&mut width)
            .add_option(&["--width"], Store, "Maze width")
            .metavar("width");
        ap.refer(&mut height)
            .add_option(&["--height"], Store, "Maze height")
            .metavar("height");
        ap.refer(&mut algo_name)
            .add_argument("algorithm", StoreOption, "Carving algorithm")
            .metavar("name")
            .required();
        ap.parse_args_or_exit();

    }
    
    if let Some(name) = algo_name {
        if let Some(algo) = KnownAlgo::from(&name) {
            cmd_args.algo = Some(algo);
        } else {
            let valid_algos =
                KnownAlgo::all().iter()
                .fold(String::new(), |mut res, ref algo| {
                    if !res.is_empty() {
                        res += ", ";
                    }
                    res + algo.name()
                });

            eprintln!("Invalid algorithm '{}'. Choices are {}.", name, valid_algos);
        }
    }
 }


fn main() {
    let mut cmd_args = CmdArgs {
        size: [650, 500],
        algo: None
    };

    parse_args(&mut cmd_args);
}
