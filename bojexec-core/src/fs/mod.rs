use crate::fs::extension::CodeExtension;
pub use crate::fs::structure::FsConfig;
use crate::fs::template::render_markdown;
use crate::problem::{Problem, TestCase};
use std::path::PathBuf;
use std::{fs, io};

pub mod extension;
pub mod structure;
pub mod template;

const TEST_CASES_FILENAME: &str = "test-cases.json";
const DESCRIPTION_FILENAME: &str = "description.md";

pub fn read_solution(config: &FsConfig, problem_id: u32) -> io::Result<(String, CodeExtension)> {
    let solution_dir = config.get_solution_dir(problem_id)?;
    let mut solution_file: Option<PathBuf> = None;

    for entry in fs::read_dir(solution_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                let extension = CodeExtension::from_ext_str(ext_str);
                if extension != CodeExtension::Unknown {
                    solution_file = Some(path);
                    break;
                }
            }
        }
    }

    let solution_file = solution_file.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Solution file not found in the solution directory",
        )
    })?;

    let code = fs::read_to_string(&solution_file)?;
    let extension = solution_file
        .extension()
        .and_then(|ext| ext.to_str())
        .map(CodeExtension::from_ext_str)
        .unwrap_or(CodeExtension::Unknown);

    Ok((code, extension))
}

pub fn read_test_cases(config: &FsConfig, problem_id: u32) -> io::Result<Vec<TestCase>> {
    let problem_dir = config
        .get_problem_dir(problem_id)?
        .join(TEST_CASES_FILENAME);

    let saved_test_cases =
        fs::read_to_string(&problem_dir).expect("failed to read test-cases.json");

    Ok(serde_json::from_str(&saved_test_cases).expect("failed to deserialize test cases"))
}

pub fn save_problem(config: &FsConfig, problem: &Problem) -> io::Result<()> {
    let description = render_markdown(problem).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Template render error: {}", e),
        )
    })?;

    let problem_dir = config.get_problem_dir(problem.id)?;
    fs::write(problem_dir.join(DESCRIPTION_FILENAME), description)?;
    fs::write(
        problem_dir.join(TEST_CASES_FILENAME),
        serde_json::to_string_pretty(&problem.testcases)?,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::structure::DirectoryStructure;
    use crate::problem::{Description, TestCase};
    use std::fs;
    use tempfile::tempdir;

    fn create_test_config() -> (FsConfig, tempfile::TempDir) {
        let temp_dir = tempdir().expect("failed to create temporary directory");
        let base_dir = temp_dir.path();

        let config = FsConfig {
            problems_base: base_dir.join("problems"),
            solutions_base: Some(base_dir.join("solutions")),
            structure: DirectoryStructure::Separate,
            problem_set: Default::default(),
        };

        fs::create_dir_all(&config.problems_base).expect("failed to create problems directory");
        fs::create_dir_all(config.solutions_base.as_ref().unwrap())
            .expect("failed to create solutions directory");

        (config, temp_dir)
    }

    #[test]
    fn test_read_solution() {
        let (config, _temp) = create_test_config();
        let problem_id = 1000;
        let solution_dir = config.get_solution_dir(problem_id).unwrap();
        fs::create_dir_all(&solution_dir).expect("failed to create solution directory");

        let solution_content = "print('Hello, World!')";
        fs::write(solution_dir.join("solution.py"), solution_content)
            .expect("failed to write solution file");

        let (code, extension) =
            read_solution(&config, problem_id).expect("failed to read solution");
        assert_eq!(code, solution_content);
        assert_eq!(extension, CodeExtension::Python);
    }

    #[test]
    fn test_read_test_cases() {
        let (config, _temp) = create_test_config();
        let problem_id = 1000;
        let problem_dir = config.get_problem_dir(problem_id).unwrap();
        fs::create_dir_all(&problem_dir).expect("failed to create problem directory");

        let test_cases = vec![
            TestCase {
                input: "Input 1".to_string(),
                output: "Output 1".to_string(),
            },
            TestCase {
                input: "Input 2".to_string(),
                output: "Output 2".to_string(),
            },
        ];

        fs::write(
            problem_dir.join(TEST_CASES_FILENAME),
            serde_json::to_string_pretty(&test_cases).unwrap(),
        )
        .expect("failed to write test cases file");

        let loaded_test_cases =
            read_test_cases(&config, problem_id).expect("failed to read test cases");
        assert_eq!(loaded_test_cases.len(), 2);
        assert_eq!(loaded_test_cases[0].input, "Input 1");
        assert_eq!(loaded_test_cases[1].output, "Output 2");
    }

    #[test]
    fn test_save_problem() {
        let (config, _temp) = create_test_config();

        let problem = Problem {
            id: 1000,
            title: "Sample Problem".to_string(),
            description: Description {
                problem: "This is a sample problem description.".to_string(),
                input: "Sample input description.".to_string(),
                output: "Sample output description.".to_string(),
            },
            testcases: vec![
                TestCase {
                    input: "Input 1".to_string(),
                    output: "Output 1".to_string(),
                },
                TestCase {
                    input: "Input 2".to_string(),
                    output: "Output 2".to_string(),
                },
            ],
        };

        save_problem(&config, &problem).expect("failed to save problem");

        let problem_dir = config.get_problem_dir(problem.id).unwrap();
        assert!(problem_dir.exists(), "problem directory does not exist");

        let desc_path = problem_dir.join(DESCRIPTION_FILENAME);
        let test_cases_path = problem_dir.join(TEST_CASES_FILENAME);
        assert!(desc_path.exists(), "description.md does not exist");
        assert!(test_cases_path.exists(), "test-cases.json does not exist");

        let saved_desc = fs::read_to_string(&desc_path).expect("failed to read description.md");
        assert!(saved_desc.contains("# Sample Problem"));
        assert!(saved_desc.contains("This is a sample problem description."));

        let saved_test_cases =
            fs::read_to_string(&test_cases_path).expect("failed to read test-cases.json");
        let deserialized: Vec<TestCase> =
            serde_json::from_str(&saved_test_cases).expect("failed to deserialize test cases");
        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0].input, "Input 1");
        assert_eq!(deserialized[1].output, "Output 2");
    }

    #[test]
    fn test_read_solution_not_found() {
        let (config, _temp) = create_test_config();
        let problem_id = 1000;
        let solution_dir = config.get_solution_dir(problem_id).unwrap();
        fs::create_dir_all(&solution_dir).expect("failed to create solution directory");

        let result = read_solution(&config, problem_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }
}
