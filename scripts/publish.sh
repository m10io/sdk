#! /bin/sh
pushd rust
pushd sdk-protos
ln -s ../../protobuf/ protobuf
sed -i '.bak' 's/..\/..\/protobuf/.\/protobuf/g' build.rs
popd
cargo workspaces publish --from-git --allow-dirty --dry-run
popd
