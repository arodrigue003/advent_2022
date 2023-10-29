#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DirEntry<'a> {
    pub name: &'a str,
    pub size: i64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FileEntry<'a> {
    pub name: &'a str,
    pub size: i64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Inode<'a> {
    Dir(DirEntry<'a>),
    File(FileEntry<'a>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum CdTarget<'a> {
    Root,
    Up,
    Directory(&'a str),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command<'a> {
    Ls(Vec<Inode<'a>>),
    Cd(CdTarget<'a>),
}
