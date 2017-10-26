// Code generated by protoc-gen-gogo.
// source: importpb.proto
// DO NOT EDIT!

/*
	Package importpb is a generated protocol buffer package.

	It is generated from these files:
		importpb.proto

	It has these top-level messages:
		SSTMeta
		SSTHandle
		UploadSSTRequest
		UploadSSTResponse
*/
package importpb

import (
	"fmt"
	"io"
	"math"

	proto "github.com/golang/protobuf/proto"

	metapb "github.com/pingcap/kvproto/pkg/metapb"

	context "golang.org/x/net/context"

	grpc "google.golang.org/grpc"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.ProtoPackageIsVersion2 // please upgrade the proto package

type SSTMeta struct {
	// The size of the file.
	Len uint64 `protobuf:"varint,1,opt,name=len,proto3" json:"len,omitempty"`
	// The CRC32 checksum of the file.
	Crc32 uint32 `protobuf:"varint,2,opt,name=crc32,proto3" json:"crc32,omitempty"`
	// The handle which identifies the file.
	Handle *SSTHandle `protobuf:"bytes,3,opt,name=handle" json:"handle,omitempty"`
}

func (m *SSTMeta) Reset()                    { *m = SSTMeta{} }
func (m *SSTMeta) String() string            { return proto.CompactTextString(m) }
func (*SSTMeta) ProtoMessage()               {}
func (*SSTMeta) Descriptor() ([]byte, []int) { return fileDescriptorImportpb, []int{0} }

func (m *SSTMeta) GetLen() uint64 {
	if m != nil {
		return m.Len
	}
	return 0
}

func (m *SSTMeta) GetCrc32() uint32 {
	if m != nil {
		return m.Crc32
	}
	return 0
}

func (m *SSTMeta) GetHandle() *SSTHandle {
	if m != nil {
		return m.Handle
	}
	return nil
}

type SSTHandle struct {
	// The UUID of the file, to distinguish files with the same data.
	Uuid []byte `protobuf:"bytes,1,opt,name=uuid,proto3" json:"uuid,omitempty"`
	// The CF that this file will be ingested to.
	CfName string `protobuf:"bytes,2,opt,name=cf_name,json=cfName,proto3" json:"cf_name,omitempty"`
	// The region that this file will be ingested to.
	RegionId uint64 `protobuf:"varint,3,opt,name=region_id,json=regionId,proto3" json:"region_id,omitempty"`
	// The epoch of the region when this file is uploaded.
	RegionEpoch *metapb.RegionEpoch `protobuf:"bytes,4,opt,name=region_epoch,json=regionEpoch" json:"region_epoch,omitempty"`
}

func (m *SSTHandle) Reset()                    { *m = SSTHandle{} }
func (m *SSTHandle) String() string            { return proto.CompactTextString(m) }
func (*SSTHandle) ProtoMessage()               {}
func (*SSTHandle) Descriptor() ([]byte, []int) { return fileDescriptorImportpb, []int{1} }

func (m *SSTHandle) GetUuid() []byte {
	if m != nil {
		return m.Uuid
	}
	return nil
}

func (m *SSTHandle) GetCfName() string {
	if m != nil {
		return m.CfName
	}
	return ""
}

func (m *SSTHandle) GetRegionId() uint64 {
	if m != nil {
		return m.RegionId
	}
	return 0
}

func (m *SSTHandle) GetRegionEpoch() *metapb.RegionEpoch {
	if m != nil {
		return m.RegionEpoch
	}
	return nil
}

type UploadSSTRequest struct {
	Meta *SSTMeta `protobuf:"bytes,1,opt,name=meta" json:"meta,omitempty"`
	Data []byte   `protobuf:"bytes,2,opt,name=data,proto3" json:"data,omitempty"`
}

func (m *UploadSSTRequest) Reset()                    { *m = UploadSSTRequest{} }
func (m *UploadSSTRequest) String() string            { return proto.CompactTextString(m) }
func (*UploadSSTRequest) ProtoMessage()               {}
func (*UploadSSTRequest) Descriptor() ([]byte, []int) { return fileDescriptorImportpb, []int{2} }

func (m *UploadSSTRequest) GetMeta() *SSTMeta {
	if m != nil {
		return m.Meta
	}
	return nil
}

func (m *UploadSSTRequest) GetData() []byte {
	if m != nil {
		return m.Data
	}
	return nil
}

type UploadSSTResponse struct {
}

func (m *UploadSSTResponse) Reset()                    { *m = UploadSSTResponse{} }
func (m *UploadSSTResponse) String() string            { return proto.CompactTextString(m) }
func (*UploadSSTResponse) ProtoMessage()               {}
func (*UploadSSTResponse) Descriptor() ([]byte, []int) { return fileDescriptorImportpb, []int{3} }

func init() {
	proto.RegisterType((*SSTMeta)(nil), "importpb.SSTMeta")
	proto.RegisterType((*SSTHandle)(nil), "importpb.SSTHandle")
	proto.RegisterType((*UploadSSTRequest)(nil), "importpb.UploadSSTRequest")
	proto.RegisterType((*UploadSSTResponse)(nil), "importpb.UploadSSTResponse")
}

// Reference imports to suppress errors if they are not otherwise used.
var _ context.Context
var _ grpc.ClientConn

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
const _ = grpc.SupportPackageIsVersion4

// Client API for Import service

type ImportClient interface {
	UploadSST(ctx context.Context, opts ...grpc.CallOption) (Import_UploadSSTClient, error)
}

type importClient struct {
	cc *grpc.ClientConn
}

func NewImportClient(cc *grpc.ClientConn) ImportClient {
	return &importClient{cc}
}

func (c *importClient) UploadSST(ctx context.Context, opts ...grpc.CallOption) (Import_UploadSSTClient, error) {
	stream, err := grpc.NewClientStream(ctx, &_Import_serviceDesc.Streams[0], c.cc, "/importpb.Import/UploadSST", opts...)
	if err != nil {
		return nil, err
	}
	x := &importUploadSSTClient{stream}
	return x, nil
}

type Import_UploadSSTClient interface {
	Send(*UploadSSTRequest) error
	CloseAndRecv() (*UploadSSTResponse, error)
	grpc.ClientStream
}

type importUploadSSTClient struct {
	grpc.ClientStream
}

func (x *importUploadSSTClient) Send(m *UploadSSTRequest) error {
	return x.ClientStream.SendMsg(m)
}

func (x *importUploadSSTClient) CloseAndRecv() (*UploadSSTResponse, error) {
	if err := x.ClientStream.CloseSend(); err != nil {
		return nil, err
	}
	m := new(UploadSSTResponse)
	if err := x.ClientStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

// Server API for Import service

type ImportServer interface {
	UploadSST(Import_UploadSSTServer) error
}

func RegisterImportServer(s *grpc.Server, srv ImportServer) {
	s.RegisterService(&_Import_serviceDesc, srv)
}

func _Import_UploadSST_Handler(srv interface{}, stream grpc.ServerStream) error {
	return srv.(ImportServer).UploadSST(&importUploadSSTServer{stream})
}

type Import_UploadSSTServer interface {
	SendAndClose(*UploadSSTResponse) error
	Recv() (*UploadSSTRequest, error)
	grpc.ServerStream
}

type importUploadSSTServer struct {
	grpc.ServerStream
}

func (x *importUploadSSTServer) SendAndClose(m *UploadSSTResponse) error {
	return x.ServerStream.SendMsg(m)
}

func (x *importUploadSSTServer) Recv() (*UploadSSTRequest, error) {
	m := new(UploadSSTRequest)
	if err := x.ServerStream.RecvMsg(m); err != nil {
		return nil, err
	}
	return m, nil
}

var _Import_serviceDesc = grpc.ServiceDesc{
	ServiceName: "importpb.Import",
	HandlerType: (*ImportServer)(nil),
	Methods:     []grpc.MethodDesc{},
	Streams: []grpc.StreamDesc{
		{
			StreamName:    "UploadSST",
			Handler:       _Import_UploadSST_Handler,
			ClientStreams: true,
		},
	},
	Metadata: "importpb.proto",
}

func (m *SSTMeta) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *SSTMeta) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Len != 0 {
		dAtA[i] = 0x8
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(m.Len))
	}
	if m.Crc32 != 0 {
		dAtA[i] = 0x10
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(m.Crc32))
	}
	if m.Handle != nil {
		dAtA[i] = 0x1a
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(m.Handle.Size()))
		n1, err := m.Handle.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n1
	}
	return i, nil
}

