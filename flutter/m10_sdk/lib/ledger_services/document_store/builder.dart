import 'package:protobuf/protobuf.dart';
import 'package:recase/recase.dart';
import 'package:m10_sdk/src/generated/google/protobuf/field_mask.pb.dart';

class DocumentUpdate<D extends GeneratedMessage> {
  D _document;
  FieldMask _mask = FieldMask();

  DocumentUpdate(D document) : _document = document;

  FieldMask get mask => _mask;
  D get document => _document;

  @override
  noSuchMethod(Invocation invocation) {
    final fields = invocation.memberName.toString().split('"');
    var field = fields[1];
    if (invocation.isSetter) {
      field = field.substring(0, field.length - 1);
    }
    final normalizedPath = new ReCase(field).snakeCase;
    _mask.paths.add(normalizedPath);
    final tag = _document.getTagNumber(field)!;

    try {
      _document.setField(tag, invocation.positionalArguments[0]);
    } on ArgumentError catch (_) {
      _document.getField(tag).addAll(invocation.positionalArguments[0]);
    }
    return this;
  }

  DocumentUpdate<D> owner(List<int> owner) {
    final tag = _document.getTagNumber("Owner");
    if (tag == null) {
      throw ArgumentError('Document does not have tag "Owner"');
    }

    _document.setField(tag, owner);
    _mask.paths.add("owner");
    return this;
  }

  DocumentUpdate<D> updateFields(Map fieldMap) {
    fieldMap.entries.where((entry) => entry.value != null).forEach((entry) {
      final tag = _document.getTagNumber(entry.key)!;
      try {
        _document.setField(tag, entry.value);
      } catch (e) {
        _document.getField(tag).addAll(entry.value);
      }

      final normalizedPath = new ReCase(entry.key).snakeCase;
      _mask.paths.add(normalizedPath);
    });
    return this;
  }
}
