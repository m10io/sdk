//
//  Generated code. Do not modify.
//  source: sdk/metadata.proto
//
// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Enumerates the possible types of attachments.
class Attachment_AttachmentType extends $pb.ProtobufEnum {
  static const Attachment_AttachmentType OBJECT = Attachment_AttachmentType._(0, _omitEnumNames ? '' : 'OBJECT');
  static const Attachment_AttachmentType IMAGE = Attachment_AttachmentType._(1, _omitEnumNames ? '' : 'IMAGE');

  static const $core.List<Attachment_AttachmentType> values = <Attachment_AttachmentType> [
    OBJECT,
    IMAGE,
  ];

  static final $core.Map<$core.int, Attachment_AttachmentType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static Attachment_AttachmentType? valueOf($core.int value) => _byValue[value];

  const Attachment_AttachmentType._(super.v, super.n);
}

/// Enumerates the possible statuses of a payment request.
class PaymentRequest_PaymentRequestStatus extends $pb.ProtobufEnum {
  static const PaymentRequest_PaymentRequestStatus PENDING = PaymentRequest_PaymentRequestStatus._(0, _omitEnumNames ? '' : 'PENDING');
  static const PaymentRequest_PaymentRequestStatus DECLINED = PaymentRequest_PaymentRequestStatus._(1, _omitEnumNames ? '' : 'DECLINED');
  static const PaymentRequest_PaymentRequestStatus CANCELED = PaymentRequest_PaymentRequestStatus._(2, _omitEnumNames ? '' : 'CANCELED');
  static const PaymentRequest_PaymentRequestStatus IN_PROGRESS = PaymentRequest_PaymentRequestStatus._(3, _omitEnumNames ? '' : 'IN_PROGRESS');

  static const $core.List<PaymentRequest_PaymentRequestStatus> values = <PaymentRequest_PaymentRequestStatus> [
    PENDING,
    DECLINED,
    CANCELED,
    IN_PROGRESS,
  ];

  static final $core.Map<$core.int, PaymentRequest_PaymentRequestStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static PaymentRequest_PaymentRequestStatus? valueOf($core.int value) => _byValue[value];

  const PaymentRequest_PaymentRequestStatus._(super.v, super.n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
