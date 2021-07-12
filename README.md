```shell
sudo apt install -y lld
rustup toolchain install nightly
rustup override set nightly
mkdir -p .cargo/
curl https://raw.githubusercontent.com/bevyengine/bevy/main/.cargo/config_fast_builds > .cargo/config
```
