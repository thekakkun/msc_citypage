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
use xsd_parser::models::meta::{BuildInMeta, CustomMeta};
use xsd_parser::{Config, Error};
use xsd_parser::{IdentType, generate};

pub(crate) fn gen_forecast_full() -> Result<(), Error> {
    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File("schema_files/forecastFull.xsd".into())];
    config.parser.flags = ParserFlags::DEFAULT_NAMESPACES;
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.interpreter.types = vec![
        custom_type!(dateStampType, models::general, "DateStampType"),
        custom_type!(textSummaryType, models::general, "TextSummaryType"),
        // custom_type!(dateTimeUTCType, models::general, "DateTimeUTCType"),
        (
            IdentTriple::from((IdentType::Type, "dateTimeUTCType")),
            MetaType::from(BuildInMeta::String),
        ),
        custom_type!(regionalNormalsType, models::weather, "RegionalNormalsType"),
        custom_type!(periodType, models::weather, "PeriodType"),
        custom_type!(cloudPrecipType, models::weather, "CloudPrecipType"),
        custom_type!(
            abbreviatedForecastType,
            models::weather,
            "AbbreviatedForecastType"
        ),
        custom_type!(temperaturesType, models::weather, "TemperaturesType"),
        custom_type!(windsType, models::weather, "WindsType"),
        custom_type!(
            precipTypeForecast,
            models::weather,
            "PrecipTypeForecastType"
        ),
        custom_type!(snowLevelType, models::weather, "SnowLevelType"),
        custom_type!(windChillType, models::weather, "WindChillType"),
        custom_type!(
            visibilityTypeForecast,
            models::weather,
            "VisibilityTypeForecastType"
        ),
        custom_type!(uvType, models::weather, "UvType"),
        custom_type!(
            relativeHumidityType,
            models::weather,
            "RelativeHumidityType"
        ),
        custom_type!(humidexType, models::weather, "HumidexType"),
        custom_type!(frostType, models::weather, "FrostType"),
        custom_type!(iconCodeHourlyType, models::weather, "IconCodeHourlyType"),
        custom_type!(
            temperatureHourlyType,
            models::weather,
            "TemperatureHourlyType"
        ),
        custom_type!(lopHourlyType, models::weather, "LopHourlyType"),
        custom_type!(windChillHourlyType, models::weather, "WindChillHourlyType"),
        custom_type!(humidexHourlyType, models::weather, "HumidexHourlyType"),
        custom_type!(windHourlyType, models::weather, "WindHourlyType"),
        custom_type!(uvHourlyType, models::weather, "UvHourlyType"),
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
    let dest_path = Path::new(&out_dir).join("forecast_full.rs");
    let mut file = File::create(dest_path)?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}
