use std::{
    error::Error,
    io::Write,
    process::{Command, Output, Stdio},
};

#[macro_export]
macro_rules! custom_type {
    ($ident_name:ident, $path:path, $type_name:expr) => {
        (
            IdentTriple::from((IdentType::Type, stringify!($ident_name))),
            MetaType::from(CustomMeta::new($type_name).include_from(concat!(
                "crate::",
                stringify!($path),
                "::",
                $type_name
            ))),
        )
    };
}

// A small helper to call `rustfmt` when generating file(s).
// This may be useful to compare different versions of generated file
pub(crate) fn rustfmt_pretty_print(code: String) -> Result<String, Box<dyn Error>> {
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
