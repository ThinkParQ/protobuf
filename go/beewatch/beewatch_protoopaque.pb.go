// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.36.2
// 	protoc        v5.29.2
// source: beewatch.proto

//go:build protoopaque

package beewatch

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type V1Event_Type int32

const (
	V1Event_FLUSH       V1Event_Type = 0
	V1Event_TRUNCATE    V1Event_Type = 1
	V1Event_SETATTR     V1Event_Type = 2
	V1Event_CLOSE_WRITE V1Event_Type = 3
	V1Event_CREATE      V1Event_Type = 4
	V1Event_MKDIR       V1Event_Type = 5
	V1Event_MKNOD       V1Event_Type = 6
	V1Event_SYMLINK     V1Event_Type = 7
	V1Event_RMDIR       V1Event_Type = 8
	V1Event_UNLINK      V1Event_Type = 9
	V1Event_HARDLINK    V1Event_Type = 10
	V1Event_RENAME      V1Event_Type = 11
	V1Event_READ        V1Event_Type = 12
)

// Enum value maps for V1Event_Type.
var (
	V1Event_Type_name = map[int32]string{
		0:  "FLUSH",
		1:  "TRUNCATE",
		2:  "SETATTR",
		3:  "CLOSE_WRITE",
		4:  "CREATE",
		5:  "MKDIR",
		6:  "MKNOD",
		7:  "SYMLINK",
		8:  "RMDIR",
		9:  "UNLINK",
		10: "HARDLINK",
		11: "RENAME",
		12: "READ",
	}
	V1Event_Type_value = map[string]int32{
		"FLUSH":       0,
		"TRUNCATE":    1,
		"SETATTR":     2,
		"CLOSE_WRITE": 3,
		"CREATE":      4,
		"MKDIR":       5,
		"MKNOD":       6,
		"SYMLINK":     7,
		"RMDIR":       8,
		"UNLINK":      9,
		"HARDLINK":    10,
		"RENAME":      11,
		"READ":        12,
	}
)

func (x V1Event_Type) Enum() *V1Event_Type {
	p := new(V1Event_Type)
	*p = x
	return p
}

func (x V1Event_Type) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (V1Event_Type) Descriptor() protoreflect.EnumDescriptor {
	return file_beewatch_proto_enumTypes[0].Descriptor()
}

func (V1Event_Type) Type() protoreflect.EnumType {
	return &file_beewatch_proto_enumTypes[0]
}

func (x V1Event_Type) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

type V2Event_Type int32

const (
	V2Event_INVALID            V2Event_Type = 0
	V2Event_FLUSH              V2Event_Type = 1
	V2Event_TRUNCATE           V2Event_Type = 2
	V2Event_SETATTR            V2Event_Type = 3
	V2Event_CLOSE_WRITE        V2Event_Type = 4
	V2Event_CREATE             V2Event_Type = 5
	V2Event_MKDIR              V2Event_Type = 6
	V2Event_MKNOD              V2Event_Type = 7
	V2Event_SYMLINK            V2Event_Type = 8
	V2Event_RMDIR              V2Event_Type = 9
	V2Event_UNLINK             V2Event_Type = 10
	V2Event_HARDLINK           V2Event_Type = 11
	V2Event_RENAME             V2Event_Type = 12
	V2Event_OPEN_READ          V2Event_Type = 13
	V2Event_OPEN_WRITE         V2Event_Type = 14
	V2Event_OPEN_READ_WRITE    V2Event_Type = 15
	V2Event_LAST_WRITER_CLOSED V2Event_Type = 16
	V2Event_OPEN_BLOCKED       V2Event_Type = 17
)

// Enum value maps for V2Event_Type.
var (
	V2Event_Type_name = map[int32]string{
		0:  "INVALID",
		1:  "FLUSH",
		2:  "TRUNCATE",
		3:  "SETATTR",
		4:  "CLOSE_WRITE",
		5:  "CREATE",
		6:  "MKDIR",
		7:  "MKNOD",
		8:  "SYMLINK",
		9:  "RMDIR",
		10: "UNLINK",
		11: "HARDLINK",
		12: "RENAME",
		13: "OPEN_READ",
		14: "OPEN_WRITE",
		15: "OPEN_READ_WRITE",
		16: "LAST_WRITER_CLOSED",
		17: "OPEN_BLOCKED",
	}
	V2Event_Type_value = map[string]int32{
		"INVALID":            0,
		"FLUSH":              1,
		"TRUNCATE":           2,
		"SETATTR":            3,
		"CLOSE_WRITE":        4,
		"CREATE":             5,
		"MKDIR":              6,
		"MKNOD":              7,
		"SYMLINK":            8,
		"RMDIR":              9,
		"UNLINK":             10,
		"HARDLINK":           11,
		"RENAME":             12,
		"OPEN_READ":          13,
		"OPEN_WRITE":         14,
		"OPEN_READ_WRITE":    15,
		"LAST_WRITER_CLOSED": 16,
		"OPEN_BLOCKED":       17,
	}
)

