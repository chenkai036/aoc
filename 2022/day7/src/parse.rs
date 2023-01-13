use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{
    alphanumeric1,
    digit1,
    line_ending,
    multispace0,
    not_line_ending,
    space1,
};
use nom::multi::{many0, fold_many0};
use nom::sequence::{delimited, separated_pair};

use crate::fs::{BasicFileSystem, DirectoryHandle};

#[derive(PartialEq, Eq, Debug)]
struct ChangeDirectoryOperation<'a> {
    path: &'a str,
}

#[derive(PartialEq, Eq, Debug)]
struct FileEntry<'a> {
    name: &'a str,
    size: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct DirectoryEntry<'a> {
    name: &'a str,
}

#[derive(PartialEq, Eq, Debug)]
enum ListEntry<'a> {
    File(FileEntry<'a>),
    Directory(DirectoryEntry<'a>),
}

#[derive(PartialEq, Eq, Debug)]
struct ListOperation<'a> {
    entries: Vec<ListEntry<'a>>,
}

#[derive(PartialEq, Eq, Debug)]
enum ShellOperation<'a> {
    ChangeDirectory(ChangeDirectoryOperation<'a>),
    List(ListOperation<'a>),
}

fn parse_cd(input: &str) -> IResult<&str, ShellOperation> {
    let (input, _) = tag("cd")(input)?;
    let (input, path) = delimited(
        space1,
        alt((alphanumeric1, tag("/"), tag(".."))),
        line_ending,
    )(input)?;
    Ok((input, ShellOperation::ChangeDirectory(ChangeDirectoryOperation { path })))
}

fn file_entry(input: &str) -> IResult<&str, ListEntry> {
    let (input, (size, name)) = separated_pair(digit1, space1, not_line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, ListEntry::File(FileEntry {name, size: size.parse().unwrap_or(0)})))
}

fn directory_entry(input: &str) -> IResult<&str, ListEntry> {
    let (input, (_, name)) = separated_pair(tag("dir"), space1, not_line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, ListEntry::Directory(DirectoryEntry { name })))
}

fn parse_ls(input: &str) -> IResult<&str, ShellOperation> {
    let (input, _) = tag("ls")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, entries) = many0(alt((file_entry, directory_entry)))(input)?;
    Ok((input, ShellOperation::List(ListOperation { entries })))
}

fn parse_cmd(input: &str) -> IResult<&str, ShellOperation> {
    // skip any whitespaces
    let (input, _) = multispace0(input)?;
    // parse prompt
    let (input, _) = tag("$ ")(input)?;
    // parse command
    alt((parse_cd, parse_ls))(input)
}

#[derive(Debug)]
pub struct ShellState {
    pub fs: BasicFileSystem,
    pub cwd: DirectoryHandle,
}

impl ShellState {
    fn new() -> Self {
        let fs = BasicFileSystem::new();
        let cwd = fs.root();
        ShellState { fs, cwd }
    }
}

pub fn parse(input: String) -> Result<ShellState, String> {
    let fold_fn = move |mut state: ShellState, operation: ShellOperation| -> ShellState {
        match operation {
            ShellOperation::ChangeDirectory(op) => {
                if op.path == ".." {
                    state.cwd = state.cwd.parent(&mut state.fs);
                } else if op.path == "/" {
                    state.cwd = state.fs.root();
                } else {
                    state.cwd = state.cwd.new_directory(op.path.into(), &mut state.fs).unwrap();
                }
            },
            ShellOperation::List(op) => {
                op.entries.iter().for_each(|entry| {
                    match entry {
                        ListEntry::File(file) => {
                            let _ = state.cwd.new_file(file.name.into(), file.size, &mut state.fs);
                        },
                        ListEntry::Directory(directory) => {
                            let _ = state.cwd.new_directory(directory.name.into(), &mut state.fs);
                        },
                    };
                });
            },
        }
        state
    };
    match fold_many0(parse_cmd, ShellState::new, fold_fn)(input.as_str()) {
        Ok((_, state)) => Ok(state),
        Err(error) => Err(error.to_string()),
    }
}

#[cfg(test)]
mod unittest {

    use super::*;
    use crate::fs::{
        FileHandle,
        File,
        Directory,
        Path,
    };

    #[test]
    fn parse_cd() {
        assert_eq!(
            parse_cmd("$ cd /\n"),
            Ok(("", ShellOperation::ChangeDirectory(ChangeDirectoryOperation { path: "/" }))));

        assert_eq!(
            parse_cmd("$ cd abc\n"),
            Ok(("", ShellOperation::ChangeDirectory(ChangeDirectoryOperation { path: "abc" }))));

        assert!(parse_cmd("$ cd\n").is_err());
    }

