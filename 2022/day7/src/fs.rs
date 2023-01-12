pub trait Path {
    fn abspath(&self, fs: &BasicFileSystem) -> String;
}

pub trait Tree {
    fn size(&self, fs: &BasicFileSystem) -> usize;
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct FileHandle {
    pub index: usize,
}

impl FileHandle {
    pub fn view<'a>(&'a self, fs: &'a BasicFileSystem) -> &File {
        fs.file_entry(self.index).unwrap()
    }

    pub fn directory(&self, fs: &BasicFileSystem) -> DirectoryHandle {
        self.view(&fs).dir
    }
}

impl Path for FileHandle {
    fn abspath(&self, fs: &BasicFileSystem) -> String {
        let view = self.view(&fs);
        let directory = self.directory(&fs);
        let mut path = directory.abspath(&fs);
        path.push_str(view.name.as_str());
        path
    }
}

impl Tree for FileHandle {
    fn size(&self, fs: &BasicFileSystem) -> usize {
        self.view(&fs).size
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct DirectoryHandle {
    pub index: usize,
}

impl DirectoryHandle {
    pub fn view<'a>(&'a self, fs: &'a BasicFileSystem) -> &Directory {
        fs.dir_entry(self.index).unwrap()
    }

    pub fn new_file<'a>(
        &'a mut self,
        name: String,
        size: usize,
        fs: &'a mut BasicFileSystem,
    ) -> Result<FileHandle, &str> {
        fs.new_file(name, size, self.index).map(|file| file.handle)
    }

    pub fn new_directory<'a>(
        &'a mut self,
        name: String,
        fs: &'a mut BasicFileSystem,
    ) -> Result<DirectoryHandle, &str> {
        fs.new_directory(name, self.index).map(|dir| dir.handle)
    }

    pub fn parent(&self, fs: &BasicFileSystem) -> DirectoryHandle {
        self.view(&fs).parent
    }

    pub fn walk<DirectoryFn, FileFn>(
        &self,
        fs: &BasicFileSystem,
        dir_fn: &DirectoryFn,
        file_fn: &FileFn,
    ) where
        DirectoryFn: Fn(&Directory),
        FileFn: Fn(&File),
    {
        let view = self.view(&fs);
        dir_fn(&view);
        view.files
            .iter()
            .for_each(|file| file_fn(fs.file_entry(file.index).unwrap()));
        view.dirs
            .iter()
            .for_each(|dir| dir.walk(&fs, dir_fn, file_fn));
    }

    pub fn fold<AccumTy, EntryFn>(
        &self,
        fs: &BasicFileSystem,
        accum: AccumTy,
        entry_fn: &EntryFn,
    ) -> AccumTy
    where
        EntryFn: Fn(AccumTy, Entry) -> AccumTy,
    {
        let view = self.view(&fs);
        let accum = view
            .dirs
            .iter()
            .fold(accum, |accum, handle| handle.fold(&fs, accum, entry_fn));
        let accum = view.files.iter().fold(accum, |accum, handle| {
            entry_fn(accum, Entry::File(handle.view(&fs)))
        });
        entry_fn(accum, Entry::Directory(view))
    }
}

impl Path for DirectoryHandle {
    fn abspath(&self, fs: &BasicFileSystem) -> String {
        let view = self.view(&fs);
        if view.is_root() {
            fs.sep().into()
        } else {
            let parent = self.parent(&fs);
            let mut path = parent.abspath(&fs);
            path.push_str(view.name.as_str());
            path.push_str(fs.sep());
            path
        }
    }
}

