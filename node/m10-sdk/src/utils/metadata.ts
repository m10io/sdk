import * as googleProtobufAny from "../protobufs/google/protobuf/any";
import * as sdkMetadata from "../protobufs/sdk/metadata";

/**
 * @param memo - this is limited to 512 KiB serialized
 */
export function convertMemoToAny(memo: sdkMetadata.Memo): googleProtobufAny.Any {
    return googleProtobufAny.Any.create({
        typeUrl: "m10.sdk.metadata.Memo",
        value: sdkMetadata.Memo.toBinary(memo),
    });
}
