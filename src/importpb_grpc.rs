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

const METHOD_DUMP_SST_WRITE: ::grpcio::Method<super::importpb::WriteRequest, super::importpb::WriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/importpb.DumpSST/Write",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DUMP_SST_FLUSH: ::grpcio::Method<super::importpb::FlushRequest, super::importpb::FlushResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/importpb.DumpSST/Flush",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct DumpSstClient {
    client: ::grpcio::Client,
}

impl DumpSstClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DumpSstClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_opt(&self, opt: ::grpcio::CallOption) -> (::grpcio::ClientCStreamSender<super::importpb::WriteRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::WriteResponse>) {
        self.client.client_streaming(&METHOD_DUMP_SST_WRITE, opt)
    }

    pub fn write(&self) -> (::grpcio::ClientCStreamSender<super::importpb::WriteRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::WriteResponse>) {
        self.write_opt(::grpcio::CallOption::default())
    }

    pub fn flush_opt(&self, req: super::importpb::FlushRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::importpb::FlushResponse> {
        self.client.unary_call(&METHOD_DUMP_SST_FLUSH, req, opt)
    }

    pub fn flush(&self, req: super::importpb::FlushRequest) -> ::grpcio::Result<super::importpb::FlushResponse> {
        self.flush_opt(req, ::grpcio::CallOption::default())
    }

    pub fn flush_async_opt(&self, req: super::importpb::FlushRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::importpb::FlushResponse> {
        self.client.unary_call_async(&METHOD_DUMP_SST_FLUSH, req, opt)
    }

    pub fn flush_async(&self, req: super::importpb::FlushRequest) -> ::grpcio::ClientUnaryReceiver<super::importpb::FlushResponse> {
        self.flush_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait DumpSst {
    fn write(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::importpb::WriteRequest>, sink: ::grpcio::ClientStreamingSink<super::importpb::WriteResponse>);
    fn flush(&self, ctx: ::grpcio::RpcContext, req: super::importpb::FlushRequest, sink: ::grpcio::UnarySink<super::importpb::FlushResponse>);
}

pub fn create_dump_sst<S: DumpSst + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_DUMP_SST_WRITE, move |ctx, req, resp| {
        instance.write(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DUMP_SST_FLUSH, move |ctx, req, resp| {
        instance.flush(ctx, req, resp)
    });
    builder.build()
}

const METHOD_LOAD_SST_UPLOAD: ::grpcio::Method<super::importpb::UploadRequest, super::importpb::UploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/importpb.LoadSST/Upload",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LOAD_SST_INGEST: ::grpcio::Method<super::importpb::IngestRequest, super::importpb::IngestResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/importpb.LoadSST/Ingest",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct LoadSstClient {
    client: ::grpcio::Client,
}

impl LoadSstClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        LoadSstClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn upload_opt(&self, opt: ::grpcio::CallOption) -> (::grpcio::ClientCStreamSender<super::importpb::UploadRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::UploadResponse>) {
        self.client.client_streaming(&METHOD_LOAD_SST_UPLOAD, opt)
    }

    pub fn upload(&self) -> (::grpcio::ClientCStreamSender<super::importpb::UploadRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::UploadResponse>) {
        self.upload_opt(::grpcio::CallOption::default())
    }

    pub fn ingest_opt(&self, req: super::importpb::IngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::importpb::IngestResponse> {
        self.client.unary_call(&METHOD_LOAD_SST_INGEST, req, opt)
    }

    pub fn ingest(&self, req: super::importpb::IngestRequest) -> ::grpcio::Result<super::importpb::IngestResponse> {
        self.ingest_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ingest_async_opt(&self, req: super::importpb::IngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::importpb::IngestResponse> {
        self.client.unary_call_async(&METHOD_LOAD_SST_INGEST, req, opt)
    }

    pub fn ingest_async(&self, req: super::importpb::IngestRequest) -> ::grpcio::ClientUnaryReceiver<super::importpb::IngestResponse> {
        self.ingest_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait LoadSst {
    fn upload(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::importpb::UploadRequest>, sink: ::grpcio::ClientStreamingSink<super::importpb::UploadResponse>);
    fn ingest(&self, ctx: ::grpcio::RpcContext, req: super::importpb::IngestRequest, sink: ::grpcio::UnarySink<super::importpb::IngestResponse>);
}

pub fn create_load_sst<S: LoadSst + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_LOAD_SST_UPLOAD, move |ctx, req, resp| {
        instance.upload(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LOAD_SST_INGEST, move |ctx, req, resp| {
        instance.ingest(ctx, req, resp)
    });
    builder.build()
}
