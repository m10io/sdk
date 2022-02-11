#! /bin/sh
pushd rust
pushd protos
ln -s ../../protobuf/ protobuf
sed -i 's/..\/..\/protobuf/.\/protobuf/g' build.rs
popd
cargo workspaces publish --from-git --allow-dirty
popd
