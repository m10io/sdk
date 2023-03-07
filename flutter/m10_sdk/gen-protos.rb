#!/usr/bin/env ruby

def generate_protos()
  # Generate M10 Dart SDK Protobuffer Bindings, Make sure `protoc_plugin` version is 20.0.0
  # NOTE: this could be made into a Dart/Flutter build step instead, removing the need for this
  puts 'Generating Protcol Buffer bindings for ./shared/flutter/m10_sdk..'
  `export PATH="$HOME/.pub-cache/bin":"$PATH" && \
        flutter pub global activate protoc_plugin 20.0.0 && \
        protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/sdk/*.proto
        protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/sdk/**/*.proto
        protoc --dart_out=grpc:lib/src/generated -I=../../protobuf/ ../../protobuf/directory/api.proto`
end

if __FILE__ == $0
  generate_protos()
end
