use day07::model::{CdTarget, Command, DirEntry, FileEntry, Inode};
use day07::parser::{
    parse_cd_command, parse_dir_line, parse_file_line, parse_input, parse_ls_command, parse_ls_line,
};

#[test]
fn test_parse_dir_line() {
    assert_eq!(parse_dir_line("dir a\n"), Ok(("", DirEntry { name: "a" })));
}

#[test]
fn test_parse_file_line() {
    assert_eq!(
        parse_file_line("14848514 b.txt\n"),
        Ok((
            "",
            FileEntry {
                name: "b.txt",
                size: 14848514
            }
        ))
    )
}

#[test]
fn test_parse_ls_line() {
    assert_eq!(
        parse_ls_line("dir a\n"),
        Ok(("", Inode::Dir(DirEntry { name: "a" })))
    );
    assert_eq!(
        parse_ls_line("14848514 b.txt\n"),
        Ok((
            "",
            Inode::File(FileEntry {
                name: "b.txt",
                size: 14848514
            })
        ))
    )
}

#[test]
fn test_parse_ls_command() {
    let to_parse: &'static str = "$ ls
dir a
14848514 b.txt
8504156 c.dat
";

    assert_eq!(
        parse_ls_command(to_parse),
        Ok((
            "",
            vec![
                Inode::Dir(DirEntry { name: "a" }),
                Inode::File(FileEntry {
                    name: "b.txt",
                    size: 14848514
                }),
                Inode::File(FileEntry {
                    name: "c.dat",
                    size: 8504156
                })
            ]
        ))
    )
}

#[test]
fn test_parse_cd_line() {
    assert_eq!(parse_cd_command("$ cd /\n"), Ok(("", CdTarget::Root)));
    assert_eq!(parse_cd_command("$ cd ..\n"), Ok(("", CdTarget::Up)));
    assert_eq!(
        parse_cd_command("$ cd a\n"),
        Ok(("", CdTarget::Directory("a")))
    );
}

#[test]
fn test_parse_input() {
    let to_parse: &'static str = "$ cd /
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
";

    assert_eq!(
        parse_input(to_parse),
        Ok((
            "",
            vec![
                Command::Cd(CdTarget::Root),
                Command::Ls(vec![
                    Inode::Dir(DirEntry { name: "a" }),
                    Inode::File(FileEntry {
                        name: "b.txt",
                        size: 14848514
                    }),
                    Inode::File(FileEntry {
                        name: "c.dat",
                        size: 8504156
                    }),
                    Inode::Dir(DirEntry { name: "d" })
                ]),
                Command::Cd(CdTarget::Directory("a")),
                Command::Ls(vec![
                    Inode::Dir(DirEntry { name: "e" }),
                    Inode::File(FileEntry {
                        name: "f",
                        size: 29116
                    }),
                    Inode::File(FileEntry {
                        name: "g",
                        size: 2557
                    }),
                    Inode::File(FileEntry {
                        name: "h.lst",
                        size: 62596
                    }),
                ]),
                Command::Cd(CdTarget::Directory("e")),
                Command::Ls(vec![Inode::File(FileEntry {
                    name: "i",
                    size: 584
                })]),
                Command::Cd(CdTarget::Up),
                Command::Cd(CdTarget::Up),
                Command::Cd(CdTarget::Directory("d")),
                Command::Ls(vec![
                    Inode::File(FileEntry {
                        name: "j",
                        size: 4060174
                    }),
                    Inode::File(FileEntry {
                        name: "d.log",
                        size: 8033020
                    }),
                    Inode::File(FileEntry {
                        name: "d.ext",
                        size: 5626152
                    }),
                    Inode::File(FileEntry {
                        name: "k",
                        size: 7214296
                    }),
                ])
            ]
        ))
    );
}