    #[test]
    fn parse_ls() {
        assert_eq!(
            parse_cmd("\n$ ls\n"),
            Ok(("", ShellOperation::List(ListOperation { entries: [].into() }))));

        let input =
r#"
$ ls
dir bzgf
199775 dngdnvv.qdf
dir fhhwv
"#;
        assert_eq!(
            parse_cmd(input),
            Ok(("", ShellOperation::List(ListOperation {
                entries: [
                    ListEntry::Directory(DirectoryEntry { name: "bzgf" }),
                    ListEntry::File(FileEntry { name: "dngdnvv.qdf", size: 199775 }),
                    ListEntry::Directory(DirectoryEntry { name: "fhhwv" }),
                ].into(),
            }))));

       assert!(parse_cmd("$ lsxxx\n").is_err());
    }

    #[test]
    fn parse_example() {
        // - / (dir)
        //  - a (dir)
        //    - e (dir)
        //      - i (file, size=584)
        //    - f (file, size=29116)
        //    - g (file, size=2557)
        //    - h.lst (file, size=62596)
        //  - b.txt (file, size=14848514)
        //  - c.dat (file, size=8504156)
        //  - d (dir)
        //    - j (file, size=4060174)
        //    - d.log (file, size=8033020)
        //    - d.ext (file, size=5626152)
        //    - k (file, size=7214296)
        let input =
r#"
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"#;
        let result = parse(input.into());
        assert!(result.is_ok());

        let state = result.unwrap();
        assert_eq!(state.cwd.view(&state.fs).name, "d");
        assert_eq!(state.cwd.abspath(&state.fs), "/d/");

        assert_eq!(state.fs.files, [
            File { handle: FileHandle { index: 0 }, name: "b.txt".into(), size: 14848514, dir: DirectoryHandle { index: 0 }},
            File { handle: FileHandle { index: 1 }, name: "c.dat".into(), size: 8504156, dir: DirectoryHandle { index: 0 }},
            File { handle: FileHandle { index: 2 }, name: "f".into(), size: 29116, dir: DirectoryHandle { index: 1 }},
            File { handle: FileHandle { index: 3 }, name: "g".into(), size: 2557, dir: DirectoryHandle { index: 1 }},
            File { handle: FileHandle { index: 4 }, name: "h.lst".into(), size: 62596, dir: DirectoryHandle { index: 1 }},
            File { handle: FileHandle { index: 5 }, name: "i".into(), size: 584, dir: DirectoryHandle { index: 3 }},
            File { handle: FileHandle { index: 6 }, name: "j".into(), size: 4060174, dir: DirectoryHandle { index: 2 }},
            File { handle: FileHandle { index: 7 }, name: "d.log".into(), size: 8033020, dir: DirectoryHandle { index: 2 }},
            File { handle: FileHandle { index: 8 }, name: "d.ext".into(), size: 5626152, dir: DirectoryHandle { index: 2 }},
            File { handle: FileHandle { index: 9 }, name: "k".into(), size: 7214296, dir: DirectoryHandle { index: 2 }},
        ]);
        assert_eq!(state.fs.dirs, [
            Directory {
                handle: DirectoryHandle { index: 0 },
                name: "/".into(),
                parent: DirectoryHandle { index: 0 },
                dirs: [
                    DirectoryHandle { index: 1 },
                    DirectoryHandle { index: 2 }
                ].into(),
                files: [
                    FileHandle { index: 0 },
                    FileHandle { index: 1 },
                ].into(),
            },
            Directory {
                handle: DirectoryHandle { index: 1 },
                name: "a".into(),
                parent: DirectoryHandle { index: 0 },
                dirs: [
                    DirectoryHandle { index: 3 },
                ].into(),
                files: [
                    FileHandle { index: 2 },
                    FileHandle { index: 3 },
                    FileHandle { index: 4 },
                ].into(),
            },
            Directory {
                handle: DirectoryHandle { index: 2 },
                name: "d".into(),
                parent: DirectoryHandle { index: 0 },
                dirs: [].into(),
                files: [
                    FileHandle { index: 6 },
                    FileHandle { index: 7 },
                    FileHandle { index: 8 },
                    FileHandle { index: 9 },
                ].into(),
            },
            Directory {
                handle: DirectoryHandle { index: 3 },
                name: "e".into(),
                parent: DirectoryHandle { index: 1 },
                dirs: [].into(),
                files: [
                    FileHandle { index: 5 },
                ].into(),
            },
        ]);
    }

}

