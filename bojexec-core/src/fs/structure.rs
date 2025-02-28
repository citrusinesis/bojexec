use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum DirectoryStructure {
    /// Combined: everything in one directory.
    /// Example: problems/baekjoon/1000/description.md, test-cases.json, solution.rs
    Combined,
    /// Separate: one directory for solutions and one for problems.
    /// Example:
    ///   solutions/baekjoon/1000/solution.rs
    ///   problems/baekjoon/1000/description.md, test-cases.json
    Separate,
}

#[derive(Debug, Clone)]
pub struct FsConfig {
    pub problems_base: PathBuf,
    pub solutions_base: Option<PathBuf>,
    pub structure: DirectoryStructure,
    pub problem_set: PathBuf,
}

impl FsConfig {
    pub fn get_problem_dir(&self, problem_id: u32) -> io::Result<PathBuf> {
        let dir = self
            .problems_base
            .join(&self.problem_set)
            .join(problem_id.to_string());
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    pub fn get_solution_dir(&self, problem_id: u32) -> io::Result<PathBuf> {
        let dir = match self.structure {
            DirectoryStructure::Combined => self
                .problems_base
                .join(&self.problem_set)
                .join(problem_id.to_string()),
            DirectoryStructure::Separate => {
                if let Some(ref base) = self.solutions_base {
                    base.join(&self.problem_set).join(problem_id.to_string())
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "solutions_base is not set for Separate structure",
                    ));
                }
            }
        };
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}
