# web_kvm
REST service controller for the rack mountable TESmart 8port HDMI enterprise grade KVM (TES-HKS0801A1U-USBK)

* TESmart 8x1 HDMI KVM switch (TES-HKS0801A1U-USBK)
  - 3840*2160@60Hz 4:4:4
  - HDR10 and DolbyVision
  - HDCP 2.2
  - https://www.newegg.com/p/1DJ-00RG-00030

* 4 Space (4U) wooden studio equipment rack (STR4U)
  - https://www.audiorax.com/audiorax-4-space-straight-rack

* US‑16‑XG - fully managed, 16‑port, 10G fiber switch
  - https://www.ui.com/unifi-switching/unifi-switch-16-xg/

* Pyle USA Multi-Outlet Surge Protect Power Supply (PCO865)
  - https://pyleusa.com/products/pco865?variant=32917634187299


```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --git https://github.com/gwihlidal/web_kvm --branch main
RUST_LOG=info web_kvm
```

```
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
```

* https://unifi-network.ui.com/#unifi
* https://www.ui.com/download/unifi/default/default/unifi-network-controller-6041-windows
* https://hub.docker.com/r/linuxserver/unifi-controller


