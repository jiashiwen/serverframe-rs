# Rust client bugs

* 程序后台运行后 put 命令失效

# 开启enable TTl transaction 报错

```
thread 'main' panicked at 'Committing read-only transaction should not fail: MultipleKeyErrors([KeyError(KeyError { locked: None, retryable: "", abort: "Error(Other(\"[src/storage/mod.rs:1072]: can't sched txn cmd(prewrite) with TTL enabled\"))", conflict: None, already_exist: None, deadlock: None, commit_ts_expired: None, txn_not_found: None, commit_ts_too_large: None, assertion_failed: None })])', examples/tikv_txn.rs:34:14
```