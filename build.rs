use quote::ToTokens;
use std::{
    env,
    fs::File,
    io::Write,
    path::Path,
    process::{Command, Output, Stdio},
};
use xsd_parser::{
    Config, Error, IdentType, MetaTypes,
    config::{
        GeneratorFlags, IdentTriple, InterpreterFlags, MetaType, OptimizerFlags, RenderStep, Schema,
    },
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::meta::{CustomMeta, MetaTypeVariant},
};
fn main() -> Result<(), Error> {
    let mut config = Config::default();
    config.parser.schemas = vec![
        Schema::File("schema_files/forecastFull.xsd".into()),
        Schema::File("schema_files/general.xsd".into()),
        Schema::File("schema_files/site.xsd".into()),
        Schema::File("schema_files/siteList.xsd".into()),
        Schema::File("schema_files/weather.xsd".into()),
    ];
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.interpreter.types = vec![
        (
            IdentTriple::from((IdentType::Type, "timeStampType")),
            MetaType::from(
                CustomMeta::new("TimeStampType").include_from("crate::schemas::TimeStampType"),
            ),
        ),
        (
            IdentTriple::from((IdentType::Type, "dateTimeUTCType")),
            MetaType::from(
                CustomMeta::new("DateTimeUtcType").include_from("crate::schemas::DateTimeUtcType"),
            ),
        ),
    ];
    config.optimizer.flags = OptimizerFlags::all()
        - OptimizerFlags::REMOVE_EMPTY_ENUM_VARIANTS
        - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all();

    let config = config.with_render_steps([
        RenderStep::Types,
        RenderStep::Defaults,
        RenderStep::NamespaceConstants,
        RenderStep::QuickXmlDeserialize {
            boxed_deserializer: false,
        },
    ]);

    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = replace_variant_names(meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;

    let code = module.to_token_stream().to_string();
    let code = rustfmt_pretty_print(code).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");

    let mut file = File::create(dest_path)?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}

fn replace_variant_names(mut types: MetaTypes) -> MetaTypes {
    for (_ident, ty) in types.items.iter_mut() {
        if let MetaTypeVariant::Enumeration(enum_meta) = &mut ty.variant {
            for variant in enum_meta.variants.iter_mut() {
                match variant.ident.name.as_str() {
                    "%" => {
                        variant.display_name = Some("Percent".to_string());
                    }
                    "" => {
                        variant.display_name = Some("None".to_string());
                    }
                    "Aujourd&apos;hui" => {
                        variant.display_name = Some("AujourdHui".to_string());
                    }
                    "ce soir et cette nuit" => {
                        variant.display_name = Some("ceSoirEtCetteNuit".to_string());
                    }
                    "lundi soir et nuit" => {
                        variant.display_name = Some("lundiSoirEtNuit".to_string());
                    }
                    "mardi soir et nuit" => {
                        variant.display_name = Some("mardiSoirEtNuit".to_string());
                    }
                    "mercredi soir et nuit" => {
                        variant.display_name = Some("mercrediSoirEtNuit".to_string());
                    }
                    "jeudi soir et nuit" => {
                        variant.display_name = Some("jeudiSoirEtNuit".to_string());
                    }
                    "vendredi soir et nuit" => {
                        variant.display_name = Some("vendrediSoirEtNuit".to_string());
                    }
                    "samedi soir et nuit" => {
                        variant.display_name = Some("samediSoirEtNuit".to_string());
                    }
                    "dimanche soir et nuit" => {
                        variant.display_name = Some("dimancheSoirEtNuit".to_string());
                    }
                    "lundi" => {
                        variant.display_name = Some("lundi".to_string());
                    }
                    "mardi" => {
                        variant.display_name = Some("mardi".to_string());
                    }
                    "mercredi" => {
                        variant.display_name = Some("mercredi".to_string());
                    }
                    "jeudi" => {
                        variant.display_name = Some("jeudi".to_string());
                    }
                    "vendredi" => {
                        variant.display_name = Some("vendredi".to_string());
                    }
                    "samedi" => {
                        variant.display_name = Some("samedi".to_string());
                    }
                    "dimanche" => {
                        variant.display_name = Some("dima".to_string());
                    }
                    _ => {}
                }
            }
        }
    }

    types
}

// A small helper to call `rustfmt` when generating file(s).
// This may be useful to compare different versions of generated files.
fn rustfmt_pretty_print(code: String) -> Result<String, Error> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();

    write!(stdin, "{code}")?;
    stdin.flush()?;
    drop(stdin);

    let Output {
        status,
        stdout,
        stderr,
    } = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&stdout);
    let stderr = String::from_utf8_lossy(&stderr);

    if !status.success() {
        let code = status.code();
        match code {
            Some(code) => {
                if code != 0 {
                    panic!("The `rustfmt` command failed with return code {code}!\n{stderr}");
                }
            }
            None => {
                panic!("The `rustfmt` command failed!\n{stderr}")
            }
        }
    }

    Ok(stdout.into())
}
