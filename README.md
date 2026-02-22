# rwconfig

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/rwconfig)
![Crates.io Total Downloads](https://img.shields.io/crates/d/rwconfig)
![docs.rs](https://img.shields.io/docsrs/rwconfig)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/un-rust/rwconfig)
![GitHub Repo stars](https://img.shields.io/github/stars/un-rust/rwconfig)
<!-- /automdrs -->

<!-- automdrs:description -->

Read/write config files with get/set and dirty-tracking; save() writes all changes at once

<!-- /automdrs -->

**[Full documentation →](https://docs.rs/rwconfig/)**

## Quick start

<!-- automdrs:cargo-add -->

```sh
cargo add rwconfig
```

<!-- /automdrs -->

## Usage

<!-- automdrs:file src="./src/main.rs" -->
```rust
use rwconfig::hello;

fn main() {
    println!("{}", hello("un-rust"));
}
```
<!-- /automdrs -->

## License

<!-- automdrs:contributors author="UnRUST" license="Apache-2.0" -->
Published under the [Apache-2.0](./LICENSE) license.
Made by [@UnRUST](https://github.com/un-rust) 💛
<br><br>
<a href="https://github.com/un-rust/rwconfig/graphs/contributors">
<img src="https://contrib.rocks/image?repo=un-rust/rwconfig" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->