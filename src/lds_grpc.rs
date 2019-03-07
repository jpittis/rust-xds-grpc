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

const METHOD_LISTENER_DISCOVERY_SERVICE_STREAM_LISTENERS: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.api.v2.ListenerDiscoveryService/StreamListeners",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_LISTENER_DISCOVERY_SERVICE_FETCH_LISTENERS: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/envoy.api.v2.ListenerDiscoveryService/FetchListeners",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ListenerDiscoveryServiceClient {
    client: ::grpcio::Client,
}

impl ListenerDiscoveryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ListenerDiscoveryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stream_listeners_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.client.duplex_streaming(&METHOD_LISTENER_DISCOVERY_SERVICE_STREAM_LISTENERS, opt)
    }

    pub fn stream_listeners(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.stream_listeners_opt(::grpcio::CallOption::default())
    }

    pub fn fetch_listeners_opt(&self, req: &super::discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::discovery::DiscoveryResponse> {
        self.client.unary_call(&METHOD_LISTENER_DISCOVERY_SERVICE_FETCH_LISTENERS, req, opt)
    }

    pub fn fetch_listeners(&self, req: &super::discovery::DiscoveryRequest) -> ::grpcio::Result<super::discovery::DiscoveryResponse> {
        self.fetch_listeners_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fetch_listeners_async_opt(&self, req: &super::discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::discovery::DiscoveryResponse>> {
        self.client.unary_call_async(&METHOD_LISTENER_DISCOVERY_SERVICE_FETCH_LISTENERS, req, opt)
    }

    pub fn fetch_listeners_async(&self, req: &super::discovery::DiscoveryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::discovery::DiscoveryResponse>> {
        self.fetch_listeners_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ListenerDiscoveryService {
    fn stream_listeners(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::discovery::DiscoveryRequest>, sink: ::grpcio::DuplexSink<super::discovery::DiscoveryResponse>);
    fn fetch_listeners(&mut self, ctx: ::grpcio::RpcContext, req: super::discovery::DiscoveryRequest, sink: ::grpcio::UnarySink<super::discovery::DiscoveryResponse>);
}

pub fn create_listener_discovery_service<S: ListenerDiscoveryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_LISTENER_DISCOVERY_SERVICE_STREAM_LISTENERS, move |ctx, req, resp| {
        instance.stream_listeners(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_LISTENER_DISCOVERY_SERVICE_FETCH_LISTENERS, move |ctx, req, resp| {
        instance.fetch_listeners(ctx, req, resp)
    });
    builder.build()
}
