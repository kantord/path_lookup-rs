use std::{collections::HashSet, env, fs};

/// Returns all exectuabqles found in $PATH as a HashSet<String>.
/// There is no performance penalty compared to iteration.
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

/// Returns an iterator of the exectuables found in $PATH.
/// Note that it still allocates memory because it is necessary to
/// deduplicate the results.
pub fn iterate_executables() -> impl Iterator<Item = String> {
    // I spent significant amount of time realizing that allocating a data structure
    // for deduplication purposes is necessary, thus iteration without memory allocation
    // is not possible.

    // This function is functionally redundant, but it can save time by signaling
    // that is is already the optimal iteration algorithm. It also leaves the door
    // open for any future optimization without breaking changes.

    get_executables().into_iter()
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

    #[test]
    fn iteration_and_hashmap_have_the_same_results() {
        let mut results_from_iterator: HashSet<String> = HashSet::new();
        for command in iterate_executables() {
            results_from_iterator.insert(command);
        }

        assert_eq!(results_from_iterator, get_executables());
    }
}
