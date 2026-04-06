// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'types.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$ReadFrom {

 BigInt get field0;
/// Create a copy of ReadFrom
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ReadFromCopyWith<ReadFrom> get copyWith => _$ReadFromCopyWithImpl<ReadFrom>(this as ReadFrom, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ReadFrom&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ReadFrom(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ReadFromCopyWith<$Res>  {
  factory $ReadFromCopyWith(ReadFrom value, $Res Function(ReadFrom) _then) = _$ReadFromCopyWithImpl;
@useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$ReadFromCopyWithImpl<$Res>
    implements $ReadFromCopyWith<$Res> {
  _$ReadFromCopyWithImpl(this._self, this._then);

  final ReadFrom _self;
  final $Res Function(ReadFrom) _then;

/// Create a copy of ReadFrom
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? field0 = null,}) {
  return _then(_self.copyWith(
field0: null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}

}


/// Adds pattern-matching-related methods to [ReadFrom].
extension ReadFromPatterns on ReadFrom {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ReadFrom_SeqNum value)?  seqNum,TResult Function( ReadFrom_Timestamp value)?  timestamp,TResult Function( ReadFrom_TailOffset value)?  tailOffset,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ReadFrom_SeqNum() when seqNum != null:
return seqNum(_that);case ReadFrom_Timestamp() when timestamp != null:
return timestamp(_that);case ReadFrom_TailOffset() when tailOffset != null:
return tailOffset(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ReadFrom_SeqNum value)  seqNum,required TResult Function( ReadFrom_Timestamp value)  timestamp,required TResult Function( ReadFrom_TailOffset value)  tailOffset,}){
final _that = this;
switch (_that) {
case ReadFrom_SeqNum():
return seqNum(_that);case ReadFrom_Timestamp():
return timestamp(_that);case ReadFrom_TailOffset():
return tailOffset(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ReadFrom_SeqNum value)?  seqNum,TResult? Function( ReadFrom_Timestamp value)?  timestamp,TResult? Function( ReadFrom_TailOffset value)?  tailOffset,}){
final _that = this;
switch (_that) {
case ReadFrom_SeqNum() when seqNum != null:
return seqNum(_that);case ReadFrom_Timestamp() when timestamp != null:
return timestamp(_that);case ReadFrom_TailOffset() when tailOffset != null:
return tailOffset(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt field0)?  seqNum,TResult Function( BigInt field0)?  timestamp,TResult Function( BigInt field0)?  tailOffset,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ReadFrom_SeqNum() when seqNum != null:
return seqNum(_that.field0);case ReadFrom_Timestamp() when timestamp != null:
return timestamp(_that.field0);case ReadFrom_TailOffset() when tailOffset != null:
return tailOffset(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt field0)  seqNum,required TResult Function( BigInt field0)  timestamp,required TResult Function( BigInt field0)  tailOffset,}) {final _that = this;
switch (_that) {
case ReadFrom_SeqNum():
return seqNum(_that.field0);case ReadFrom_Timestamp():
return timestamp(_that.field0);case ReadFrom_TailOffset():
return tailOffset(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt field0)?  seqNum,TResult? Function( BigInt field0)?  timestamp,TResult? Function( BigInt field0)?  tailOffset,}) {final _that = this;
switch (_that) {
case ReadFrom_SeqNum() when seqNum != null:
return seqNum(_that.field0);case ReadFrom_Timestamp() when timestamp != null:
return timestamp(_that.field0);case ReadFrom_TailOffset() when tailOffset != null:
return tailOffset(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class ReadFrom_SeqNum extends ReadFrom {
  const ReadFrom_SeqNum(this.field0): super._();
  

@override final  BigInt field0;

/// Create a copy of ReadFrom
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ReadFrom_SeqNumCopyWith<ReadFrom_SeqNum> get copyWith => _$ReadFrom_SeqNumCopyWithImpl<ReadFrom_SeqNum>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ReadFrom_SeqNum&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ReadFrom.seqNum(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ReadFrom_SeqNumCopyWith<$Res> implements $ReadFromCopyWith<$Res> {
  factory $ReadFrom_SeqNumCopyWith(ReadFrom_SeqNum value, $Res Function(ReadFrom_SeqNum) _then) = _$ReadFrom_SeqNumCopyWithImpl;
@override @useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$ReadFrom_SeqNumCopyWithImpl<$Res>
    implements $ReadFrom_SeqNumCopyWith<$Res> {
  _$ReadFrom_SeqNumCopyWithImpl(this._self, this._then);

  final ReadFrom_SeqNum _self;
  final $Res Function(ReadFrom_SeqNum) _then;

/// Create a copy of ReadFrom
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ReadFrom_SeqNum(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ReadFrom_Timestamp extends ReadFrom {
  const ReadFrom_Timestamp(this.field0): super._();
  

@override final  BigInt field0;

/// Create a copy of ReadFrom
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ReadFrom_TimestampCopyWith<ReadFrom_Timestamp> get copyWith => _$ReadFrom_TimestampCopyWithImpl<ReadFrom_Timestamp>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ReadFrom_Timestamp&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ReadFrom.timestamp(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ReadFrom_TimestampCopyWith<$Res> implements $ReadFromCopyWith<$Res> {
  factory $ReadFrom_TimestampCopyWith(ReadFrom_Timestamp value, $Res Function(ReadFrom_Timestamp) _then) = _$ReadFrom_TimestampCopyWithImpl;
@override @useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$ReadFrom_TimestampCopyWithImpl<$Res>
    implements $ReadFrom_TimestampCopyWith<$Res> {
  _$ReadFrom_TimestampCopyWithImpl(this._self, this._then);

  final ReadFrom_Timestamp _self;
  final $Res Function(ReadFrom_Timestamp) _then;

/// Create a copy of ReadFrom
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ReadFrom_Timestamp(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ReadFrom_TailOffset extends ReadFrom {
  const ReadFrom_TailOffset(this.field0): super._();
  

@override final  BigInt field0;

/// Create a copy of ReadFrom
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ReadFrom_TailOffsetCopyWith<ReadFrom_TailOffset> get copyWith => _$ReadFrom_TailOffsetCopyWithImpl<ReadFrom_TailOffset>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ReadFrom_TailOffset&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ReadFrom.tailOffset(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ReadFrom_TailOffsetCopyWith<$Res> implements $ReadFromCopyWith<$Res> {
  factory $ReadFrom_TailOffsetCopyWith(ReadFrom_TailOffset value, $Res Function(ReadFrom_TailOffset) _then) = _$ReadFrom_TailOffsetCopyWithImpl;
@override @useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$ReadFrom_TailOffsetCopyWithImpl<$Res>
    implements $ReadFrom_TailOffsetCopyWith<$Res> {
  _$ReadFrom_TailOffsetCopyWithImpl(this._self, this._then);

  final ReadFrom_TailOffset _self;
  final $Res Function(ReadFrom_TailOffset) _then;

/// Create a copy of ReadFrom
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ReadFrom_TailOffset(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc
mixin _$ResourceSet {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ResourceSet);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ResourceSet()';
}


}

/// @nodoc
class $ResourceSetCopyWith<$Res>  {
$ResourceSetCopyWith(ResourceSet _, $Res Function(ResourceSet) __);
}


/// Adds pattern-matching-related methods to [ResourceSet].
extension ResourceSetPatterns on ResourceSet {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ResourceSet_None value)?  none,TResult Function( ResourceSet_Exact value)?  exact,TResult Function( ResourceSet_Prefix value)?  prefix,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ResourceSet_None() when none != null:
return none(_that);case ResourceSet_Exact() when exact != null:
return exact(_that);case ResourceSet_Prefix() when prefix != null:
return prefix(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ResourceSet_None value)  none,required TResult Function( ResourceSet_Exact value)  exact,required TResult Function( ResourceSet_Prefix value)  prefix,}){
final _that = this;
switch (_that) {
case ResourceSet_None():
return none(_that);case ResourceSet_Exact():
return exact(_that);case ResourceSet_Prefix():
return prefix(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ResourceSet_None value)?  none,TResult? Function( ResourceSet_Exact value)?  exact,TResult? Function( ResourceSet_Prefix value)?  prefix,}){
final _that = this;
switch (_that) {
case ResourceSet_None() when none != null:
return none(_that);case ResourceSet_Exact() when exact != null:
return exact(_that);case ResourceSet_Prefix() when prefix != null:
return prefix(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  none,TResult Function( String field0)?  exact,TResult Function( String field0)?  prefix,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ResourceSet_None() when none != null:
return none();case ResourceSet_Exact() when exact != null:
return exact(_that.field0);case ResourceSet_Prefix() when prefix != null:
return prefix(_that.field0);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  none,required TResult Function( String field0)  exact,required TResult Function( String field0)  prefix,}) {final _that = this;
switch (_that) {
case ResourceSet_None():
return none();case ResourceSet_Exact():
return exact(_that.field0);case ResourceSet_Prefix():
return prefix(_that.field0);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  none,TResult? Function( String field0)?  exact,TResult? Function( String field0)?  prefix,}) {final _that = this;
switch (_that) {
case ResourceSet_None() when none != null:
return none();case ResourceSet_Exact() when exact != null:
return exact(_that.field0);case ResourceSet_Prefix() when prefix != null:
return prefix(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class ResourceSet_None extends ResourceSet {
  const ResourceSet_None(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ResourceSet_None);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ResourceSet.none()';
}


}




/// @nodoc


class ResourceSet_Exact extends ResourceSet {
  const ResourceSet_Exact(this.field0): super._();
  

 final  String field0;

/// Create a copy of ResourceSet
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ResourceSet_ExactCopyWith<ResourceSet_Exact> get copyWith => _$ResourceSet_ExactCopyWithImpl<ResourceSet_Exact>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ResourceSet_Exact&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ResourceSet.exact(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ResourceSet_ExactCopyWith<$Res> implements $ResourceSetCopyWith<$Res> {
  factory $ResourceSet_ExactCopyWith(ResourceSet_Exact value, $Res Function(ResourceSet_Exact) _then) = _$ResourceSet_ExactCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ResourceSet_ExactCopyWithImpl<$Res>
    implements $ResourceSet_ExactCopyWith<$Res> {
  _$ResourceSet_ExactCopyWithImpl(this._self, this._then);

  final ResourceSet_Exact _self;
  final $Res Function(ResourceSet_Exact) _then;

/// Create a copy of ResourceSet
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ResourceSet_Exact(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc


class ResourceSet_Prefix extends ResourceSet {
  const ResourceSet_Prefix(this.field0): super._();
  

 final  String field0;

/// Create a copy of ResourceSet
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ResourceSet_PrefixCopyWith<ResourceSet_Prefix> get copyWith => _$ResourceSet_PrefixCopyWithImpl<ResourceSet_Prefix>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ResourceSet_Prefix&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'ResourceSet.prefix(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $ResourceSet_PrefixCopyWith<$Res> implements $ResourceSetCopyWith<$Res> {
  factory $ResourceSet_PrefixCopyWith(ResourceSet_Prefix value, $Res Function(ResourceSet_Prefix) _then) = _$ResourceSet_PrefixCopyWithImpl;
@useResult
$Res call({
 String field0
});




}
/// @nodoc
class _$ResourceSet_PrefixCopyWithImpl<$Res>
    implements $ResourceSet_PrefixCopyWith<$Res> {
  _$ResourceSet_PrefixCopyWithImpl(this._self, this._then);

  final ResourceSet_Prefix _self;
  final $Res Function(ResourceSet_Prefix) _then;

/// Create a copy of ResourceSet
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(ResourceSet_Prefix(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

// dart format on
