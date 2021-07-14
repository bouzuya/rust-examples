fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        process::{Command, Stdio},
    };

    use anyhow::anyhow;
    use chrono::{TimeZone, Utc};
    use git2::Repository;
    use tempfile::tempdir;

    fn read_dir_all(p: &Path) -> anyhow::Result<Vec<PathBuf>> {
        let mut files = vec![];
        for dir_entry in p.read_dir()? {
            let dir_entry = dir_entry?;
            files.push(dir_entry.path().canonicalize()?);
            if dir_entry.path().is_dir() {
                files.extend(read_dir_all(dir_entry.path().as_path())?);
            }
        }
        Ok(files)
    }

    fn print_all(a: &[PathBuf]) {
        for a_i in a {
            eprintln!("{:?}", a_i);
        }
    }

    fn original_git_init(current_dir: &Path) -> anyhow::Result<bool> {
        let status = Command::new("git")
            .arg("init")
            .current_dir(current_dir)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        Ok(status.success())
    }

    fn original_git_add_all(current_dir: &Path) -> anyhow::Result<bool> {
        let status = Command::new("git")
            .arg("add")
            .arg("--all")
            .current_dir(current_dir)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        Ok(status.success())
    }

    fn original_git_commit_message(current_dir: &Path, message: &str) -> anyhow::Result<bool> {
        let status = Command::new("git")
            .arg("commit")
            .arg("--message")
            .arg(message)
            .current_dir(current_dir)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        Ok(status.success())
    }

    fn original_git_tag(current_dir: &Path, name: &str) -> anyhow::Result<bool> {
        let status = Command::new("git")
            .arg("tag")
            .arg(name)
            .current_dir(current_dir)
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        Ok(status.success())
    }

    #[test]
    fn init() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let git1 = temp_dir.path().join("git1");
        fs::create_dir_all(git1.as_path())?;
        let git2 = temp_dir.path().join("git2");
        fs::create_dir_all(git2.as_path())?;
        assert_eq!(original_git_init(git1.as_path())?, true);
        Repository::init(git2.as_path())?;

        // サンプルフック分の差異が出ている様子
        print_all(&read_dir_all(git1.as_path())?);
        print_all(&read_dir_all(git2.as_path())?);
        assert!(dir_diff::is_different(git1.as_path(), git2.as_path())
            .map_err(|_| { anyhow!("not eq") })?);
        Ok(())
    }

    #[test]
    fn open_and_commit_messages() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let git1 = temp_dir.path().join("git1");
        fs::create_dir_all(git1.as_path())?;
        assert_eq!(original_git_init(git1.as_path())?, true);
        let readme = git1.join("README.md");
        fs::write(readme.as_path(), "Hello, git2")?;
        assert_eq!(original_git_add_all(git1.as_path())?, true);
        assert_eq!(original_git_commit_message(git1.as_path(), "1st")?, true);
        fs::write(readme.as_path(), "Hello, git2!!!!!")?;
        assert_eq!(original_git_add_all(git1.as_path())?, true);
        assert_eq!(original_git_commit_message(git1.as_path(), "2nd")?, true);

        let repository = Repository::open(git1.as_path())?;
        let head = repository.head()?;
        assert_eq!(head.name(), Some("refs/heads/master"));
        let commit = head.peel_to_commit()?;
        assert_eq!(commit.message(), Some("2nd\n"));
        let parent = commit.parent(0)?;
        assert_eq!(parent.message(), Some("1st\n"));

        Ok(())
    }

    #[test]
    fn tags() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let git1 = temp_dir.path().join("git1");
        fs::create_dir_all(git1.as_path())?;
        assert_eq!(original_git_init(git1.as_path())?, true);
        let readme = git1.join("README.md");
        fs::write(readme.as_path(), "1")?;
        assert_eq!(original_git_add_all(git1.as_path())?, true);
        assert_eq!(original_git_commit_message(git1.as_path(), "1")?, true);
        assert_eq!(original_git_tag(git1.as_path(), "t1")?, true);
        fs::write(readme.as_path(), "2")?;
        assert_eq!(original_git_add_all(git1.as_path())?, true);
        assert_eq!(original_git_commit_message(git1.as_path(), "2")?, true);
        assert_eq!(original_git_tag(git1.as_path(), "t2")?, true);

        let repository = Repository::open(git1.as_path())?;
        let mut oids = vec![];
        let mut tags1 = vec![];
        repository.tag_foreach(|oid, name| {
            oids.push(oid);
            tags1.push(name.iter().copied().collect::<Vec<u8>>());
            true
        })?;
        assert_eq!(
            tags1,
            vec![
                "refs/tags/t1".bytes().collect::<Vec<u8>>(),
                "refs/tags/t2".bytes().collect::<Vec<u8>>(),
            ]
        );

        let mut commits = vec![];
        for oid in oids {
            let commit = repository.find_commit(oid)?;
            commits.push(commit);
        }
        assert_eq!(commits[0].message(), Some("1\n"));
        if false {
            println!("{:?}", Utc.timestamp(commits[0].time().seconds(), 0)); // commiter time
            println!(
                "{:?}",
                Utc.timestamp(commits[0].author().when().seconds(), 0)
            ); // author time
        }
        assert_eq!(commits[1].message(), Some("2\n"));

        let mut tags2 = vec![];
        let arr = repository.tag_names(None)?;
        for i in 0..arr.len() {
            if let Some(name) = arr.get(i) {
                tags2.push(name);
            }
        }
        assert_eq!(tags2, vec!["t1", "t2"]);

        Ok(())
    }
}
