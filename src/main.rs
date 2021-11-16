use color_eyre::eyre::{eyre, Result};
use degit::degit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
///
struct EleventyStarterArgs {
    project_name: String,

    #[structopt(long)]
    template_url: Option<String>,
}

#[paw::main]
fn main(args: EleventyStarterArgs) -> Result<()> {
    color_eyre::install()?;
    let template_source = {
        if let Some(template_url) = args.template_url {
            template_url
        } else {
            get_template_from_user()?
        }
    };
    let now = std::time::Instant::now();
    degit(&template_source, &args.project_name);
    print!(
        "{}",
        now.elapsed().as_secs_f32(),
    );
    Ok(())
}

fn get_template_from_user() -> Result<String> {
    Err(eyre!(
        "Getting a template repo from the user has not been implemented yet!"
    ))
}
