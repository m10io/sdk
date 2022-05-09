///
//  Generated code. Do not modify.
//  source: sdk/metadata.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,unnecessary_const,non_constant_identifier_names,library_prefixes,unused_import,unused_shown_name,return_of_invalid_type,unnecessary_this,prefer_final_fields

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class Attachment_AttachmentType extends $pb.ProtobufEnum {
  static const Attachment_AttachmentType OBJECT = Attachment_AttachmentType._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'OBJECT');
  static const Attachment_AttachmentType IMAGE = Attachment_AttachmentType._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'IMAGE');

  static const $core.List<Attachment_AttachmentType> values = <Attachment_AttachmentType> [
    OBJECT,
    IMAGE,
  ];

  static final $core.Map<$core.int, Attachment_AttachmentType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Attachment_AttachmentType? valueOf($core.int value) => _byValue[value];

  const Attachment_AttachmentType._($core.int v, $core.String n) : super(v, n);
}

class PaymentRequest_PaymentRequestStatus extends $pb.ProtobufEnum {
  static const PaymentRequest_PaymentRequestStatus PENDING = PaymentRequest_PaymentRequestStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'PENDING');
  static const PaymentRequest_PaymentRequestStatus DECLINED = PaymentRequest_PaymentRequestStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'DECLINED');
  static const PaymentRequest_PaymentRequestStatus CANCELED = PaymentRequest_PaymentRequestStatus._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'CANCELED');
  static const PaymentRequest_PaymentRequestStatus IN_PROGRESS = PaymentRequest_PaymentRequestStatus._(3, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'IN_PROGRESS');

  static const $core.List<PaymentRequest_PaymentRequestStatus> values = <PaymentRequest_PaymentRequestStatus> [
    PENDING,
    DECLINED,
    CANCELED,
    IN_PROGRESS,
  ];

  static final $core.Map<$core.int, PaymentRequest_PaymentRequestStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static PaymentRequest_PaymentRequestStatus? valueOf($core.int value) => _byValue[value];

  const PaymentRequest_PaymentRequestStatus._($core.int v, $core.String n) : super(v, n);
}

