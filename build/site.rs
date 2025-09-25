use crate::custom_type;
use crate::utils::rustfmt_pretty_print;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xsd_parser::config::{
    GeneratorFlags, IdentTriple, InterpreterFlags, MetaType, OptimizerFlags, ParserFlags,
    RenderStep, Schema,
};
use xsd_parser::models::meta::CustomMeta;
use xsd_parser::{Config, Error};
use xsd_parser::{IdentType, generate};

pub(crate) fn gen_site() -> Result<(), Error> {
    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File("schema_files/site.xsd".into())];
    config.parser.flags = ParserFlags::DEFAULT_NAMESPACES;
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.interpreter.types = vec![
        custom_type!(dateStampType, models::general, "DateStampType"),
        custom_type!(locationType, models::general, "LocationType"),
        custom_type!(warningsType, models::weather, "WarningsType"),
        custom_type!(
            currentConditionsType,
            models::weather,
            "CurrentConditionsType"
        ),
        custom_type!(
            forecastGroupTypeFull,
            models::forecast_full,
            "ForecastGroupTypeFullType"
        ),
        custom_type!(
            hourlyForecastGroupTypeFull,
            models::forecast_full,
            "HourlyForecastGroupTypeFullType"
        ),
        custom_type!(riseSetType, models::weather, "RiseSetType"),
    ];
    config.optimizer.flags = OptimizerFlags::all()
        - OptimizerFlags::REMOVE_EMPTY_ENUM_VARIANTS
        - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all() - GeneratorFlags::MIXED_TYPE_SUPPORT;

    let config = config.with_render_steps([
        RenderStep::Types,
        RenderStep::Defaults,
        RenderStep::NamespaceConstants,
        RenderStep::QuickXmlDeserialize {
            boxed_deserializer: false,
        },
    ]);

    let code = generate(config)?.to_string();
    let code = rustfmt_pretty_print(code).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("site.rs");
    let mut file = File::create(dest_path)?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}
