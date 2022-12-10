use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use std::vec;

use anyhow::{Error, format_err, Result};
use itertools::Itertools;

use challenges_common::MyIterTools;

fn main() {
    let input_lines = challenges_common::get_input_lines(&["aoc", "2022", "7.txt"]);
    let commands = parse(input_lines);
    println!("part1: {}", part1(commands).unwrap());
}

fn parse(input_lines: impl IntoIterator<Item=impl AsRef<str>>) -> Vec<Command> {
    input_lines
        .into_iter()
        .chunks_starting_by(|line| line.as_ref().starts_with("$"))
        .map(Command::try_from)
        .map(Result::unwrap)
        .collect()
}

fn part1(commands: Vec<Command>) -> Result<u32> {
    let mut context = Context::new();
    for command in commands {
        context.apply(&command)?;
    }
    let root = context.root.borrow();
    Ok(root.size())
}

#[derive(Debug, PartialEq)]
enum Command {
    Cd { dir: CdTarget },
    Ls { files: Vec<LsFileOutput> },
}

impl<S: AsRef<str>> TryFrom<Vec<S>> for Command {
    type Error = Error;

    fn try_from(value: Vec<S>) -> std::result::Result<Self, Self::Error> {
        let mut lines = value.iter();
        let command = lines.next().unwrap().as_ref();
        match command.split(" ").collect_vec()[..] {
            ["$", "cd", target] => Ok(Command::Cd {
                dir: target.parse()?,
            }),
            ["$", "ls"] => Ok(Command::Ls {
                files: lines
                    .map(AsRef::as_ref)
                    .map(|ls_output_line| ls_output_line.parse())
                    .collect::<Result<_>>()?,
            }),
            _ => Err(format_err!("command not supported: {}", command)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum CdTarget {
    Root,
    Parent,
    Dir { name: String },
}

impl FromStr for CdTarget {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "/" => Ok(Self::Root),
            ".." => Ok(Self::Parent),
            _ => Ok(Self::Dir {
                name: s.to_string(),
            }),
        }
    }
}

#[derive(PartialEq, Debug)]
enum LsFileOutput {
    Dir { name: String },
    File { name: String, size: u32 },
}

impl FromStr for LsFileOutput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.split(" ").collect_vec()[..] {
            ["dir", dir_name] => Ok(Self::Dir {
                name: dir_name.to_string(),
            }),
            [size, file_name] => Ok(Self::File {
                name: file_name.to_string(),
                size: size.parse()?,
            }),
            _ => Err(format_err!("cannot parse ls file output from: {}", s)),
        }
    }
}

#[derive(Clone)]
enum FileNode {
    Dir {
        name: String,
        files: Vec<Rc<RefCell<Self>>>,
        cached_size: Option<u32>,
    },
    File {
        name: String,
        size: u32,
    },
}

impl FileNode {
    fn dir(name: impl Into<String>) -> Self {
        Self::Dir {
            name: name.into(),
            cached_size: None,
            files: Vec::new(),
        }
    }

    fn file(name: impl Into<String>, size: u32) -> Self {
        Self::File {
            name: name.into(),
            size,
        }
    }

    fn size(&self) -> u32 {
        match self {
            FileNode::Dir {
                mut cached_size,
                files,
                ..
            } => *cached_size
                .get_or_insert_with(|| files.iter().map(|file| file.borrow().size()).sum()),
            FileNode::File { size, .. } => *size,
        }
    }

    fn name(&self) -> &String {
        match self {
            FileNode::File { name, .. } | FileNode::Dir { name, .. } => name,
        }
    }

    fn child(&self, name: &String) -> Option<Rc<RefCell<Self>>> {
        match self {
            FileNode::Dir { files, .. } => files
                .iter()
                .find(|f| f.borrow().name() == name)
                .map(Rc::clone),
            FileNode::File { .. } => None,
        }
    }

    fn add_file(&mut self, file: Self) -> Result<()> {
        match self {
            FileNode::Dir { files, .. } => {
                files.push(Rc::new(RefCell::new(file)));
                Ok(())
            }
            FileNode::File { .. } => Err(Error::msg("file cannot have childs")),
        }
    }
}

struct Context {
    root: Rc<RefCell<FileNode>>,
    current_path: Vec<Rc<RefCell<FileNode>>>,
}

