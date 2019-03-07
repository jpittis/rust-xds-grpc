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

const METHOD_ROUTE_DISCOVERY_SERVICE_STREAM_ROUTES: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.api.v2.RouteDiscoveryService/StreamRoutes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTE_DISCOVERY_SERVICE_INCREMENTAL_ROUTES: ::grpcio::Method<super::discovery::IncrementalDiscoveryRequest, super::discovery::IncrementalDiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.api.v2.RouteDiscoveryService/IncrementalRoutes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ROUTE_DISCOVERY_SERVICE_FETCH_ROUTES: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/envoy.api.v2.RouteDiscoveryService/FetchRoutes",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RouteDiscoveryServiceClient {
    client: ::grpcio::Client,
}

impl RouteDiscoveryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RouteDiscoveryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stream_routes_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.client.duplex_streaming(&METHOD_ROUTE_DISCOVERY_SERVICE_STREAM_ROUTES, opt)
    }

    pub fn stream_routes(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.stream_routes_opt(::grpcio::CallOption::default())
    }

    pub fn incremental_routes_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::IncrementalDiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::IncrementalDiscoveryResponse>)> {
        self.client.duplex_streaming(&METHOD_ROUTE_DISCOVERY_SERVICE_INCREMENTAL_ROUTES, opt)
    }

    pub fn incremental_routes(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::IncrementalDiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::IncrementalDiscoveryResponse>)> {
        self.incremental_routes_opt(::grpcio::CallOption::default())
    }

    pub fn fetch_routes_opt(&self, req: &super::discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::discovery::DiscoveryResponse> {
        self.client.unary_call(&METHOD_ROUTE_DISCOVERY_SERVICE_FETCH_ROUTES, req, opt)
    }

    pub fn fetch_routes(&self, req: &super::discovery::DiscoveryRequest) -> ::grpcio::Result<super::discovery::DiscoveryResponse> {
        self.fetch_routes_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fetch_routes_async_opt(&self, req: &super::discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::discovery::DiscoveryResponse>> {
        self.client.unary_call_async(&METHOD_ROUTE_DISCOVERY_SERVICE_FETCH_ROUTES, req, opt)
    }

    pub fn fetch_routes_async(&self, req: &super::discovery::DiscoveryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::discovery::DiscoveryResponse>> {
        self.fetch_routes_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait RouteDiscoveryService {
    fn stream_routes(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::discovery::DiscoveryRequest>, sink: ::grpcio::DuplexSink<super::discovery::DiscoveryResponse>);
    fn incremental_routes(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::discovery::IncrementalDiscoveryRequest>, sink: ::grpcio::DuplexSink<super::discovery::IncrementalDiscoveryResponse>);
    fn fetch_routes(&mut self, ctx: ::grpcio::RpcContext, req: super::discovery::DiscoveryRequest, sink: ::grpcio::UnarySink<super::discovery::DiscoveryResponse>);
}

pub fn create_route_discovery_service<S: RouteDiscoveryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ROUTE_DISCOVERY_SERVICE_STREAM_ROUTES, move |ctx, req, resp| {
        instance.stream_routes(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_ROUTE_DISCOVERY_SERVICE_INCREMENTAL_ROUTES, move |ctx, req, resp| {
        instance.incremental_routes(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ROUTE_DISCOVERY_SERVICE_FETCH_ROUTES, move |ctx, req, resp| {
        instance.fetch_routes(ctx, req, resp)
    });
    builder.build()
}
