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

const METHOD_CLUSTER_DISCOVERY_SERVICE_STREAM_CLUSTERS: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.api.v2.ClusterDiscoveryService/StreamClusters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_DISCOVERY_SERVICE_INCREMENTAL_CLUSTERS: ::grpcio::Method<super::discovery::IncrementalDiscoveryRequest, super::discovery::IncrementalDiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/envoy.api.v2.ClusterDiscoveryService/IncrementalClusters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_CLUSTER_DISCOVERY_SERVICE_FETCH_CLUSTERS: ::grpcio::Method<super::discovery::DiscoveryRequest, super::discovery::DiscoveryResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/envoy.api.v2.ClusterDiscoveryService/FetchClusters",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ClusterDiscoveryServiceClient {
    client: ::grpcio::Client,
}

impl ClusterDiscoveryServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ClusterDiscoveryServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn stream_clusters_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.client.duplex_streaming(&METHOD_CLUSTER_DISCOVERY_SERVICE_STREAM_CLUSTERS, opt)
    }

    pub fn stream_clusters(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::DiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::DiscoveryResponse>)> {
        self.stream_clusters_opt(::grpcio::CallOption::default())
    }

    pub fn incremental_clusters_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::IncrementalDiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::IncrementalDiscoveryResponse>)> {
        self.client.duplex_streaming(&METHOD_CLUSTER_DISCOVERY_SERVICE_INCREMENTAL_CLUSTERS, opt)
    }

    pub fn incremental_clusters(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::discovery::IncrementalDiscoveryRequest>, ::grpcio::ClientDuplexReceiver<super::discovery::IncrementalDiscoveryResponse>)> {
        self.incremental_clusters_opt(::grpcio::CallOption::default())
    }

    pub fn fetch_clusters_opt(&self, req: &super::discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::discovery::DiscoveryResponse> {
        self.client.unary_call(&METHOD_CLUSTER_DISCOVERY_SERVICE_FETCH_CLUSTERS, req, opt)
    }

    pub fn fetch_clusters(&self, req: &super::discovery::DiscoveryRequest) -> ::grpcio::Result<super::discovery::DiscoveryResponse> {
        self.fetch_clusters_opt(req, ::grpcio::CallOption::default())
    }

    pub fn fetch_clusters_async_opt(&self, req: &super::discovery::DiscoveryRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::discovery::DiscoveryResponse>> {
        self.client.unary_call_async(&METHOD_CLUSTER_DISCOVERY_SERVICE_FETCH_CLUSTERS, req, opt)
    }

    pub fn fetch_clusters_async(&self, req: &super::discovery::DiscoveryRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::discovery::DiscoveryResponse>> {
        self.fetch_clusters_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ClusterDiscoveryService {
    fn stream_clusters(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::discovery::DiscoveryRequest>, sink: ::grpcio::DuplexSink<super::discovery::DiscoveryResponse>);
    fn incremental_clusters(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::discovery::IncrementalDiscoveryRequest>, sink: ::grpcio::DuplexSink<super::discovery::IncrementalDiscoveryResponse>);
    fn fetch_clusters(&mut self, ctx: ::grpcio::RpcContext, req: super::discovery::DiscoveryRequest, sink: ::grpcio::UnarySink<super::discovery::DiscoveryResponse>);
}

pub fn create_cluster_discovery_service<S: ClusterDiscoveryService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_CLUSTER_DISCOVERY_SERVICE_STREAM_CLUSTERS, move |ctx, req, resp| {
        instance.stream_clusters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_CLUSTER_DISCOVERY_SERVICE_INCREMENTAL_CLUSTERS, move |ctx, req, resp| {
        instance.incremental_clusters(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CLUSTER_DISCOVERY_SERVICE_FETCH_CLUSTERS, move |ctx, req, resp| {
        instance.fetch_clusters(ctx, req, resp)
    });
    builder.build()
}
