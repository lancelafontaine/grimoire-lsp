use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "grimoire")]
#[clap(bin_name = "grimoire")]
enum Grimoire {
    /// Initializes a Grimoire project
    Init(grimoire_lsp::init::Init),
    /// Runs the LSP server for a Grimoire project
    Lsp(grimoire_lsp::lsp::Lsp),
}

fn main() -> grimoire_lsp::Result<()> {
    match Grimoire::parse() {
        Grimoire::Init(args) => grimoire_lsp::init::call(args),
        Grimoire::Lsp(args) => grimoire_lsp::lsp::run(args),
    }
}