impl Context {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(FileNode::dir("/")));
        let root_for_path = root.clone();
        Self {
            root,
            current_path: vec![root_for_path],
        }
    }

    fn current_node(&self) -> &Rc<RefCell<FileNode>> {
        self.current_path.last().unwrap()
    }

    fn apply(&mut self, command: &Command) -> Result<()> {
        match command {
            Command::Cd { dir } => match dir {
                CdTarget::Root => {
                    self.current_path = vec![self.root.clone()];
                }
                CdTarget::Parent => {
                    self.current_path.pop().ok_or(Error::msg("current path should not be empty"))?;
                    if self.current_path.is_empty() {
                        return Err(Error::msg("empty current dir"));
                    }
                }
                CdTarget::Dir { name } => {
                    let file = self
                        .current_node()
                        .borrow()
                        .child(name)
                        .ok_or(Error::msg("no such file"))?;
                    self.current_path.push(file)
                }
            },
            Command::Ls { files } => {
                let mut current_node = self.current_node().borrow_mut();
                for file in files {
                    let file = match file {
                        LsFileOutput::Dir { name } => FileNode::dir(name),
                        LsFileOutput::File { name, size } => FileNode::file(name, *size),
                    };
                    current_node.add_file(file)?;
                }
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod parsing {
        mod command {
            mod output {
                use crate::*;

                #[test]
                fn parsing_ls_dir_output() {
                    assert_eq!(
                        "dir dirname".parse::<LsFileOutput>().unwrap(),
                        LsFileOutput::Dir {
                            name: "dirname".to_string()
                        }
                    )
                }

                #[test]
                fn parsing_ls_file_output() {
                    assert_eq!(
                        "321 file.txt".parse::<LsFileOutput>().unwrap(),
                        LsFileOutput::File {
                            name: "file.txt".to_string(),
                            size: 321,
                        }
                    )
                }
            }

            mod cd {
                use crate::*;

                #[test]
                fn parsing_cd_root_command() {
                    let command: Command = vec!["$ cd /"].try_into().unwrap();
                    assert_eq!(
                        command,
                        Command::Cd {
                            dir: CdTarget::Root
                        }
                    );
                }

                #[test]
                fn parsing_cd_parent_command() {
                    let command: Command = vec!["$ cd .."].try_into().unwrap();
                    assert_eq!(
                        command,
                        Command::Cd {
                            dir: CdTarget::Parent
                        }
                    );
                }

                #[test]
                fn parsing_cd_dir_command() {
                    let command: Command = vec!["$ cd abc"].try_into().unwrap();
                    assert_eq!(
                        command,
                        Command::Cd {
                            dir: CdTarget::Dir {
                                name: "abc".to_string()
                            }
                        }
                    );
                }
            }

            mod ls {
                use crate::*;

                #[test]
                fn parsing_ls_command() {
                    let command: Command = vec!["$ ls", "dir a", "14848514 b.txt", "8504156 c.dat"]
                        .try_into()
                        .unwrap();
                    assert_eq!(
                        command,
                        Command::Ls {
                            files: vec![
                                LsFileOutput::Dir {
                                    name: "a".to_string()
                                },
                                LsFileOutput::File {
                                    name: "b.txt".to_string(),
                                    size: 14848514,
                                },
                                LsFileOutput::File {
                                    name: "c.dat".to_string(),
                                    size: 8504156,
                                },
                            ]
                        }
                    )
                }
            }
        }

        mod commands {
            use CdTarget::*;

            use crate::*;

            #[test]
            fn parse_simplified_given_example() {
                let commands = parse(vec![
                    "$ cd /",
                    "$ ls",
                    "dir a",
                    "14848514 b.txt",
                    "8504156 c.dat",
                    "$ cd a",
                    "$ cd ..",
                ]);

                assert_eq!(
                    commands,
                    vec![
                        Command::Cd { dir: Root },
                        Command::Ls {
                            files: vec![
                                LsFileOutput::Dir {
                                    name: "a".to_string()
                                },
                                LsFileOutput::File {
                                    name: "b.txt".to_string(),
                                    size: 14848514,
                                },
                                LsFileOutput::File {
                                    name: "c.dat".to_string(),
                                    size: 8504156,
                                },
                            ]
                        },
                        Command::Cd {
                            dir: Dir {
                                name: "a".to_string()
                            }
                        },
                        Command::Cd { dir: Parent },
                    ]
                )
            }
        }
    }
}
