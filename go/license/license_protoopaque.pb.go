// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.36.2
// 	protoc        v5.29.2
// source: license.proto

//go:build protoopaque

package license

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	timestamppb "google.golang.org/protobuf/types/known/timestamppb"
	reflect "reflect"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

// The three types of result a certificate verification can produce.
type VerifyResult int32

const (
	VerifyResult_VERIFY_UNSPECIFIED VerifyResult = 0
	VerifyResult_VERIFY_ERROR       VerifyResult = 1
	VerifyResult_VERIFY_VALID       VerifyResult = 2
	VerifyResult_VERIFY_INVALID     VerifyResult = 3
)

// Enum value maps for VerifyResult.
var (
	VerifyResult_name = map[int32]string{
		0: "VERIFY_UNSPECIFIED",
		1: "VERIFY_ERROR",
		2: "VERIFY_VALID",
		3: "VERIFY_INVALID",
	}
	VerifyResult_value = map[string]int32{
		"VERIFY_UNSPECIFIED": 0,
		"VERIFY_ERROR":       1,
		"VERIFY_VALID":       2,
		"VERIFY_INVALID":     3,
	}
)

func (x VerifyResult) Enum() *VerifyResult {
	p := new(VerifyResult)
	*p = x
	return p
}

func (x VerifyResult) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (VerifyResult) Descriptor() protoreflect.EnumDescriptor {
	return file_license_proto_enumTypes[0].Descriptor()
}

func (VerifyResult) Type() protoreflect.EnumType {
	return &file_license_proto_enumTypes[0]
}

func (x VerifyResult) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// The different types of certificates used in BeeGFS license generation and verification. At this
// point, only Customer and Partner certificates are passed through protocol buffers.
type CertType int32

const (
	CertType_CERT_TYPE_UNSPECIFIED     CertType = 0
	CertType_CERT_TYPE_CA_ROOT         CertType = 1
	CertType_CERT_TYPE_CA_INTERMEDIATE CertType = 2
	CertType_CERT_TYPE_PARTNER         CertType = 3
	CertType_CERT_TYPE_CUSTOMER        CertType = 4
	CertType_CERT_TYPE_TEMPORARY       CertType = 5
)

// Enum value maps for CertType.
var (
	CertType_name = map[int32]string{
		0: "CERT_TYPE_UNSPECIFIED",
		1: "CERT_TYPE_CA_ROOT",
		2: "CERT_TYPE_CA_INTERMEDIATE",
		3: "CERT_TYPE_PARTNER",
		4: "CERT_TYPE_CUSTOMER",
		5: "CERT_TYPE_TEMPORARY",
	}
	CertType_value = map[string]int32{
		"CERT_TYPE_UNSPECIFIED":     0,
		"CERT_TYPE_CA_ROOT":         1,
		"CERT_TYPE_CA_INTERMEDIATE": 2,
		"CERT_TYPE_PARTNER":         3,
		"CERT_TYPE_CUSTOMER":        4,
		"CERT_TYPE_TEMPORARY":       5,
	}
)

func (x CertType) Enum() *CertType {
	p := new(CertType)
	*p = x
	return p
}

