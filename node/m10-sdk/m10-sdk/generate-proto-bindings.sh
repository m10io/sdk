#!/bin/sh -e

rm -rf lib/generated
mkdir lib/generated

# Arcadius

# Directory
directory_proto='../../protobuf/directory/api.proto'

# SDK
sdk_protos="../../protobuf/sdk/*.proto"
for sdk_proto in ../../protobuf/sdk/*; do
    if [ -d "$sdk_proto" ]; then
    sdk_protos="$sdk_protos ${sdk_proto}/*.proto"
    fi
done

# Generate Protobufs
grpc_tools_node_protoc -I ../../protobuf --grpc_out=grpc_js:lib/generated --js_out=import_style=commonjs,binary:lib/generated $sdk_protos $directory_proto
