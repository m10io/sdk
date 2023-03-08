import { google, m10 } from "../../protobufs";


/**
 * @param memo - this is limited to 512 KiB serialized
 */
export function convertMemoToAny(memo: m10.sdk.metadata.IMemo): google.protobuf.Any {
    return new google.protobuf.Any({
        type_url: "m10.sdk.metadata.Memo",
        value: m10.sdk.metadata.Memo.encode(memo).finish(),
    });
}