func (x CertType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (CertType) Descriptor() protoreflect.EnumDescriptor {
	return file_license_proto_enumTypes[1].Descriptor()
}

func (CertType) Type() protoreflect.EnumType {
	return &file_license_proto_enumTypes[1]
}

func (x CertType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// The result of a certificate verfication operation.
type VerifyCertResult struct {
	state              protoimpl.MessageState `protogen:"opaque.v1"`
	xxx_hidden_Result  VerifyResult           `protobuf:"varint,1,opt,name=result,proto3,enum=license.VerifyResult" json:"result,omitempty"`
	xxx_hidden_Serial  string                 `protobuf:"bytes,2,opt,name=serial,proto3" json:"serial,omitempty"`
	xxx_hidden_Message string                 `protobuf:"bytes,3,opt,name=message,proto3" json:"message,omitempty"`
	unknownFields      protoimpl.UnknownFields
	sizeCache          protoimpl.SizeCache
}

func (x *VerifyCertResult) Reset() {
	*x = VerifyCertResult{}
	mi := &file_license_proto_msgTypes[0]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *VerifyCertResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VerifyCertResult) ProtoMessage() {}

func (x *VerifyCertResult) ProtoReflect() protoreflect.Message {
	mi := &file_license_proto_msgTypes[0]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (x *VerifyCertResult) GetResult() VerifyResult {
	if x != nil {
		return x.xxx_hidden_Result
	}
	return VerifyResult_VERIFY_UNSPECIFIED
}

func (x *VerifyCertResult) GetSerial() string {
	if x != nil {
		return x.xxx_hidden_Serial
	}
	return ""
}

func (x *VerifyCertResult) GetMessage() string {
	if x != nil {
		return x.xxx_hidden_Message
	}
	return ""
}

func (x *VerifyCertResult) SetResult(v VerifyResult) {
	x.xxx_hidden_Result = v
}

func (x *VerifyCertResult) SetSerial(v string) {
	x.xxx_hidden_Serial = v
}

func (x *VerifyCertResult) SetMessage(v string) {
	x.xxx_hidden_Message = v
}

type VerifyCertResult_builder struct {
	_ [0]func() // Prevents comparability and use of unkeyed literals for the builder.

	Result VerifyResult
	// The certificates string serial. Empty if verification failed.
	Serial string
	// Error or status message. Empty if verification succeeded. Contains the error message if
	// result == VERIFY_ERROR and the reason for verification failure if result == VERIFY_INVALID.
	Message string
}

func (b0 VerifyCertResult_builder) Build() *VerifyCertResult {
	m0 := &VerifyCertResult{}
	b, x := &b0, m0
	_, _ = b, x
	x.xxx_hidden_Result = b.Result
	x.xxx_hidden_Serial = b.Serial
	x.xxx_hidden_Message = b.Message
	return m0
}

// The result of a feature verification operation.
type VerifyFeatureResult struct {
	state              protoimpl.MessageState `protogen:"opaque.v1"`
	xxx_hidden_Result  VerifyResult           `protobuf:"varint,1,opt,name=result,proto3,enum=license.VerifyResult" json:"result,omitempty"`
	xxx_hidden_Message string                 `protobuf:"bytes,2,opt,name=message,proto3" json:"message,omitempty"`
	unknownFields      protoimpl.UnknownFields
	sizeCache          protoimpl.SizeCache
}

func (x *VerifyFeatureResult) Reset() {
	*x = VerifyFeatureResult{}
	mi := &file_license_proto_msgTypes[1]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *VerifyFeatureResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*VerifyFeatureResult) ProtoMessage() {}

func (x *VerifyFeatureResult) ProtoReflect() protoreflect.Message {
	mi := &file_license_proto_msgTypes[1]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (x *VerifyFeatureResult) GetResult() VerifyResult {
	if x != nil {
		return x.xxx_hidden_Result
	}
	return VerifyResult_VERIFY_UNSPECIFIED
}

func (x *VerifyFeatureResult) GetMessage() string {
	if x != nil {
		return x.xxx_hidden_Message
	}
	return ""
}

func (x *VerifyFeatureResult) SetResult(v VerifyResult) {
	x.xxx_hidden_Result = v
}

func (x *VerifyFeatureResult) SetMessage(v string) {
	x.xxx_hidden_Message = v
}

type VerifyFeatureResult_builder struct {
	_ [0]func() // Prevents comparability and use of unkeyed literals for the builder.

	Result VerifyResult
	// Error or status message. Empty if verification succeeded. Contains the error message if
	// result == VERIFY_ERROR and the reason for verification failure if result == VERIFY_INVALID.
	Message string
}

func (b0 VerifyFeatureResult_builder) Build() *VerifyFeatureResult {
	m0 := &VerifyFeatureResult{}
	b, x := &b0, m0
	_, _ = b, x
	x.xxx_hidden_Result = b.Result
	x.xxx_hidden_Message = b.Message
	return m0
}

// The result of an operation that fetches the data of the currently loaded certificate.
type GetCertDataResult struct {
	state              protoimpl.MessageState `protogen:"opaque.v1"`
	xxx_hidden_Result  VerifyResult           `protobuf:"varint,1,opt,name=result,proto3,enum=license.VerifyResult" json:"result,omitempty"`
	xxx_hidden_Data    *CertData              `protobuf:"bytes,2,opt,name=data,proto3" json:"data,omitempty"`
	xxx_hidden_Message string                 `protobuf:"bytes,3,opt,name=message,proto3" json:"message,omitempty"`
	unknownFields      protoimpl.UnknownFields
	sizeCache          protoimpl.SizeCache
}

func (x *GetCertDataResult) Reset() {
	*x = GetCertDataResult{}
	mi := &file_license_proto_msgTypes[2]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *GetCertDataResult) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*GetCertDataResult) ProtoMessage() {}

func (x *GetCertDataResult) ProtoReflect() protoreflect.Message {
	mi := &file_license_proto_msgTypes[2]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (x *GetCertDataResult) GetResult() VerifyResult {
	if x != nil {
		return x.xxx_hidden_Result
	}
	return VerifyResult_VERIFY_UNSPECIFIED
}

func (x *GetCertDataResult) GetData() *CertData {
	if x != nil {
		return x.xxx_hidden_Data
	}
	return nil
}

func (x *GetCertDataResult) GetMessage() string {
	if x != nil {
		return x.xxx_hidden_Message
	}
	return ""
}

func (x *GetCertDataResult) SetResult(v VerifyResult) {
	x.xxx_hidden_Result = v
}

func (x *GetCertDataResult) SetData(v *CertData) {
	x.xxx_hidden_Data = v
}

func (x *GetCertDataResult) SetMessage(v string) {
	x.xxx_hidden_Message = v
}

func (x *GetCertDataResult) HasData() bool {
	if x == nil {
		return false
	}
	return x.xxx_hidden_Data != nil
}

func (x *GetCertDataResult) ClearData() {
	x.xxx_hidden_Data = nil
}

type GetCertDataResult_builder struct {
	_ [0]func() // Prevents comparability and use of unkeyed literals for the builder.

	Result VerifyResult
	// The data of the certificate currently loaded in the library. Data is present even if
	// verification failed.
	Data *CertData
	// Error or status message. Empty if verification succeeded. Contains the error message if
	// result == VERIFY_ERROR and the reason for verification failure if result == VERIFY_INVALID.
	Message string
}

func (b0 GetCertDataResult_builder) Build() *GetCertDataResult {
	m0 := &GetCertDataResult{}
	b, x := &b0, m0
	_, _ = b, x
	x.xxx_hidden_Result = b.Result
	x.xxx_hidden_Data = b.Data
	x.xxx_hidden_Message = b.Message
	return m0
}

// CertData is a simplified version of a Go x509.certificate that contains a subset of its
// information and some additional information relevant to BeeGFS licenses like the certificate
// type.
type CertData struct {
	state                         protoimpl.MessageState `protogen:"opaque.v1"`
	xxx_hidden_Type               CertType               `protobuf:"varint,1,opt,name=type,proto3,enum=license.CertType" json:"type,omitempty"`
	xxx_hidden_Serial             int64                  `protobuf:"varint,2,opt,name=serial,proto3" json:"serial,omitempty"`
	xxx_hidden_Organization       string                 `protobuf:"bytes,3,opt,name=organization,proto3" json:"organization,omitempty"`
	xxx_hidden_OrganizationalUnit string                 `protobuf:"bytes,4,opt,name=organizational_unit,json=organizationalUnit,proto3" json:"organizational_unit,omitempty"`
	xxx_hidden_Country            string                 `protobuf:"bytes,5,opt,name=country,proto3" json:"country,omitempty"`
	xxx_hidden_Locality           string                 `protobuf:"bytes,6,opt,name=locality,proto3" json:"locality,omitempty"`
	xxx_hidden_CommonName         string                 `protobuf:"bytes,7,opt,name=common_name,json=commonName,proto3" json:"common_name,omitempty"`
	xxx_hidden_SubjectSerial      string                 `protobuf:"bytes,8,opt,name=subject_serial,json=subjectSerial,proto3" json:"subject_serial,omitempty"`
	xxx_hidden_ValidFrom          *timestamppb.Timestamp `protobuf:"bytes,9,opt,name=valid_from,json=validFrom,proto3" json:"valid_from,omitempty"`
	xxx_hidden_ValidUntil         *timestamppb.Timestamp `protobuf:"bytes,10,opt,name=valid_until,json=validUntil,proto3" json:"valid_until,omitempty"`
	xxx_hidden_DnsNames           []string               `protobuf:"bytes,11,rep,name=dns_names,json=DNSNames,proto3" json:"dns_names,omitempty"`
	xxx_hidden_IsCa               bool                   `protobuf:"varint,12,opt,name=is_ca,json=isCa,proto3" json:"is_ca,omitempty"`
	xxx_hidden_ParentData         *CertData              `protobuf:"bytes,13,opt,name=parent_data,json=parentData,proto3,oneof" json:"parent_data,omitempty"`
	unknownFields                 protoimpl.UnknownFields
	sizeCache                     protoimpl.SizeCache
}

func (x *CertData) Reset() {
	*x = CertData{}
	mi := &file_license_proto_msgTypes[3]
	ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
	ms.StoreMessageInfo(mi)
}

func (x *CertData) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*CertData) ProtoMessage() {}

func (x *CertData) ProtoReflect() protoreflect.Message {
	mi := &file_license_proto_msgTypes[3]
	if x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

func (x *CertData) GetType() CertType {
	if x != nil {
		return x.xxx_hidden_Type
	}
	return CertType_CERT_TYPE_UNSPECIFIED
}

func (x *CertData) GetSerial() int64 {
	if x != nil {
		return x.xxx_hidden_Serial
	}
	return 0
}

func (x *CertData) GetOrganization() string {
	if x != nil {
		return x.xxx_hidden_Organization
	}
	return ""
}

func (x *CertData) GetOrganizationalUnit() string {
	if x != nil {
		return x.xxx_hidden_OrganizationalUnit
	}
	return ""
}

func (x *CertData) GetCountry() string {
	if x != nil {
		return x.xxx_hidden_Country
	}
	return ""
}

func (x *CertData) GetLocality() string {
	if x != nil {
		return x.xxx_hidden_Locality
	}
	return ""
}

func (x *CertData) GetCommonName() string {
	if x != nil {
		return x.xxx_hidden_CommonName
	}
	return ""
}

func (x *CertData) GetSubjectSerial() string {
	if x != nil {
		return x.xxx_hidden_SubjectSerial
	}
	return ""
}

func (x *CertData) GetValidFrom() *timestamppb.Timestamp {
	if x != nil {
		return x.xxx_hidden_ValidFrom
	}
	return nil
}

func (x *CertData) GetValidUntil() *timestamppb.Timestamp {
	if x != nil {
		return x.xxx_hidden_ValidUntil
	}
	return nil
}

func (x *CertData) GetDnsNames() []string {
	if x != nil {
		return x.xxx_hidden_DnsNames
	}
	return nil
}

func (x *CertData) GetIsCa() bool {
	if x != nil {
		return x.xxx_hidden_IsCa
	}
	return false
}

func (x *CertData) GetParentData() *CertData {
	if x != nil {
		return x.xxx_hidden_ParentData
	}
	return nil
}

func (x *CertData) SetType(v CertType) {
	x.xxx_hidden_Type = v
}

func (x *CertData) SetSerial(v int64) {
	x.xxx_hidden_Serial = v
}

func (x *CertData) SetOrganization(v string) {
	x.xxx_hidden_Organization = v
}

func (x *CertData) SetOrganizationalUnit(v string) {
	x.xxx_hidden_OrganizationalUnit = v
}

func (x *CertData) SetCountry(v string) {
	x.xxx_hidden_Country = v
}

func (x *CertData) SetLocality(v string) {
	x.xxx_hidden_Locality = v
}

func (x *CertData) SetCommonName(v string) {
	x.xxx_hidden_CommonName = v
}

func (x *CertData) SetSubjectSerial(v string) {
	x.xxx_hidden_SubjectSerial = v
}

func (x *CertData) SetValidFrom(v *timestamppb.Timestamp) {
	x.xxx_hidden_ValidFrom = v
}

func (x *CertData) SetValidUntil(v *timestamppb.Timestamp) {
	x.xxx_hidden_ValidUntil = v
}

func (x *CertData) SetDnsNames(v []string) {
	x.xxx_hidden_DnsNames = v
}

func (x *CertData) SetIsCa(v bool) {
	x.xxx_hidden_IsCa = v
}

func (x *CertData) SetParentData(v *CertData) {
	x.xxx_hidden_ParentData = v
}

func (x *CertData) HasValidFrom() bool {
	if x == nil {
		return false
	}
	return x.xxx_hidden_ValidFrom != nil
}

func (x *CertData) HasValidUntil() bool {
	if x == nil {
		return false
	}
	return x.xxx_hidden_ValidUntil != nil
}

func (x *CertData) HasParentData() bool {
	if x == nil {
		return false
	}
	return x.xxx_hidden_ParentData != nil
}

func (x *CertData) ClearValidFrom() {
	x.xxx_hidden_ValidFrom = nil
}

func (x *CertData) ClearValidUntil() {
	x.xxx_hidden_ValidUntil = nil
}

func (x *CertData) ClearParentData() {
	x.xxx_hidden_ParentData = nil
}

type CertData_builder struct {
	_ [0]func() // Prevents comparability and use of unkeyed literals for the builder.

	// Certificate type. Encoded in the subject SerialNumber and CommonName of the x509 certificate
	// together with the account ID.
	Type CertType
	// Random int64 x509 serial number that uniquely identifies the certificate
	Serial int64
	// Fields 3-8 contain an x509 certificate subject's attributes
	Organization       string
	OrganizationalUnit string
	Country            string
	Locality           string
	// String serial number that contains the type (P-, SP-, ...) and the partner account ID or
	// support contract ID
	CommonName string
	// Can contain a different string serial number (for example a customer ID). It doesn't carry any
	// semantic meaning for now.
	SubjectSerial string
	// Fields 8 and 9 encode the certificate's validity period
	ValidFrom  *timestamppb.Timestamp
	ValidUntil *timestamppb.Timestamp
	// The DNS names the certificate is valid for. BeeGFS license certificates encode licensed
	// features as DNS names, e.g. "io.beegfs.mirroring" or "io.beegfs.numservers.4"
	DnsNames []string
	// Indicates whether the certificate is part of a CA. Always false for customer and partner
	// certificates.
	IsCa bool
	// The certificate's parent or "Issuer" certificate
	ParentData *CertData
}

func (b0 CertData_builder) Build() *CertData {
	m0 := &CertData{}
	b, x := &b0, m0
	_, _ = b, x
	x.xxx_hidden_Type = b.Type
	x.xxx_hidden_Serial = b.Serial
	x.xxx_hidden_Organization = b.Organization
	x.xxx_hidden_OrganizationalUnit = b.OrganizationalUnit
	x.xxx_hidden_Country = b.Country
	x.xxx_hidden_Locality = b.Locality
	x.xxx_hidden_CommonName = b.CommonName
	x.xxx_hidden_SubjectSerial = b.SubjectSerial
	x.xxx_hidden_ValidFrom = b.ValidFrom
	x.xxx_hidden_ValidUntil = b.ValidUntil
	x.xxx_hidden_DnsNames = b.DnsNames
	x.xxx_hidden_IsCa = b.IsCa
	x.xxx_hidden_ParentData = b.ParentData
	return m0
}

var File_license_proto protoreflect.FileDescriptor

var file_license_proto_rawDesc = []byte{
	0x0a, 0x0d, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
	0x07, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
	0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74,
	0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x73, 0x0a, 0x10, 0x56, 0x65, 0x72,
	0x69, 0x66, 0x79, 0x43, 0x65, 0x72, 0x74, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x2d, 0x0a,
	0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x15, 0x2e,
	0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x56, 0x65, 0x72, 0x69, 0x66, 0x79, 0x52, 0x65,
	0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x16, 0x0a, 0x06,
	0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x65,
	0x72, 0x69, 0x61, 0x6c, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18,
	0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x5e,
	0x0a, 0x13, 0x56, 0x65, 0x72, 0x69, 0x66, 0x79, 0x46, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x52,
	0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x2d, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x15, 0x2e, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e,
	0x56, 0x65, 0x72, 0x69, 0x66, 0x79, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65,
	0x73, 0x75, 0x6c, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18,
	0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x83,
	0x01, 0x0a, 0x11, 0x47, 0x65, 0x74, 0x43, 0x65, 0x72, 0x74, 0x44, 0x61, 0x74, 0x61, 0x52, 0x65,
	0x73, 0x75, 0x6c, 0x74, 0x12, 0x2d, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01,
	0x20, 0x01, 0x28, 0x0e, 0x32, 0x15, 0x2e, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x56,
	0x65, 0x72, 0x69, 0x66, 0x79, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73,
	0x75, 0x6c, 0x74, 0x12, 0x25, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x11, 0x2e, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x43, 0x65, 0x72, 0x74,
	0x44, 0x61, 0x74, 0x61, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65,
	0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73,
	0x73, 0x61, 0x67, 0x65, 0x22, 0x8f, 0x04, 0x0a, 0x08, 0x43, 0x65, 0x72, 0x74, 0x44, 0x61, 0x74,
	0x61, 0x12, 0x25, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
	0x11, 0x2e, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x43, 0x65, 0x72, 0x74, 0x54, 0x79,
	0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x65, 0x72, 0x69,
	0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x06, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c,
	0x12, 0x22, 0x0a, 0x0c, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e,
	0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2f, 0x0a, 0x13, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61,
	0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x5f, 0x75, 0x6e, 0x69, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x12, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x61,
	0x6c, 0x55, 0x6e, 0x69, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79,
	0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x72, 0x79, 0x12,
	0x1a, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x18, 0x06, 0x20, 0x01, 0x28,
	0x09, 0x52, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x69, 0x74, 0x79, 0x12, 0x1f, 0x0a, 0x0b, 0x63,
	0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x0a, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x25, 0x0a, 0x0e,
	0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x5f, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x18, 0x08,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x53, 0x65, 0x72,
	0x69, 0x61, 0x6c, 0x12, 0x39, 0x0a, 0x0a, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x5f, 0x66, 0x72, 0x6f,
	0x6d, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74,
	0x61, 0x6d, 0x70, 0x52, 0x09, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x46, 0x72, 0x6f, 0x6d, 0x12, 0x3b,
	0x0a, 0x0b, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x5f, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x18, 0x0a, 0x20,
	0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52,
	0x0a, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x55, 0x6e, 0x74, 0x69, 0x6c, 0x12, 0x1b, 0x0a, 0x09, 0x64,
	0x6e, 0x73, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x18, 0x0b, 0x20, 0x03, 0x28, 0x09, 0x52, 0x08,
	0x44, 0x4e, 0x53, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x12, 0x13, 0x0a, 0x05, 0x69, 0x73, 0x5f, 0x63,
	0x61, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x08, 0x52, 0x04, 0x69, 0x73, 0x43, 0x61, 0x12, 0x37, 0x0a,
	0x0b, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x0d, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e, 0x43, 0x65, 0x72,
	0x74, 0x44, 0x61, 0x74, 0x61, 0x48, 0x00, 0x52, 0x0a, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x44,
	0x61, 0x74, 0x61, 0x88, 0x01, 0x01, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x70, 0x61, 0x72, 0x65, 0x6e,
	0x74, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x2a, 0x5e, 0x0a, 0x0c, 0x56, 0x65, 0x72, 0x69, 0x66, 0x79,
	0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x16, 0x0a, 0x12, 0x56, 0x45, 0x52, 0x49, 0x46, 0x59,
	0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x10,
	0x0a, 0x0c, 0x56, 0x45, 0x52, 0x49, 0x46, 0x59, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x01,
	0x12, 0x10, 0x0a, 0x0c, 0x56, 0x45, 0x52, 0x49, 0x46, 0x59, 0x5f, 0x56, 0x41, 0x4c, 0x49, 0x44,
	0x10, 0x02, 0x12, 0x12, 0x0a, 0x0e, 0x56, 0x45, 0x52, 0x49, 0x46, 0x59, 0x5f, 0x49, 0x4e, 0x56,
	0x41, 0x4c, 0x49, 0x44, 0x10, 0x03, 0x2a, 0xa3, 0x01, 0x0a, 0x08, 0x43, 0x65, 0x72, 0x74, 0x54,
	0x79, 0x70, 0x65, 0x12, 0x19, 0x0a, 0x15, 0x43, 0x45, 0x52, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45,
	0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x15,
	0x0a, 0x11, 0x43, 0x45, 0x52, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x43, 0x41, 0x5f, 0x52,
	0x4f, 0x4f, 0x54, 0x10, 0x01, 0x12, 0x1d, 0x0a, 0x19, 0x43, 0x45, 0x52, 0x54, 0x5f, 0x54, 0x59,
	0x50, 0x45, 0x5f, 0x43, 0x41, 0x5f, 0x49, 0x4e, 0x54, 0x45, 0x52, 0x4d, 0x45, 0x44, 0x49, 0x41,
	0x54, 0x45, 0x10, 0x02, 0x12, 0x15, 0x0a, 0x11, 0x43, 0x45, 0x52, 0x54, 0x5f, 0x54, 0x59, 0x50,
	0x45, 0x5f, 0x50, 0x41, 0x52, 0x54, 0x4e, 0x45, 0x52, 0x10, 0x03, 0x12, 0x16, 0x0a, 0x12, 0x43,
	0x45, 0x52, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x43, 0x55, 0x53, 0x54, 0x4f, 0x4d, 0x45,
	0x52, 0x10, 0x04, 0x12, 0x17, 0x0a, 0x13, 0x43, 0x45, 0x52, 0x54, 0x5f, 0x54, 0x59, 0x50, 0x45,
	0x5f, 0x54, 0x45, 0x4d, 0x50, 0x4f, 0x52, 0x41, 0x52, 0x59, 0x10, 0x05, 0x42, 0x2a, 0x5a, 0x28,
	0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x74, 0x68, 0x69, 0x6e, 0x6b,
	0x70, 0x61, 0x72, 0x71, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x67, 0x6f,
	0x2f, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var file_license_proto_enumTypes = make([]protoimpl.EnumInfo, 2)
var file_license_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_license_proto_goTypes = []any{
	(VerifyResult)(0),             // 0: license.VerifyResult
	(CertType)(0),                 // 1: license.CertType
	(*VerifyCertResult)(nil),      // 2: license.VerifyCertResult
	(*VerifyFeatureResult)(nil),   // 3: license.VerifyFeatureResult
	(*GetCertDataResult)(nil),     // 4: license.GetCertDataResult
	(*CertData)(nil),              // 5: license.CertData
	(*timestamppb.Timestamp)(nil), // 6: google.protobuf.Timestamp
}
var file_license_proto_depIdxs = []int32{
	0, // 0: license.VerifyCertResult.result:type_name -> license.VerifyResult
	0, // 1: license.VerifyFeatureResult.result:type_name -> license.VerifyResult
	0, // 2: license.GetCertDataResult.result:type_name -> license.VerifyResult
	5, // 3: license.GetCertDataResult.data:type_name -> license.CertData
	1, // 4: license.CertData.type:type_name -> license.CertType
	6, // 5: license.CertData.valid_from:type_name -> google.protobuf.Timestamp
	6, // 6: license.CertData.valid_until:type_name -> google.protobuf.Timestamp
	5, // 7: license.CertData.parent_data:type_name -> license.CertData
	8, // [8:8] is the sub-list for method output_type
	8, // [8:8] is the sub-list for method input_type
	8, // [8:8] is the sub-list for extension type_name
	8, // [8:8] is the sub-list for extension extendee
	0, // [0:8] is the sub-list for field type_name
}

func init() { file_license_proto_init() }
func file_license_proto_init() {
	if File_license_proto != nil {
		return
	}
	file_license_proto_msgTypes[3].OneofWrappers = []any{}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_license_proto_rawDesc,
			NumEnums:      2,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_license_proto_goTypes,
		DependencyIndexes: file_license_proto_depIdxs,
		EnumInfos:         file_license_proto_enumTypes,
		MessageInfos:      file_license_proto_msgTypes,
	}.Build()
	File_license_proto = out.File
	file_license_proto_rawDesc = nil
	file_license_proto_goTypes = nil
	file_license_proto_depIdxs = nil
}