func (m *SSTHandle) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *SSTHandle) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if len(m.Uuid) > 0 {
		dAtA[i] = 0xa
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(len(m.Uuid)))
		i += copy(dAtA[i:], m.Uuid)
	}
	if len(m.CfName) > 0 {
		dAtA[i] = 0x12
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(len(m.CfName)))
		i += copy(dAtA[i:], m.CfName)
	}
	if m.RegionId != 0 {
		dAtA[i] = 0x18
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(m.RegionId))
	}
	if m.RegionEpoch != nil {
		dAtA[i] = 0x22
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(m.RegionEpoch.Size()))
		n2, err := m.RegionEpoch.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n2
	}
	return i, nil
}

func (m *UploadSSTRequest) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *UploadSSTRequest) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	if m.Meta != nil {
		dAtA[i] = 0xa
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(m.Meta.Size()))
		n3, err := m.Meta.MarshalTo(dAtA[i:])
		if err != nil {
			return 0, err
		}
		i += n3
	}
	if len(m.Data) > 0 {
		dAtA[i] = 0x12
		i++
		i = encodeVarintImportpb(dAtA, i, uint64(len(m.Data)))
		i += copy(dAtA[i:], m.Data)
	}
	return i, nil
}

func (m *UploadSSTResponse) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalTo(dAtA)
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *UploadSSTResponse) MarshalTo(dAtA []byte) (int, error) {
	var i int
	_ = i
	var l int
	_ = l
	return i, nil
}

