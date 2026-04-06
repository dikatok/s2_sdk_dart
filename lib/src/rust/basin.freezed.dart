// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'basin.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$RetentionPolicy {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetentionPolicy);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RetentionPolicy()';
}


}

/// @nodoc
class $RetentionPolicyCopyWith<$Res>  {
$RetentionPolicyCopyWith(RetentionPolicy _, $Res Function(RetentionPolicy) __);
}


/// Adds pattern-matching-related methods to [RetentionPolicy].
extension RetentionPolicyPatterns on RetentionPolicy {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( RetentionPolicy_Infinite value)?  infinite,TResult Function( RetentionPolicy_Age value)?  age,required TResult orElse(),}){
final _that = this;
switch (_that) {
case RetentionPolicy_Infinite() when infinite != null:
return infinite(_that);case RetentionPolicy_Age() when age != null:
return age(_that);case _:
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

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( RetentionPolicy_Infinite value)  infinite,required TResult Function( RetentionPolicy_Age value)  age,}){
final _that = this;
switch (_that) {
case RetentionPolicy_Infinite():
return infinite(_that);case RetentionPolicy_Age():
return age(_that);}
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( RetentionPolicy_Infinite value)?  infinite,TResult? Function( RetentionPolicy_Age value)?  age,}){
final _that = this;
switch (_that) {
case RetentionPolicy_Infinite() when infinite != null:
return infinite(_that);case RetentionPolicy_Age() when age != null:
return age(_that);case _:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function()?  infinite,TResult Function( BigInt field0)?  age,required TResult orElse(),}) {final _that = this;
switch (_that) {
case RetentionPolicy_Infinite() when infinite != null:
return infinite();case RetentionPolicy_Age() when age != null:
return age(_that.field0);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function()  infinite,required TResult Function( BigInt field0)  age,}) {final _that = this;
switch (_that) {
case RetentionPolicy_Infinite():
return infinite();case RetentionPolicy_Age():
return age(_that.field0);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function()?  infinite,TResult? Function( BigInt field0)?  age,}) {final _that = this;
switch (_that) {
case RetentionPolicy_Infinite() when infinite != null:
return infinite();case RetentionPolicy_Age() when age != null:
return age(_that.field0);case _:
  return null;

}
}

}

/// @nodoc


class RetentionPolicy_Infinite extends RetentionPolicy {
  const RetentionPolicy_Infinite(): super._();
  






@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetentionPolicy_Infinite);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'RetentionPolicy.infinite()';
}


}




/// @nodoc


class RetentionPolicy_Age extends RetentionPolicy {
  const RetentionPolicy_Age(this.field0): super._();
  

 final  BigInt field0;

/// Create a copy of RetentionPolicy
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$RetentionPolicy_AgeCopyWith<RetentionPolicy_Age> get copyWith => _$RetentionPolicy_AgeCopyWithImpl<RetentionPolicy_Age>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is RetentionPolicy_Age&&(identical(other.field0, field0) || other.field0 == field0));
}


@override
int get hashCode => Object.hash(runtimeType,field0);

@override
String toString() {
  return 'RetentionPolicy.age(field0: $field0)';
}


}

/// @nodoc
abstract mixin class $RetentionPolicy_AgeCopyWith<$Res> implements $RetentionPolicyCopyWith<$Res> {
  factory $RetentionPolicy_AgeCopyWith(RetentionPolicy_Age value, $Res Function(RetentionPolicy_Age) _then) = _$RetentionPolicy_AgeCopyWithImpl;
@useResult
$Res call({
 BigInt field0
});




}
/// @nodoc
class _$RetentionPolicy_AgeCopyWithImpl<$Res>
    implements $RetentionPolicy_AgeCopyWith<$Res> {
  _$RetentionPolicy_AgeCopyWithImpl(this._self, this._then);

  final RetentionPolicy_Age _self;
  final $Res Function(RetentionPolicy_Age) _then;

/// Create a copy of RetentionPolicy
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? field0 = null,}) {
  return _then(RetentionPolicy_Age(
null == field0 ? _self.field0 : field0 // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

// dart format on
