# rim

An exercise in rust and an attempt to fix an issue I have with `rm`

## Rationale

I'm working through the [rustbook](https://doc.rust-lang.org/book/) and I've got to the chapter on "Building a command line tool" so I'm having a go with this.

I use `pushd` and `popd` and find this problem that when I remove a directory on the directory stack and then try to `popd` back up the stack it gets stuck...

So `rim` (Remove-IMproved) will, when deleting a directory, see if it is on the directory stack and remove it if it is.

