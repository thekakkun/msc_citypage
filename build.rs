use std::{fs::File, io::Write, str::FromStr};

use url::Url;
use xsd_parser::{
    Config, Error,
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, RenderStep, Schema},
    generate,
};
fn main() -> Result<(), Error> {
    let mut config = Config::default();
    config.parser.schemas = vec![
        Schema::File("schemas/forecastFull.xsd".into()),
        Schema::File("schemas/general.xsd".into()),
        Schema::File("schemas/site.xsd".into()),
        Schema::File("schemas/siteList.xsd".into()),
        Schema::File("schemas/weather.xsd".into()),
    ];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all();
    config.generator.flags = GeneratorFlags::all();

    let config = config.with_render_steps([
        RenderStep::Types,
        RenderStep::Defaults,
        RenderStep::NamespaceConstants,
        RenderStep::QuickXmlDeserialize {
            boxed_deserializer: false,
        },
    ]);

    let code = generate(config)?;
    let code = code.to_string();

    let mut file = File::create("src/schema.rs")?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}
