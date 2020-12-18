// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This file is generated by rust-protobuf 2.18.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `op_store.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_0;

#[derive(PartialEq,Clone,Default)]
pub struct View {
    // message fields
    pub head_ids: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub checkout: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a View {
    fn default() -> &'a View {
        <View as ::protobuf::Message>::default_instance()
    }
}

impl View {
    pub fn new() -> View {
        ::std::default::Default::default()
    }

    // repeated bytes head_ids = 1;


    pub fn get_head_ids(&self) -> &[::std::vec::Vec<u8>] {
        &self.head_ids
    }
    pub fn clear_head_ids(&mut self) {
        self.head_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_head_ids(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.head_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_head_ids(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.head_ids
    }

    // Take field
    pub fn take_head_ids(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.head_ids, ::protobuf::RepeatedField::new())
    }

    // bytes checkout = 2;


    pub fn get_checkout(&self) -> &[u8] {
        &self.checkout
    }
    pub fn clear_checkout(&mut self) {
        self.checkout.clear();
    }

    // Param is passed by value, moved
    pub fn set_checkout(&mut self, v: ::std::vec::Vec<u8>) {
        self.checkout = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checkout(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.checkout
    }

    // Take field
    pub fn take_checkout(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.checkout, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for View {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.head_ids)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.checkout)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.head_ids {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if !self.checkout.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.checkout);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.head_ids {
            os.write_bytes(1, &v)?;
        };
        if !self.checkout.is_empty() {
            os.write_bytes(2, &self.checkout)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> View {
        View::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "head_ids",
                |m: &View| { &m.head_ids },
                |m: &mut View| { &mut m.head_ids },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "checkout",
                |m: &View| { &m.checkout },
                |m: &mut View| { &mut m.checkout },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<View>(
                "View",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static View {
        static instance: ::protobuf::rt::LazyV2<View> = ::protobuf::rt::LazyV2::INIT;
        instance.get(View::new)
    }
}

impl ::protobuf::Clear for View {
    fn clear(&mut self) {
        self.head_ids.clear();
        self.checkout.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for View {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for View {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Operation {
    // message fields
    pub view_id: ::std::vec::Vec<u8>,
    pub parents: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    pub metadata: ::protobuf::SingularPtrField<OperationMetadata>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Operation {
    fn default() -> &'a Operation {
        <Operation as ::protobuf::Message>::default_instance()
    }
}

impl Operation {
    pub fn new() -> Operation {
        ::std::default::Default::default()
    }

    // bytes view_id = 1;


    pub fn get_view_id(&self) -> &[u8] {
        &self.view_id
    }
    pub fn clear_view_id(&mut self) {
        self.view_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_view_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.view_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_view_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.view_id
    }

    // Take field
    pub fn take_view_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.view_id, ::std::vec::Vec::new())
    }

    // repeated bytes parents = 2;


    pub fn get_parents(&self) -> &[::std::vec::Vec<u8>] {
        &self.parents
    }
    pub fn clear_parents(&mut self) {
        self.parents.clear();
    }

    // Param is passed by value, moved
    pub fn set_parents(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.parents = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parents(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.parents
    }

    // Take field
    pub fn take_parents(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.parents, ::protobuf::RepeatedField::new())
    }

    // .OperationMetadata metadata = 3;


    pub fn get_metadata(&self) -> &OperationMetadata {
        self.metadata.as_ref().unwrap_or_else(|| <OperationMetadata as ::protobuf::Message>::default_instance())
    }
    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: OperationMetadata) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut OperationMetadata {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> OperationMetadata {
        self.metadata.take().unwrap_or_else(|| OperationMetadata::new())
    }
}

impl ::protobuf::Message for Operation {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.view_id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.parents)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.view_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.view_id);
        }
        for value in &self.parents {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.view_id.is_empty() {
            os.write_bytes(1, &self.view_id)?;
        }
        for v in &self.parents {
            os.write_bytes(2, &v)?;
        };
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Operation {
        Operation::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "view_id",
                |m: &Operation| { &m.view_id },
                |m: &mut Operation| { &mut m.view_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "parents",
                |m: &Operation| { &m.parents },
                |m: &mut Operation| { &mut m.parents },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OperationMetadata>>(
                "metadata",
                |m: &Operation| { &m.metadata },
                |m: &mut Operation| { &mut m.metadata },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Operation>(
                "Operation",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Operation {
        static instance: ::protobuf::rt::LazyV2<Operation> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Operation::new)
    }
}

impl ::protobuf::Clear for Operation {
    fn clear(&mut self) {
        self.view_id.clear();
        self.parents.clear();
        self.metadata.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Operation {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Timestamp {
    // message fields
    pub millis_since_epoch: u64,
    pub tz_offset: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Timestamp {
    fn default() -> &'a Timestamp {
        <Timestamp as ::protobuf::Message>::default_instance()
    }
}

impl Timestamp {
    pub fn new() -> Timestamp {
        ::std::default::Default::default()
    }

    // uint64 millis_since_epoch = 1;


    pub fn get_millis_since_epoch(&self) -> u64 {
        self.millis_since_epoch
    }
    pub fn clear_millis_since_epoch(&mut self) {
        self.millis_since_epoch = 0;
    }

    // Param is passed by value, moved
    pub fn set_millis_since_epoch(&mut self, v: u64) {
        self.millis_since_epoch = v;
    }

    // int32 tz_offset = 2;


    pub fn get_tz_offset(&self) -> i32 {
        self.tz_offset
    }
    pub fn clear_tz_offset(&mut self) {
        self.tz_offset = 0;
    }

    // Param is passed by value, moved
    pub fn set_tz_offset(&mut self, v: i32) {
        self.tz_offset = v;
    }
}

impl ::protobuf::Message for Timestamp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.millis_since_epoch = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.tz_offset = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.millis_since_epoch != 0 {
            my_size += ::protobuf::rt::value_size(1, self.millis_since_epoch, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.tz_offset != 0 {
            my_size += ::protobuf::rt::value_size(2, self.tz_offset, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.millis_since_epoch != 0 {
            os.write_uint64(1, self.millis_since_epoch)?;
        }
        if self.tz_offset != 0 {
            os.write_int32(2, self.tz_offset)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Timestamp {
        Timestamp::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "millis_since_epoch",
                |m: &Timestamp| { &m.millis_since_epoch },
                |m: &mut Timestamp| { &mut m.millis_since_epoch },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "tz_offset",
                |m: &Timestamp| { &m.tz_offset },
                |m: &mut Timestamp| { &mut m.tz_offset },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Timestamp>(
                "Timestamp",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Timestamp {
        static instance: ::protobuf::rt::LazyV2<Timestamp> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Timestamp::new)
    }
}

impl ::protobuf::Clear for Timestamp {
    fn clear(&mut self) {
        self.millis_since_epoch = 0;
        self.tz_offset = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Timestamp {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OperationMetadata {
    // message fields
    pub start_time: ::protobuf::SingularPtrField<Timestamp>,
    pub end_time: ::protobuf::SingularPtrField<Timestamp>,
    pub description: ::std::string::String,
    pub hostname: ::std::string::String,
    pub username: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a OperationMetadata {
    fn default() -> &'a OperationMetadata {
        <OperationMetadata as ::protobuf::Message>::default_instance()
    }
}

impl OperationMetadata {
    pub fn new() -> OperationMetadata {
        ::std::default::Default::default()
    }

    // .Timestamp start_time = 1;


    pub fn get_start_time(&self) -> &Timestamp {
        self.start_time.as_ref().unwrap_or_else(|| <Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_start_time(&mut self) {
        self.start_time.clear();
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: Timestamp) {
        self.start_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_time(&mut self) -> &mut Timestamp {
        if self.start_time.is_none() {
            self.start_time.set_default();
        }
        self.start_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_time(&mut self) -> Timestamp {
        self.start_time.take().unwrap_or_else(|| Timestamp::new())
    }

    // .Timestamp end_time = 2;


    pub fn get_end_time(&self) -> &Timestamp {
        self.end_time.as_ref().unwrap_or_else(|| <Timestamp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_end_time(&mut self) {
        self.end_time.clear();
    }

    pub fn has_end_time(&self) -> bool {
        self.end_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_time(&mut self, v: Timestamp) {
        self.end_time = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_time(&mut self) -> &mut Timestamp {
        if self.end_time.is_none() {
            self.end_time.set_default();
        }
        self.end_time.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_time(&mut self) -> Timestamp {
        self.end_time.take().unwrap_or_else(|| Timestamp::new())
    }

    // string description = 3;


    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        &mut self.description
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.description, ::std::string::String::new())
    }

    // string hostname = 4;


    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }
    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        &mut self.hostname
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hostname, ::std::string::String::new())
    }

    // string username = 5;


    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        &mut self.username
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.username, ::std::string::String::new())
    }
}

impl ::protobuf::Message for OperationMetadata {
    fn is_initialized(&self) -> bool {
        for v in &self.start_time {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.end_time {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start_time)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end_time)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.description)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hostname)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.username)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.start_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.end_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.description);
        }
        if !self.hostname.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.hostname);
        }
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.username);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.start_time.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.end_time.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.description.is_empty() {
            os.write_string(3, &self.description)?;
        }
        if !self.hostname.is_empty() {
            os.write_string(4, &self.hostname)?;
        }
        if !self.username.is_empty() {
            os.write_string(5, &self.username)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> OperationMetadata {
        OperationMetadata::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Timestamp>>(
                "start_time",
                |m: &OperationMetadata| { &m.start_time },
                |m: &mut OperationMetadata| { &mut m.start_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Timestamp>>(
                "end_time",
                |m: &OperationMetadata| { &m.end_time },
                |m: &mut OperationMetadata| { &mut m.end_time },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "description",
                |m: &OperationMetadata| { &m.description },
                |m: &mut OperationMetadata| { &mut m.description },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "hostname",
                |m: &OperationMetadata| { &m.hostname },
                |m: &mut OperationMetadata| { &mut m.hostname },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "username",
                |m: &OperationMetadata| { &m.username },
                |m: &mut OperationMetadata| { &mut m.username },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<OperationMetadata>(
                "OperationMetadata",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static OperationMetadata {
        static instance: ::protobuf::rt::LazyV2<OperationMetadata> = ::protobuf::rt::LazyV2::INIT;
        instance.get(OperationMetadata::new)
    }
}

impl ::protobuf::Clear for OperationMetadata {
    fn clear(&mut self) {
        self.start_time.clear();
        self.end_time.clear();
        self.description.clear();
        self.hostname.clear();
        self.username.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OperationMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OperationMetadata {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eop_store.proto\"C\n\x04View\x12\x1b\n\x08head_ids\x18\x01\x20\x03(\
    \x0cR\x07headIdsB\0\x12\x1c\n\x08checkout\x18\x02\x20\x01(\x0cR\x08check\
    outB\0:\0\"v\n\tOperation\x12\x19\n\x07view_id\x18\x01\x20\x01(\x0cR\x06\
    viewIdB\0\x12\x1a\n\x07parents\x18\x02\x20\x03(\x0cR\x07parentsB\0\x120\
    \n\x08metadata\x18\x03\x20\x01(\x0b2\x12.OperationMetadataR\x08metadataB\
    \0:\0\"\\\n\tTimestamp\x12.\n\x12millis_since_epoch\x18\x01\x20\x01(\x04\
    R\x10millisSinceEpochB\0\x12\x1d\n\ttz_offset\x18\x02\x20\x01(\x05R\x08t\
    zOffsetB\0:\0\"\xcb\x01\n\x11OperationMetadata\x12+\n\nstart_time\x18\
    \x01\x20\x01(\x0b2\n.TimestampR\tstartTimeB\0\x12'\n\x08end_time\x18\x02\
    \x20\x01(\x0b2\n.TimestampR\x07endTimeB\0\x12\"\n\x0bdescription\x18\x03\
    \x20\x01(\tR\x0bdescriptionB\0\x12\x1c\n\x08hostname\x18\x04\x20\x01(\tR\
    \x08hostnameB\0\x12\x1c\n\x08username\x18\x05\x20\x01(\tR\x08usernameB\0\
    :\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}