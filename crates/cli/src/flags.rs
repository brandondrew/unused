use read_ctags::Language;
use std::str::FromStr;
use structopt::StructOpt;
use token_analysis::{OrderField, UsageLikelihoodStatus};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "unused-rs",
    about = "A command line tool to identify potentially unused code",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
pub struct Flags {
    /// Disable color output
    #[structopt(long)]
    pub no_color: bool,

    /// Disable summary
    #[structopt(long)]
    pub no_summary: bool,

    /// Render output as JSON
    #[structopt(long)]
    pub json: bool,

    /// Hide progress bar
    #[structopt(long, short = "P")]
    pub no_progress: bool,

    /// Include tokens that fall into any likelihood category
    #[structopt(long, short = "a")]
    pub all_likelihoods: bool,

    /// Limit token output to those that match the provided likelihood(s)
    ///
    /// This allows for a comma-delimited list of likelihoods.
    #[structopt(long = "likelihood", short = "l", use_delimiter = true, default_value = "high", possible_values = &["high", "medium", "low"])]
    pub likelihoods: Vec<UsageLikelihoodStatus>,

    /// Sort output
    #[structopt(long, possible_values = &OrderField::variants(), default_value, case_insensitive = true)]
    pub sort_order: OrderField,

    /// Reverse sort order
    #[structopt(long)]
    pub reverse: bool,

    /// Limit tokens to those defined in the provided file extension(s)
    #[structopt(long, possible_values = &Language::extensions(), use_delimiter = true)]
    pub only_filetypes: Vec<Language>,

    /// Limit tokens to those defined except for the provided file extension(s)
    #[structopt(long, possible_values = &Language::extensions(), use_delimiter = true)]
    pub except_filetypes: Vec<Language>,

    /// Format output
    #[structopt(long, possible_values = &["standard", "compact", "json"], default_value = "standard", case_insensitive = true)]
    pub format: Format,
}

#[derive(Debug)]
pub enum Format {
    Standard,
    Compact,
    Json,
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "standard" => Ok(Format::Standard),
            "compact" => Ok(Format::Compact),
            "json" => Ok(Format::Json),
            v => Err(format!("Unknown format: {}", v)),
        }
    }
}