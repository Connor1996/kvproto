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

const METHOD_IMPORT_WRITE: ::grpcio::Method<super::import::WriteRequest, super::import::WriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/importpb.Import/Write",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMPORT_FLUSH: ::grpcio::Method<super::import::FlushRequest, super::import::FlushResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/importpb.Import/Flush",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct ImportClient {
    client: ::grpcio::Client,
}

impl ImportClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImportClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn write_opt(&self, opt: ::grpcio::CallOption) -> (::grpcio::ClientCStreamSender<super::import::WriteRequest>, ::grpcio::ClientCStreamReceiver<super::import::WriteResponse>) {
        self.client.client_streaming(&METHOD_IMPORT_WRITE, opt)
    }

    pub fn write(&self) -> (::grpcio::ClientCStreamSender<super::import::WriteRequest>, ::grpcio::ClientCStreamReceiver<super::import::WriteResponse>) {
        self.write_opt(::grpcio::CallOption::default())
    }

    pub fn flush_opt(&self, req: super::import::FlushRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::import::FlushResponse> {
        self.client.unary_call(&METHOD_IMPORT_FLUSH, req, opt)
    }

    pub fn flush(&self, req: super::import::FlushRequest) -> ::grpcio::Result<super::import::FlushResponse> {
        self.flush_opt(req, ::grpcio::CallOption::default())
    }

    pub fn flush_async_opt(&self, req: super::import::FlushRequest, opt: ::grpcio::CallOption) -> ::grpcio::ClientUnaryReceiver<super::import::FlushResponse> {
        self.client.unary_call_async(&METHOD_IMPORT_FLUSH, req, opt)
    }

    pub fn flush_async(&self, req: super::import::FlushRequest) -> ::grpcio::ClientUnaryReceiver<super::import::FlushResponse> {
        self.flush_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Import {
    fn write(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::import::WriteRequest>, sink: ::grpcio::ClientStreamingSink<super::import::WriteResponse>);
    fn flush(&self, ctx: ::grpcio::RpcContext, req: super::import::FlushRequest, sink: ::grpcio::UnarySink<super::import::FlushResponse>);
}

pub fn create_import<S: Import + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_IMPORT_WRITE, move |ctx, req, resp| {
        instance.write(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMPORT_FLUSH, move |ctx, req, resp| {
        instance.flush(ctx, req, resp)
    });
    builder.build()
}