impl Tree for DirectoryHandle {
    fn size(&self, fs: &BasicFileSystem) -> usize {
        self.fold(&fs, 0usize, &|accum, entry| match entry {
            Entry::File(file) => accum + file.size,
            Entry::Directory(_) => accum,
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct File {
    pub handle: FileHandle,
    pub name: String,
    pub size: usize,
    pub dir: DirectoryHandle,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Directory {
    pub handle: DirectoryHandle,
    pub name: String,
    pub dirs: Vec<DirectoryHandle>,
    pub files: Vec<FileHandle>,
    pub parent: DirectoryHandle,
}

impl Directory {
    pub fn is_root(&self) -> bool {
        self.handle == self.parent
    }
}

pub enum Entry<'a> {
    File(&'a File),
    Directory(&'a Directory),
}

#[derive(Debug)]
pub struct BasicFileSystem {
    pub dirs: Vec<Directory>,
    pub files: Vec<File>,
}

impl BasicFileSystem {
    pub fn new() -> Self {
        let handle = DirectoryHandle { index: 0 };
        let root = Directory {
            handle,
            name: "/".into(),
            parent: handle,
            dirs: [].into(),
            files: [].into(),
        };
        BasicFileSystem {
            dirs: [root].into(),
            files: [].into(),
        }
    }

    fn sep(&self) -> &str {
        "/"
    }

    pub fn root(&self) -> DirectoryHandle {
        DirectoryHandle { index: 0 }
    }

    fn dir_entry(&self, index: usize) -> Option<&Directory> {
        self.dirs.get(index)
    }

    fn file_entry(&self, index: usize) -> Option<&File> {
        self.files.get(index)
    }

    fn new_file(&mut self, name: String, size: usize, directory: usize) -> Result<&File, &str> {
        let parent = self.dirs.get(directory);
        if parent.is_none() {
            return Err("Bad directory index");
        }
        let mut files = parent.unwrap().files.iter();
        if let Some(handle) = files.find(|handle| handle.view(&self).name == name) {
            return self.file_entry(handle.index).ok_or("Invalid file index")
        }
        let index = self.files.len();
        let handle = FileHandle { index };
        let parent = self.dirs.get_mut(directory).unwrap();
        self.files.push(File {
            name,
            size,
            handle,
            dir: parent.handle,
        });
        parent.files.push(handle);
        self.file_entry(index).ok_or("Invalid file index")
    }

    fn new_directory(&mut self, name: String, parent: usize) -> Result<&Directory, &str> {
        let parent_directory = self.dirs.get(parent);
        if parent_directory.is_none() {
            return Err("Bad parent index");
        }
        let mut dirs = parent_directory.unwrap().dirs.iter();
        if let Some(handle) = dirs.find(|handle| handle.view(&self).name == name) {
            return self.dir_entry(handle.index).ok_or("Invalid directory index");
        }
        let index = self.dirs.len();
        let handle = DirectoryHandle { index };
        let parent_directory = self.dirs.get_mut(parent).unwrap();
        let directory = Directory {
            handle,
            name,
            parent: parent_directory.handle,
            dirs: [].into(),
            files: [].into(),
        };
        parent_directory.dirs.push(handle);
        self.dirs.push(directory);
        self.dir_entry(index).ok_or("Invalid directory index")
    }
}

#[cfg(test)]
mod unittest {

    use super::*;

    /// A basic test for [FileHandle]
    #[test]
    fn basic_file_handle() {
        let mut fs = BasicFileSystem::new();
        let mut root = fs.root();

        let file1 = root.new_file("file1".into(), 42, &mut fs);
        assert!(file1.is_ok());

        let file1 = file1.unwrap();
        assert_eq!(file1.view(&fs).name, "file1");
        assert_eq!(file1.view(&fs).dir.index, root.index);
    }

    #[test]
    fn duplicated_entry() {
        let mut fs = BasicFileSystem::new();
        let mut root = fs.root();
        let file1 = root.new_file("file".into(), 42, &mut fs).unwrap();
        let file2 = root.new_file("file".into(), 42, &mut fs).unwrap();
        assert_eq!(file1, file2);

        let mut dir1 = root.new_directory("dir".into(), &mut fs).unwrap();
        let dir2 = root.new_directory("dir".into(), &mut fs).unwrap();
        assert_eq!(dir1, dir2);

        let file3 = dir1.new_file("file".into(), 42, &mut fs).unwrap();
        assert_ne!(file1, file3);

        let dir3 = dir1.new_directory("dir".into(), &mut fs).unwrap();
        assert_ne!(dir1, dir3);
    }

    /// A basic test for [DirectoryHandle]
    #[test]
    fn basic_dir_handle() {
        let mut fs = BasicFileSystem::new();
        let mut root = fs.root();

        let dir1 = root.new_directory("dir1".into(), &mut fs);
        assert!(dir1.is_ok());

        let mut dir1 = dir1.unwrap();
        let view1 = dir1.view(&fs);
        assert_eq!(view1.name, "dir1");
        assert_eq!(view1.parent, root);

        let dir2 = dir1.new_directory("dir2".into(), &mut fs);
        assert!(dir2.is_ok());

        let mut dir2 = dir2.unwrap();
        let view2 = dir2.view(&fs);
        assert_eq!(view2.name, "dir2");
        assert_eq!(view2.parent, dir1);
        assert!(dir1.view(&fs).dirs.contains(&dir2));

        let file1 = dir2.new_file("file1".into(), 42, &mut fs);
        assert!(file1.is_ok());
        let file1 = file1.unwrap();
        assert!(dir2.view(&fs).files.contains(&file1));
    }

    /// A basic test for absolute path
    #[test]
    fn abspath() {
        let mut fs = BasicFileSystem::new();
        let mut root = fs.root();
        assert_eq!(root.abspath(&fs), "/");

        let dir1 = root.new_directory("dir1".into(), &mut fs);
        assert!(dir1.is_ok());

        let mut dir1 = dir1.unwrap();
        assert_eq!(dir1.abspath(&fs), "/dir1/");

        let file1 = dir1.new_file("file1".into(), 42, &mut fs);
        assert!(file1.is_ok());

        let file1 = file1.unwrap();
        assert_eq!(file1.abspath(&fs), "/dir1/file1");
    }

    /// Test tree walk and `size`
    #[test]
    fn walk_and_size() {
        let mut fs = BasicFileSystem::new();
        let mut root = fs.root();

        let dir1 = root.new_directory("dir1".into(), &mut fs);
        let mut dir1 = dir1.unwrap();
        let _ = root.new_directory("dir2".into(), &mut fs);
        let _ = root.new_file("file1".into(), 42, &mut fs);

        let dir3 = dir1.new_directory("dir3".into(), &mut fs);
        let mut dir3 = dir3.unwrap();
        let _ = dir3.new_file("file2".into(), 99, &mut fs);

        let disp_dir = |dir: &Directory| {
            println!("{:?}", dir.handle.abspath(&fs));
        };
        let disp_file = |file: &File| {
            println!("{:?}", file.handle.abspath(&fs));
        };

        root.walk(&fs, &disp_dir, &disp_file);

        assert_eq!(dir3.size(&fs), 99);
        assert_eq!(root.size(&fs), 42 + 99);
    }
}

