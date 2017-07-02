use clap::ArgMatches;

pub struct Options {
    pub kernels_path: String
}

impl<'a> From<&'a ArgMatches<'a>> for Options {
    fn from(matches: &ArgMatches) -> Options {
        Options {
            kernels_path: matches.value_of("kernels-path").unwrap().to_string()
        }
    }
}