func (x V2Event_Type) Enum() *V2Event_Type {
	p := new(V2Event_Type)
	*p = x
	return p
}

func (x V2Event_Type) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (V2Event_Type) Descriptor() protoreflect.EnumDescriptor {
	return file_beewatch_proto_enumTypes[1].Descriptor()
}

func (V2Event_Type) Type() protoreflect.EnumType {
	return &file_beewatch_proto_enumTypes[1]
}

func (x V2Event_Type) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// The event message provides common header fields and wraps all event versions that can be handled
// by BeeGFS Watch. It notably does not provide a way to differentiate between minor versions of the
// API. Those should only affect the serialization format between the meta and watch services, and
// should adhere to standard protocol buffer best practices. Notably minor updates should be
// additive and not remove or change the meaning of existing fields.
type Event struct {
	state                  protoimpl.MessageState `protogen:"opaque.v1"`
	xxx_hidden_SeqId       uint64                 `protobuf:"varint,1,opt,name=seq_id,json=seqId,proto3" json:"seq_id,omitempty"`
	xxx_hidden_MetaId      uint32                 `protobuf:"varint,2,opt,name=meta_id,json=metaId,proto3" json:"meta_id,omitempty"`
	xxx_hidden_MetaMirror  uint32                 `protobuf:"varint,3,opt,name=meta_mirror,json=metaMirror,proto3,oneof" json:"meta_mirror,omitempty"`
	xxx_hidden_EventFlags  uint32                 `protobuf:"varint,4,opt,name=event_flags,json=eventFlags,proto3" json:"event_flags,omitempty"`
	xxx_hidden_EventData   isEvent_EventData      `protobuf_oneof:"event_data"`
	XXX_raceDetectHookData protoimpl.RaceDetectHookData
	XXX_presence           [1]uint32
	unknownFields          protoimpl.UnknownFields
	sizeCache              protoimpl.SizeCache
}

func (x *Event) Reset() {
	*x = Event{}
	mi := &file_beewatch_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Event) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Event) ProtoMessage() {}

