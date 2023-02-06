#!/bin/bash
cargo build
export TARGET=target/cxxbridge/frontend-client-cxx/src
cp $TARGET/lib.rs.h frontend-client-cxx/cxx/frontend_client_cxx_generated.h
cp $TARGET/lib.rs.cc frontend-client-cxx/cxx/frontend_client_cxx_generated.cc
sh ../scripts/format_code.sh
