use std::path::{Path, PathBuf};

use erg_common::config::ErgConfig;
use erg_common::io::Input;
use erg_common::spawn::exec_new_thread;
use erg_compiler::artifact::{CompleteArtifact, IncompleteArtifact};
use pylyzer_core::PythonAnalyzer;

#[allow(clippy::result_large_err)]
pub fn exec_analyzer(file_path: &'static str) -> Result<CompleteArtifact, IncompleteArtifact> {
    let cfg = ErgConfig {
        input: Input::file(PathBuf::from(file_path)),
        effect_check: false,
        ownership_check: false,
        ..Default::default()
    };
    let mut analyzer = PythonAnalyzer::new(cfg);
    let py_code = analyzer.cfg.input.read();
    analyzer.analyze(py_code, "exec")
}

fn _expect(file_path: &'static str, warns: usize, errors: usize) -> Result<(), String> {
    println!("Testing {file_path} ...");
    match exec_analyzer(file_path) {
        Ok(artifact) => {
            if artifact.warns.len() != warns {
                eprintln!("warns: {}", artifact.warns);
                return Err(format!(
                    "Expected {warns} warnings, found {}",
                    artifact.warns.len()
                ));
            }
            if errors != 0 {
                return Err(format!("Expected {errors} errors, found 0"));
            }
            Ok(())
        }
        Err(artifact) => {
            if artifact.warns.len() != warns {
                eprintln!("warns: {}", artifact.warns);
                return Err(format!(
                    "Expected {warns} warnings, found {}",
                    artifact.warns.len()
                ));
            }
            if artifact.errors.len() != errors {
                eprintln!("errors: {}", artifact.errors);
                return Err(format!(
                    "Expected {errors} errors, found {}",
                    artifact.errors.len()
                ));
            }
            Ok(())
        }
    }
}

pub fn expect(file_path: &'static str, warns: usize, errors: usize) -> Result<(), String> {
    exec_new_thread(move || _expect(file_path, warns, errors), file_path)
}

#[test]
fn exec_abc() -> Result<(), String> {
    expect("tests/abc.py", 0, 0)
}

#[test]
fn exec_test() -> Result<(), String> {
    expect("tests/test.py", 0, 11)
}

#[test]
fn exec_import() -> Result<(), String> {
    if Path::new("tests/__pycache__").exists() {
        std::fs::remove_dir_all("tests/__pycache__").unwrap();
    }
    if Path::new("tests/foo/__pycache__").exists() {
        std::fs::remove_dir_all("tests/foo/__pycache__").unwrap();
    }
    if Path::new("tests/bar/__pycache__").exists() {
        std::fs::remove_dir_all("tests/bar/__pycache__").unwrap();
    }
    expect("tests/import.py", 1, 2)
}

#[test]
fn exec_dict() -> Result<(), String> {
    expect("tests/dict.py", 0, 2)
}

#[test]
fn exec_export() -> Result<(), String> {
    expect("tests/export.py", 0, 0)
}

#[test]
fn exec_func() -> Result<(), String> {
    expect("tests/func.py", 0, 1)
}

#[test]
fn exec_class() -> Result<(), String> {
    expect("tests/class.py", 0, 8)
}

#[test]
fn exec_class_err() -> Result<(), String> {
    expect("tests/err/class.py", 0, 3)
}

#[test]
fn exec_errors() -> Result<(), String> {
    expect("tests/errors.py", 0, 3)
}

#[test]
fn exec_warns() -> Result<(), String> {
    expect("tests/warns.py", 2, 0)
}

#[test]
fn exec_typespec() -> Result<(), String> {
    expect("tests/typespec.py", 0, 16)
}

#[test]
fn exec_projection() -> Result<(), String> {
    expect("tests/projection.py", 0, 5)
}

#[test]
fn exec_property() -> Result<(), String> {
    expect("tests/property.py", 0, 0)
}

#[test]
fn exec_property_err() -> Result<(), String> {
    expect("tests/err/property.py", 0, 1)
}

#[test]
fn exec_pyi() -> Result<(), String> {
    expect("tests/pyi.py", 0, 5)
}

#[test]
fn exec_list() -> Result<(), String> {
    expect("tests/list.py", 0, 2)
}

#[test]
fn exec_literal() -> Result<(), String> {
    expect("tests/literal.py", 0, 2)
}

#[test]
fn exec_narrowing() -> Result<(), String> {
    expect("tests/narrowing.py", 0, 1)
}

#[test]
fn exec_casting() -> Result<(), String> {
    expect("tests/casting.py", 4, 1)
}

#[test]
fn exec_collection() -> Result<(), String> {
    expect("tests/collection.py", 0, 5)
}

#[test]
fn exec_call() -> Result<(), String> {
    expect("tests/call.py", 0, 6)
}

#[test]
fn exec_decl() -> Result<(), String> {
    expect("tests/decl.py", 0, 1)
}

#[test]
fn exec_shadowing() -> Result<(), String> {
    expect("tests/shadowing.py", 0, 4)
}

#[test]
fn exec_typevar() -> Result<(), String> {
    expect("tests/typevar.py", 0, 3)
}

#[test]
fn exec_type_spec() -> Result<(), String> {
    expect("tests/err/type_spec.py", 0, 6)
}

#[test]
fn exec_union() -> Result<(), String> {
    expect("tests/union.py", 0, 0)
}

#[test]
fn exec_widening() -> Result<(), String> {
    expect("tests/widening.py", 0, 1)
}
