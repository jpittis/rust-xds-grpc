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

const METHOD_AGGREGATED_DISCOVERY_SERVICE_STREAM_AGGREGATED_RESOURCES: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.service.discovery.v2.AggregatedDiscoveryService/StreamAggregatedResources",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_AGGREGATED_DISCOVERY_SERVICE_INCREMENTAL_AGGREGATED_RESOURCES: ::grpcio::Method<super::discovery::IncrementalDiscoveryRequest, super::discovery::IncrementalDiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.service.discovery.v2.AggregatedDiscoveryService/IncrementalAggregatedResources",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AggregatedDiscoveryServiceClient {
    client: ::grpcio::Client,
}

impl AggregatedDiscoveryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AggregatedDiscoveryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stream_aggregated_resources_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.client.duplex_streaming(&METHOD_AGGREGATED_DISCOVERY_SERVICE_STREAM_AGGREGATED_RESOURCES, opt)
    }

    pub fn stream_aggregated_resources(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.stream_aggregated_resources_opt(::grpcio::CallOption::default())
    }

    pub fn incremental_aggregated_resources_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::IncrementalDiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::IncrementalDiscoveryResponse>)> {
        self.client.duplex_streaming(&METHOD_AGGREGATED_DISCOVERY_SERVICE_INCREMENTAL_AGGREGATED_RESOURCES, opt)
    }

    pub fn incremental_aggregated_resources(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::IncrementalDiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::IncrementalDiscoveryResponse>)> {
        self.incremental_aggregated_resources_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AggregatedDiscoveryService {
    fn stream_aggregated_resources(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::discovery::DiscoveryRequest>, sink: ::grpcio::DuplexSink<super::discovery::DiscoveryResponse>);
    fn incremental_aggregated_resources(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::discovery::IncrementalDiscoveryRequest>, sink: ::grpcio::DuplexSink<super::discovery::IncrementalDiscoveryResponse>);
}

pub fn create_aggregated_discovery_service<S: AggregatedDiscoveryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_AGGREGATED_DISCOVERY_SERVICE_STREAM_AGGREGATED_RESOURCES, move |ctx, req, resp| {
        instance.stream_aggregated_resources(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_AGGREGATED_DISCOVERY_SERVICE_INCREMENTAL_AGGREGATED_RESOURCES, move |ctx, req, resp| {
        instance.incremental_aggregated_resources(ctx, req, resp)
    });
    builder.build()
}
