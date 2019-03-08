git clone https://github.com/envoyproxy/data-plane-api.git
git clone https://github.com/googleapis/googleapis.git
git clone https://github.com/lyft/protoc-gen-validate.git

protoc \
  --rust_out=../src --grpc_out=../src \
  --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
  -Igoogleapis \
  -Idata-plane-api \
  -Iprotoc-gen-validate \
  data-plane-api/envoy/type/*.proto \
  data-plane-api/envoy/api/v2/core/*.proto \
  data-plane-api/envoy/api/v2/cluster/*.proto \
  data-plane-api/envoy/api/v2/endpoint/*.proto \
  data-plane-api/envoy/api/v2/listener/*.proto \
  data-plane-api/envoy/api/v2/route/*.proto \
  data-plane-api/envoy/api/v2/auth/*.proto \
  data-plane-api/envoy/api/v2/*.proto \
  data-plane-api/envoy/service/discovery/v2/*.proto \
  googleapis/google/api/http.proto \
  googleapis/google/rpc/status.proto \
  googleapis/google/api/annotations.proto \
  protoc-gen-validate/gogoproto/gogo.proto \
  protoc-gen-validate/validate/validate.proto
