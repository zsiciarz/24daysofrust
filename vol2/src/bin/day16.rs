#[cfg(target_family = "unix")]
extern crate git2;
extern crate time;

#[cfg(target_family = "unix")]
use git2::{Commit, Direction, ObjectType, Oid, Repository, Signature};
#[cfg(target_family = "unix")]
use std::fs::{File, canonicalize};
#[cfg(target_family = "unix")]
use std::io::Write;
#[cfg(target_family = "unix")]
use std::path::Path;

#[cfg(target_family = "unix")]
fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| {
        git2::Error::from_str("Couldn't find commit")
    })
}

#[cfg(target_family = "unix")]
fn display_commit(commit: &Commit) {
    let timestamp = commit.time().seconds();
    let tm = time::at(time::Timespec::new(timestamp, 0));
    println!(
        "commit {}\nAuthor: {}\nDate:   {}\n\n    {}",
        commit.id(),
        commit.author(),
        tm.rfc822(),
        commit.message().unwrap_or("no commit message")
    );
}

#[cfg(target_family = "unix")]
fn add_and_commit(repo: &Repository, path: &Path, message: &str) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    index.add_path(path)?;
    let oid = index.write_tree()?;
    let signature = Signature::now("Zbigniew Siciarz", "zbigniew@siciarz.net")?;
    let parent_commit = find_last_commit(repo)?;
    let tree = repo.find_tree(oid)?;
    repo.commit(
        Some("HEAD"), //  point HEAD to our new commit
        &signature, // author
        &signature, // committer
        message, // commit message
        &tree, // tree
        &[&parent_commit],
    ) // parents
}

#[cfg(target_family = "unix")]
fn push(repo: &Repository, url: &str) -> Result<(), git2::Error> {
    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => repo.remote("origin", url)?,
    };
    remote.connect(Direction::Push)?;
    remote.push(&["refs/heads/master:refs/heads/master"], None)
}

#[cfg(target_family = "windows")]
fn main() {
    println!("TODO");
}

#[cfg(target_family = "unix")]
fn main() {
    println!("24 Days of Rust vol. 2 - git2");
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    println!("{} state={:?}", repo.path().display(), repo.state());
    let commit = find_last_commit(&repo).expect("Couldn't find last commit");
    display_commit(&commit);

    let relative_path = Path::new("example.txt");
    {
        let file_path = Path::new(repo_root.as_str()).join(relative_path);
        let mut file = File::create(file_path.clone()).expect("Couldn't create file");
        file.write_all(b"Hello git2").unwrap();
    }
    let commit_id = add_and_commit(&repo, relative_path, "Add example text file")
        .expect("Couldn't add file to repo");
    println!("New commit: {}", commit_id);

    let remote_url = format!(
        "file://{}",
        canonicalize("../../git_remote").unwrap().display()
    );
    println!("Pushing to: {}", remote_url);
    push(&repo, remote_url.as_str()).expect("Couldn't push to remote repo");
}