func (x *Event) ProtoReflect() protoreflect.Message {
	mi := &file_beewatch_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (x *Event) GetSeqId() uint64 {
	if x != nil {
		return x.xxx_hidden_SeqId
	}
	return 0
}

func (x *Event) GetMetaId() uint32 {
	if x != nil {
		return x.xxx_hidden_MetaId
	}
	return 0
}

func (x *Event) GetMetaMirror() uint32 {
	if x != nil {
		return x.xxx_hidden_MetaMirror
	}
	return 0
}

func (x *Event) GetEventFlags() uint32 {
	if x != nil {
		return x.xxx_hidden_EventFlags
	}
	return 0
}

func (x *Event) GetV1() *V1Event {
	if x != nil {
		if x, ok := x.xxx_hidden_EventData.(*event_V1); ok {
			return x.V1
		}
	}
	return nil
}

func (x *Event) GetV2() *V2Event {
	if x != nil {
		if x, ok := x.xxx_hidden_EventData.(*event_V2); ok {
			return x.V2
		}
	}
	return nil
}

func (x *Event) SetSeqId(v uint64) {
	x.xxx_hidden_SeqId = v
}

func (x *Event) SetMetaId(v uint32) {
	x.xxx_hidden_MetaId = v
}

func (x *Event) SetMetaMirror(v uint32) {
	x.xxx_hidden_MetaMirror = v
	protoimpl.X.SetPresent(&(x.XXX_presence[0]), 2, 5)
}

func (x *Event) SetEventFlags(v uint32) {
	x.xxx_hidden_EventFlags = v
}

func (x *Event) SetV1(v *V1Event) {
	if v == nil {
		x.xxx_hidden_EventData = nil
		return
	}
	x.xxx_hidden_EventData = &event_V1{v}
}

func (x *Event) SetV2(v *V2Event) {
	if v == nil {
		x.xxx_hidden_EventData = nil
		return
	}
	x.xxx_hidden_EventData = &event_V2{v}
}

func (x *Event) HasMetaMirror() bool {
	if x == nil {
		return false
	}
	return protoimpl.X.Present(&(x.XXX_presence[0]), 2)
}

func (x *Event) HasEventData() bool {
	if x == nil {
		return false
	}
	return x.xxx_hidden_EventData != nil
}

func (x *Event) HasV1() bool {
	if x == nil {
		return false
	}
	_, ok := x.xxx_hidden_EventData.(*event_V1)
	return ok
}

func (x *Event) HasV2() bool {
	if x == nil {
		return false
	}
	_, ok := x.xxx_hidden_EventData.(*event_V2)
	return ok
}

func (x *Event) ClearMetaMirror() {
	protoimpl.X.ClearPresent(&(x.XXX_presence[0]), 2)
	x.xxx_hidden_MetaMirror = 0
}

func (x *Event) ClearEventData() {
	x.xxx_hidden_EventData = nil
}

func (x *Event) ClearV1() {
	if _, ok := x.xxx_hidden_EventData.(*event_V1); ok {
		x.xxx_hidden_EventData = nil
	}
}

func (x *Event) ClearV2() {
	if _, ok := x.xxx_hidden_EventData.(*event_V2); ok {
		x.xxx_hidden_EventData = nil
	}
}

const Event_EventData_not_set_case case_Event_EventData = 0
const Event_V1_case case_Event_EventData = 11
const Event_V2_case case_Event_EventData = 12

func (x *Event) WhichEventData() case_Event_EventData {
	if x == nil {
		return Event_EventData_not_set_case
	}
	switch x.xxx_hidden_EventData.(type) {
	case *event_V1:
		return Event_V1_case
	case *event_V2:
		return Event_V2_case
	default:
		return Event_EventData_not_set_case
	}
}

type Event_builder struct {
	_ [0]func() // Prevents comparability and use of unkeyed literals for the builder.

	SeqId      uint64
	MetaId     uint32
	MetaMirror *uint32
	// Bitmask representing event flags as defined by the EVENTFLAG constants in EventContext.h
	EventFlags uint32
	// Fields of oneof xxx_hidden_EventData:
	V1 *V1Event
	V2 *V2Event
	// -- end of xxx_hidden_EventData
}

func (b0 Event_builder) Build() *Event {
	m0 := &Event{}
	b, x := &b0, m0
	_, _ = b, x
	x.xxx_hidden_SeqId = b.SeqId
	x.xxx_hidden_MetaId = b.MetaId
	if b.MetaMirror != nil {
		protoimpl.X.SetPresentNonAtomic(&(x.XXX_presence[0]), 2, 5)
		x.xxx_hidden_MetaMirror = *b.MetaMirror
	}
	x.xxx_hidden_EventFlags = b.EventFlags
	if b.V1 != nil {
		x.xxx_hidden_EventData = &event_V1{b.V1}
	}
	if b.V2 != nil {
		x.xxx_hidden_EventData = &event_V2{b.V2}
	}
	return m0
}

type case_Event_EventData protoreflect.FieldNumber

func (x case_Event_EventData) String() string {
	md := file_beewatch_proto_msgTypes[0].Descriptor()
	if x == 0 {
		return "not set"
	}
	return protoimpl.X.MessageFieldStringOf(md, protoreflect.FieldNumber(x))
}

type isEvent_EventData interface {
	isEvent_EventData()
}

type event_V1 struct {
	V1 *V1Event `protobuf:"bytes,11,opt,name=v1,proto3,oneof"`
}

type event_V2 struct {
	V2 *V2Event `protobuf:"bytes,12,opt,name=v2,proto3,oneof"`
}

func (*event_V1) isEvent_EventData() {}

func (*event_V2) isEvent_EventData() {}

// The v1 event format is the legacy format from BeeGFS v7.
type V1Event struct {
	state                     protoimpl.MessageState `protogen:"opaque.v1"`
	xxx_hidden_Type           V1Event_Type           `protobuf:"varint,1,opt,name=type,proto3,enum=beewatch.V1Event_Type" json:"type,omitempty"`
	xxx_hidden_DroppedSeq     uint64                 `protobuf:"varint,2,opt,name=dropped_seq,json=droppedSeq,proto3" json:"dropped_seq,omitempty"`
	xxx_hidden_MissedSeq      uint64                 `protobuf:"varint,3,opt,name=missed_seq,json=missedSeq,proto3" json:"missed_seq,omitempty"`
	xxx_hidden_Path           string                 `protobuf:"bytes,4,opt,name=path,proto3" json:"path,omitempty"`
	xxx_hidden_EntryId        string                 `protobuf:"bytes,5,opt,name=entry_id,json=entryId,proto3" json:"entry_id,omitempty"`
	xxx_hidden_ParentEntryId  string                 `protobuf:"bytes,6,opt,name=parent_entry_id,json=parentEntryId,proto3" json:"parent_entry_id,omitempty"`
	xxx_hidden_TargetPath     string                 `protobuf:"bytes,7,opt,name=target_path,json=targetPath,proto3" json:"target_path,omitempty"`
	xxx_hidden_TargetParentId string                 `protobuf:"bytes,8,opt,name=target_parent_id,json=targetParentId,proto3" json:"target_parent_id,omitempty"`
	unknownFields             protoimpl.UnknownFields
	sizeCache                 protoimpl.SizeCache
}

func (x *V1Event) Reset() {
	*x = V1Event{}
	mi := &file_beewatch_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *V1Event) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*V1Event) ProtoMessage() {}

