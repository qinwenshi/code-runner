use std::path;
use crate::code_runner::non_empty_vec;


#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Assembly,
    Ats,
    Bash,
    C,
    Clojure,
    Cobol,
    CoffeeScript,
    Cpp,
    Crystal,
    Csharp,
    D,
    Elixir,
    Erlang,
    Haskell,
    Python,
}


#[derive(Debug)]
pub struct RunInstructions {
    pub build_commands: Vec<String>,
    pub run_command: String,
}


// TODO: implement all languages
pub fn run_instructions(language: &Language, files: non_empty_vec::NonEmptyVec<path::PathBuf>) -> RunInstructions {
    let (main_file, other_files) = files.parts();
    let main_file_str = main_file.to_string_lossy();

    match language {
        Language::Assembly => {
            RunInstructions{
                build_commands: vec![
                    format!("nasm -f elf64 -o a.o {}", main_file_str),
                    "ld -o a.out a.o".to_string(),
                ],
                run_command: "./a.out".to_string(),
            }
        }

        Language::Ats => {
            RunInstructions{
                build_commands: vec![
                    format!("patscc -o a.out {} {}", main_file_str, source_files(other_files, "dats")),
                ],
                run_command: "./a.out".to_string(),
            }
        }

        Language::Bash => {
            RunInstructions{
                build_commands: vec![],
                run_command: format!("bash {}", main_file_str),
            }
        }

        Language::C => {
            RunInstructions{
                build_commands: vec![
                    format!("clang -o a.out -lm {} {}", main_file_str, source_files(other_files, "c")),
                ],
                run_command: "./a.out".to_string(),
            }
        }

        Language::Clojure => {
            RunInstructions{
                build_commands: vec![],
                run_command: format!("clj {}", main_file_str),
            }
        }

        Language::Cobol => {
            RunInstructions{
                build_commands: vec![
                    format!("cobc -x -o a.out {} {}", main_file_str, source_files(other_files, "cob")),
                ],
                run_command: "./a.out".to_string(),
            }
        }

        Language::CoffeeScript => {
            RunInstructions{
                build_commands: vec![],
                run_command: format!("coffee {}", main_file_str),
            }
        }

        Language::Cpp => {
            RunInstructions{
                build_commands: vec![
                    format!("clang++ -std=c++11 -o a.out {} {}", main_file_str, source_files(other_files, "c")),
                ],
                run_command: "./a.out".to_string(),
            }
        }

        Language::Crystal => {
            RunInstructions{
                build_commands: vec![],
                run_command: format!("crystal run {}", main_file_str),
            }
        }

        Language::Csharp => {
            RunInstructions{
                build_commands: vec![
                    format!("mcs -out:a.exe {} {}", main_file_str, source_files(other_files, "cs"))
                ],
                run_command: "mono a.exe".to_string(),
            }
        }

        Language::D => {
            RunInstructions{
                build_commands: vec![
                    format!("dmd -ofa.out {} {}", main_file_str, source_files(other_files, "d"))
                ],
                run_command: "./a.out".to_string(),
            }
        }

        Language::Elixir => {
            RunInstructions{
                build_commands: vec![],
                run_command: format!("elixirc {} {}", main_file_str, source_files(other_files, "ex")),
            }
        }

        Language::Erlang => {
            RunInstructions{
                build_commands: filter_by_extension(other_files, "erl").iter().map(|file| {
                    format!("erlc {}", file.to_string_lossy())
                }).collect(),
                run_command: format!("escript {}", main_file_str),
            }
        }

        Language::Haskell => {
            RunInstructions{
                build_commands: vec![],
                run_command: format!("runghc {}", main_file_str),
            }
        }

        Language::Python => {
            RunInstructions{
                build_commands: vec![],
                run_command: format!("python {}", main_file_str),
            }
        }
    }
}

fn source_files(files: Vec<path::PathBuf>, extension: &str) -> String {
    space_separated_files(filter_by_extension(files, extension))
}

fn filter_by_extension(files: Vec<path::PathBuf>, extension: &str) -> Vec<path::PathBuf> {
    files
        .into_iter()
        .filter(|file| file.extension().and_then(|s| s.to_str()) == Some(extension))
        .collect()
}

fn space_separated_files(files: Vec<path::PathBuf>) -> String {
    files
        .iter()
        .map(|file| file.to_string_lossy().to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
