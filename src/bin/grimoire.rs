use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "grimoire")]
#[clap(bin_name = "grimoire")]
enum Grimoire {
    /// Initializes a Grimoire project
    Init(grimoire_lsp::subcommands::Init),
    /// Surfaces errors or warnings relating to project structure
    Reindex(grimoire_lsp::subcommands::Reindex),
    /// Runs the LSP server for a Grimoire project
    Lsp(grimoire_lsp::subcommands::Lsp),
}

fn main() -> grimoire_lsp::Result<()> {
    match Grimoire::parse() {
        Grimoire::Init(args) => grimoire_lsp::subcommands::Init::call(args),
        Grimoire::Lsp(args) => grimoire_lsp::subcommands::Lsp::call(args),
        Grimoire::Reindex(args) => grimoire_lsp::subcommands::Reindex::call(args),
    }
}
