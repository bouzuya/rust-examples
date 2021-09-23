use std::{
    fs::{read_dir, DirEntry},
    io,
    path::{Path, PathBuf},
};

fn read_dir_sorted<P: AsRef<Path>>(path: P) -> io::Result<Vec<DirEntry>> {
    let mut dir_entries = read_dir(path)?
        .into_iter()
        .collect::<io::Result<Vec<DirEntry>>>()?;
    dir_entries.sort_by_key(|dir_entry| dir_entry.file_name());
    Ok(dir_entries)
}

#[derive(Debug)]
struct DirEntryIter {
    i: usize,
    v: Vec<DirEntry>,
}

impl From<Vec<DirEntry>> for DirEntryIter {
    fn from(v: Vec<DirEntry>) -> DirEntryIter {
        Self { i: 0, v }
    }
}

impl Iterator for DirEntryIter {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.i;
        if index < self.v.len() {
            self.i += 1;
            Some(self.v[index].path())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    fn setup() -> anyhow::Result<TempDir> {
        let tempdir = tempfile::tempdir()?;
        let root_dir = tempdir.path();
        let dir1 = root_dir.join("dir1");
        fs::create_dir_all(dir1.as_path())?;
        let dir2 = root_dir.join("dir2");
        fs::create_dir_all(dir2.as_path())?;
        let file1 = dir1.join("file1");
        fs::write(file1, "file1 contents")?;
        let file2 = dir1.join("file2");
        fs::write(file2, "file2 contents")?;
        let file3 = dir2.join("file1");
        fs::write(file3, "file1 contents")?;
        let file4 = dir2.join("file2");
        fs::write(file4, "file2 contents")?;
        Ok(tempdir)
    }

    #[test]
    fn read_dir_sorted_test() -> anyhow::Result<()> {
        let tempdir = setup()?;
        let dir_entries = read_dir_sorted(tempdir.path())?;
        assert_eq!(
            dir_entries
                .into_iter()
                .map(|e| e.path())
                .collect::<Vec<PathBuf>>(),
            vec![tempdir.path().join("dir1"), tempdir.path().join("dir2")]
        );
        Ok(())
    }

    #[test]
    fn dir_entry_iter_test() -> anyhow::Result<()> {
        let tempdir = setup()?;
        {
            let dir_entries = read_dir_sorted(tempdir.path())?;
            let iter = DirEntryIter::from(dir_entries);
            assert_eq!(
                iter.collect::<Vec<PathBuf>>(),
                vec![tempdir.path().join("dir1"), tempdir.path().join("dir2")]
            );
        }
        {
            let dir_entries = read_dir_sorted(tempdir.path().join("dir1"))?;
            let iter = DirEntryIter::from(dir_entries);
            assert_eq!(
                iter.collect::<Vec<PathBuf>>(),
                vec![
                    tempdir.path().join("dir1").join("file1"),
                    tempdir.path().join("dir1").join("file2"),
                ]
            );
        }
        Ok(())
    }
}
