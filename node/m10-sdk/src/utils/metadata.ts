import { google, m10 } from "../../protobufs";


export function convertMemoToAny(memo: m10.sdk.metadata.IMemo): google.protobuf.IAny {
    return new google.protobuf.Any({
        type_url: "m10.sdk.metadata.Memo",
        value: m10.sdk.metadata.Memo.encode(memo).finish(),
    });
}