func encodeFixed64Importpb(dAtA []byte, offset int, v uint64) int {
	dAtA[offset] = uint8(v)
	dAtA[offset+1] = uint8(v >> 8)
	dAtA[offset+2] = uint8(v >> 16)
	dAtA[offset+3] = uint8(v >> 24)
	dAtA[offset+4] = uint8(v >> 32)
	dAtA[offset+5] = uint8(v >> 40)
	dAtA[offset+6] = uint8(v >> 48)
	dAtA[offset+7] = uint8(v >> 56)
	return offset + 8
}
func encodeFixed32Importpb(dAtA []byte, offset int, v uint32) int {
	dAtA[offset] = uint8(v)
	dAtA[offset+1] = uint8(v >> 8)
	dAtA[offset+2] = uint8(v >> 16)
	dAtA[offset+3] = uint8(v >> 24)
	return offset + 4
}
func encodeVarintImportpb(dAtA []byte, offset int, v uint64) int {
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return offset + 1
}
func (m *SSTMeta) Size() (n int) {
	var l int
	_ = l
	if m.Len != 0 {
		n += 1 + sovImportpb(uint64(m.Len))
	}
	if m.Crc32 != 0 {
		n += 1 + sovImportpb(uint64(m.Crc32))
	}
	if m.Handle != nil {
		l = m.Handle.Size()
		n += 1 + l + sovImportpb(uint64(l))
	}
	return n
}

func (m *SSTHandle) Size() (n int) {
	var l int
	_ = l
	l = len(m.Uuid)
	if l > 0 {
		n += 1 + l + sovImportpb(uint64(l))
	}
	l = len(m.CfName)
	if l > 0 {
		n += 1 + l + sovImportpb(uint64(l))
	}
	if m.RegionId != 0 {
		n += 1 + sovImportpb(uint64(m.RegionId))
	}
	if m.RegionEpoch != nil {
		l = m.RegionEpoch.Size()
		n += 1 + l + sovImportpb(uint64(l))
	}
	return n
}

func (m *UploadSSTRequest) Size() (n int) {
	var l int
	_ = l
	if m.Meta != nil {
		l = m.Meta.Size()
		n += 1 + l + sovImportpb(uint64(l))
	}
	l = len(m.Data)
	if l > 0 {
		n += 1 + l + sovImportpb(uint64(l))
	}
	return n
}

func (m *UploadSSTResponse) Size() (n int) {
	var l int
	_ = l
	return n
}