func (x *V1Event) ProtoReflect() protoreflect.Message {
	mi := &file_beewatch_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (x *V1Event) GetType() V1Event_Type {
	if x != nil {
		return x.xxx_hidden_Type
	}
	return V1Event_FLUSH
}

func (x *V1Event) GetDroppedSeq() uint64 {
	if x != nil {
		return x.xxx_hidden_DroppedSeq
	}
	return 0
}

func (x *V1Event) GetMissedSeq() uint64 {
	if x != nil {
		return x.xxx_hidden_MissedSeq
	}
	return 0
}

func (x *V1Event) GetPath() string {
	if x != nil {
		return x.xxx_hidden_Path
	}
	return ""
}

func (x *V1Event) GetEntryId() string {
	if x != nil {
		return x.xxx_hidden_EntryId
	}
	return ""
}

func (x *V1Event) GetParentEntryId() string {
	if x != nil {
		return x.xxx_hidden_ParentEntryId
	}
	return ""
}

func (x *V1Event) GetTargetPath() string {
	if x != nil {
		return x.xxx_hidden_TargetPath
	}
	return ""
}

func (x *V1Event) GetTargetParentId() string {
	if x != nil {
		return x.xxx_hidden_TargetParentId
	}
	return ""
}

func (x *V1Event) SetType(v V1Event_Type) {
	x.xxx_hidden_Type = v
}

func (x *V1Event) SetDroppedSeq(v uint64) {
	x.xxx_hidden_DroppedSeq = v
}

func (x *V1Event) SetMissedSeq(v uint64) {
	x.xxx_hidden_MissedSeq = v
}

func (x *V1Event) SetPath(v string) {
	x.xxx_hidden_Path = v
}

func (x *V1Event) SetEntryId(v string) {
	x.xxx_hidden_EntryId = v
}

func (x *V1Event) SetParentEntryId(v string) {
	x.xxx_hidden_ParentEntryId = v
}

func (x *V1Event) SetTargetPath(v string) {
	x.xxx_hidden_TargetPath = v
}

func (x *V1Event) SetTargetParentId(v string) {
	x.xxx_hidden_TargetParentId = v
}

type V1Event_builder struct {
	_ [0]func() // Prevents comparability and use of unkeyed literals for the builder.

	Type           V1Event_Type
	DroppedSeq     uint64
	MissedSeq      uint64
	Path           string
	EntryId        string
	ParentEntryId  string
	TargetPath     string
	TargetParentId string
}

func (b0 V1Event_builder) Build() *V1Event {
	m0 := &V1Event{}
	b, x := &b0, m0
	_, _ = b, x
	x.xxx_hidden_Type = b.Type
	x.xxx_hidden_DroppedSeq = b.DroppedSeq
	x.xxx_hidden_MissedSeq = b.MissedSeq
	x.xxx_hidden_Path = b.Path
	x.xxx_hidden_EntryId = b.EntryId
	x.xxx_hidden_ParentEntryId = b.ParentEntryId
	x.xxx_hidden_TargetPath = b.TargetPath
	x.xxx_hidden_TargetParentId = b.TargetParentId
	return m0
}

// The v2 event format was introduced in BeeGFS v8.
type V2Event struct {
	state                     protoimpl.MessageState `protogen:"opaque.v1"`
	xxx_hidden_Type           V2Event_Type           `protobuf:"varint,1,opt,name=type,proto3,enum=beewatch.V2Event_Type" json:"type,omitempty"`
	xxx_hidden_NumLinks       uint64                 `protobuf:"varint,2,opt,name=num_links,json=numLinks,proto3" json:"num_links,omitempty"`
	xxx_hidden_Path           string                 `protobuf:"bytes,3,opt,name=path,proto3" json:"path,omitempty"`
	xxx_hidden_EntryId        string                 `protobuf:"bytes,4,opt,name=entry_id,json=entryId,proto3" json:"entry_id,omitempty"`
	xxx_hidden_ParentEntryId  string                 `protobuf:"bytes,5,opt,name=parent_entry_id,json=parentEntryId,proto3" json:"parent_entry_id,omitempty"`
	xxx_hidden_TargetPath     string                 `protobuf:"bytes,6,opt,name=target_path,json=targetPath,proto3" json:"target_path,omitempty"`
	xxx_hidden_TargetParentId string                 `protobuf:"bytes,7,opt,name=target_parent_id,json=targetParentId,proto3" json:"target_parent_id,omitempty"`
	xxx_hidden_MsgUserId      uint32                 `protobuf:"varint,8,opt,name=msg_user_id,json=msgUserId,proto3" json:"msg_user_id,omitempty"`
	xxx_hidden_Timestamp      int64                  `protobuf:"varint,9,opt,name=timestamp,proto3" json:"timestamp,omitempty"`
	unknownFields             protoimpl.UnknownFields
	sizeCache                 protoimpl.SizeCache
}

func (x *V2Event) Reset() {
	*x = V2Event{}
	mi := &file_beewatch_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *V2Event) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*V2Event) ProtoMessage() {}

