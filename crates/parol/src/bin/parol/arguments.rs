use std::path::PathBuf;

use clap::Parser;
use parol::InnerAttributes;

// LL(k) Parser Generator written in Rust
#[derive(Parser)]
#[command(author, version, about)]
pub(crate) struct CliArgs {
    /// Input grammar file
    #[arg(short = 'f', long = "file")]
    pub grammar: Option<PathBuf>,

    /// Lookahead limit for Lookahead DFA calculation
    #[arg(short = 'k', long, default_value = "5")]
    pub lookahead: usize,

    /// Output file for the generated parser source
    #[arg(short = 'p', long = "parser")]
    pub parser: Option<PathBuf>,

    /// Output file for the expanded grammar. Use -e-- to output to stdout
    #[arg(short, long)]
    pub expanded: Option<PathBuf>,

    /// Writes the internal parsed grammar (ParolGrammar)
    #[arg(short = 'i', long)]
    pub write_internal: Option<PathBuf>,

    /// Writes the untransformed parsed grammar
    #[arg(short = 'u', long)]
    pub write_untransformed: Option<PathBuf>,

    /// Writes the transformed parsed grammar
    #[arg(short, long)]
    pub write_transformed: Option<PathBuf>,

    /// Output file for the generated trait with semantic actions
    #[arg(short, long)]
    pub actions: Option<PathBuf>,

    /// User type that implements the language processing
    #[arg(short = 't', long)]
    pub user_type: Option<String>,

    /// User type's module name
    #[arg(short, long)]
    pub module: Option<String>,

    /// Activates the auto-generation of semantic actions in expanded grammar - recommended
    #[arg(short = 'g', long)]
    pub auto_generate: bool,

    /// Activates the generation of a SVG file with the parse tree of the given grammar
    #[arg(short = 's', long = "svg")]
    pub generate_tree_graph: bool,

    /// Increased verbosity
    #[arg(short, long)]
    pub verbose: bool,

    /// Generate range information for AST types
    #[arg(short, long)]
    pub range: bool,

    /// Inserts the given inner attributes at the top of the generated trait source.
    #[arg(long, value_enum)]
    pub inner_attributes: Vec<InnerAttributes>,

    #[command(subcommand)]
    pub subcommand: Option<super::tools::ToolsSubcommands>,
}
