# n2k-test

Test binary for the combined message parser from n2k-codegen, and the n2k CAN library.

1) Download `pgns.xml` from https://github.com/canboat/canboat/blob/master/analyzer/pgns.xml
1) To re-generate the `messages` folder with the NMEA2000 decoding messages, run the following from the `n2k-codegen` directory, passing any PGNs you wish to generate parsers for:

        RUST_LOG=info cargo run -- --pgns-xml pgns.xml -o ../n2k-test/src/messages  -p 127505 -p 127506 -p 130314 -p 60928 -p 59904 -p 126996 -p 127510  -p 127237 -p 130312 -p 130314 -p 130316 -p 130306 -p 127250 -p 127251 -p 127257 -p 129025 -p 127245 -p 65359 -p 130919

    This will generate the `messages` module inside the this binary with the appropriate parsers.

3. From this directory, run
        
        RUST_LOG=info cargo run

    to parse the included candump file using the generated `n2k-messages` crate.