func (x *V2Event) ProtoReflect() protoreflect.Message {
	mi := &file_beewatch_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (x *V2Event) GetType() V2Event_Type {
	if x != nil {
		return x.xxx_hidden_Type
	}
	return V2Event_INVALID
}

func (x *V2Event) GetNumLinks() uint64 {
	if x != nil {
		return x.xxx_hidden_NumLinks
	}
	return 0
}

func (x *V2Event) GetPath() string {
	if x != nil {
		return x.xxx_hidden_Path
	}
	return ""
}

func (x *V2Event) GetEntryId() string {
	if x != nil {
		return x.xxx_hidden_EntryId
	}
	return ""
}

func (x *V2Event) GetParentEntryId() string {
	if x != nil {
		return x.xxx_hidden_ParentEntryId
	}
	return ""
}

func (x *V2Event) GetTargetPath() string {
	if x != nil {
		return x.xxx_hidden_TargetPath
	}
	return ""
}

func (x *V2Event) GetTargetParentId() string {
	if x != nil {
		return x.xxx_hidden_TargetParentId
	}
	return ""
}

func (x *V2Event) GetMsgUserId() uint32 {
	if x != nil {
		return x.xxx_hidden_MsgUserId
	}
	return 0
}

func (x *V2Event) GetTimestamp() int64 {
	if x != nil {
		return x.xxx_hidden_Timestamp
	}
	return 0
}

func (x *V2Event) SetType(v V2Event_Type) {
	x.xxx_hidden_Type = v
}

func (x *V2Event) SetNumLinks(v uint64) {
	x.xxx_hidden_NumLinks = v
}

func (x *V2Event) SetPath(v string) {
	x.xxx_hidden_Path = v
}

func (x *V2Event) SetEntryId(v string) {
	x.xxx_hidden_EntryId = v
}

func (x *V2Event) SetParentEntryId(v string) {
	x.xxx_hidden_ParentEntryId = v
}

func (x *V2Event) SetTargetPath(v string) {
	x.xxx_hidden_TargetPath = v
}

func (x *V2Event) SetTargetParentId(v string) {
	x.xxx_hidden_TargetParentId = v
}

func (x *V2Event) SetMsgUserId(v uint32) {
	x.xxx_hidden_MsgUserId = v
}

func (x *V2Event) SetTimestamp(v int64) {
	x.xxx_hidden_Timestamp = v
}

type V2Event_builder struct {
	_ [0]func() // Prevents comparability and use of unkeyed literals for the builder.

	Type           V2Event_Type
	NumLinks       uint64
	Path           string
	EntryId        string
	ParentEntryId  string
	TargetPath     string
	TargetParentId string
	MsgUserId      uint32
	Timestamp      int64
}

func (b0 V2Event_builder) Build() *V2Event {
	m0 := &V2Event{}
	b, x := &b0, m0
	_, _ = b, x
	x.xxx_hidden_Type = b.Type
	x.xxx_hidden_NumLinks = b.NumLinks
	x.xxx_hidden_Path = b.Path
	x.xxx_hidden_EntryId = b.EntryId
	x.xxx_hidden_ParentEntryId = b.ParentEntryId
	x.xxx_hidden_TargetPath = b.TargetPath
	x.xxx_hidden_TargetParentId = b.TargetParentId
	x.xxx_hidden_MsgUserId = b.MsgUserId
	x.xxx_hidden_Timestamp = b.Timestamp
	return m0
}

// Response messages allow the subscribers to acknowledge events they have processed and request a graceful shutdown.
type Response struct {
	state                   protoimpl.MessageState `protogen:"opaque.v1"`
	xxx_hidden_CompletedSeq uint64                 `protobuf:"varint,1,opt,name=completed_seq,json=completedSeq,proto3" json:"completed_seq,omitempty"`
	xxx_hidden_ShuttingDown bool                   `protobuf:"varint,2,opt,name=shutting_down,json=shuttingDown,proto3" json:"shutting_down,omitempty"`
	unknownFields           protoimpl.UnknownFields
	sizeCache               protoimpl.SizeCache
}

func (x *Response) Reset() {
	*x = Response{}
	mi := &file_beewatch_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *Response) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Response) ProtoMessage() {}

