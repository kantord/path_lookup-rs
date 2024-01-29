use std::{collections::HashSet, env, fs};

pub fn get_executables() -> HashSet<String> {
    let serialized_paths = env::var("PATH").expect("Could not read PATH variable");
    let deserialized_paths = serialized_paths.split(":");
    let mut results: HashSet<String> = HashSet::new();

    for path in deserialized_paths {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if !entry.is_ok() {
                    continue;
                }

                let path = entry.unwrap().path();
                let command_name = path
                    .file_name()
                    .expect("Could not get file name")
                    .to_os_string()
                    .into_string()
                    .expect("Could not convert file name to string");

                if !path.is_file() {
                    continue;
                }

                if is_executable::is_executable(path) {
                    results.insert(command_name);
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_commands_that_definitely_exist_on_the_system() {
        let result = get_executables();
        assert!(result.contains("ls"));
        assert!(result.contains("mkdir"));
        assert!(result.contains("kill"));
        assert!(result.contains("man"));
    }

    #[test]
    fn does_not_return_commands_that_dont_exist_in_the_system() {
        let result = get_executables();
        assert!(!result.contains("asdfsdfadsfasdfdasfadsfadsfadsfafafdsfasf"));
    }
}
