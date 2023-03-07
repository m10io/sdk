# m10_sdk

## Generate protobuf code

Make sure `protoc_plugin` version is 20.0.0 or later.
Run the following to upgrade:
```bash
pub global activate protoc_plugin
```
Add `.pub-cache/bin` in your home dir to your `PATH`.

Ensure protobuf-compiler is installed:

```bash
# Linux
sudo apt-get update && sudo apt-get install protobuf-compiler

# Mac
brew install protobuf
```

```bash
protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/sdk/*.proto
protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/sdk/**/*.proto
protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/directory/*.proto
protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/firehose/*.proto
protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/google/*.proto
protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/google/protobuf/*.proto
```

## How to run the tests

```bash
pub run test ./test/all_test.dart
```