func sovImportpb(x uint64) (n int) {
	for {
		n++
		x >>= 7
		if x == 0 {
			break
		}
	}
	return n
}
func sozImportpb(x uint64) (n int) {
	return sovImportpb(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *SSTMeta) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowImportpb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: SSTMeta: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: SSTMeta: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Len", wireType)
			}
			m.Len = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Len |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field Crc32", wireType)
			}
			m.Crc32 = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.Crc32 |= (uint32(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Handle", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthImportpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Handle == nil {
				m.Handle = &SSTHandle{}
			}
			if err := m.Handle.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipImportpb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthImportpb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *SSTHandle) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowImportpb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: SSTHandle: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: SSTHandle: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Uuid", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthImportpb
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Uuid = append(m.Uuid[:0], dAtA[iNdEx:postIndex]...)
			if m.Uuid == nil {
				m.Uuid = []byte{}
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field CfName", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthImportpb
			}
			postIndex := iNdEx + intStringLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.CfName = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 3:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field RegionId", wireType)
			}
			m.RegionId = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.RegionId |= (uint64(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		case 4:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field RegionEpoch", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthImportpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.RegionEpoch == nil {
				m.RegionEpoch = &metapb.RegionEpoch{}
			}
			if err := m.RegionEpoch.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipImportpb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthImportpb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *UploadSSTRequest) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowImportpb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: UploadSSTRequest: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: UploadSSTRequest: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Meta", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthImportpb
			}
			postIndex := iNdEx + msglen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if m.Meta == nil {
				m.Meta = &SSTMeta{}
			}
			if err := m.Meta.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Data", wireType)
			}
			var byteLen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				byteLen |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if byteLen < 0 {
				return ErrInvalidLengthImportpb
			}
			postIndex := iNdEx + byteLen
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Data = append(m.Data[:0], dAtA[iNdEx:postIndex]...)
			if m.Data == nil {
				m.Data = []byte{}
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipImportpb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthImportpb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *UploadSSTResponse) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowImportpb
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: UploadSSTResponse: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: UploadSSTResponse: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		default:
			iNdEx = preIndex
			skippy, err := skipImportpb(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if skippy < 0 {
				return ErrInvalidLengthImportpb
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipImportpb(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowImportpb
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
			return iNdEx, nil
		case 1:
			iNdEx += 8
			return iNdEx, nil
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowImportpb
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			iNdEx += length
			if length < 0 {
				return 0, ErrInvalidLengthImportpb
			}
			return iNdEx, nil
		case 3:
			for {
				var innerWire uint64
				var start int = iNdEx
				for shift := uint(0); ; shift += 7 {
					if shift >= 64 {
						return 0, ErrIntOverflowImportpb
					}
					if iNdEx >= l {
						return 0, io.ErrUnexpectedEOF
					}
					b := dAtA[iNdEx]
					iNdEx++
					innerWire |= (uint64(b) & 0x7F) << shift
					if b < 0x80 {
						break
					}
				}
				innerWireType := int(innerWire & 0x7)
				if innerWireType == 4 {
					break
				}
				next, err := skipImportpb(dAtA[start:])
				if err != nil {
					return 0, err
				}
				iNdEx = start + next
			}
			return iNdEx, nil
		case 4:
			return iNdEx, nil
		case 5:
			iNdEx += 4
			return iNdEx, nil
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
	}
	panic("unreachable")
}

var (
	ErrInvalidLengthImportpb = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowImportpb   = fmt.Errorf("proto: integer overflow")
)

func init() { proto.RegisterFile("importpb.proto", fileDescriptorImportpb) }

var fileDescriptorImportpb = []byte{
	// 357 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x6c, 0x91, 0xc1, 0x6a, 0xf2, 0x40,
	0x14, 0x85, 0x9d, 0xdf, 0xfc, 0xd1, 0x5c, 0xd3, 0xa2, 0xa3, 0xd0, 0x10, 0x21, 0x48, 0xa0, 0x10,
	0x5a, 0x48, 0x21, 0x42, 0x1f, 0xa0, 0x50, 0xd0, 0x85, 0x5d, 0x4c, 0xec, 0xb6, 0x32, 0x26, 0x63,
	0x0c, 0x9a, 0xcc, 0x34, 0x46, 0xdf, 0xa2, 0xfb, 0x3e, 0x52, 0x97, 0x7d, 0x84, 0x62, 0x5f, 0xa4,
	0x64, 0x12, 0xb5, 0x2d, 0x5d, 0xe5, 0xdc, 0xf3, 0x25, 0xe7, 0x9e, 0xc9, 0xc0, 0x79, 0x9c, 0x08,
	0x9e, 0xe5, 0x62, 0xee, 0x8a, 0x8c, 0xe7, 0x1c, 0x37, 0x0f, 0xb3, 0xa9, 0x27, 0x2c, 0xa7, 0x07,
	0xdf, 0xec, 0x45, 0x3c, 0xe2, 0x52, 0xde, 0x14, 0xaa, 0x74, 0xed, 0x27, 0x68, 0xf8, 0xfe, 0x74,
	0xc2, 0x72, 0x8a, 0xdb, 0x50, 0x5f, 0xb3, 0xd4, 0x40, 0x03, 0xe4, 0x28, 0xa4, 0x90, 0xb8, 0x07,
	0xff, 0x83, 0x2c, 0x18, 0x7a, 0xc6, 0xbf, 0x01, 0x72, 0xce, 0x48, 0x39, 0xe0, 0x6b, 0x50, 0x97,
	0x34, 0x0d, 0xd7, 0xcc, 0xa8, 0x0f, 0x90, 0xd3, 0xf2, 0xba, 0xee, 0xb1, 0x81, 0xef, 0x4f, 0x47,
	0x12, 0x91, 0xea, 0x15, 0xfb, 0x05, 0x81, 0x76, 0x74, 0x31, 0x06, 0x65, 0xbb, 0x8d, 0x43, 0xb9,
	0x43, 0x27, 0x52, 0xe3, 0x0b, 0x68, 0x04, 0x8b, 0x59, 0x4a, 0x13, 0x26, 0xd7, 0x68, 0x44, 0x0d,
	0x16, 0x0f, 0x34, 0x61, 0xb8, 0x0f, 0x5a, 0xc6, 0xa2, 0x98, 0xa7, 0xb3, 0x38, 0x94, 0xab, 0x14,
	0xd2, 0x2c, 0x8d, 0x71, 0x88, 0x6f, 0x41, 0xaf, 0x20, 0x13, 0x3c, 0x58, 0x1a, 0x4a, 0x55, 0xa5,
	0x3a, 0x32, 0x91, 0xec, 0xbe, 0x40, 0xa4, 0x95, 0x9d, 0x06, 0x7b, 0x02, 0xed, 0x47, 0xb1, 0xe6,
	0x34, 0xf4, 0xfd, 0x29, 0x61, 0xcf, 0x5b, 0xb6, 0xc9, 0xf1, 0x25, 0x28, 0xc5, 0x67, 0xb2, 0x55,
	0xcb, 0xeb, 0xfc, 0x38, 0x4e, 0xf1, 0x67, 0x88, 0xc4, 0x45, 0xf9, 0x90, 0xe6, 0x54, 0xb6, 0xd4,
	0x89, 0xd4, 0x76, 0x17, 0x3a, 0xdf, 0xe2, 0x36, 0x82, 0xa7, 0x1b, 0xe6, 0x11, 0x50, 0xc7, 0x32,
	0x02, 0x8f, 0x40, 0x3b, 0x62, 0x6c, 0x9e, 0x82, 0x7f, 0x57, 0x30, 0xfb, 0x7f, 0xb2, 0x32, 0xcf,
	0xae, 0x39, 0xe8, 0xee, 0xea, 0x6d, 0x6f, 0xa1, 0xf7, 0xbd, 0x85, 0x3e, 0xf6, 0x16, 0x7a, 0xfd,
	0xb4, 0x6a, 0x60, 0x04, 0x3c, 0x71, 0x45, 0x9c, 0x46, 0x01, 0x15, 0x6e, 0x1e, 0xaf, 0x76, 0xee,
	0x6a, 0x27, 0xef, 0x74, 0xae, 0xca, 0xc7, 0xf0, 0x2b, 0x00, 0x00, 0xff, 0xff, 0xcf, 0x5b, 0xa0,
	0xf9, 0x1a, 0x02, 0x00, 0x00,
}