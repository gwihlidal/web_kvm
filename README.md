# web_kvm
REST service controller for the rack mountable TESmart 8x8 HDMI matrix switcher hardware

https://www.tesmart.com/collections/hdmi-matrix/products/tesmart-8-inputs-8-outputs-hdmi-matrix-switcher-4k-30hz-8x8-hdmi-matrix-switch-rack-mountable-tcp-ip-rs-232-and-ir-control

https://www.audiorax.com/audiorax-4-space-straight-rack

https://www.ui.com/unifi-switching/unifi-switch-16-xg/

https://pyleusa.com/products/pco865?variant=32917634187299

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/gwihlidal/web_kvm --branch main

RUST_LOG=info web_kvm

curl -X GET http://localhost:5577/current

curl -X POST http://localhost:5577/switch/1
curl -X POST http://localhost:5577/switch/2
curl -X POST http://localhost:5577/switch/3
curl -X POST http://localhost:5577/switch/4
curl -X POST http://localhost:5577/switch/5
curl -X POST http://localhost:5577/switch/6
curl -X POST http://localhost:5577/switch/7
curl -X POST http://localhost:5577/switch/8

curl -X POST http://localhost:5577/beeping_mode/0
curl -X POST http://localhost:5577/beeping_mode/1

curl -X POST http://localhost:5577/detect_mode/0
curl -X POST http://localhost:5577/detect_mode/1

curl -X POST http://localhost:5577/led_mode/0
curl -X POST http://localhost:5577/led_mode/1
curl -X POST http://localhost:5577/led_mode/2


