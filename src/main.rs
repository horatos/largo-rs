use eyre::Result;

use largo_rs::app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let app = App::new()?;
    app.run()?;

    Ok(())
}
