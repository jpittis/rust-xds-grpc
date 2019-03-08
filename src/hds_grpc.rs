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

const METHOD_HEALTH_DISCOVERY_SERVICE_STREAM_HEALTH_CHECK: ::grpcio::Method<super::hds::HealthCheckRequestOrEndpointHealthResponse, super::hds::HealthCheckSpecifier> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.service.discovery.v2.HealthDiscoveryService/StreamHealthCheck",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_HEALTH_DISCOVERY_SERVICE_FETCH_HEALTH_CHECK: ::grpcio::Method<super::hds::HealthCheckRequestOrEndpointHealthResponse, super::hds::HealthCheckSpecifier> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/envoy.service.discovery.v2.HealthDiscoveryService/FetchHealthCheck",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct HealthDiscoveryServiceClient {
    client: ::grpcio::Client,
}

impl HealthDiscoveryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        HealthDiscoveryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stream_health_check_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::hds::HealthCheckRequestOrEndpointHealthResponse>, ::grpcio::ClientDuplexReceiver<super::hds::HealthCheckSpecifier>)> {
        self.client.duplex_streaming(&METHOD_HEALTH_DISCOVERY_SERVICE_STREAM_HEALTH_CHECK, opt)
    }

    pub fn stream_health_check(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::hds::HealthCheckRequestOrEndpointHealthResponse>, ::grpcio::ClientDuplexReceiver<super::hds::HealthCheckSpecifier>)> {
        self.stream_health_check_opt(::grpcio::CallOption::default())
    }

    pub fn fetch_health_check_opt(&self, req: &super::hds::HealthCheckRequestOrEndpointHealthResponse, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::hds::HealthCheckSpecifier> {
        self.client.unary_call(&METHOD_HEALTH_DISCOVERY_SERVICE_FETCH_HEALTH_CHECK, req, opt)
    }

    pub fn fetch_health_check(&self, req: &super::hds::HealthCheckRequestOrEndpointHealthResponse) -> ::grpcio::Result<super::hds::HealthCheckSpecifier> {
        self.fetch_health_check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fetch_health_check_async_opt(&self, req: &super::hds::HealthCheckRequestOrEndpointHealthResponse, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hds::HealthCheckSpecifier>> {
        self.client.unary_call_async(&METHOD_HEALTH_DISCOVERY_SERVICE_FETCH_HEALTH_CHECK, req, opt)
    }

    pub fn fetch_health_check_async(&self, req: &super::hds::HealthCheckRequestOrEndpointHealthResponse) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::hds::HealthCheckSpecifier>> {
        self.fetch_health_check_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait HealthDiscoveryService {
    fn stream_health_check(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::hds::HealthCheckRequestOrEndpointHealthResponse>, sink: ::grpcio::DuplexSink<super::hds::HealthCheckSpecifier>);
    fn fetch_health_check(&mut self, ctx: ::grpcio::RpcContext, req: super::hds::HealthCheckRequestOrEndpointHealthResponse, sink: ::grpcio::UnarySink<super::hds::HealthCheckSpecifier>);
}

pub fn create_health_discovery_service<S: HealthDiscoveryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_HEALTH_DISCOVERY_SERVICE_STREAM_HEALTH_CHECK, move |ctx, req, resp| {
        instance.stream_health_check(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_HEALTH_DISCOVERY_SERVICE_FETCH_HEALTH_CHECK, move |ctx, req, resp| {
        instance.fetch_health_check(ctx, req, resp)
    });
    builder.build()
}
