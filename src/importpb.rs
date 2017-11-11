// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Mutation {
    // message fields
    pub op: Mutation_OP,
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mutation {}

impl Mutation {
    pub fn new() -> Mutation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mutation {
        static mut instance: ::protobuf::lazy::Lazy<Mutation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mutation,
        };
        unsafe {
            instance.get(Mutation::new)
        }
    }

    // .importpb.Mutation.OP op = 1;

    pub fn clear_op(&mut self) {
        self.op = Mutation_OP::Put;
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: Mutation_OP) {
        self.op = v;
    }

    pub fn get_op(&self) -> Mutation_OP {
        self.op
    }

    fn get_op_for_reflect(&self) -> &Mutation_OP {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut Mutation_OP {
        &mut self.op
    }

    // bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for Mutation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.op = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
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
        if self.op != Mutation_OP::Put {
            my_size += ::protobuf::rt::enum_size(1, self.op);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.op != Mutation_OP::Put {
            os.write_enum(1, self.op.value())?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mutation {
    fn new() -> Mutation {
        Mutation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mutation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Mutation_OP>>(
                    "op",
                    Mutation::get_op_for_reflect,
                    Mutation::mut_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    Mutation::get_key_for_reflect,
                    Mutation::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Mutation::get_value_for_reflect,
                    Mutation::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mutation>(
                    "Mutation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        self.clear_op();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mutation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Mutation_OP {
    Put = 0,
}

impl ::protobuf::ProtobufEnum for Mutation_OP {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Mutation_OP> {
        match value {
            0 => ::std::option::Option::Some(Mutation_OP::Put),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Mutation_OP] = &[
            Mutation_OP::Put,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Mutation_OP>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Mutation_OP", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Mutation_OP {
}

impl ::std::default::Default for Mutation_OP {
    fn default() -> Self {
        Mutation_OP::Put
    }
}

impl ::protobuf::reflect::ProtobufValue for Mutation_OP {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteBatch {
    // message fields
    pub commit_ts: u64,
    pub mutations: ::protobuf::RepeatedField<Mutation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteBatch {}

impl WriteBatch {
    pub fn new() -> WriteBatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteBatch {
        static mut instance: ::protobuf::lazy::Lazy<WriteBatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteBatch,
        };
        unsafe {
            instance.get(WriteBatch::new)
        }
    }

    // uint64 commit_ts = 1;

    pub fn clear_commit_ts(&mut self) {
        self.commit_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_ts(&mut self, v: u64) {
        self.commit_ts = v;
    }

    pub fn get_commit_ts(&self) -> u64 {
        self.commit_ts
    }

    fn get_commit_ts_for_reflect(&self) -> &u64 {
        &self.commit_ts
    }

    fn mut_commit_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.commit_ts
    }

    // repeated .importpb.Mutation mutations = 2;

    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }

    // Param is passed by value, moved
    pub fn set_mutations(&mut self, v: ::protobuf::RepeatedField<Mutation>) {
        self.mutations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mutations(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // Take field
    pub fn take_mutations(&mut self) -> ::protobuf::RepeatedField<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::protobuf::RepeatedField::new())
    }

    pub fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    fn get_mutations_for_reflect(&self) -> &::protobuf::RepeatedField<Mutation> {
        &self.mutations
    }

    fn mut_mutations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }
}

impl ::protobuf::Message for WriteBatch {
    fn is_initialized(&self) -> bool {
        for v in &self.mutations {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit_ts = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mutations)?;
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
        if self.commit_ts != 0 {
            my_size += ::protobuf::rt::value_size(1, self.commit_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.mutations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.commit_ts != 0 {
            os.write_uint64(1, self.commit_ts)?;
        }
        for v in &self.mutations {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteBatch {
    fn new() -> WriteBatch {
        WriteBatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteBatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_ts",
                    WriteBatch::get_commit_ts_for_reflect,
                    WriteBatch::mut_commit_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mutation>>(
                    "mutations",
                    WriteBatch::get_mutations_for_reflect,
                    WriteBatch::mut_mutations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteBatch>(
                    "WriteBatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteBatch {
    fn clear(&mut self) {
        self.clear_commit_ts();
        self.clear_mutations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteBatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteBatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteRequest {
    // message fields
    pub head: ::protobuf::SingularPtrField<WriteRequest_Head>,
    pub batch: ::protobuf::SingularPtrField<WriteBatch>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteRequest {}

impl WriteRequest {
    pub fn new() -> WriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<WriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteRequest,
        };
        unsafe {
            instance.get(WriteRequest::new)
        }
    }

    // .importpb.WriteRequest.Head head = 1;

    pub fn clear_head(&mut self) {
        self.head.clear();
    }

    pub fn has_head(&self) -> bool {
        self.head.is_some()
    }

    // Param is passed by value, moved
    pub fn set_head(&mut self, v: WriteRequest_Head) {
        self.head = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_head(&mut self) -> &mut WriteRequest_Head {
        if self.head.is_none() {
            self.head.set_default();
        }
        self.head.as_mut().unwrap()
    }

    // Take field
    pub fn take_head(&mut self) -> WriteRequest_Head {
        self.head.take().unwrap_or_else(|| WriteRequest_Head::new())
    }

    pub fn get_head(&self) -> &WriteRequest_Head {
        self.head.as_ref().unwrap_or_else(|| WriteRequest_Head::default_instance())
    }

    fn get_head_for_reflect(&self) -> &::protobuf::SingularPtrField<WriteRequest_Head> {
        &self.head
    }

    fn mut_head_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<WriteRequest_Head> {
        &mut self.head
    }

    // .importpb.WriteBatch batch = 2;

    pub fn clear_batch(&mut self) {
        self.batch.clear();
    }

    pub fn has_batch(&self) -> bool {
        self.batch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_batch(&mut self, v: WriteBatch) {
        self.batch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_batch(&mut self) -> &mut WriteBatch {
        if self.batch.is_none() {
            self.batch.set_default();
        }
        self.batch.as_mut().unwrap()
    }

    // Take field
    pub fn take_batch(&mut self) -> WriteBatch {
        self.batch.take().unwrap_or_else(|| WriteBatch::new())
    }

    pub fn get_batch(&self) -> &WriteBatch {
        self.batch.as_ref().unwrap_or_else(|| WriteBatch::default_instance())
    }

    fn get_batch_for_reflect(&self) -> &::protobuf::SingularPtrField<WriteBatch> {
        &self.batch
    }

    fn mut_batch_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<WriteBatch> {
        &mut self.batch
    }
}

impl ::protobuf::Message for WriteRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.head {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.batch {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.head)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.batch)?;
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
        if let Some(ref v) = self.head.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.batch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.head.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.batch.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteRequest {
    fn new() -> WriteRequest {
        WriteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<WriteRequest_Head>>(
                    "head",
                    WriteRequest::get_head_for_reflect,
                    WriteRequest::mut_head_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<WriteBatch>>(
                    "batch",
                    WriteRequest::get_batch_for_reflect,
                    WriteRequest::mut_batch_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteRequest>(
                    "WriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteRequest {
    fn clear(&mut self) {
        self.clear_head();
        self.clear_batch();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteRequest_Head {
    // message fields
    pub uuid: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteRequest_Head {}

impl WriteRequest_Head {
    pub fn new() -> WriteRequest_Head {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteRequest_Head {
        static mut instance: ::protobuf::lazy::Lazy<WriteRequest_Head> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteRequest_Head,
        };
        unsafe {
            instance.get(WriteRequest_Head::new)
        }
    }

    // bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }
}

impl ::protobuf::Message for WriteRequest_Head {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.uuid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uuid.is_empty() {
            os.write_bytes(1, &self.uuid)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteRequest_Head {
    fn new() -> WriteRequest_Head {
        WriteRequest_Head::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteRequest_Head>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    WriteRequest_Head::get_uuid_for_reflect,
                    WriteRequest_Head::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteRequest_Head>(
                    "WriteRequest_Head",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteRequest_Head {
    fn clear(&mut self) {
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteRequest_Head {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteRequest_Head {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteResponse {}

impl WriteResponse {
    pub fn new() -> WriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<WriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteResponse,
        };
        unsafe {
            instance.get(WriteResponse::new)
        }
    }
}

impl ::protobuf::Message for WriteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WriteResponse {
    fn new() -> WriteResponse {
        WriteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<WriteResponse>(
                    "WriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FlushRequest {
    // message fields
    pub uuid: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FlushRequest {}

impl FlushRequest {
    pub fn new() -> FlushRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FlushRequest {
        static mut instance: ::protobuf::lazy::Lazy<FlushRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FlushRequest,
        };
        unsafe {
            instance.get(FlushRequest::new)
        }
    }

    // bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }
}

impl ::protobuf::Message for FlushRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.uuid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uuid.is_empty() {
            os.write_bytes(1, &self.uuid)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FlushRequest {
    fn new() -> FlushRequest {
        FlushRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<FlushRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    FlushRequest::get_uuid_for_reflect,
                    FlushRequest::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FlushRequest>(
                    "FlushRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FlushRequest {
    fn clear(&mut self) {
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FlushRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FlushRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FlushResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FlushResponse {}

impl FlushResponse {
    pub fn new() -> FlushResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FlushResponse {
        static mut instance: ::protobuf::lazy::Lazy<FlushResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FlushResponse,
        };
        unsafe {
            instance.get(FlushResponse::new)
        }
    }
}

impl ::protobuf::Message for FlushResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FlushResponse {
    fn new() -> FlushResponse {
        FlushResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<FlushResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<FlushResponse>(
                    "FlushResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FlushResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FlushResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FlushResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eimportpb.proto\x12\x08importpb\"h\n\x08Mutation\x12%\n\x02op\x18\
    \x01\x20\x01(\x0e2\x15.importpb.Mutation.OPR\x02op\x12\x10\n\x03key\x18\
    \x02\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\x03\x20\x01(\x0cR\x05va\
    lue\"\r\n\x02OP\x12\x07\n\x03Put\x10\0\"[\n\nWriteBatch\x12\x1b\n\tcommi\
    t_ts\x18\x01\x20\x01(\x04R\x08commitTs\x120\n\tmutations\x18\x02\x20\x03\
    (\x0b2\x12.importpb.MutationR\tmutations\"\x87\x01\n\x0cWriteRequest\x12\
    /\n\x04head\x18\x01\x20\x01(\x0b2\x1b.importpb.WriteRequest.HeadR\x04hea\
    d\x12*\n\x05batch\x18\x02\x20\x01(\x0b2\x14.importpb.WriteBatchR\x05batc\
    h\x1a\x1a\n\x04Head\x12\x12\n\x04uuid\x18\x01\x20\x01(\x0cR\x04uuid\"\
    \x0f\n\rWriteResponse\"\"\n\x0cFlushRequest\x12\x12\n\x04uuid\x18\x01\
    \x20\x01(\x0cR\x04uuid\"\x0f\n\rFlushResponse2\x82\x01\n\x06Import\x12<\
    \n\x05Write\x12\x16.importpb.WriteRequest\x1a\x17.importpb.WriteResponse\
    \"\0(\x01\x12:\n\x05Flush\x12\x16.importpb.FlushRequest\x1a\x17.importpb\
    .FlushResponse\"\0J\xa6\x08\n\x06\x12\x04\0\0'\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x10\n\n\n\x02\x06\0\x12\x04\
    \x04\0\x07\x01\n\n\n\x03\x06\0\x01\x12\x03\x04\x08\x0e\n\x0b\n\x04\x06\0\
    \x02\0\x12\x03\x05\x04=\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x05\x08\r\n\
    \x0c\n\x05\x06\0\x02\0\x05\x12\x03\x05\x0e\x14\n\x0c\n\x05\x06\0\x02\0\
    \x02\x12\x03\x05\x15!\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x05,9\n\x0b\n\
    \x04\x06\0\x02\x01\x12\x03\x06\x046\n\x0c\n\x05\x06\0\x02\x01\x01\x12\
    \x03\x06\x08\r\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\x06\x0e\x1a\n\x0c\n\
    \x05\x06\0\x02\x01\x03\x12\x03\x06%2\n\n\n\x02\x04\0\x12\x04\t\0\x10\x01\
    \n\n\n\x03\x04\0\x01\x12\x03\t\x08\x10\n\x0c\n\x04\x04\0\x04\0\x12\x04\n\
    \x04\x0c\x05\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\n\t\x0b\n\r\n\x06\x04\0\
    \x04\0\x02\0\x12\x03\x0b\x08\x10\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\
    \x03\x0b\x08\x0b\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03\x0b\x0e\x0f\n\
    \x0b\n\x04\x04\0\x02\0\x12\x03\r\x04\x0e\n\r\n\x05\x04\0\x02\0\x04\x12\
    \x04\r\x04\x0c\x05\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\r\x04\x06\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\r\x07\t\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    \r\x0c\r\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0e\x04\x12\n\r\n\x05\x04\0\
    \x02\x01\x04\x12\x04\x0e\x04\r\x0e\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x0e\x04\t\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0e\n\r\n\x0c\n\x05\x04\
    \0\x02\x01\x03\x12\x03\x0e\x10\x11\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0f\
    \x04\x14\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x0f\x04\x0e\x12\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03\x0f\x04\t\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03\x0f\n\x0f\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0f\x12\x13\n\n\n\
    \x02\x04\x01\x12\x04\x12\0\x15\x01\n\n\n\x03\x04\x01\x01\x12\x03\x12\x08\
    \x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x13\x04\x19\n\r\n\x05\x04\x01\x02\
    \0\x04\x12\x04\x13\x04\x12\x14\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x13\
    \x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x13\x0b\x14\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x13\x17\x18\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\
    \x14\x04$\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x14\x04\x0c\n\x0c\n\
    \x05\x04\x01\x02\x01\x06\x12\x03\x14\r\x15\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03\x14\x16\x1f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x14\"#\
    \n\n\n\x02\x04\x02\x12\x04\x17\0\x1d\x01\n\n\n\x03\x04\x02\x01\x12\x03\
    \x17\x08\x14\n\x0c\n\x04\x04\x02\x03\0\x12\x04\x18\x04\x1a\x05\n\x0c\n\
    \x05\x04\x02\x03\0\x01\x12\x03\x18\x0c\x10\n\r\n\x06\x04\x02\x03\0\x02\0\
    \x12\x03\x19\x08\x17\n\x0f\n\x07\x04\x02\x03\0\x02\0\x04\x12\x04\x19\x08\
    \x18\x12\n\x0e\n\x07\x04\x02\x03\0\x02\0\x05\x12\x03\x19\x08\r\n\x0e\n\
    \x07\x04\x02\x03\0\x02\0\x01\x12\x03\x19\x0e\x12\n\x0e\n\x07\x04\x02\x03\
    \0\x02\0\x03\x12\x03\x19\x15\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1b\
    \x04\x12\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x1b\x04\x1a\x05\n\x0c\n\x05\
    \x04\x02\x02\0\x06\x12\x03\x1b\x04\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03\x1b\t\r\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1b\x10\x11\n\x0b\n\
    \x04\x04\x02\x02\x01\x12\x03\x1c\x04\x19\n\r\n\x05\x04\x02\x02\x01\x04\
    \x12\x04\x1c\x04\x1b\x12\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03\x1c\x04\
    \x0e\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1c\x0f\x14\n\x0c\n\x05\x04\
    \x02\x02\x01\x03\x12\x03\x1c\x17\x18\n\n\n\x02\x04\x03\x12\x04\x1f\0\x20\
    \x01\n\n\n\x03\x04\x03\x01\x12\x03\x1f\x08\x15\n\n\n\x02\x04\x04\x12\x04\
    \"\0$\x01\n\n\n\x03\x04\x04\x01\x12\x03\"\x08\x14\n\x0b\n\x04\x04\x04\
    \x02\0\x12\x03#\x04\x13\n\r\n\x05\x04\x04\x02\0\x04\x12\x04#\x04\"\x16\n\
    \x0c\n\x05\x04\x04\x02\0\x05\x12\x03#\x04\t\n\x0c\n\x05\x04\x04\x02\0\
    \x01\x12\x03#\n\x0e\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03#\x11\x12\n\n\n\
    \x02\x04\x05\x12\x04&\0'\x01\n\n\n\x03\x04\x05\x01\x12\x03&\x08\x15b\x06\
    proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
