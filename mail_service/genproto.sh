#!/bin/bash 
# [START gke_emailservice_genproto]

python3 -m grpc_tools.protoc -I../../protos --python_out=. --pyi_out=. --grpc_python_out=. ../../protos/demo.proto

