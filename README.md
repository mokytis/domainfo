# domainfo

Show information about a Domain Name or IP Address.

## Install

If you have cargo installed, you can simply run `cargo
install domainfo`, or use `cargo build --release` and add
`./target/release/netinfo` to somewhere in your path
(probably `/usr/local/bin`).

## Usage

Simply run `domainfo <query>` where `query` is a Domain Name, or IP Address.

    $ domainfo 8.8.8.8
    8.8.8.8 (8.8.8.0/24) is advertised by Google LLC (AS15169 ARIN US)

    $ domainfo google.com
    google.com has IPv4 142.250.187.238
    142.250.187.238 (142.250.0.0/15) is advertised by Google LLC (AS15169 ARIN US)

    $ domainfo github.com
    github.com has IPv4 140.82.121.4
    140.82.121.4 (140.82.121.0/24) is advertised by GitHub, Inc. (AS36459 ARIN US)

    $ domainfo www.gov.uk
    www.gov.uk has IPv4 151.101.0.144
    151.101.0.144 (151.101.0.0/22) is advertised by Fastly  (AS54113 ARIN US)

    www.gov.uk has IPv4 151.101.64.144
    151.101.64.144 (151.101.64.0/22) is advertised by Fastly  (AS54113 ARIN US)

    www.gov.uk has IPv4 151.101.128.144
    151.101.128.144 (151.101.128.0/22) is advertised by Fastly  (AS54113 ARIN US)

    www.gov.uk has IPv4 151.101.192.144
    151.101.192.144 (151.101.192.0/22) is advertised by Fastly  (AS54113 ARIN US)
