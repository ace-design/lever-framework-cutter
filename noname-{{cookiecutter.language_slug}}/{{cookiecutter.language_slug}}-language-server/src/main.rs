#[tokio::main]
async fn main() {
    let language_def = include_str!("../language_def/rules.ron");

    ls_framework::start_server(language_def, tree_sitter_{{cookiecutter.language_slug}}::language()).await;
}
