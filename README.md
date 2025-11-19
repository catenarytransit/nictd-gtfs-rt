# nictd-gtfs-rt

Alerts for NICTD (South Shore Line).

The rest of the GTFS-RT data should currently be obtained from ETA SPOT. However, the data is a bit nonstandard, and might not work in your app.

* https://s3.amazonaws.com/etatransit.gtfs/southshore.etaspot.net/position_updates.pb
* https://s3.amazonaws.com/etatransit.gtfs/southshore.etaspot.net/trip_updates.pb

## Contributing

### Dependencies

[Install Rust](https://www.rust-lang.org/tools/install). If Rust is already installed, use `rustup update` to update.

Make sure the OpenSSL headers and the Protobuf compiler (`protoc`) are installed. For example:

```bash
# Debian
sudo apt install libssl-dev protobuf-compiler
# openSUSE
sudo zypper in libopenssl-3-devel protobuf-devel
```

### Test data

Download the CTA GTFS timetable and extract it to static/:

```bash
mkdir -p static/
cd static
wget https://www.transitchicago.com/downloads/sch_data/google_transit.zip
unzip google_transit.zip
cd ..
```

### Testing

Then you can test the library using

```bash
cargo test
```

If you want to see the output on `stdout`, use

```bash
cargo test -- --nocapture
```

## Contact

Please join us on discord to help us work on this!!! 

https://discord.gg/yVV6dguwtq

### More information

This package uses both https://docs.rs/gtfs-realtime and https://docs.rs/gtfs-structures