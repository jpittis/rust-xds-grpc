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

const METHOD_ENDPOINT_DISCOVERY_SERVICE_STREAM_ENDPOINTS: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.api.v2.EndpointDiscoveryService/StreamEndpoints",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ENDPOINT_DISCOVERY_SERVICE_FETCH_ENDPOINTS: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/envoy.api.v2.EndpointDiscoveryService/FetchEndpoints",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct EndpointDiscoveryServiceClient {
    client: ::grpcio::Client,
}

impl EndpointDiscoveryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        EndpointDiscoveryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stream_endpoints_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.client.duplex_streaming(&METHOD_ENDPOINT_DISCOVERY_SERVICE_STREAM_ENDPOINTS, opt)
    }

    pub fn stream_endpoints(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.stream_endpoints_opt(::grpcio::CallOption::default())
    }

    pub fn fetch_endpoints_opt(&self, req: &super::discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::discovery::DiscoveryResponse> {
        self.client.unary_call(&METHOD_ENDPOINT_DISCOVERY_SERVICE_FETCH_ENDPOINTS, req, opt)
    }

    pub fn fetch_endpoints(&self, req: &super::discovery::DiscoveryRequest) -> ::grpcio::Result<super::discovery::DiscoveryResponse> {
        self.fetch_endpoints_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fetch_endpoints_async_opt(&self, req: &super::discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::discovery::DiscoveryResponse>> {
        self.client.unary_call_async(&METHOD_ENDPOINT_DISCOVERY_SERVICE_FETCH_ENDPOINTS, req, opt)
    }

    pub fn fetch_endpoints_async(&self, req: &super::discovery::DiscoveryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::discovery::DiscoveryResponse>> {
        self.fetch_endpoints_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait EndpointDiscoveryService {
    fn stream_endpoints(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::discovery::DiscoveryRequest>, sink: ::grpcio::DuplexSink<super::discovery::DiscoveryResponse>);
    fn fetch_endpoints(&mut self, ctx: ::grpcio::RpcContext, req: super::discovery::DiscoveryRequest, sink: ::grpcio::UnarySink<super::discovery::DiscoveryResponse>);
}

pub fn create_endpoint_discovery_service<S: EndpointDiscoveryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ENDPOINT_DISCOVERY_SERVICE_STREAM_ENDPOINTS, move |ctx, req, resp| {
        instance.stream_endpoints(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ENDPOINT_DISCOVERY_SERVICE_FETCH_ENDPOINTS, move |ctx, req, resp| {
        instance.fetch_endpoints(ctx, req, resp)
    });
    builder.build()
}
