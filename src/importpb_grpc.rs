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

const METHOD_IMPORT_KV_WRITE: ::grpcio::Method<super::importpb::WriteRequest, super::importpb::WriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/importpb.ImportKV/Write",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_FLUSH: ::grpcio::Method<super::importpb::FlushRequest, super::importpb::FlushResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/importpb.ImportKV/Flush",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_IMPORT: ::grpcio::Method<super::importpb::ImportRequest, super::importpb::ImportResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/importpb.ImportKV/Import",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_KV_CLEANUP: ::grpcio::Method<super::importpb::CleanupRequest, super::importpb::CleanupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/importpb.ImportKV/Cleanup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ImportKvClient {
    client: ::grpcio::Client,
}

impl ImportKvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImportKvClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_opt(&self, opt: ::grpcio::CallOption) -> (::grpcio::ClientCStreamSender<super::importpb::WriteRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::WriteResponse>) {
        self.client.client_streaming(&METHOD_IMPORT_KV_WRITE, opt)
    }

    pub fn write(&self) -> (::grpcio::ClientCStreamSender<super::importpb::WriteRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::WriteResponse>) {
        self.write_opt(::grpcio::CallOption::default())
    }

    pub fn flush_opt(&self, req: super::importpb::FlushRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::importpb::FlushResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_FLUSH, req, opt)
    }

    pub fn flush(&self, req: super::importpb::FlushRequest) -> ::grpcio::Result<super::importpb::FlushResponse> {
        self.flush_opt(req, ::grpcio::CallOption::default())
    }

    pub fn flush_async_opt(&self, req: super::importpb::FlushRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::importpb::FlushResponse> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_FLUSH, req, opt)
    }

    pub fn flush_async(&self, req: super::importpb::FlushRequest) -> ::grpcio::ClientUnaryReceiver<super::importpb::FlushResponse> {
        self.flush_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_opt(&self, req: super::importpb::ImportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::importpb::ImportResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_IMPORT, req, opt)
    }

    pub fn import(&self, req: super::importpb::ImportRequest) -> ::grpcio::Result<super::importpb::ImportResponse> {
        self.import_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_async_opt(&self, req: super::importpb::ImportRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::importpb::ImportResponse> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_IMPORT, req, opt)
    }

    pub fn import_async(&self, req: super::importpb::ImportRequest) -> ::grpcio::ClientUnaryReceiver<super::importpb::ImportResponse> {
        self.import_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cleanup_opt(&self, req: super::importpb::CleanupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::importpb::CleanupResponse> {
        self.client.unary_call(&METHOD_IMPORT_KV_CLEANUP, req, opt)
    }

    pub fn cleanup(&self, req: super::importpb::CleanupRequest) -> ::grpcio::Result<super::importpb::CleanupResponse> {
        self.cleanup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cleanup_async_opt(&self, req: super::importpb::CleanupRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::importpb::CleanupResponse> {
        self.client.unary_call_async(&METHOD_IMPORT_KV_CLEANUP, req, opt)
    }

    pub fn cleanup_async(&self, req: super::importpb::CleanupRequest) -> ::grpcio::ClientUnaryReceiver<super::importpb::CleanupResponse> {
        self.cleanup_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ImportKv {
    fn write(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::importpb::WriteRequest>, sink: ::grpcio::ClientStreamingSink<super::importpb::WriteResponse>);
    fn flush(&self, ctx: ::grpcio::RpcContext, req: super::importpb::FlushRequest, sink: ::grpcio::UnarySink<super::importpb::FlushResponse>);
    fn import(&self, ctx: ::grpcio::RpcContext, req: super::importpb::ImportRequest, sink: ::grpcio::UnarySink<super::importpb::ImportResponse>);
    fn cleanup(&self, ctx: ::grpcio::RpcContext, req: super::importpb::CleanupRequest, sink: ::grpcio::UnarySink<super::importpb::CleanupResponse>);
}

pub fn create_import_kv<S: ImportKv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_IMPORT_KV_WRITE, move |ctx, req, resp| {
        instance.write(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_FLUSH, move |ctx, req, resp| {
        instance.flush(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_IMPORT, move |ctx, req, resp| {
        instance.import(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_KV_CLEANUP, move |ctx, req, resp| {
        instance.cleanup(ctx, req, resp)
    });
    builder.build()
}

const METHOD_IMPORT_SST_UPLOAD: ::grpcio::Method<super::importpb::UploadRequest, super::importpb::UploadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/importpb.ImportSST/Upload",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_SST_INGEST: ::grpcio::Method<super::importpb::IngestRequest, super::importpb::IngestResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/importpb.ImportSST/Ingest",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ImportSstClient {
    client: ::grpcio::Client,
}

impl ImportSstClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImportSstClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn upload_opt(&self, opt: ::grpcio::CallOption) -> (::grpcio::ClientCStreamSender<super::importpb::UploadRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::UploadResponse>) {
        self.client.client_streaming(&METHOD_IMPORT_SST_UPLOAD, opt)
    }

    pub fn upload(&self) -> (::grpcio::ClientCStreamSender<super::importpb::UploadRequest>, ::grpcio::ClientCStreamReceiver<super::importpb::UploadResponse>) {
        self.upload_opt(::grpcio::CallOption::default())
    }

    pub fn ingest_opt(&self, req: super::importpb::IngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::importpb::IngestResponse> {
        self.client.unary_call(&METHOD_IMPORT_SST_INGEST, req, opt)
    }

    pub fn ingest(&self, req: super::importpb::IngestRequest) -> ::grpcio::Result<super::importpb::IngestResponse> {
        self.ingest_opt(req, ::grpcio::CallOption::default())
    }

    pub fn ingest_async_opt(&self, req: super::importpb::IngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::importpb::IngestResponse> {
        self.client.unary_call_async(&METHOD_IMPORT_SST_INGEST, req, opt)
    }

    pub fn ingest_async(&self, req: super::importpb::IngestRequest) -> ::grpcio::ClientUnaryReceiver<super::importpb::IngestResponse> {
        self.ingest_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ImportSst {
    fn upload(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::importpb::UploadRequest>, sink: ::grpcio::ClientStreamingSink<super::importpb::UploadResponse>);
    fn ingest(&self, ctx: ::grpcio::RpcContext, req: super::importpb::IngestRequest, sink: ::grpcio::UnarySink<super::importpb::IngestResponse>);
}

pub fn create_import_sst<S: ImportSst + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_IMPORT_SST_UPLOAD, move |ctx, req, resp| {
        instance.upload(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_SST_INGEST, move |ctx, req, resp| {
        instance.ingest(ctx, req, resp)
    });
    builder.build()
}
