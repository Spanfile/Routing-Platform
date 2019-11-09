# Routing-Platform

[![Build Status](https://api.travis-ci.org/Spanfile/Routing-Platform.svg?branch=master)](https://travis-ci.org/Spanfile/Routing-Platform)

A software suite to configure and control a Linux-based router appliance. Built with Rust.

## Features

* [[Under work](crates/rp_schema)] Configuration schema
* [[Under work](crates/rp_config)] Configuration management
* [[Under work](crates/rp_shell)] Configuration editor shell
* [Planned] Schema live extension (plugins)
* [Planned] Configuration API
* [Planned] Web configuration app (interacts with configuration API)
* [Planned] Mobile configuration app (interacts with configuration API)
* [Planned] FRR interaction for static and dynamic routing
* [Planned] nftables interaction for firewalling, NAT, and other networking features
* [Planned] Kea/ISC DHCP server interaction for running a DHCP server
* [Planned] PowerDNS Recursor interaction for running a recursive/forwarding DNS resolver
* [Planned] Custom Linux distro

## Building

Build normally with `cargo build`. Requires a recent nightly build of Rust and the following external packages (these are what they're on Ubuntu, other distros might use different names):

* `libnftnl-dev`
* `libmnl-dev`