func (x *Response) ProtoReflect() protoreflect.Message {
	mi := &file_beewatch_proto_msgTypes[3]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (x *Response) GetCompletedSeq() uint64 {
	if x != nil {
		return x.xxx_hidden_CompletedSeq
	}
	return 0
}

func (x *Response) GetShuttingDown() bool {
	if x != nil {
		return x.xxx_hidden_ShuttingDown
	}
	return false
}

func (x *Response) SetCompletedSeq(v uint64) {
	x.xxx_hidden_CompletedSeq = v
}

func (x *Response) SetShuttingDown(v bool) {
	x.xxx_hidden_ShuttingDown = v
}

type Response_builder struct {
	_ [0]func() // Prevents comparability and use of unkeyed literals for the builder.

	CompletedSeq uint64
	ShuttingDown bool
}

func (b0 Response_builder) Build() *Response {
	m0 := &Response{}
	b, x := &b0, m0
	_, _ = b, x
	x.xxx_hidden_CompletedSeq = b.CompletedSeq
	x.xxx_hidden_ShuttingDown = b.ShuttingDown
	return m0
}

var File_beewatch_proto protoreflect.FileDescriptor

var file_beewatch_proto_rawDesc = []byte{
	0x0a, 0x0e, 0x62, 0x65, 0x65, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x12, 0x08, 0x62, 0x65, 0x65, 0x77, 0x61, 0x74, 0x63, 0x68, 0x22, 0xe6, 0x01, 0x0a, 0x05, 0x45,
	0x76, 0x65, 0x6e, 0x74, 0x12, 0x15, 0x0a, 0x06, 0x73, 0x65, 0x71, 0x5f, 0x69, 0x64, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x73, 0x65, 0x71, 0x49, 0x64, 0x12, 0x17, 0x0a, 0x07, 0x6d,
	0x65, 0x74, 0x61, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06, 0x6d, 0x65,
	0x74, 0x61, 0x49, 0x64, 0x12, 0x24, 0x0a, 0x0b, 0x6d, 0x65, 0x74, 0x61, 0x5f, 0x6d, 0x69, 0x72,
	0x72, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x01, 0x52, 0x0a, 0x6d, 0x65, 0x74,
	0x61, 0x4d, 0x69, 0x72, 0x72, 0x6f, 0x72, 0x88, 0x01, 0x01, 0x12, 0x1f, 0x0a, 0x0b, 0x65, 0x76,
	0x65, 0x6e, 0x74, 0x5f, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x52,
	0x0a, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x46, 0x6c, 0x61, 0x67, 0x73, 0x12, 0x23, 0x0a, 0x02, 0x76,
	0x31, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x62, 0x65, 0x65, 0x77, 0x61, 0x74,
	0x63, 0x68, 0x2e, 0x56, 0x31, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x48, 0x00, 0x52, 0x02, 0x76, 0x31,
	0x12, 0x23, 0x0a, 0x02, 0x76, 0x32, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x62,
	0x65, 0x65, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x56, 0x32, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x48,
	0x00, 0x52, 0x02, 0x76, 0x32, 0x42, 0x0c, 0x0a, 0x0a, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x64,
	0x61, 0x74, 0x61, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x6d, 0x65, 0x74, 0x61, 0x5f, 0x6d, 0x69, 0x72,
	0x72, 0x6f, 0x72, 0x22, 0xc1, 0x03, 0x0a, 0x07, 0x56, 0x31, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12,
	0x2a, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x16, 0x2e,
	0x62, 0x65, 0x65, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x56, 0x31, 0x45, 0x76, 0x65, 0x6e, 0x74,
	0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x64,
	0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x5f, 0x73, 0x65, 0x71, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
	0x52, 0x0a, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x53, 0x65, 0x71, 0x12, 0x1d, 0x0a, 0x0a,
	0x6d, 0x69, 0x73, 0x73, 0x65, 0x64, 0x5f, 0x73, 0x65, 0x71, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04,
	0x52, 0x09, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x64, 0x53, 0x65, 0x71, 0x12, 0x12, 0x0a, 0x04, 0x70,
	0x61, 0x74, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68, 0x12,
	0x19, 0x0a, 0x08, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x07, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x70, 0x61,
	0x72, 0x65, 0x6e, 0x74, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20,
	0x01, 0x28, 0x09, 0x52, 0x0d, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x45, 0x6e, 0x74, 0x72, 0x79,
	0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x70, 0x61, 0x74,
	0x68, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x50,
	0x61, 0x74, 0x68, 0x12, 0x28, 0x0a, 0x10, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x70, 0x61,
	0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x74,
	0x61, 0x72, 0x67, 0x65, 0x74, 0x50, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x22, 0xa7, 0x01,
	0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x4c, 0x55, 0x53, 0x48, 0x10,
	0x00, 0x12, 0x0c, 0x0a, 0x08, 0x54, 0x52, 0x55, 0x4e, 0x43, 0x41, 0x54, 0x45, 0x10, 0x01, 0x12,
	0x0b, 0x0a, 0x07, 0x53, 0x45, 0x54, 0x41, 0x54, 0x54, 0x52, 0x10, 0x02, 0x12, 0x0f, 0x0a, 0x0b,
	0x43, 0x4c, 0x4f, 0x53, 0x45, 0x5f, 0x57, 0x52, 0x49, 0x54, 0x45, 0x10, 0x03, 0x12, 0x0a, 0x0a,
	0x06, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x10, 0x04, 0x12, 0x09, 0x0a, 0x05, 0x4d, 0x4b, 0x44,
	0x49, 0x52, 0x10, 0x05, 0x12, 0x09, 0x0a, 0x05, 0x4d, 0x4b, 0x4e, 0x4f, 0x44, 0x10, 0x06, 0x12,
	0x0b, 0x0a, 0x07, 0x53, 0x59, 0x4d, 0x4c, 0x49, 0x4e, 0x4b, 0x10, 0x07, 0x12, 0x09, 0x0a, 0x05,
	0x52, 0x4d, 0x44, 0x49, 0x52, 0x10, 0x08, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x4e, 0x4c, 0x49, 0x4e,
	0x4b, 0x10, 0x09, 0x12, 0x0c, 0x0a, 0x08, 0x48, 0x41, 0x52, 0x44, 0x4c, 0x49, 0x4e, 0x4b, 0x10,
	0x0a, 0x12, 0x0a, 0x0a, 0x06, 0x52, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x10, 0x0b, 0x12, 0x08, 0x0a,
	0x04, 0x52, 0x45, 0x41, 0x44, 0x10, 0x0c, 0x22, 0xbd, 0x04, 0x0a, 0x07, 0x56, 0x32, 0x45, 0x76,
	0x65, 0x6e, 0x74, 0x12, 0x2a, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
	0x0e, 0x32, 0x16, 0x2e, 0x62, 0x65, 0x65, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x56, 0x32, 0x45,
	0x76, 0x65, 0x6e, 0x74, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12,
	0x1b, 0x0a, 0x09, 0x6e, 0x75, 0x6d, 0x5f, 0x6c, 0x69, 0x6e, 0x6b, 0x73, 0x18, 0x02, 0x20, 0x01,
	0x28, 0x04, 0x52, 0x08, 0x6e, 0x75, 0x6d, 0x4c, 0x69, 0x6e, 0x6b, 0x73, 0x12, 0x12, 0x0a, 0x04,
	0x70, 0x61, 0x74, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x70, 0x61, 0x74, 0x68,
	0x12, 0x19, 0x0a, 0x08, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01,
	0x28, 0x09, 0x52, 0x07, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x49, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x70,
	0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x5f, 0x69, 0x64, 0x18, 0x05,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x45, 0x6e, 0x74, 0x72,
	0x79, 0x49, 0x64, 0x12, 0x1f, 0x0a, 0x0b, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x70, 0x61,
	0x74, 0x68, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74,
	0x50, 0x61, 0x74, 0x68, 0x12, 0x28, 0x0a, 0x10, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x5f, 0x70,
	0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e,
	0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x50, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x1e,
	0x0a, 0x0b, 0x6d, 0x73, 0x67, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20,
	0x01, 0x28, 0x0d, 0x52, 0x09, 0x6d, 0x73, 0x67, 0x55, 0x73, 0x65, 0x72, 0x49, 0x64, 0x12, 0x1c,
	0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x09, 0x20, 0x01, 0x28,
	0x03, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x88, 0x02, 0x0a,
	0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49, 0x44,
	0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x46, 0x4c, 0x55, 0x53, 0x48, 0x10, 0x01, 0x12, 0x0c, 0x0a,
	0x08, 0x54, 0x52, 0x55, 0x4e, 0x43, 0x41, 0x54, 0x45, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x53,
	0x45, 0x54, 0x41, 0x54, 0x54, 0x52, 0x10, 0x03, 0x12, 0x0f, 0x0a, 0x0b, 0x43, 0x4c, 0x4f, 0x53,
	0x45, 0x5f, 0x57, 0x52, 0x49, 0x54, 0x45, 0x10, 0x04, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x52, 0x45,
	0x41, 0x54, 0x45, 0x10, 0x05, 0x12, 0x09, 0x0a, 0x05, 0x4d, 0x4b, 0x44, 0x49, 0x52, 0x10, 0x06,
	0x12, 0x09, 0x0a, 0x05, 0x4d, 0x4b, 0x4e, 0x4f, 0x44, 0x10, 0x07, 0x12, 0x0b, 0x0a, 0x07, 0x53,
	0x59, 0x4d, 0x4c, 0x49, 0x4e, 0x4b, 0x10, 0x08, 0x12, 0x09, 0x0a, 0x05, 0x52, 0x4d, 0x44, 0x49,
	0x52, 0x10, 0x09, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x4e, 0x4c, 0x49, 0x4e, 0x4b, 0x10, 0x0a, 0x12,
	0x0c, 0x0a, 0x08, 0x48, 0x41, 0x52, 0x44, 0x4c, 0x49, 0x4e, 0x4b, 0x10, 0x0b, 0x12, 0x0a, 0x0a,
	0x06, 0x52, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x10, 0x0c, 0x12, 0x0d, 0x0a, 0x09, 0x4f, 0x50, 0x45,
	0x4e, 0x5f, 0x52, 0x45, 0x41, 0x44, 0x10, 0x0d, 0x12, 0x0e, 0x0a, 0x0a, 0x4f, 0x50, 0x45, 0x4e,
	0x5f, 0x57, 0x52, 0x49, 0x54, 0x45, 0x10, 0x0e, 0x12, 0x13, 0x0a, 0x0f, 0x4f, 0x50, 0x45, 0x4e,
	0x5f, 0x52, 0x45, 0x41, 0x44, 0x5f, 0x57, 0x52, 0x49, 0x54, 0x45, 0x10, 0x0f, 0x12, 0x16, 0x0a,
	0x12, 0x4c, 0x41, 0x53, 0x54, 0x5f, 0x57, 0x52, 0x49, 0x54, 0x45, 0x52, 0x5f, 0x43, 0x4c, 0x4f,
	0x53, 0x45, 0x44, 0x10, 0x10, 0x12, 0x10, 0x0a, 0x0c, 0x4f, 0x50, 0x45, 0x4e, 0x5f, 0x42, 0x4c,
	0x4f, 0x43, 0x4b, 0x45, 0x44, 0x10, 0x11, 0x22, 0x54, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f,
	0x6e, 0x73, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64,
	0x5f, 0x73, 0x65, 0x71, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x63, 0x6f, 0x6d, 0x70,
	0x6c, 0x65, 0x74, 0x65, 0x64, 0x53, 0x65, 0x71, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x68, 0x75, 0x74,
	0x74, 0x69, 0x6e, 0x67, 0x5f, 0x64, 0x6f, 0x77, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52,
	0x0c, 0x73, 0x68, 0x75, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x44, 0x6f, 0x77, 0x6e, 0x32, 0x46, 0x0a,
	0x0a, 0x53, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x72, 0x12, 0x38, 0x0a, 0x0d, 0x52,
	0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x0f, 0x2e, 0x62,
	0x65, 0x65, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x1a, 0x12, 0x2e,
	0x62, 0x65, 0x65, 0x77, 0x61, 0x74, 0x63, 0x68, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
	0x65, 0x28, 0x01, 0x30, 0x01, 0x42, 0x2b, 0x5a, 0x29, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e,
	0x63, 0x6f, 0x6d, 0x2f, 0x74, 0x68, 0x69, 0x6e, 0x6b, 0x70, 0x61, 0x72, 0x71, 0x2f, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x67, 0x6f, 0x2f, 0x62, 0x65, 0x65, 0x77, 0x61, 0x74,
	0x63, 0x68, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var file_beewatch_proto_enumTypes = make([]protoimpl.EnumInfo, 2)
var file_beewatch_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_beewatch_proto_goTypes = []any{
	(V1Event_Type)(0), // 0: beewatch.V1Event.Type
	(V2Event_Type)(0), // 1: beewatch.V2Event.Type
	(*Event)(nil),     // 2: beewatch.Event
	(*V1Event)(nil),   // 3: beewatch.V1Event
	(*V2Event)(nil),   // 4: beewatch.V2Event
	(*Response)(nil),  // 5: beewatch.Response
}
var file_beewatch_proto_depIdxs = []int32{
	3, // 0: beewatch.Event.v1:type_name -> beewatch.V1Event
	4, // 1: beewatch.Event.v2:type_name -> beewatch.V2Event
	0, // 2: beewatch.V1Event.type:type_name -> beewatch.V1Event.Type
	1, // 3: beewatch.V2Event.type:type_name -> beewatch.V2Event.Type
	2, // 4: beewatch.Subscriber.ReceiveEvents:input_type -> beewatch.Event
	5, // 5: beewatch.Subscriber.ReceiveEvents:output_type -> beewatch.Response
	5, // [5:6] is the sub-list for method output_type
	4, // [4:5] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
}

func init() { file_beewatch_proto_init() }
func file_beewatch_proto_init() {
	if File_beewatch_proto != nil {
		return
	}
	file_beewatch_proto_msgTypes[0].OneofWrappers = []any{
		(*event_V1)(nil),
		(*event_V2)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_beewatch_proto_rawDesc,
			NumEnums:      2,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   1,
		},
		GoTypes:           file_beewatch_proto_goTypes,
		DependencyIndexes: file_beewatch_proto_depIdxs,
		EnumInfos:         file_beewatch_proto_enumTypes,
		MessageInfos:      file_beewatch_proto_msgTypes,
	}.Build()
	File_beewatch_proto = out.File
	file_beewatch_proto_rawDesc = nil
	file_beewatch_proto_goTypes = nil
	file_beewatch_proto_depIdxs = nil
}
