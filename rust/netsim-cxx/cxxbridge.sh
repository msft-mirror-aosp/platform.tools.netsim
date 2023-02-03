#!/bin/bash

cargo build
export TARGET=target/cxxbridge/netsim-cxx/src
cp $TARGET/lib.rs.h netsim-cxx/cxx/netsim_cxx_generated.h
cp $TARGET/lib.rs.cc netsim-cxx/cxx/netsim_cxx_generated.cc
sh ../scripts/format_code.sh
