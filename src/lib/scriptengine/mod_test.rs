use super::*;
use crate::test;

#[test]
fn get_engine_type_no_runner_no_script() {
    let task = Task::new();

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Unsupported);
}

#[test]
fn get_engine_type_no_runner() {
    let mut task = Task::new();
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::OS);
}

#[test]
fn get_engine_type_no_script() {
    let mut task = Task::new();
    task.script_runner = Some("@rust".to_string());

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Unsupported);
}

#[test]
fn get_engine_type_runner_no_extension() {
    let mut task = Task::new();
    task.script_runner = Some("@bad".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::OS);
}

#[test]
fn get_engine_type_rust() {
    let mut task = Task::new();
    task.script_runner = Some("@rust".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Rust);
}

#[test]
fn get_engine_type_shell_to_batch() {
    let mut task = Task::new();
    task.script_runner = Some("@shell".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Shell2Batch);
}

#[test]
fn get_engine_type_generic() {
    let mut task = Task::new();
    task.script_runner = Some("test1".to_string());
    task.script_extension = Some("test2".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Generic);
}

#[test]
fn invoke_no_runner() {
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);

    let output = invoke(&task, &vec![]);

    assert!(output);
}

#[test]
fn invoke_no_script_no_runner() {
    let task = Task::new();

    let output = invoke(&task, &vec![]);

    assert!(!output);
}

#[test]
fn invoke_no_script() {
    let mut task = Task::new();
    task.script_runner = Some("@rust".to_string());

    let output = invoke(&task, &vec![]);

    assert!(!output);
}

#[test]
fn invoke_os_runner() {
    if !test::is_windows_on_travis_ci() {
        let mut task = Task::new();
        task.script_runner = Some(test::get_os_runner());
        task.script = Some(vec!["echo test".to_string()]);

        let output = invoke(&task, &vec![]);

        assert!(output);
    }
}

#[test]
fn invoke_rust_runner() {
    if test::should_test(false) {
        let mut task = Task::new();
        task.script_runner = Some("@rust".to_string());
        task.script = Some(vec!["fn main() {println!(\"test\");}".to_string()]);

        let output = invoke(&task, &vec![]);

        assert!(output);
    }
}

#[test]
#[should_panic]
fn invoke_rust_runner_error() {
    if test::should_test(true) {
        let mut task = Task::new();
        task.script_runner = Some("@rust".to_string());
        task.script = Some(vec!["fn main() {bad!(\"test\");}".to_string()]);

        let output = invoke(&task, &vec![]);

        assert!(output);
    }
}

#[test]
fn invoke_shell_to_batch_runner() {
    let mut task = Task::new();
    task.script_runner = Some("@shell".to_string());
    task.script = Some(vec!["echo test".to_string()]);

    let output = invoke(&task, &vec![]);

    assert!(output);
}

#[test]
#[should_panic]
fn invoke_shell_to_batch_runner_error() {
    let mut task = Task::new();
    task.script_runner = Some("@shell".to_string());
    task.script = Some(vec!["exit 1".to_string()]);

    let output = invoke(&task, &vec![]);

    assert!(output);
}

#[test]
fn invoke_generic_runner() {
    let mut task = Task::new();
    task.script_runner = Some(test::get_os_runner());
    task.script_extension = Some(test::get_os_extension());
    task.script = Some(vec!["echo test".to_string()]);

    let output = invoke(&task, &vec![]);

    assert!(output);
}

#[test]
#[should_panic]
fn invoke_generic_runner_error() {
    let mut task = Task::new();
    task.script_runner = Some(test::get_os_runner());
    task.script_extension = Some(test::get_os_extension());
    task.script = Some(vec!["exit 1".to_string()]);

    let output = invoke(&task, &vec![]);

    assert!(output);
}
