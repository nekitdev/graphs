#[inline(never)]
#[cold]
#[track_caller]
pub const fn prune_recursive() -> ! {
    panic!("`dfs_recursive` pruned unexpectedly");
}

#[inline(never)]
#[cold]
#[track_caller]
pub const fn finish_recursive() -> ! {
    panic!("`dfs_recursive` did not finish");
}

#[inline(never)]
#[cold]
#[track_caller]
pub const fn prune_on_finish() -> ! {
    panic!("pruning on `finish` is not supported");
}
