# Day 16 - git2

Most of software developers are to some extent familar with
[Git](https://git-scm.com/). This version control system won our hearts
even though it is not perfect. *Practicality beats purity*, as quoted
from [The Zen of Python](https://www.python.org/dev/peps/pep-0020/).

But Git is not only the `git` executable. It's the entire ecosystem around
Git that made it so successful - IDE integrations, hosting services and
third-party tools and libraries. One of these is a very popular C library called
[`libgit2`](https://libgit2.github.com/) which implements Git APIs (although
`git` itself doesn't use `libgit2` and
[probably never will](http://stackoverflow.com/questions/17151597/which-code-is-shared-between-the-original-git-and-libgit2/29473588#29473588)).
There are bindings to `libgit2` in many languages, but we're interested
primarily in Rust, right? The [`git2`](https://crates.io/crates/git2) crate
provides safe Rust bindings.

Open a repository
-----------------

Just like most of our work with Git happens in a repository, our interaction
with `git2` starts with a `Repository` struct. This type has a few associated
functions (aka static methods) to create a value of the type. Among those
are `clone()` or `init()` which correspond to `git` subcommands,
but we're going to use `open()` in the example.

```rust
extern crate git2;

use git2::Repository;

fn main() {
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    println!("{} state={:?}", repo.path().display(), repo.state());
}
```

`open()` assumes an already existing repository. Once we have a `Repository`
object, we can inspect its location and state using `path()` and `state()`
methods.

```text
$ cargo run -- ../repo
/home/zbyszek/Development/Rust/repo/.git/ state=Clean
```

Display latest commit
---------------------

After we have opened a repository, we can explore its history. We'll start
with writing two helper functions. One will fetch the latest commit (usually
`HEAD`) and the other will display any commit in a format known from `git log`.

```rust
use git2::{Commit, ObjectType, Repository};

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn display_commit(commit: &Commit) {
    let timestamp = commit.time().seconds();
    let tm = time::at(time::Timespec::new(timestamp, 0));
    println!("commit {}\nAuthor: {}\nDate:   {}\n\n    {}",
             commit.id(),
             commit.author(),
             tm.rfc822(),
             commit.message().unwrap_or("no commit message"));
}

let commit = find_last_commit(&repo).expect("Couldn't find last commit");
display_commit(&commit);
```

Putting these two functions together should print out some information about
the most recent commit. Is that so?

```text
$ cargo run -- ../repo
commit 8ad23c6f8888a0f90711cdf2d0da91b64c2a9333
Author: Zbigniew Siciarz <zbigniew@siciarz.net>
Date:   Fri Dec 16 18:44:16 2016 +0100

    Initial commit

$ cd ../repo && git log
commit 8ad23c6f8888a0f90711cdf2d0da91b64c2a9333
Author: Zbigniew Siciarz <zbigniew@siciarz.net>
Date:   Fri Dec 16 18:44:16 2016 +0100

    Initial commit
```

Hey, it works!

Add and commit a file
---------------------

But we're not confined to a read-only view of the repository. We can use
`git2` APIs to add or remove files, stage changes and commit them as well.
Here's a helper function that adds and commits a single file (signing off
the commit as yours truly).

```rust
use git2::{Oid, Signature};
use std::path::Path;

fn add_and_commit(repo: &Repository, path: &Path, message: &str) -> Result<Oid, git2::Error> {
    let mut index = repo.index()?;
    index.add_path(path)?;
    let oid = index.write_tree()?;
    let signature = Signature::now("Zbigniew Siciarz", "zbigniew@siciarz.net")?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;
    repo.commit(Some("HEAD"), //  point HEAD to our new commit
                &signature, // author
                &signature, // committer
                message, // commit message
                &tree, // tree
                &[&parent_commit]) // parents
}
```

The standard practice in Git is to stage changes before committing.
The area containing staged changes is known in the Git lingo as the *index*.

> Feel free to consult the [Git glossary](https://git-scm.com/docs/gitglossary)
> for explanation of a few other terms that appear here.

We can access the index from a `Repository` by calling the `index()` method.
Next, we add our file (this must be a path relative to repo root) and
write staged changes to a *tree object*. This object has its own ID that's
used later to find the actual tree. Before really committing, we need to have
a few pieces of information at hand:

 - should we update a ref (such as `HEAD`) to point to the new commit
 - author's and committer's signature (it may be the same person)
 - commit message ([look here for inspiration](http://whatthecommit.com/))
 - the tree object that will be committed
 - and a reference to a parent commit (or more parents if this is a merge)

Whew! But thankfully that is enough. The `commit()` method returns an
object ID (if successful), which in this context is the hash of our new
commit.

```rust
use std::fs::File;
use std::io::Write;

let relative_path = Path::new("example.txt");
{
    let file_path = Path::new(repo_root.as_str()).join(relative_path);
    let mut file = File::create(file_path.clone()).expect("Couldn't create file");
    file.write_all(b"Hello git2").unwrap();
}
let commit_id = add_and_commit(&repo, &relative_path, "Add example text file")
    .expect("Couldn't add file to repo");
println!("New commit: {}", commit_id);

```

We're creating a text file on the fly, making sure it's properly closed
after writing (hence the nested scope). So what happens if we try to commit
this new file?

```text
$ cargo run -- ../repo
new commit: fae1be0d7582c5859820f16980b484e1a138728f

$ cd ../repo && git log
commit fae1be0d7582c5859820f16980b484e1a138728f
Author: Zbigniew Siciarz <zbigniew@siciarz.net>
Date:   Fri Dec 16 19:06:36 2016 +0100

    Add example text file
```

Fantastic! A new commit appeared in the repository.

Push to remote repository
-------------------------

There's one more thing left to do. What good is a commit if we cannot share
it with the Open Source world? Let's push our changes to a remote repository!

```rust
use git2::Direction;

fn push(repo: &Repository, url: &str) -> Result<(), git2::Error> {
    let mut remote = match repo.find_remote("origin") {
        Ok(r) => r,
        Err(_) => repo.remote("origin", url)?,
    };
    remote.connect(Direction::Push)?;
    remote.push(&["refs/heads/master:refs/heads/master"], None)
}
```

This is actually the simplest helper function of all we wrote today. We check
if a remote named `origin` is already configured in the repository and
add it with `remote()` if it isn't. Next we establish connection to the
remote and finally push our local master branch to a remote ref.

```rust
use std::fs::canonicalize;

let remote_url = format!("file://{}",
                         canonicalize("../git_remote").unwrap().display());
println!("Pushing to: {}", remote_url);
let _ = push(&repo, remote_url.as_str()).expect("Couldn't push to remote repo");
```

If we run this and check `git log` in the `git_remote` directory, we'll find
the freshly created commit.

**Note**: I decided to use a local directory for the remote to avoid dealing with
authentication, such as setting up an SSH connection. This is of course possible
in `git2` using the
[`remote_callbacks()`](http://alexcrichton.com/git2-rs/git2/struct.PushOptions.html#method.remote_callbacks)
method. But then the `push()` function would get too complex and I wanted
something short and sweet for the grande finale.

To sum it all up: we started with opening an existing repository and checking
its state. Later on we learned how to read and display the latest commit. That
was important when we actually committed a file, because the new commit needed
a reference to its parent. Finally we configured a remote repository and
pushed all commits there. Wasn't that hard, was it? :-)

For comparison, these are the equivalent `git` commands:

```text
$ git status
$ git show HEAD
$ echo "Hello git2" > example.txt
$ git add example.txt
$ git commit -m "Add example text file"
$ git remote add origin ../git_remote
$ git push origin master
```

Further reading
---------------

 - [libgit2](https://libgit2.github.com/)
 - [libgit2 "log" example](https://github.com/libgit2/libgit2/blob/799e22ea0c3f20f1900011573a10053dc3ea9138/examples/log.c) - in C
 - [git2 "log" example](https://github.com/alexcrichton/git2-rs/blob/master/examples/log.rs) - in Rust
 - [Git from the Bottom Up](https://jwiegley.github.io/git-from-the-bottom-up/) - a deep dive into the internals of Git
 - [Move Fast and Fix Things](http://githubengineering.com/move-fast/)
