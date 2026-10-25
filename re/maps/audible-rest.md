# Channel settings → register writes

From `05-audible-rest.pcap` (951 changes). Words are `address:value` (hex, bits 30–16 : 15–0).

## Per parameter

| Row | Channel | Parameter | Addresses | OSC value → display → words |
|---|---|---|---|---|
| input | AN 7/8 | gain | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | AN 7/8 | gainRight | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | AN 7/8 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 7/8 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 7/8 | phase | `0606` `12e6` `4006` | 1 → ? → 0606:0001 12e6:f960 12e6:f9c8 4006:5a71<br>1 → ? → 0606:0000 12e6:f960 12e6:f9c8 4006:258f |
| input | AN 7/8 | phaseRight | `0706` `13e7` `4047` | 1 → ? → 0706:0001 13e7:0648 13e7:f960 4047:5a71<br>1 → ? → 0706:0000 13e7:0648 13e7:f960 4047:258f |
| input | AN 7/8 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | AN 7/8 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 7/8 | msProc | `0605` `0705` `12e6` `12e7` `13e6` `13e7` `4006` `4007` `4046` `4047` | 1 → ? → 0605:0001 0705:0001 12e6:0008 12e6:f770 12e7:0008 12e7:f770 13e6:0008 13e6:f770 13e7:0008 13e7:f770 4006:12c7 4007:12c7 4046:12c7 4047:6d39<br>1 → ? → 0605:0000 0705:0000 12e6:f960 12e6:f9c8 12e7:0648 12e7:f960 13e6:f960 13e6:f9c8 13e7:0648 13e7:f960 4006:258f 4007:0000 4046:0000 4047:258f |
| input | AN 7/8 | width | `12e6` `12e7` `13e6` `13e7` `4006` `4007` `4046` `4047` | 0.75 → 0.50 → 12e6:f810 12e6:fc58 12e7:03b8 12e7:f810 13e6:f810 13e6:fc58 13e7:03b8 13e7:f810 4006:1c2b 4007:0963 4046:0963 4047:1c2b<br>1 → 1.00 → 12e6:f960 12e6:f9c8 12e7:0648 12e7:f960 13e6:f960 13e6:f9c8 13e7:0648 13e7:f960 4006:258f 4007:0000 4046:0000 4047:258f |
| input | AN 7/8 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 7/8 | alevEnable | `0680` `0780` | 1 → ? → 0680:0001 0780:0001<br>1 → ? → 0680:0000 0780:0000 |
| input | AN 7/8 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | AN 7/8 | volume | `12e6` `13e7` `4006` `4047` | 0.75 → -2.2 dB → 12e6:f9c8 12e6:fea0 13e7:0648 13e7:fea0 4006:8c69 4047:8c69<br>0.5264 → -10.6 dB → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f |
| input | AN 7/8 | pan | `12e6` `4006` | 0.75 → R50 → 12e6:f590 12e6:f9c8 4006:12c7<br>0.5 → 0 → 12e6:f960 12e6:f9c8 4006:258f |
| input | AN 7/8 | mute | `0600` `0700` `12e6` `13e7` `4006` `4047` | 1 → ? → 0600:0001 0700:0001 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4047:0000<br>1 → ? → 0600:0000 0700:0000 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f |
| input | AN 7/8 | solo | `4020` `4061` | 1 → ? → 4020:0000 4061:0000<br>1 → ? → 4020:9000 4061:9000 |
| input | AN 7/8 | reverbSend | `0601` `0701` | 0.75 → -16.5 dB → 0601:ff5b 0701:ff5b<br>0 → -oo → 0601:fd76 0701:fd76 |
| input | AN 7/8 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| input | SPDIF | gain | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | SPDIF | gainRight | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | SPDIF | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | SPDIF | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | SPDIF | phase | `0806` | 1 → ? → 0806:0001<br>1 → ? → 0806:0000 |
| input | SPDIF | phaseRight | `0906` | 1 → ? → 0906:0001<br>1 → ? → 0906:0000 |
| input | SPDIF | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | SPDIF | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | SPDIF | msProc | `0805` `0905` | 1 → ? → 0805:0001 0905:0001<br>1 → ? → 0805:0000 0905:0000 |
| input | SPDIF | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | SPDIF | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | SPDIF | alevEnable | `0880` `0980` | 1 → ? → 0880:0001 0980:0001<br>1 → ? → 0880:0000 0980:0000 |
| input | SPDIF | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | SPDIF | volume | `12e8` `13e9` `4008` `4049` | 0.75 → -2.2 dB → 12e8:f9c8 12e8:fea0 13e9:0648 13e9:fea0 4008:8c69 4049:8c69<br>0 → -oo → 12e8:d760 13e9:d760 4008:0000 4049:0000 |
| input | SPDIF | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | SPDIF | mute | `0800` `0900` | 1 → ? → 0800:0001 0900:0001<br>1 → ? → 0800:0000 0900:0000 |
| input | SPDIF | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | SPDIF | reverbSend | `0801` `0901` | 0.75 → -16.5 dB → 0801:ff5b 0901:ff5b<br>0 → -oo → 0801:fd76 0901:fd76 |
| input | SPDIF | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| input | AS 1/2 | gain | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | AS 1/2 | gainRight | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | AS 1/2 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AS 1/2 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AS 1/2 | phase | `0a06` | 1 → ? → 0a06:0001<br>1 → ? → 0a06:0000 |
| input | AS 1/2 | phaseRight | `0b06` | 1 → ? → 0b06:0001<br>1 → ? → 0b06:0000 |
| input | AS 1/2 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | AS 1/2 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AS 1/2 | msProc | `0a05` `0b05` | 1 → ? → 0a05:0001 0b05:0001<br>1 → ? → 0a05:0000 0b05:0000 |
| input | AS 1/2 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | AS 1/2 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AS 1/2 | alevEnable | `0a80` `0b80` | 1 → ? → 0a80:0001 0b80:0001<br>1 → ? → 0a80:0000 0b80:0000 |
| input | AS 1/2 | crossFeed | none | 0.75 → ? → (none)<br>0 → ? → (none) |
| input | AS 1/2 | volume | `12ea` `13eb` `400a` `404b` | 0.75 → -2.2 dB → 12ea:f9c8 12ea:fea0 13eb:0648 13eb:fea0 400a:8c69 404b:8c69<br>0 → -oo → 12ea:d760 13eb:d760 400a:0000 404b:0000 |
| input | AS 1/2 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | AS 1/2 | mute | `0a00` `0b00` | 1 → ? → 0a00:0001 0b00:0001<br>1 → ? → 0a00:0000 0b00:0000 |
| input | AS 1/2 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | AS 1/2 | reverbSend | `0a01` `0b01` | 0.75 → -16.5 dB → 0a01:ff5b 0b01:ff5b<br>0 → -oo → 0a01:fd76 0b01:fd76 |
| input | AS 1/2 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| input | ADAT 3/4 | gain | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | ADAT 3/4 | gainRight | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | ADAT 3/4 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 3/4 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 3/4 | phase | `0c06` | 1 → ? → 0c06:0001<br>1 → ? → 0c06:0000 |
| input | ADAT 3/4 | phaseRight | `0d06` | 1 → ? → 0d06:0001<br>1 → ? → 0d06:0000 |
| input | ADAT 3/4 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | ADAT 3/4 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 3/4 | msProc | `0c05` `0d05` | 1 → ? → 0c05:0001 0d05:0001<br>1 → ? → 0c05:0000 0d05:0000 |
| input | ADAT 3/4 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | ADAT 3/4 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 3/4 | alevEnable | `0c80` `0d80` | 1 → ? → 0c80:0001 0d80:0001<br>1 → ? → 0c80:0000 0d80:0000 |
| input | ADAT 3/4 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | ADAT 3/4 | volume | `12ec` `13ed` `400c` `404d` | 0.75 → -2.2 dB → 12ec:f9c8 12ec:fea0 13ed:0648 13ed:fea0 400c:8c69 404d:8c69<br>0 → -oo → 12ec:d760 13ed:d760 400c:0000 404d:0000 |
| input | ADAT 3/4 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | ADAT 3/4 | mute | `0c00` `0d00` | 1 → ? → 0c00:0001 0d00:0001<br>1 → ? → 0c00:0000 0d00:0000 |
| input | ADAT 3/4 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | ADAT 3/4 | reverbSend | `0c01` `0d01` | 0.75 → -16.5 dB → 0c01:ff5b 0d01:ff5b<br>0 → -oo → 0c01:fd76 0d01:fd76 |
| input | ADAT 3/4 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| input | ADAT 5/6 | gain | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | ADAT 5/6 | gainRight | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | ADAT 5/6 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 5/6 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 5/6 | phase | `0e06` | 1 → ? → 0e06:0001<br>1 → ? → 0e06:0000 |
| input | ADAT 5/6 | phaseRight | `0f06` | 1 → ? → 0f06:0001<br>1 → ? → 0f06:0000 |
| input | ADAT 5/6 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | ADAT 5/6 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 5/6 | msProc | `0e05` `0f05` | 1 → ? → 0e05:0001 0f05:0001<br>1 → ? → 0e05:0000 0f05:0000 |
| input | ADAT 5/6 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | ADAT 5/6 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 5/6 | alevEnable | `0e80` `0f80` | 1 → ? → 0e80:0001 0f80:0001<br>1 → ? → 0e80:0000 0f80:0000 |
| input | ADAT 5/6 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | ADAT 5/6 | volume | `12ee` `13ef` `400e` `404f` | 0.75 → -2.2 dB → 12ee:f9c8 12ee:fea0 13ef:0648 13ef:fea0 400e:8c69 404f:8c69<br>0 → -oo → 12ee:d760 13ef:d760 400e:0000 404f:0000 |
| input | ADAT 5/6 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | ADAT 5/6 | mute | `0e00` `0f00` | 1 → ? → 0e00:0001 0f00:0001<br>1 → ? → 0e00:0000 0f00:0000 |
| input | ADAT 5/6 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | ADAT 5/6 | reverbSend | `0e01` `0f01` | 0.75 → -16.5 dB → 0e01:ff5b 0f01:ff5b<br>0 → -oo → 0e01:fd76 0f01:fd76 |
| input | ADAT 5/6 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| input | ADAT 7/8 | gain | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | ADAT 7/8 | gainRight | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | ADAT 7/8 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 7/8 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 7/8 | phase | `1006` | 1 → ? → 1006:0001<br>1 → ? → 1006:0000 |
| input | ADAT 7/8 | phaseRight | `1106` | 1 → ? → 1106:0001<br>1 → ? → 1106:0000 |
| input | ADAT 7/8 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | ADAT 7/8 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 7/8 | msProc | `1005` `1105` | 1 → ? → 1005:0001 1105:0001<br>1 → ? → 1005:0000 1105:0000 |
| input | ADAT 7/8 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | ADAT 7/8 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | ADAT 7/8 | alevEnable | `1080` `1180` | 1 → ? → 1080:0001 1180:0001<br>1 → ? → 1080:0000 1180:0000 |
| input | ADAT 7/8 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | ADAT 7/8 | volume | `12f0` `13f1` `4010` `4051` | 0.75 → -2.2 dB → 12f0:f9c8 12f0:fea0 13f1:0648 13f1:fea0 4010:8c69 4051:8c69<br>0 → -oo → 12f0:d760 13f1:d760 4010:0000 4051:0000 |
| input | ADAT 7/8 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | ADAT 7/8 | mute | `1000` `1100` | 1 → ? → 1000:0001 1100:0001<br>1 → ? → 1000:0000 1100:0000 |
| input | ADAT 7/8 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | ADAT 7/8 | reverbSend | `1001` `1101` | 0.75 → -16.5 dB → 1001:ff5b 1101:ff5b<br>0 → -oo → 1001:fd76 1101:fd76 |
| input | ADAT 7/8 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| playback | AN 1/2 | gain | none | 0.25 → 0 → (none)<br>0.5 → 0 → (none)<br>0.75 → 0 → (none)<br>1 → 0 → (none)<br>0 → 0 → (none) |
| playback | AN 1/2 | gainRight | none | 0.25 → 0 → (none)<br>0.5 → 0 → (none)<br>0.75 → 0 → (none)<br>1 → 0 → (none)<br>0 → 0 → (none) |
| playback | AN 1/2 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | phase | `4020` `41a0` | 1 → ? → 4020:f000 41a0:f000<br>1 → ? → 4020:9000 41a0:9000 |
| playback | AN 1/2 | phaseRight | `4061` `41e1` | 1 → ? → 4061:f000 41e1:f000<br>1 → ? → 4061:9000 41e1:9000 |
| playback | AN 1/2 | refLevel | none | 0 → 0 → (none)<br>0.25 → 0 → (none)<br>0.5 → 1 → (none)<br>0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | AN 1/2 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | msProc | `4020` `4021` `4060` `4061` `41a0` `41a1` `41e0` `41e1` | 1 → ? → 4020:8800 4021:8800 4060:8800 4061:4000 41a0:8800 41a1:8800 41e0:8800 41e1:4000<br>1 → ? → 4020:9000 4021:0000 4060:0000 4061:9000 41a0:9000 41a1:0000 41e0:0000 41e1:9000 |
| playback | AN 1/2 | width | `4020` `4021` `4060` `4061` `41a0` `41a1` `41e0` `41e1` | 0 → -1.00 → 4020:0000 4021:9000 4060:9000 4061:0000 41a0:0000 41a1:9000 41e0:9000 41e1:0000<br>0.25 → -0.50 → 4020:2000 4021:8c00 4060:8c00 4061:2000 41a0:2000 41a1:8c00 41e0:8c00 41e1:2000<br>0.5 → 0.00 → 4020:8800 4021:8800 4060:8800 4061:8800 41a0:8800 41a1:8800 41e0:8800 41e1:8800<br>0.75 → 0.50 → 4020:8c00 4021:2000 4060:2000 4061:8c00 41a0:8c00 41a1:2000 41e0:2000 41e1:8c00<br>1 → 1.00 → 4020:9000 4021:0000 4060:0000 4061:9000 41a0:9000 41a1:0000 41e0:0000 41e1:9000 |
| playback | AN 1/2 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | crossFeed | none | 0.25 → Off → (none)<br>0.5 → Off → (none)<br>0.75 → Off → (none)<br>1 → Off → (none)<br>0 → Off → (none) |
| playback | AN 1/2 | volume | `4020` `4061` `47a0` `47e1` | 0 → -oo → 4020:0000 4061:0000<br>0.25 → -32.6 dB → 4020:02fd 4061:02fd<br>0.5 → -12.1 dB → 4020:1fb1 4061:1fb1<br>0.75 → -2.2 dB → 4020:8c69 4061:8c69<br>1 → 6.0 dB → 4020:9fec 4061:9fec 47a0:0024 47e1:0024<br>0.8172 → 0.0 dB → 4020:9000 4061:9000 47a0:0000 47e1:0000 |
| playback | AN 1/2 | pan | `4061` `4020` | 0 → L100 → 4061:0000<br>0.25 → L50 → 4061:8800<br>0.75 → R50 → 4020:8800 4061:9000<br>1 → R100 → 4020:0000<br>0.5 → 0 → 4020:9000 |
| playback | AN 1/2 | mute | `4020` `4061` `41a0` `41e1` | 1 → ? → 4020:0000 4061:0000 41a0:0000 41e1:0000<br>1 → ? → 4020:9000 4061:9000 41a0:9000 41e1:9000 |
| playback | AN 1/2 | solo | `12e6` `13e7` `4006` `4047` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4047:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f |
| playback | AN 1/2 | reverbSend | `47a0` `47e1` | 0.25 → -49.0 dB → 47a0:0074 47e1:0074<br>0.5 → -32.5 dB → 47a0:0309 47e1:0309<br>0.75 → -16.5 dB → 47a0:1326 47e1:1326<br>1 → 0.0 dB → 47a0:9000 47e1:9000<br>0 → -oo → 47a0:0000 47e1:0000 |
| playback | AN 1/2 | reverbReturn | none | 0.25 → -oo → (none)<br>0.5 → -oo → (none)<br>0.75 → -oo → (none)<br>1 → -oo → (none)<br>0 → -oo → (none) |
| playback | AN 3/4 | gain | none | 0.75 → ? → (none)<br>0 → ? → (none) |
| playback | AN 3/4 | gainRight | none | 0.75 → ? → (none)<br>0 → ? → (none) |
| playback | AN 3/4 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 3/4 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 3/4 | phase | `40a2` `41a2` | 1 → ? → 40a2:f000 41a2:f000<br>1 → ? → 40a2:9000 41a2:9000 |
| playback | AN 3/4 | phaseRight | `40e3` `41e3` | 1 → ? → 40e3:f000 41e3:f000<br>1 → ? → 40e3:9000 41e3:9000 |
| playback | AN 3/4 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | AN 3/4 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 3/4 | msProc | `40a2` `40a3` `40e2` `40e3` `41a2` `41a3` `41e2` `41e3` | 1 → ? → 40a2:8800 40a3:8800 40e2:8800 40e3:4000 41a2:8800 41a3:8800 41e2:8800 41e3:4000<br>1 → ? → 40a2:9000 40a3:0000 40e2:0000 40e3:9000 41a2:9000 41a3:0000 41e2:0000 41e3:9000 |
| playback | AN 3/4 | width | `40a2` `40a3` `40e2` `40e3` `41a2` `41a3` `41e2` `41e3` | 0.75 → 0.50 → 40a2:8c00 40a3:2000 40e2:2000 40e3:8c00 41a2:8c00 41a3:2000 41e2:2000 41e3:8c00<br>1 → 1.00 → 40a2:9000 40a3:0000 40e2:0000 40e3:9000 41a2:9000 41a3:0000 41e2:0000 41e3:9000 |
| playback | AN 3/4 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 3/4 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 3/4 | crossFeed | none | 0.75 → Off → (none)<br>0 → ? → (none) |
| playback | AN 3/4 | volume | `4022` `4063` | 0.75 → -2.2 dB → 4022:8c69 4063:8c69<br>0 → -oo → 4022:0000 4063:0000 |
| playback | AN 3/4 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| playback | AN 3/4 | mute | `40a2` `40e3` `41a2` `41e3` | 1 → ? → 40a2:0000 40e3:0000 41a2:0000 41e3:0000<br>1 → ? → 40a2:9000 40e3:9000 41a2:9000 41e3:9000 |
| playback | AN 3/4 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | AN 3/4 | reverbSend | `47a2` `47e3` | 0.75 → -16.5 dB → 47a2:1326 47e3:1326<br>0 → -oo → 47a2:0000 47e3:0000 |
| playback | AN 3/4 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → ? → (none) |
| playback | AN 5/6 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | AN 5/6 | gainRight | none | 0.75 → ? → (none)<br>0 → ? → (none) |
| playback | AN 5/6 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | phase | `4124` `41a4` | 1 → ? → 4124:f000 41a4:f000<br>1 → ? → 4124:9000 41a4:9000 |
| playback | AN 5/6 | phaseRight | `4165` `41e5` | 1 → ? → 4165:f000 41e5:f000<br>1 → ? → 4165:9000 41e5:9000 |
| playback | AN 5/6 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | AN 5/6 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | msProc | `4124` `4125` `4164` `4165` `41a4` `41a5` `41e4` `41e5` | 1 → ? → 4124:8800 4125:8800 4164:8800 4165:4000 41a4:8800 41a5:8800 41e4:8800 41e5:4000<br>1 → ? → 4124:9000 4125:0000 4164:0000 4165:9000 41a4:9000 41a5:0000 41e4:0000 41e5:9000 |
| playback | AN 5/6 | width | `4124` `4125` `4164` `4165` `41a4` `41a5` `41e4` `41e5` | 0.75 → 0.50 → 4124:8c00 4125:2000 4164:2000 4165:8c00 41a4:8c00 41a5:2000 41e4:2000 41e5:8c00<br>1 → 1.00 → 4124:9000 4125:0000 4164:0000 4165:9000 41a4:9000 41a5:0000 41e4:0000 41e5:9000 |
| playback | AN 5/6 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| playback | AN 5/6 | volume | `4024` `4065` | 0.75 → -2.2 dB → 4024:8c69 4065:8c69<br>0 → -oo → 4024:0000 4065:0000 |
| playback | AN 5/6 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| playback | AN 5/6 | mute | `4124` `4165` `41a4` `41e5` | 1 → ? → 4124:0000 4165:0000 41a4:0000 41e5:0000<br>1 → ? → 4124:9000 4165:9000 41a4:9000 41e5:9000 |
| playback | AN 5/6 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | AN 5/6 | reverbSend | `47a4` `47e5` | 0.75 → -16.5 dB → 47a4:1326 47e5:1326<br>0 → -oo → 47a4:0000 47e5:0000 |
| playback | AN 5/6 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| playback | PH 7/8 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | PH 7/8 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | PH 7/8 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | PH 7/8 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | PH 7/8 | phase | `41a6` | 1 → ? → 41a6:f000<br>1 → ? → 41a6:9000 |
| playback | PH 7/8 | phaseRight | `41e7` | 1 → ? → 41e7:f000<br>1 → ? → 41e7:9000 |
| playback | PH 7/8 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | PH 7/8 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | PH 7/8 | msProc | `41a6` `41a7` `41e6` `41e7` | 1 → ? → 41a6:8800 41a7:8800 41e6:8800 41e7:4000<br>1 → ? → 41a6:9000 41a7:0000 41e6:0000 41e7:9000 |
| playback | PH 7/8 | width | `41a6` `41a7` `41e6` `41e7` | 0.75 → 0.50 → 41a6:8c00 41a7:2000 41e6:2000 41e7:8c00<br>1 → 1.00 → 41a6:9000 41a7:0000 41e6:0000 41e7:9000 |
| playback | PH 7/8 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | PH 7/8 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | PH 7/8 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| playback | PH 7/8 | volume | `4026` `4067` | 0.75 → -2.2 dB → 4026:8c69 4067:8c69<br>0 → -oo → 4026:0000 4067:0000 |
| playback | PH 7/8 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| playback | PH 7/8 | mute | `41a6` `41e7` | 1 → ? → 41a6:0000 41e7:0000<br>1 → ? → 41a6:9000 41e7:9000 |
| playback | PH 7/8 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | PH 7/8 | reverbSend | `47a6` `47e7` | 0.75 → -16.5 dB → 47a6:1326 47e7:1326<br>0 → -oo → 47a6:0000 47e7:0000 |
| playback | PH 7/8 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| playback | SPDIF | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | SPDIF | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | SPDIF | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | SPDIF | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | SPDIF | phase | `41a8` `4228` | 1 → ? → 41a8:f000 4228:f000<br>1 → ? → 41a8:9000 4228:9000 |
| playback | SPDIF | phaseRight | `41e9` `4269` | 1 → ? → 41e9:f000 4269:f000<br>1 → ? → 41e9:9000 4269:9000 |
| playback | SPDIF | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | SPDIF | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | SPDIF | msProc | `41a8` `41a9` `41e8` `41e9` `4228` `4229` `4268` `4269` | 1 → ? → 41a8:8800 41a9:8800 41e8:8800 41e9:4000 4228:8800 4229:8800 4268:8800 4269:4000<br>1 → ? → 41a8:9000 41a9:0000 41e8:0000 41e9:9000 4228:9000 4229:0000 4268:0000 4269:9000 |
| playback | SPDIF | width | `41a8` `41a9` `41e8` `41e9` `4228` `4229` `4268` `4269` | 0.75 → 0.50 → 41a8:8c00 41a9:2000 41e8:2000 41e9:8c00 4228:8c00 4229:2000 4268:2000 4269:8c00<br>1 → 1.00 → 41a8:9000 41a9:0000 41e8:0000 41e9:9000 4228:9000 4229:0000 4268:0000 4269:9000 |
| playback | SPDIF | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | SPDIF | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | SPDIF | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| playback | SPDIF | volume | `4028` `4069` | 0.75 → -2.2 dB → 4028:8c69 4069:8c69<br>0 → -oo → 4028:0000 4069:0000 |
| playback | SPDIF | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| playback | SPDIF | mute | `41a8` `41e9` `4228` `4269` | 1 → ? → 41a8:0000 41e9:0000 4228:0000 4269:0000<br>1 → ? → 41a8:9000 41e9:9000 4228:9000 4269:9000 |
| playback | SPDIF | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | SPDIF | reverbSend | `47a8` `47e9` | 0.75 → -16.5 dB → 47a8:1326 47e9:1326<br>0 → -oo → 47a8:0000 47e9:0000 |
| playback | SPDIF | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| playback | AS 1/2 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | AS 1/2 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | AS 1/2 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AS 1/2 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AS 1/2 | phase | `41aa` `42aa` | 1 → ? → 41aa:f000 42aa:f000<br>1 → ? → 41aa:9000 42aa:9000 |
| playback | AS 1/2 | phaseRight | `41eb` `42eb` | 1 → ? → 41eb:f000 42eb:f000<br>1 → ? → 41eb:9000 42eb:9000 |
| playback | AS 1/2 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | AS 1/2 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AS 1/2 | msProc | `41aa` `41ab` `41ea` `41eb` `42aa` `42ab` `42ea` `42eb` | 1 → ? → 41aa:8800 41ab:8800 41ea:8800 41eb:4000 42aa:8800 42ab:8800 42ea:8800 42eb:4000<br>1 → ? → 41aa:9000 41ab:0000 41ea:0000 41eb:9000 42aa:9000 42ab:0000 42ea:0000 42eb:9000 |
| playback | AS 1/2 | width | `41aa` `41ab` `41ea` `41eb` `42aa` `42ab` `42ea` `42eb` | 0.75 → 0.50 → 41aa:8c00 41ab:2000 41ea:2000 41eb:8c00 42aa:8c00 42ab:2000 42ea:2000 42eb:8c00<br>1 → 1.00 → 41aa:9000 41ab:0000 41ea:0000 41eb:9000 42aa:9000 42ab:0000 42ea:0000 42eb:9000 |
| playback | AS 1/2 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AS 1/2 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AS 1/2 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| playback | AS 1/2 | volume | `402a` `406b` | 0.75 → -2.2 dB → 402a:8c69 406b:8c69<br>0 → -oo → 402a:0000 406b:0000 |
| playback | AS 1/2 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| playback | AS 1/2 | mute | `41aa` `41eb` `42aa` `42eb` | 1 → ? → 41aa:0000 41eb:0000 42aa:0000 42eb:0000<br>1 → ? → 41aa:9000 41eb:9000 42aa:9000 42eb:9000 |
| playback | AS 1/2 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | AS 1/2 | reverbSend | `47aa` `47eb` | 0.75 → -16.5 dB → 47aa:1326 47eb:1326<br>0 → -oo → 47aa:0000 47eb:0000 |
| playback | AS 1/2 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| playback | ADAT 3/4 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | ADAT 3/4 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | ADAT 3/4 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 3/4 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 3/4 | phase | `41ac` `432c` | 1 → ? → 41ac:f000 432c:f000<br>1 → ? → 41ac:9000 432c:9000 |
| playback | ADAT 3/4 | phaseRight | `41ed` `436d` | 1 → ? → 41ed:f000 436d:f000<br>1 → ? → 41ed:9000 436d:9000 |
| playback | ADAT 3/4 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | ADAT 3/4 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 3/4 | msProc | `41ac` `41ad` `41ec` `41ed` `432c` `432d` `436c` `436d` | 1 → ? → 41ac:8800 41ad:8800 41ec:8800 41ed:4000 432c:8800 432d:8800 436c:8800 436d:4000<br>1 → ? → 41ac:9000 41ad:0000 41ec:0000 41ed:9000 432c:9000 432d:0000 436c:0000 436d:9000 |
| playback | ADAT 3/4 | width | `41ac` `41ad` `41ec` `41ed` `432c` `432d` `436c` `436d` | 0.75 → 0.50 → 41ac:8c00 41ad:2000 41ec:2000 41ed:8c00 432c:8c00 432d:2000 436c:2000 436d:8c00<br>1 → 1.00 → 41ac:9000 41ad:0000 41ec:0000 41ed:9000 432c:9000 432d:0000 436c:0000 436d:9000 |
| playback | ADAT 3/4 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 3/4 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 3/4 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| playback | ADAT 3/4 | volume | `402c` `406d` | 0.75 → -2.2 dB → 402c:8c69 406d:8c69<br>0 → -oo → 402c:0000 406d:0000 |
| playback | ADAT 3/4 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| playback | ADAT 3/4 | mute | `41ac` `41ed` `432c` `436d` | 1 → ? → 41ac:0000 41ed:0000 432c:0000 436d:0000<br>1 → ? → 41ac:9000 41ed:9000 432c:9000 436d:9000 |
| playback | ADAT 3/4 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | ADAT 3/4 | reverbSend | `47ac` `47ed` | 0.75 → -16.5 dB → 47ac:1326 47ed:1326<br>0 → -oo → 47ac:0000 47ed:0000 |
| playback | ADAT 3/4 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| playback | ADAT 5/6 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | ADAT 5/6 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | ADAT 5/6 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 5/6 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 5/6 | phase | `41ae` `43ae` | 1 → ? → 41ae:f000 43ae:f000<br>1 → ? → 41ae:9000 43ae:9000 |
| playback | ADAT 5/6 | phaseRight | `41ef` `43ef` | 1 → ? → 41ef:f000 43ef:f000<br>1 → ? → 41ef:9000 43ef:9000 |
| playback | ADAT 5/6 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | ADAT 5/6 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 5/6 | msProc | `41ae` `41af` `41ee` `41ef` `43ae` `43af` `43ee` `43ef` | 1 → ? → 41ae:8800 41af:8800 41ee:8800 41ef:4000 43ae:8800 43af:8800 43ee:8800 43ef:4000<br>1 → ? → 41ae:9000 41af:0000 41ee:0000 41ef:9000 43ae:9000 43af:0000 43ee:0000 43ef:9000 |
| playback | ADAT 5/6 | width | `41ae` `41af` `41ee` `41ef` `43ae` `43af` `43ee` `43ef` | 0.75 → 0.50 → 41ae:8c00 41af:2000 41ee:2000 41ef:8c00 43ae:8c00 43af:2000 43ee:2000 43ef:8c00<br>1 → 1.00 → 41ae:9000 41af:0000 41ee:0000 41ef:9000 43ae:9000 43af:0000 43ee:0000 43ef:9000 |
| playback | ADAT 5/6 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 5/6 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 5/6 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| playback | ADAT 5/6 | volume | `402e` `406f` | 0.75 → -2.2 dB → 402e:8c69 406f:8c69<br>0 → -oo → 402e:0000 406f:0000 |
| playback | ADAT 5/6 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| playback | ADAT 5/6 | mute | `41ae` `41ef` `43ae` `43ef` | 1 → ? → 41ae:0000 41ef:0000 43ae:0000 43ef:0000<br>1 → ? → 41ae:9000 41ef:9000 43ae:9000 43ef:9000 |
| playback | ADAT 5/6 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | ADAT 5/6 | reverbSend | `47ae` `47ef` | 0.75 → -16.5 dB → 47ae:1326 47ef:1326<br>0 → -oo → 47ae:0000 47ef:0000 |
| playback | ADAT 5/6 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| playback | ADAT 7/8 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | ADAT 7/8 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| playback | ADAT 7/8 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 7/8 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 7/8 | phase | `41b0` `4430` | 1 → ? → 41b0:f000 4430:f000<br>1 → ? → 41b0:9000 4430:9000 |
| playback | ADAT 7/8 | phaseRight | `41f1` `4471` | 1 → ? → 41f1:f000 4471:f000<br>1 → ? → 41f1:9000 4471:9000 |
| playback | ADAT 7/8 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | ADAT 7/8 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 7/8 | msProc | `41b0` `41b1` `41f0` `41f1` `4430` `4431` `4470` `4471` | 1 → ? → 41b0:8800 41b1:8800 41f0:8800 41f1:4000 4430:8800 4431:8800 4470:8800 4471:4000<br>1 → ? → 41b0:9000 41b1:0000 41f0:0000 41f1:9000 4430:9000 4431:0000 4470:0000 4471:9000 |
| playback | ADAT 7/8 | width | `41b0` `41b1` `41f0` `41f1` `4430` `4431` `4470` `4471` | 0.75 → 0.50 → 41b0:8c00 41b1:2000 41f0:2000 41f1:8c00 4430:8c00 4431:2000 4470:2000 4471:8c00<br>1 → 1.00 → 41b0:9000 41b1:0000 41f0:0000 41f1:9000 4430:9000 4431:0000 4470:0000 4471:9000 |
| playback | ADAT 7/8 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 7/8 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 7/8 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| playback | ADAT 7/8 | volume | `4030` `4071` | 0.75 → -2.2 dB → 4030:8c69 4071:8c69<br>0 → -oo → 4030:0000 4071:0000 |
| playback | ADAT 7/8 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| playback | ADAT 7/8 | mute | `41b0` `41f1` `4430` `4471` | 1 → ? → 41b0:0000 41f1:0000 4430:0000 4471:0000<br>1 → ? → 41b0:9000 41f1:9000 4430:9000 4471:9000 |
| playback | ADAT 7/8 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | ADAT 7/8 | reverbSend | `47b0` `47f1` | 0.75 → -16.5 dB → 47b0:1326 47f1:1326<br>0 → -oo → 47b0:0000 47f1:0000 |
| playback | ADAT 7/8 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| output | AN 1/2 | gain | none | 0.25 → 0 → (none)<br>0.5 → 0 → (none)<br>0.75 → 0 → (none)<br>1 → 0 → (none)<br>0 → 0 → (none) |
| output | AN 1/2 | gainRight | none | 0.25 → 0 → (none)<br>0.5 → 0 → (none)<br>0.75 → 0 → (none)<br>1 → 0 → (none)<br>0 → 0 → (none) |
| output | AN 1/2 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 1/2 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 1/2 | phase | `1207` | 1 → ? → 1207:0001<br>1 → ? → 1207:0000 |
| output | AN 1/2 | phaseRight | `1307` | 1 → ? → 1307:0001<br>1 → ? → 1307:0000 |
| output | AN 1/2 | refLevel | `1208` `1308` `1408` `1508` `1608` `1708` | 0 → 0 → 1208:0000 1308:0000 1408:0000 1508:0000 1608:0000 1708:0000<br>0.25 → 0 → (none)<br>0.5 → 1 → 1208:0001 1308:0001 1408:0001 1508:0001 1608:0001 1708:0001<br>0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | AN 1/2 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 1/2 | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 1/2 | width | none | 0 → -1.00 → (none)<br>0.25 → -0.50 → (none)<br>0.5 → 0.00 → (none)<br>0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | AN 1/2 | loopback | none | 1 → ? → req21 v=0x0001 i=0x0000 req21 v=0x0001 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 1/2 | alevEnable | `1280` `1380` | 1 → ? → 1280:0001 1380:0001<br>1 → ? → 1280:0000 1380:0000 |
| output | AN 1/2 | crossFeed | none | 0.25 → 1 → (none)<br>0.5 → 3 → (none)<br>0.75 → 4 → (none)<br>1 → 5 → (none)<br>0 → Off → (none) |
| output | AN 1/2 | volume | `1200` `1300` | 0 → -oo → 1200:fd76 1300:fd76<br>0.25 → -32.6 dB → 1200:feba 1300:feba<br>0.5 → -12.1 dB → 1200:ff87 1300:ff87<br>0.75 → -2.2 dB → 1200:ffea 1300:ffea<br>1 → 6.0 dB → 1200:003c 1300:003c<br>0.3501 → -23.0 dB → 1200:ff1a 1300:ff1a |
| output | AN 1/2 | pan | `1201` `1301` | 0 → L100 → 1201:ff9c 1301:ff9c<br>0.25 → L50 → 1201:ffce 1301:ffce<br>0.5 → 0 → 1201:0000 1301:0000<br>0.75 → R50 → 1201:0032 1301:0032<br>1 → R100 → 1201:0064 1301:0064<br>0.475 → L5 → 1201:fffb 1301:fffb |
| output | AN 1/2 | mute | `1202` `1302` | 1 → ? → 1202:0001 1302:0001<br>1 → ? → 1202:0000 1302:0000 |
| output | AN 1/2 | reverbSend | none | 0.25 → -49.0 dB → (none)<br>0.5 → -32.5 dB → (none)<br>0.75 → -16.5 dB → (none)<br>1 → 0.0 dB → (none)<br>0 → -oo → (none) |
| output | AN 1/2 | reverbReturn | `1203` `1303` | 0.25 → -49.0 dB → 1203:fe16 1303:fe16<br>0.5 → -32.5 dB → 1203:febb 1303:febb<br>0.75 → -16.5 dB → 1203:ff5b 1303:ff5b<br>1 → 0.0 dB → 1203:0000 1303:0000<br>0 → -oo → 1203:fd76 1303:fd76 |
| output | AN 3/4 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | AN 3/4 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | AN 3/4 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 3/4 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 3/4 | phase | `1407` | 1 → ? → 1407:0001<br>1 → ? → 1407:0000 |
| output | AN 3/4 | phaseRight | `1507` | 1 → ? → 1507:0001<br>1 → ? → 1507:0000 |
| output | AN 3/4 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | AN 3/4 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 3/4 | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 3/4 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | AN 3/4 | loopback | none | 1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0001 i=0x0002 req21 v=0x0001 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 3/4 | alevEnable | `1480` `1580` | 1 → ? → 1480:0001 1580:0001<br>1 → ? → 1480:0000 1580:0000 |
| output | AN 3/4 | crossFeed | none | 0.75 → 4 → (none)<br>0 → Off → (none) |
| output | AN 3/4 | volume | `1400` `1500` | 0.75 → -2.2 dB → 1400:ffea 1500:ffea<br>0.8172 → 0.0 dB → 1400:0000 1500:0000 |
| output | AN 3/4 | pan | `1401` `1501` | 0.75 → R50 → 1401:0032 1501:0032<br>0.5 → 0 → 1401:0000 1501:0000 |
| output | AN 3/4 | mute | `1402` `1502` | 1 → ? → 1402:0001 1502:0001<br>1 → ? → 1402:0000 1502:0000 |
| output | AN 3/4 | reverbSend | none | 0.75 → -16.5 dB → (none)<br>0 → -oo → (none) |
| output | AN 3/4 | reverbReturn | `1403` `1503` | 0.75 → -16.5 dB → 1403:ff5b 1503:ff5b<br>0 → -oo → 1403:fd76 1503:fd76 |
| output | AN 5/6 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | AN 5/6 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | AN 5/6 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 5/6 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 5/6 | phase | `1607` | 1 → ? → 1607:0001<br>1 → ? → 1607:0000 |
| output | AN 5/6 | phaseRight | `1707` | 1 → ? → 1707:0001<br>1 → ? → 1707:0000 |
| output | AN 5/6 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | AN 5/6 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 5/6 | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AN 5/6 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | AN 5/6 | loopback | none | 1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0001 i=0x0004 req21 v=0x0001 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 5/6 | alevEnable | `1680` `1780` | 1 → ? → 1680:0001 1780:0001<br>1 → ? → 1680:0000 1780:0000 |
| output | AN 5/6 | crossFeed | none | 0.75 → 4 → (none)<br>0 → Off → (none) |
| output | AN 5/6 | volume | `1600` `1700` | 0.75 → -2.2 dB → 1600:ffea 1700:ffea<br>0.8172 → 0.0 dB → 1600:0000 1700:0000 |
| output | AN 5/6 | pan | `1601` `1701` | 0.75 → R50 → 1601:0032 1701:0032<br>0.5 → 0 → 1601:0000 1701:0000 |
| output | AN 5/6 | mute | `1602` `1702` | 1 → ? → 1602:0001 1702:0001<br>1 → ? → 1602:0000 1702:0000 |
| output | AN 5/6 | reverbSend | none | 0.75 → -16.5 dB → (none)<br>0 → -oo → (none) |
| output | AN 5/6 | reverbReturn | `1603` `1703` | 0.75 → -16.5 dB → 1603:ff5b 1703:ff5b<br>0 → -oo → 1603:fd76 1703:fd76 |
| output | Main | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | Main | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | Main | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | Main | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | Main | phase | `1807` | 1 → ? → 1807:0001<br>1 → ? → 1807:0000 |
| output | Main | phaseRight | `1907` | 1 → ? → 1907:0001<br>1 → ? → 1907:0000 |
| output | Main | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | Main | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | Main | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | Main | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | Main | loopback | none | 1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0001 i=0x0006 req21 v=0x0001 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | Main | alevEnable | `1880` `1980` | 1 → ? → 1880:0001 1980:0001<br>1 → ? → 1880:0000 1980:0000 |
| output | Main | crossFeed | none | 0.75 → 4 → (none)<br>0 → Off → (none) |
| output | Main | volume | `1800` `1900` | 0.75 → -2.2 dB → 1800:ffea 1900:ffea<br>0 → -oo → 1800:fd76 1900:fd76 |
| output | Main | pan | `1801` `1901` | 0.75 → R50 → 1801:0032 1901:0032<br>0.5 → 0 → 1801:0000 1901:0000 |
| output | Main | mute | `1802` `1902` `3f9e` | 1 → ? → 1802:0001 1902:0001 3f9e:0004 3f9e:0004<br>1 → ? → 1802:0000 1902:0000 |
| output | Main | reverbSend | none | 0.75 → -16.5 dB → (none)<br>0 → -oo → (none) |
| output | Main | reverbReturn | `1803` `1903` | 0.75 → -16.5 dB → 1803:ff5b 1903:ff5b<br>0 → -oo → 1803:fd76 1903:fd76 |
| output | SPDIF | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | SPDIF | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | SPDIF | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | SPDIF | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | SPDIF | phase | `1a07` | 1 → ? → 1a07:0001<br>1 → ? → 1a07:0000 |
| output | SPDIF | phaseRight | `1b07` | 1 → ? → 1b07:0001<br>1 → ? → 1b07:0000 |
| output | SPDIF | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | SPDIF | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | SPDIF | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | SPDIF | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | SPDIF | loopback | none | 1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0001 i=0x0008 req21 v=0x0001 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | SPDIF | alevEnable | `1a80` `1b80` | 1 → ? → 1a80:0001 1b80:0001<br>1 → ? → 1a80:0000 1b80:0000 |
| output | SPDIF | crossFeed | none | 0.75 → 4 → (none)<br>0 → Off → (none) |
| output | SPDIF | volume | `1a00` `1b00` | 0.75 → -2.2 dB → 1a00:ffea 1b00:ffea<br>0.8172 → 0.0 dB → 1a00:0000 1b00:0000 |
| output | SPDIF | pan | `1a01` `1b01` | 0.75 → R50 → 1a01:0032 1b01:0032<br>0.5 → 0 → 1a01:0000 1b01:0000 |
| output | SPDIF | mute | `1a02` `1b02` | 1 → ? → 1a02:0001 1b02:0001<br>1 → ? → 1a02:0000 1b02:0000 |
| output | SPDIF | reverbSend | none | 0.75 → -16.5 dB → (none)<br>0 → -oo → (none) |
| output | SPDIF | reverbReturn | `1a03` `1b03` | 0.75 → -16.5 dB → 1a03:ff5b 1b03:ff5b<br>0 → -oo → 1a03:fd76 1b03:fd76 |
| output | AS 1/2 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | AS 1/2 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | AS 1/2 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AS 1/2 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AS 1/2 | phase | `1c07` | 1 → ? → 1c07:0001<br>1 → ? → 1c07:0000 |
| output | AS 1/2 | phaseRight | `1d07` | 1 → ? → 1d07:0001<br>1 → ? → 1d07:0000 |
| output | AS 1/2 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | AS 1/2 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AS 1/2 | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | AS 1/2 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | AS 1/2 | loopback | none | 1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0001 i=0x000a req21 v=0x0001 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AS 1/2 | alevEnable | `1c80` `1d80` | 1 → ? → 1c80:0001 1d80:0001<br>1 → ? → 1c80:0000 1d80:0000 |
| output | AS 1/2 | crossFeed | none | 0.75 → 4 → (none)<br>0 → Off → (none) |
| output | AS 1/2 | volume | `1c00` `1d00` | 0.75 → -2.2 dB → 1c00:ffea 1d00:ffea<br>0.8172 → 0.0 dB → 1c00:0000 1d00:0000 |
| output | AS 1/2 | pan | `1c01` `1d01` | 0.75 → R50 → 1c01:0032 1d01:0032<br>0.5 → 0 → 1c01:0000 1d01:0000 |
| output | AS 1/2 | mute | `1c02` `1d02` | 1 → ? → 1c02:0001 1d02:0001<br>1 → ? → 1c02:0000 1d02:0000 |
| output | AS 1/2 | reverbSend | none | 0.75 → -16.5 dB → (none)<br>0 → -oo → (none) |
| output | AS 1/2 | reverbReturn | `1c03` `1d03` | 0.75 → -16.5 dB → 1c03:ff5b 1d03:ff5b<br>0 → -oo → 1c03:fd76 1d03:fd76 |
| output | ADAT 3/4 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | ADAT 3/4 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | ADAT 3/4 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 3/4 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 3/4 | phase | `1e07` | 1 → ? → 1e07:0001<br>1 → ? → 1e07:0000 |
| output | ADAT 3/4 | phaseRight | `1f07` | 1 → ? → 1f07:0001<br>1 → ? → 1f07:0000 |
| output | ADAT 3/4 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | ADAT 3/4 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 3/4 | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 3/4 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | ADAT 3/4 | loopback | none | 1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0001 i=0x000c req21 v=0x0001 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 3/4 | alevEnable | `1e80` `1f80` | 1 → ? → 1e80:0001 1f80:0001<br>1 → ? → 1e80:0000 1f80:0000 |
| output | ADAT 3/4 | crossFeed | none | 0.75 → 4 → (none)<br>0 → Off → (none) |
| output | ADAT 3/4 | volume | `1e00` `1f00` | 0.75 → -2.2 dB → 1e00:ffea 1f00:ffea<br>0.8172 → 0.0 dB → 1e00:0000 1f00:0000 |
| output | ADAT 3/4 | pan | `1e01` `1f01` | 0.75 → R50 → 1e01:0032 1f01:0032<br>0.5 → 0 → 1e01:0000 1f01:0000 |
| output | ADAT 3/4 | mute | `1e02` `1f02` | 1 → ? → 1e02:0001 1f02:0001<br>1 → ? → 1e02:0000 1f02:0000 |
| output | ADAT 3/4 | reverbSend | none | 0.75 → -16.5 dB → (none)<br>0 → -oo → (none) |
| output | ADAT 3/4 | reverbReturn | `1e03` `1f03` | 0.75 → -16.5 dB → 1e03:ff5b 1f03:ff5b<br>0 → -oo → 1e03:fd76 1f03:fd76 |
| output | ADAT 5/6 | gain | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | ADAT 5/6 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | ADAT 5/6 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 5/6 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 5/6 | phase | `2007` | 1 → ? → 2007:0001<br>1 → ? → 2007:0000 |
| output | ADAT 5/6 | phaseRight | `2107` | 1 → ? → 2107:0001<br>1 → ? → 2107:0000 |
| output | ADAT 5/6 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | ADAT 5/6 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 5/6 | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 5/6 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | ADAT 5/6 | loopback | none | 1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0001 i=0x000e req21 v=0x0001 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 5/6 | alevEnable | `2080` `2180` | 1 → ? → 2080:0001 2180:0001<br>1 → ? → 2080:0000 2180:0000 |
| output | ADAT 5/6 | crossFeed | none | 0.75 → 4 → (none)<br>0 → Off → (none) |
| output | ADAT 5/6 | volume | `2000` `2100` | 0.75 → -2.2 dB → 2000:ffea 2100:ffea<br>0.8172 → 0.0 dB → 2000:0000 2100:0000 |
| output | ADAT 5/6 | pan | `2001` `2101` | 0.75 → R50 → 2001:0032 2101:0032<br>0.5 → 0 → 2001:0000 2101:0000 |
| output | ADAT 5/6 | mute | `2002` `2102` | 1 → ? → 2002:0001 2102:0001<br>1 → ? → 2002:0000 2102:0000 |
| output | ADAT 5/6 | reverbSend | none | 0.75 → -16.5 dB → (none)<br>0 → -oo → (none) |
| output | ADAT 5/6 | reverbReturn | `2003` `2103` | 0.75 → -16.5 dB → 2003:ff5b 2103:ff5b<br>0 → -oo → 2003:fd76 2103:fd76 |
| output | ADAT 7/8 | gain | none | 0.75 → ? → (none)<br>0 → 0 → (none) |
| output | ADAT 7/8 | gainRight | none | 0.75 → 0 → (none)<br>0 → 0 → (none) |
| output | ADAT 7/8 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 7/8 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 7/8 | phase | `2207` | 1 → ? → 2207:0001<br>1 → ? → 2207:0000 |
| output | ADAT 7/8 | phaseRight | `2307` | 1 → ? → 2307:0001<br>1 → ? → 2307:0000 |
| output | ADAT 7/8 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| output | ADAT 7/8 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 7/8 | msProc | none | 1 → ? → (none)<br>1 → ? → (none) |
| output | ADAT 7/8 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| output | ADAT 7/8 | loopback | none | 1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0001 i=0x0010 req21 v=0x0001 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d<br>1 → ? → req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 7/8 | alevEnable | `2280` `2380` | 1 → ? → 2280:0001 2380:0001<br>1 → ? → 2280:0000 2380:0000 |
| output | ADAT 7/8 | crossFeed | none | 0.75 → 4 → (none)<br>0 → Off → (none) |
| output | ADAT 7/8 | volume | `2200` `2300` | 0.75 → -2.2 dB → 2200:ffea 2300:ffea<br>0.8172 → 0.0 dB → 2200:0000 2300:0000 |
| output | ADAT 7/8 | pan | `2201` `2301` | 0.75 → R50 → 2201:0032 2301:0032<br>0.5 → 0 → 2201:0000 2301:0000 |
| output | ADAT 7/8 | mute | `2202` `2302` | 1 → ? → 2202:0001 2302:0001<br>1 → ? → 2202:0000 2302:0000 |
| output | ADAT 7/8 | reverbSend | none | 0.75 → -16.5 dB → (none)<br>0 → -oo → (none) |
| output | ADAT 7/8 | reverbReturn | `2203` `2303` | 0.75 → -16.5 dB → 2203:ff5b 2303:ff5b<br>0 → -oo → 2203:fd76 2303:fd76 |

## Every change in order

| Row | Channel | Parameter | Kind | OSC value | Display | Writes |
|---|---|---|---|---|---|---|
| input | AN 7/8 | gain | probe | 0.75 | 49 | (none) |
| input | AN 7/8 | gain | restore | 0 | 0 | (none) |
| input | AN 7/8 | gainRight | probe | 0.75 | 49 | (none) |
| input | AN 7/8 | gainRight | restore | 0 | 0 | (none) |
| input | AN 7/8 | pad | press | 1 |  | (none) |
| input | AN 7/8 | pad | press back | 1 |  | (none) |
| input | AN 7/8 | instrument | press | 1 |  | (none) |
| input | AN 7/8 | instrument | press back | 1 |  | (none) |
| input | AN 7/8 | phase | press | 1 |  | 0606:0001 12e6:f960 12e6:f9c8 4006:5a71 |
| input | AN 7/8 | phase | press back | 1 |  | 0606:0000 12e6:f960 12e6:f9c8 4006:258f |
| input | AN 7/8 | phaseRight | press | 1 |  | 0706:0001 13e7:0648 13e7:f960 4047:5a71 |
| input | AN 7/8 | phaseRight | press back | 1 |  | 0706:0000 13e7:0648 13e7:f960 4047:258f |
| input | AN 7/8 | refLevel | probe | 0.75 | 1 | (none) |
| input | AN 7/8 | refLevel | restore | 1 | 1 | (none) |
| input | AN 7/8 | autoset | press | 1 |  | (none) |
| input | AN 7/8 | autoset | press back | 1 |  | (none) |
| input | AN 7/8 | msProc | press | 1 |  | 0605:0001 0705:0001 12e6:0008 12e6:f770 12e7:0008 12e7:f770 13e6:0008 13e6:f770 13e7:0008 13e7:f770 4006:12c7 4007:12c7 4046:12c7 4047:6d39 |
| input | AN 7/8 | msProc | press back | 1 |  | 0605:0000 0705:0000 12e6:f960 12e6:f9c8 12e7:0648 12e7:f960 13e6:f960 13e6:f9c8 13e7:0648 13e7:f960 4006:258f 4007:0000 4046:0000 4047:258f |
| input | AN 7/8 | width | probe | 0.75 | 0.50 | 12e6:f810 12e6:fc58 12e7:03b8 12e7:f810 13e6:f810 13e6:fc58 13e7:03b8 13e7:f810 4006:1c2b 4007:0963 4046:0963 4047:1c2b |
| input | AN 7/8 | width | restore | 1 | 1.00 | 12e6:f960 12e6:f9c8 12e7:0648 12e7:f960 13e6:f960 13e6:f9c8 13e7:0648 13e7:f960 4006:258f 4007:0000 4046:0000 4047:258f |
| input | AN 7/8 | loopback | press | 1 |  | (none) |
| input | AN 7/8 | loopback | press back | 1 |  | (none) |
| input | AN 7/8 | alevEnable | press | 1 |  | 0680:0001 0780:0001 |
| input | AN 7/8 | alevEnable | press back | 1 |  | 0680:0000 0780:0000 |
| input | AN 7/8 | crossFeed | probe | 0.75 | Off | (none) |
| input | AN 7/8 | crossFeed | restore | 0 | Off | (none) |
| input | AN 7/8 | volume | probe | 0.75 | -2.2 dB | 12e6:f9c8 12e6:fea0 13e7:0648 13e7:fea0 4006:8c69 4047:8c69 |
| input | AN 7/8 | volume | restore | 0.5264 | -10.6 dB | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f |
| input | AN 7/8 | pan | probe | 0.75 | R50 | 12e6:f590 12e6:f9c8 4006:12c7 |
| input | AN 7/8 | pan | restore | 0.5 | 0 | 12e6:f960 12e6:f9c8 4006:258f |
| input | AN 7/8 | mute | press | 1 |  | 0600:0001 0700:0001 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4047:0000 |
| input | AN 7/8 | mute | press back | 1 |  | 0600:0000 0700:0000 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f |
| input | AN 7/8 | solo | press | 1 |  | 4020:0000 4061:0000 |
| input | AN 7/8 | solo | press back | 1 |  | 4020:9000 4061:9000 |
| input | AN 7/8 | reverbSend | probe | 0.75 | -16.5 dB | 0601:ff5b 0701:ff5b |
| input | AN 7/8 | reverbSend | restore | 0 | -oo | 0601:fd76 0701:fd76 |
| input | AN 7/8 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | AN 7/8 | reverbReturn | restore | 0 | -oo | (none) |
| input | SPDIF | gain | probe | 0.75 | 49 | (none) |
| input | SPDIF | gain | restore | 0 | 0 | (none) |
| input | SPDIF | gainRight | probe | 0.75 | 49 | (none) |
| input | SPDIF | gainRight | restore | 0 | 0 | (none) |
| input | SPDIF | pad | press | 1 |  | (none) |
| input | SPDIF | pad | press back | 1 |  | (none) |
| input | SPDIF | instrument | press | 1 |  | (none) |
| input | SPDIF | instrument | press back | 1 |  | (none) |
| input | SPDIF | phase | press | 1 |  | 0806:0001 |
| input | SPDIF | phase | press back | 1 |  | 0806:0000 |
| input | SPDIF | phaseRight | press | 1 |  | 0906:0001 |
| input | SPDIF | phaseRight | press back | 1 |  | 0906:0000 |
| input | SPDIF | refLevel | probe | 0.75 | 1 | (none) |
| input | SPDIF | refLevel | restore | 1 | 1 | (none) |
| input | SPDIF | autoset | press | 1 |  | (none) |
| input | SPDIF | autoset | press back | 1 |  | (none) |
| input | SPDIF | msProc | press | 1 |  | 0805:0001 0905:0001 |
| input | SPDIF | msProc | press back | 1 |  | 0805:0000 0905:0000 |
| input | SPDIF | width | probe | 0.75 | 0.50 | (none) |
| input | SPDIF | width | restore | 1 | 1.00 | (none) |
| input | SPDIF | loopback | press | 1 |  | (none) |
| input | SPDIF | loopback | press back | 1 |  | (none) |
| input | SPDIF | alevEnable | press | 1 |  | 0880:0001 0980:0001 |
| input | SPDIF | alevEnable | press back | 1 |  | 0880:0000 0980:0000 |
| input | SPDIF | crossFeed | probe | 0.75 | Off | (none) |
| input | SPDIF | crossFeed | restore | 0 | Off | (none) |
| input | SPDIF | volume | probe | 0.75 | -2.2 dB | 12e8:f9c8 12e8:fea0 13e9:0648 13e9:fea0 4008:8c69 4049:8c69 |
| input | SPDIF | volume | restore | 0 | -oo | 12e8:d760 13e9:d760 4008:0000 4049:0000 |
| input | SPDIF | pan | probe | 0.75 | R50 | (none) |
| input | SPDIF | pan | restore | 0.5 | 0 | (none) |
| input | SPDIF | mute | press | 1 |  | 0800:0001 0900:0001 |
| input | SPDIF | mute | press back | 1 |  | 0800:0000 0900:0000 |
| input | SPDIF | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | SPDIF | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | SPDIF | reverbSend | probe | 0.75 | -16.5 dB | 0801:ff5b 0901:ff5b |
| input | SPDIF | reverbSend | restore | 0 | -oo | 0801:fd76 0901:fd76 |
| input | SPDIF | reverbReturn | probe | 0.75 | -oo | (none) |
| input | SPDIF | reverbReturn | restore | 0 | -oo | (none) |
| input | AS 1/2 | gain | probe | 0.75 | 49 | (none) |
| input | AS 1/2 | gain | restore | 0 | 0 | (none) |
| input | AS 1/2 | gainRight | probe | 0.75 | 49 | (none) |
| input | AS 1/2 | gainRight | restore | 0 | 0 | (none) |
| input | AS 1/2 | pad | press | 1 |  | (none) |
| input | AS 1/2 | pad | press back | 1 |  | (none) |
| input | AS 1/2 | instrument | press | 1 |  | (none) |
| input | AS 1/2 | instrument | press back | 1 |  | (none) |
| input | AS 1/2 | phase | press | 1 |  | 0a06:0001 |
| input | AS 1/2 | phase | press back | 1 |  | 0a06:0000 |
| input | AS 1/2 | phaseRight | press | 1 |  | 0b06:0001 |
| input | AS 1/2 | phaseRight | press back | 1 |  | 0b06:0000 |
| input | AS 1/2 | refLevel | probe | 0.75 | 1 | (none) |
| input | AS 1/2 | refLevel | restore | 1 | 1 | (none) |
| input | AS 1/2 | autoset | press | 1 |  | (none) |
| input | AS 1/2 | autoset | press back | 1 |  | (none) |
| input | AS 1/2 | msProc | press | 1 |  | 0a05:0001 0b05:0001 |
| input | AS 1/2 | msProc | press back | 1 |  | 0a05:0000 0b05:0000 |
| input | AS 1/2 | width | probe | 0.75 | 0.50 | (none) |
| input | AS 1/2 | width | restore | 1 | 1.00 | (none) |
| input | AS 1/2 | loopback | press | 1 |  | (none) |
| input | AS 1/2 | loopback | press back | 1 |  | (none) |
| input | AS 1/2 | alevEnable | press | 1 |  | 0a80:0001 0b80:0001 |
| input | AS 1/2 | alevEnable | press back | 1 |  | 0a80:0000 0b80:0000 |
| input | AS 1/2 | crossFeed | probe | 0.75 |  | (none) |
| input | AS 1/2 | crossFeed | restore | 0 |  | (none) |
| input | AS 1/2 | volume | probe | 0.75 | -2.2 dB | 12ea:f9c8 12ea:fea0 13eb:0648 13eb:fea0 400a:8c69 404b:8c69 |
| input | AS 1/2 | volume | restore | 0 | -oo | 12ea:d760 13eb:d760 400a:0000 404b:0000 |
| input | AS 1/2 | pan | probe | 0.75 | R50 | (none) |
| input | AS 1/2 | pan | restore | 0.5 | 0 | (none) |
| input | AS 1/2 | mute | press | 1 |  | 0a00:0001 0b00:0001 |
| input | AS 1/2 | mute | press back | 1 |  | 0a00:0000 0b00:0000 |
| input | AS 1/2 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | AS 1/2 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | AS 1/2 | reverbSend | probe | 0.75 | -16.5 dB | 0a01:ff5b 0b01:ff5b |
| input | AS 1/2 | reverbSend | restore | 0 | -oo | 0a01:fd76 0b01:fd76 |
| input | AS 1/2 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | AS 1/2 | reverbReturn | restore | 0 | -oo | (none) |
| input | ADAT 3/4 | gain | probe | 0.75 | 49 | (none) |
| input | ADAT 3/4 | gain | restore | 0 | 0 | (none) |
| input | ADAT 3/4 | gainRight | probe | 0.75 | 49 | (none) |
| input | ADAT 3/4 | gainRight | restore | 0 | 0 | (none) |
| input | ADAT 3/4 | pad | press | 1 |  | (none) |
| input | ADAT 3/4 | pad | press back | 1 |  | (none) |
| input | ADAT 3/4 | instrument | press | 1 |  | (none) |
| input | ADAT 3/4 | instrument | press back | 1 |  | (none) |
| input | ADAT 3/4 | phase | press | 1 |  | 0c06:0001 |
| input | ADAT 3/4 | phase | press back | 1 |  | 0c06:0000 |
| input | ADAT 3/4 | phaseRight | press | 1 |  | 0d06:0001 |
| input | ADAT 3/4 | phaseRight | press back | 1 |  | 0d06:0000 |
| input | ADAT 3/4 | refLevel | probe | 0.75 | 1 | (none) |
| input | ADAT 3/4 | refLevel | restore | 1 | 1 | (none) |
| input | ADAT 3/4 | autoset | press | 1 |  | (none) |
| input | ADAT 3/4 | autoset | press back | 1 |  | (none) |
| input | ADAT 3/4 | msProc | press | 1 |  | 0c05:0001 0d05:0001 |
| input | ADAT 3/4 | msProc | press back | 1 |  | 0c05:0000 0d05:0000 |
| input | ADAT 3/4 | width | probe | 0.75 | 0.50 | (none) |
| input | ADAT 3/4 | width | restore | 1 | 1.00 | (none) |
| input | ADAT 3/4 | loopback | press | 1 |  | (none) |
| input | ADAT 3/4 | loopback | press back | 1 |  | (none) |
| input | ADAT 3/4 | alevEnable | press | 1 |  | 0c80:0001 0d80:0001 |
| input | ADAT 3/4 | alevEnable | press back | 1 |  | 0c80:0000 0d80:0000 |
| input | ADAT 3/4 | crossFeed | probe | 0.75 | Off | (none) |
| input | ADAT 3/4 | crossFeed | restore | 0 | Off | (none) |
| input | ADAT 3/4 | volume | probe | 0.75 | -2.2 dB | 12ec:f9c8 12ec:fea0 13ed:0648 13ed:fea0 400c:8c69 404d:8c69 |
| input | ADAT 3/4 | volume | restore | 0 | -oo | 12ec:d760 13ed:d760 400c:0000 404d:0000 |
| input | ADAT 3/4 | pan | probe | 0.75 | R50 | (none) |
| input | ADAT 3/4 | pan | restore | 0.5 | 0 | (none) |
| input | ADAT 3/4 | mute | press | 1 |  | 0c00:0001 0d00:0001 |
| input | ADAT 3/4 | mute | press back | 1 |  | 0c00:0000 0d00:0000 |
| input | ADAT 3/4 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | ADAT 3/4 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | ADAT 3/4 | reverbSend | probe | 0.75 | -16.5 dB | 0c01:ff5b 0d01:ff5b |
| input | ADAT 3/4 | reverbSend | restore | 0 | -oo | 0c01:fd76 0d01:fd76 |
| input | ADAT 3/4 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | ADAT 3/4 | reverbReturn | restore | 0 | -oo | (none) |
| input | ADAT 5/6 | gain | probe | 0.75 | 49 | (none) |
| input | ADAT 5/6 | gain | restore | 0 | 0 | (none) |
| input | ADAT 5/6 | gainRight | probe | 0.75 | 49 | (none) |
| input | ADAT 5/6 | gainRight | restore | 0 | 0 | (none) |
| input | ADAT 5/6 | pad | press | 1 |  | (none) |
| input | ADAT 5/6 | pad | press back | 1 |  | (none) |
| input | ADAT 5/6 | instrument | press | 1 |  | (none) |
| input | ADAT 5/6 | instrument | press back | 1 |  | (none) |
| input | ADAT 5/6 | phase | press | 1 |  | 0e06:0001 |
| input | ADAT 5/6 | phase | press back | 1 |  | 0e06:0000 |
| input | ADAT 5/6 | phaseRight | press | 1 |  | 0f06:0001 |
| input | ADAT 5/6 | phaseRight | press back | 1 |  | 0f06:0000 |
| input | ADAT 5/6 | refLevel | probe | 0.75 | 1 | (none) |
| input | ADAT 5/6 | refLevel | restore | 1 | 1 | (none) |
| input | ADAT 5/6 | autoset | press | 1 |  | (none) |
| input | ADAT 5/6 | autoset | press back | 1 |  | (none) |
| input | ADAT 5/6 | msProc | press | 1 |  | 0e05:0001 0f05:0001 |
| input | ADAT 5/6 | msProc | press back | 1 |  | 0e05:0000 0f05:0000 |
| input | ADAT 5/6 | width | probe | 0.75 | 0.50 | (none) |
| input | ADAT 5/6 | width | restore | 1 | 1.00 | (none) |
| input | ADAT 5/6 | loopback | press | 1 |  | (none) |
| input | ADAT 5/6 | loopback | press back | 1 |  | (none) |
| input | ADAT 5/6 | alevEnable | press | 1 |  | 0e80:0001 0f80:0001 |
| input | ADAT 5/6 | alevEnable | press back | 1 |  | 0e80:0000 0f80:0000 |
| input | ADAT 5/6 | crossFeed | probe | 0.75 | Off | (none) |
| input | ADAT 5/6 | crossFeed | restore | 0 | Off | (none) |
| input | ADAT 5/6 | volume | probe | 0.75 | -2.2 dB | 12ee:f9c8 12ee:fea0 13ef:0648 13ef:fea0 400e:8c69 404f:8c69 |
| input | ADAT 5/6 | volume | restore | 0 | -oo | 12ee:d760 13ef:d760 400e:0000 404f:0000 |
| input | ADAT 5/6 | pan | probe | 0.75 | R50 | (none) |
| input | ADAT 5/6 | pan | restore | 0.5 | 0 | (none) |
| input | ADAT 5/6 | mute | press | 1 |  | 0e00:0001 0f00:0001 |
| input | ADAT 5/6 | mute | press back | 1 |  | 0e00:0000 0f00:0000 |
| input | ADAT 5/6 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | ADAT 5/6 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | ADAT 5/6 | reverbSend | probe | 0.75 | -16.5 dB | 0e01:ff5b 0f01:ff5b |
| input | ADAT 5/6 | reverbSend | restore | 0 | -oo | 0e01:fd76 0f01:fd76 |
| input | ADAT 5/6 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | ADAT 5/6 | reverbReturn | restore | 0 | -oo | (none) |
| input | ADAT 7/8 | gain | probe | 0.75 | 49 | (none) |
| input | ADAT 7/8 | gain | restore | 0 | 0 | (none) |
| input | ADAT 7/8 | gainRight | probe | 0.75 | 49 | (none) |
| input | ADAT 7/8 | gainRight | restore | 0 | 0 | (none) |
| input | ADAT 7/8 | pad | press | 1 |  | (none) |
| input | ADAT 7/8 | pad | press back | 1 |  | (none) |
| input | ADAT 7/8 | instrument | press | 1 |  | (none) |
| input | ADAT 7/8 | instrument | press back | 1 |  | (none) |
| input | ADAT 7/8 | phase | press | 1 |  | 1006:0001 |
| input | ADAT 7/8 | phase | press back | 1 |  | 1006:0000 |
| input | ADAT 7/8 | phaseRight | press | 1 |  | 1106:0001 |
| input | ADAT 7/8 | phaseRight | press back | 1 |  | 1106:0000 |
| input | ADAT 7/8 | refLevel | probe | 0.75 | 1 | (none) |
| input | ADAT 7/8 | refLevel | restore | 1 | 1 | (none) |
| input | ADAT 7/8 | autoset | press | 1 |  | (none) |
| input | ADAT 7/8 | autoset | press back | 1 |  | (none) |
| input | ADAT 7/8 | msProc | press | 1 |  | 1005:0001 1105:0001 |
| input | ADAT 7/8 | msProc | press back | 1 |  | 1005:0000 1105:0000 |
| input | ADAT 7/8 | width | probe | 0.75 | 0.50 | (none) |
| input | ADAT 7/8 | width | restore | 1 | 1.00 | (none) |
| input | ADAT 7/8 | loopback | press | 1 |  | (none) |
| input | ADAT 7/8 | loopback | press back | 1 |  | (none) |
| input | ADAT 7/8 | alevEnable | press | 1 |  | 1080:0001 1180:0001 |
| input | ADAT 7/8 | alevEnable | press back | 1 |  | 1080:0000 1180:0000 |
| input | ADAT 7/8 | crossFeed | probe | 0.75 | Off | (none) |
| input | ADAT 7/8 | crossFeed | restore | 0 | Off | (none) |
| input | ADAT 7/8 | volume | probe | 0.75 | -2.2 dB | 12f0:f9c8 12f0:fea0 13f1:0648 13f1:fea0 4010:8c69 4051:8c69 |
| input | ADAT 7/8 | volume | restore | 0 | -oo | 12f0:d760 13f1:d760 4010:0000 4051:0000 |
| input | ADAT 7/8 | pan | probe | 0.75 | R50 | (none) |
| input | ADAT 7/8 | pan | restore | 0.5 | 0 | (none) |
| input | ADAT 7/8 | mute | press | 1 |  | 1000:0001 1100:0001 |
| input | ADAT 7/8 | mute | press back | 1 |  | 1000:0000 1100:0000 |
| input | ADAT 7/8 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | ADAT 7/8 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | ADAT 7/8 | reverbSend | probe | 0.75 | -16.5 dB | 1001:ff5b 1101:ff5b |
| input | ADAT 7/8 | reverbSend | restore | 0 | -oo | 1001:fd76 1101:fd76 |
| input | ADAT 7/8 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | ADAT 7/8 | reverbReturn | restore | 0 | -oo | (none) |
| playback | AN 1/2 | gain | sweep | 0.25 | 0 | (none) |
| playback | AN 1/2 | gain | sweep | 0.5 | 0 | (none) |
| playback | AN 1/2 | gain | sweep | 0.75 | 0 | (none) |
| playback | AN 1/2 | gain | sweep | 1 | 0 | (none) |
| playback | AN 1/2 | gain | restore | 0 | 0 | (none) |
| playback | AN 1/2 | gainRight | sweep | 0.25 | 0 | (none) |
| playback | AN 1/2 | gainRight | sweep | 0.5 | 0 | (none) |
| playback | AN 1/2 | gainRight | sweep | 0.75 | 0 | (none) |
| playback | AN 1/2 | gainRight | sweep | 1 | 0 | (none) |
| playback | AN 1/2 | gainRight | restore | 0 | 0 | (none) |
| playback | AN 1/2 | pad | press | 1 |  | (none) |
| playback | AN 1/2 | pad | press back | 1 |  | (none) |
| playback | AN 1/2 | instrument | press | 1 |  | (none) |
| playback | AN 1/2 | instrument | press back | 1 |  | (none) |
| playback | AN 1/2 | phase | press | 1 |  | 4020:f000 41a0:f000 |
| playback | AN 1/2 | phase | press back | 1 |  | 4020:9000 41a0:9000 |
| playback | AN 1/2 | phaseRight | press | 1 |  | 4061:f000 41e1:f000 |
| playback | AN 1/2 | phaseRight | press back | 1 |  | 4061:9000 41e1:9000 |
| playback | AN 1/2 | refLevel | sweep | 0 | 0 | (none) |
| playback | AN 1/2 | refLevel | sweep | 0.25 | 0 | (none) |
| playback | AN 1/2 | refLevel | sweep | 0.5 | 1 | (none) |
| playback | AN 1/2 | refLevel | sweep | 0.75 | 1 | (none) |
| playback | AN 1/2 | refLevel | restore | 1 | 1 | (none) |
| playback | AN 1/2 | autoset | press | 1 |  | (none) |
| playback | AN 1/2 | autoset | press back | 1 |  | (none) |
| playback | AN 1/2 | msProc | press | 1 |  | 4020:8800 4021:8800 4060:8800 4061:4000 41a0:8800 41a1:8800 41e0:8800 41e1:4000 |
| playback | AN 1/2 | msProc | press back | 1 |  | 4020:9000 4021:0000 4060:0000 4061:9000 41a0:9000 41a1:0000 41e0:0000 41e1:9000 |
| playback | AN 1/2 | width | sweep | 0 | -1.00 | 4020:0000 4021:9000 4060:9000 4061:0000 41a0:0000 41a1:9000 41e0:9000 41e1:0000 |
| playback | AN 1/2 | width | sweep | 0.25 | -0.50 | 4020:2000 4021:8c00 4060:8c00 4061:2000 41a0:2000 41a1:8c00 41e0:8c00 41e1:2000 |
| playback | AN 1/2 | width | sweep | 0.5 | 0.00 | 4020:8800 4021:8800 4060:8800 4061:8800 41a0:8800 41a1:8800 41e0:8800 41e1:8800 |
| playback | AN 1/2 | width | sweep | 0.75 | 0.50 | 4020:8c00 4021:2000 4060:2000 4061:8c00 41a0:8c00 41a1:2000 41e0:2000 41e1:8c00 |
| playback | AN 1/2 | width | restore | 1 | 1.00 | 4020:9000 4021:0000 4060:0000 4061:9000 41a0:9000 41a1:0000 41e0:0000 41e1:9000 |
| playback | AN 1/2 | loopback | press | 1 |  | (none) |
| playback | AN 1/2 | loopback | press back | 1 |  | (none) |
| playback | AN 1/2 | alevEnable | press | 1 |  | (none) |
| playback | AN 1/2 | alevEnable | press back | 1 |  | (none) |
| playback | AN 1/2 | crossFeed | sweep | 0.25 | Off | (none) |
| playback | AN 1/2 | crossFeed | sweep | 0.5 | Off | (none) |
| playback | AN 1/2 | crossFeed | sweep | 0.75 | Off | (none) |
| playback | AN 1/2 | crossFeed | sweep | 1 | Off | (none) |
| playback | AN 1/2 | crossFeed | restore | 0 | Off | (none) |
| playback | AN 1/2 | volume | sweep | 0 | -oo | 4020:0000 4061:0000 |
| playback | AN 1/2 | volume | sweep | 0.25 | -32.6 dB | 4020:02fd 4061:02fd |
| playback | AN 1/2 | volume | sweep | 0.5 | -12.1 dB | 4020:1fb1 4061:1fb1 |
| playback | AN 1/2 | volume | sweep | 0.75 | -2.2 dB | 4020:8c69 4061:8c69 |
| playback | AN 1/2 | volume | sweep | 1 | 6.0 dB | 4020:9fec 4061:9fec 47a0:0024 47e1:0024 |
| playback | AN 1/2 | volume | restore | 0.8172 | 0.0 dB | 4020:9000 4061:9000 47a0:0000 47e1:0000 |
| playback | AN 1/2 | pan | sweep | 0 | L100 | 4061:0000 |
| playback | AN 1/2 | pan | sweep | 0.25 | L50 | 4061:8800 |
| playback | AN 1/2 | pan | sweep | 0.75 | R50 | 4020:8800 4061:9000 |
| playback | AN 1/2 | pan | sweep | 1 | R100 | 4020:0000 |
| playback | AN 1/2 | pan | restore | 0.5 | 0 | 4020:9000 |
| playback | AN 1/2 | mute | press | 1 |  | 4020:0000 4061:0000 41a0:0000 41e1:0000 |
| playback | AN 1/2 | mute | press back | 1 |  | 4020:9000 4061:9000 41a0:9000 41e1:9000 |
| playback | AN 1/2 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4047:0000 |
| playback | AN 1/2 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f |
| playback | AN 1/2 | reverbSend | sweep | 0.25 | -49.0 dB | 47a0:0074 47e1:0074 |
| playback | AN 1/2 | reverbSend | sweep | 0.5 | -32.5 dB | 47a0:0309 47e1:0309 |
| playback | AN 1/2 | reverbSend | sweep | 0.75 | -16.5 dB | 47a0:1326 47e1:1326 |
| playback | AN 1/2 | reverbSend | sweep | 1 | 0.0 dB | 47a0:9000 47e1:9000 |
| playback | AN 1/2 | reverbSend | restore | 0 | -oo | 47a0:0000 47e1:0000 |
| playback | AN 1/2 | reverbReturn | sweep | 0.25 | -oo | (none) |
| playback | AN 1/2 | reverbReturn | sweep | 0.5 | -oo | (none) |
| playback | AN 1/2 | reverbReturn | sweep | 0.75 | -oo | (none) |
| playback | AN 1/2 | reverbReturn | sweep | 1 | -oo | (none) |
| playback | AN 1/2 | reverbReturn | restore | 0 | -oo | (none) |
| playback | AN 3/4 | gain | probe | 0.75 |  | (none) |
| playback | AN 3/4 | gain | restore | 0 |  | (none) |
| playback | AN 3/4 | gainRight | probe | 0.75 |  | (none) |
| playback | AN 3/4 | gainRight | restore | 0 |  | (none) |
| playback | AN 3/4 | pad | press | 1 |  | (none) |
| playback | AN 3/4 | pad | press back | 1 |  | (none) |
| playback | AN 3/4 | instrument | press | 1 |  | (none) |
| playback | AN 3/4 | instrument | press back | 1 |  | (none) |
| playback | AN 3/4 | phase | press | 1 |  | 40a2:f000 41a2:f000 |
| playback | AN 3/4 | phase | press back | 1 |  | 40a2:9000 41a2:9000 |
| playback | AN 3/4 | phaseRight | press | 1 |  | 40e3:f000 41e3:f000 |
| playback | AN 3/4 | phaseRight | press back | 1 |  | 40e3:9000 41e3:9000 |
| playback | AN 3/4 | refLevel | probe | 0.75 | 1 | (none) |
| playback | AN 3/4 | refLevel | restore | 1 | 1 | (none) |
| playback | AN 3/4 | autoset | press | 1 |  | (none) |
| playback | AN 3/4 | autoset | press back | 1 |  | (none) |
| playback | AN 3/4 | msProc | press | 1 |  | 40a2:8800 40a3:8800 40e2:8800 40e3:4000 41a2:8800 41a3:8800 41e2:8800 41e3:4000 |
| playback | AN 3/4 | msProc | press back | 1 |  | 40a2:9000 40a3:0000 40e2:0000 40e3:9000 41a2:9000 41a3:0000 41e2:0000 41e3:9000 |
| playback | AN 3/4 | width | probe | 0.75 | 0.50 | 40a2:8c00 40a3:2000 40e2:2000 40e3:8c00 41a2:8c00 41a3:2000 41e2:2000 41e3:8c00 |
| playback | AN 3/4 | width | restore | 1 | 1.00 | 40a2:9000 40a3:0000 40e2:0000 40e3:9000 41a2:9000 41a3:0000 41e2:0000 41e3:9000 |
| playback | AN 3/4 | loopback | press | 1 |  | (none) |
| playback | AN 3/4 | loopback | press back | 1 |  | (none) |
| playback | AN 3/4 | alevEnable | press | 1 |  | (none) |
| playback | AN 3/4 | alevEnable | press back | 1 |  | (none) |
| playback | AN 3/4 | crossFeed | probe | 0.75 | Off | (none) |
| playback | AN 3/4 | crossFeed | restore | 0 |  | (none) |
| playback | AN 3/4 | volume | probe | 0.75 | -2.2 dB | 4022:8c69 4063:8c69 |
| playback | AN 3/4 | volume | restore | 0 | -oo | 4022:0000 4063:0000 |
| playback | AN 3/4 | pan | probe | 0.75 | R50 | (none) |
| playback | AN 3/4 | pan | restore | 0.5 | 0 | (none) |
| playback | AN 3/4 | mute | press | 1 |  | 40a2:0000 40e3:0000 41a2:0000 41e3:0000 |
| playback | AN 3/4 | mute | press back | 1 |  | 40a2:9000 40e3:9000 41a2:9000 41e3:9000 |
| playback | AN 3/4 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| playback | AN 3/4 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | AN 3/4 | reverbSend | probe | 0.75 | -16.5 dB | 47a2:1326 47e3:1326 |
| playback | AN 3/4 | reverbSend | restore | 0 | -oo | 47a2:0000 47e3:0000 |
| playback | AN 3/4 | reverbReturn | probe | 0.75 | -oo | (none) |
| playback | AN 3/4 | reverbReturn | restore | 0 |  | (none) |
| playback | AN 5/6 | gain | probe | 0.75 | 0 | (none) |
| playback | AN 5/6 | gain | restore | 0 | 0 | (none) |
| playback | AN 5/6 | gainRight | probe | 0.75 |  | (none) |
| playback | AN 5/6 | gainRight | restore | 0 |  | (none) |
| playback | AN 5/6 | pad | press | 1 |  | (none) |
| playback | AN 5/6 | pad | press back | 1 |  | (none) |
| playback | AN 5/6 | instrument | press | 1 |  | (none) |
| playback | AN 5/6 | instrument | press back | 1 |  | (none) |
| playback | AN 5/6 | phase | press | 1 |  | 4124:f000 41a4:f000 |
| playback | AN 5/6 | phase | press back | 1 |  | 4124:9000 41a4:9000 |
| playback | AN 5/6 | phaseRight | press | 1 |  | 4165:f000 41e5:f000 |
| playback | AN 5/6 | phaseRight | press back | 1 |  | 4165:9000 41e5:9000 |
| playback | AN 5/6 | refLevel | probe | 0.75 | 1 | (none) |
| playback | AN 5/6 | refLevel | restore | 1 | 1 | (none) |
| playback | AN 5/6 | autoset | press | 1 |  | (none) |
| playback | AN 5/6 | autoset | press back | 1 |  | (none) |
| playback | AN 5/6 | msProc | press | 1 |  | 4124:8800 4125:8800 4164:8800 4165:4000 41a4:8800 41a5:8800 41e4:8800 41e5:4000 |
| playback | AN 5/6 | msProc | press back | 1 |  | 4124:9000 4125:0000 4164:0000 4165:9000 41a4:9000 41a5:0000 41e4:0000 41e5:9000 |
| playback | AN 5/6 | width | probe | 0.75 | 0.50 | 4124:8c00 4125:2000 4164:2000 4165:8c00 41a4:8c00 41a5:2000 41e4:2000 41e5:8c00 |
| playback | AN 5/6 | width | restore | 1 | 1.00 | 4124:9000 4125:0000 4164:0000 4165:9000 41a4:9000 41a5:0000 41e4:0000 41e5:9000 |
| playback | AN 5/6 | loopback | press | 1 |  | (none) |
| playback | AN 5/6 | loopback | press back | 1 |  | (none) |
| playback | AN 5/6 | alevEnable | press | 1 |  | (none) |
| playback | AN 5/6 | alevEnable | press back | 1 |  | (none) |
| playback | AN 5/6 | crossFeed | probe | 0.75 | Off | (none) |
| playback | AN 5/6 | crossFeed | restore | 0 | Off | (none) |
| playback | AN 5/6 | volume | probe | 0.75 | -2.2 dB | 4024:8c69 4065:8c69 |
| playback | AN 5/6 | volume | restore | 0 | -oo | 4024:0000 4065:0000 |
| playback | AN 5/6 | pan | probe | 0.75 | R50 | (none) |
| playback | AN 5/6 | pan | restore | 0.5 | 0 | (none) |
| playback | AN 5/6 | mute | press | 1 |  | 4124:0000 4165:0000 41a4:0000 41e5:0000 |
| playback | AN 5/6 | mute | press back | 1 |  | 4124:9000 4165:9000 41a4:9000 41e5:9000 |
| playback | AN 5/6 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| playback | AN 5/6 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | AN 5/6 | reverbSend | probe | 0.75 | -16.5 dB | 47a4:1326 47e5:1326 |
| playback | AN 5/6 | reverbSend | restore | 0 | -oo | 47a4:0000 47e5:0000 |
| playback | AN 5/6 | reverbReturn | probe | 0.75 | -oo | (none) |
| playback | AN 5/6 | reverbReturn | restore | 0 | -oo | (none) |
| playback | PH 7/8 | gain | probe | 0.75 | 0 | (none) |
| playback | PH 7/8 | gain | restore | 0 | 0 | (none) |
| playback | PH 7/8 | gainRight | probe | 0.75 | 0 | (none) |
| playback | PH 7/8 | gainRight | restore | 0 | 0 | (none) |
| playback | PH 7/8 | pad | press | 1 |  | (none) |
| playback | PH 7/8 | pad | press back | 1 |  | (none) |
| playback | PH 7/8 | instrument | press | 1 |  | (none) |
| playback | PH 7/8 | instrument | press back | 1 |  | (none) |
| playback | PH 7/8 | phase | press | 1 |  | 41a6:f000 |
| playback | PH 7/8 | phase | press back | 1 |  | 41a6:9000 |
| playback | PH 7/8 | phaseRight | press | 1 |  | 41e7:f000 |
| playback | PH 7/8 | phaseRight | press back | 1 |  | 41e7:9000 |
| playback | PH 7/8 | refLevel | probe | 0.75 | 1 | (none) |
| playback | PH 7/8 | refLevel | restore | 1 | 1 | (none) |
| playback | PH 7/8 | autoset | press | 1 |  | (none) |
| playback | PH 7/8 | autoset | press back | 1 |  | (none) |
| playback | PH 7/8 | msProc | press | 1 |  | 41a6:8800 41a7:8800 41e6:8800 41e7:4000 |
| playback | PH 7/8 | msProc | press back | 1 |  | 41a6:9000 41a7:0000 41e6:0000 41e7:9000 |
| playback | PH 7/8 | width | probe | 0.75 | 0.50 | 41a6:8c00 41a7:2000 41e6:2000 41e7:8c00 |
| playback | PH 7/8 | width | restore | 1 | 1.00 | 41a6:9000 41a7:0000 41e6:0000 41e7:9000 |
| playback | PH 7/8 | loopback | press | 1 |  | (none) |
| playback | PH 7/8 | loopback | press back | 1 |  | (none) |
| playback | PH 7/8 | alevEnable | press | 1 |  | (none) |
| playback | PH 7/8 | alevEnable | press back | 1 |  | (none) |
| playback | PH 7/8 | crossFeed | probe | 0.75 | Off | (none) |
| playback | PH 7/8 | crossFeed | restore | 0 | Off | (none) |
| playback | PH 7/8 | volume | probe | 0.75 | -2.2 dB | 4026:8c69 4067:8c69 |
| playback | PH 7/8 | volume | restore | 0 | -oo | 4026:0000 4067:0000 |
| playback | PH 7/8 | pan | probe | 0.75 | R50 | (none) |
| playback | PH 7/8 | pan | restore | 0.5 | 0 | (none) |
| playback | PH 7/8 | mute | press | 1 |  | 41a6:0000 41e7:0000 |
| playback | PH 7/8 | mute | press back | 1 |  | 41a6:9000 41e7:9000 |
| playback | PH 7/8 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| playback | PH 7/8 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | PH 7/8 | reverbSend | probe | 0.75 | -16.5 dB | 47a6:1326 47e7:1326 |
| playback | PH 7/8 | reverbSend | restore | 0 | -oo | 47a6:0000 47e7:0000 |
| playback | PH 7/8 | reverbReturn | probe | 0.75 | -oo | (none) |
| playback | PH 7/8 | reverbReturn | restore | 0 | -oo | (none) |
| playback | SPDIF | gain | probe | 0.75 | 0 | (none) |
| playback | SPDIF | gain | restore | 0 | 0 | (none) |
| playback | SPDIF | gainRight | probe | 0.75 | 0 | (none) |
| playback | SPDIF | gainRight | restore | 0 | 0 | (none) |
| playback | SPDIF | pad | press | 1 |  | (none) |
| playback | SPDIF | pad | press back | 1 |  | (none) |
| playback | SPDIF | instrument | press | 1 |  | (none) |
| playback | SPDIF | instrument | press back | 1 |  | (none) |
| playback | SPDIF | phase | press | 1 |  | 41a8:f000 4228:f000 |
| playback | SPDIF | phase | press back | 1 |  | 41a8:9000 4228:9000 |
| playback | SPDIF | phaseRight | press | 1 |  | 41e9:f000 4269:f000 |
| playback | SPDIF | phaseRight | press back | 1 |  | 41e9:9000 4269:9000 |
| playback | SPDIF | refLevel | probe | 0.75 | 1 | (none) |
| playback | SPDIF | refLevel | restore | 1 | 1 | (none) |
| playback | SPDIF | autoset | press | 1 |  | (none) |
| playback | SPDIF | autoset | press back | 1 |  | (none) |
| playback | SPDIF | msProc | press | 1 |  | 41a8:8800 41a9:8800 41e8:8800 41e9:4000 4228:8800 4229:8800 4268:8800 4269:4000 |
| playback | SPDIF | msProc | press back | 1 |  | 41a8:9000 41a9:0000 41e8:0000 41e9:9000 4228:9000 4229:0000 4268:0000 4269:9000 |
| playback | SPDIF | width | probe | 0.75 | 0.50 | 41a8:8c00 41a9:2000 41e8:2000 41e9:8c00 4228:8c00 4229:2000 4268:2000 4269:8c00 |
| playback | SPDIF | width | restore | 1 | 1.00 | 41a8:9000 41a9:0000 41e8:0000 41e9:9000 4228:9000 4229:0000 4268:0000 4269:9000 |
| playback | SPDIF | loopback | press | 1 |  | (none) |
| playback | SPDIF | loopback | press back | 1 |  | (none) |
| playback | SPDIF | alevEnable | press | 1 |  | (none) |
| playback | SPDIF | alevEnable | press back | 1 |  | (none) |
| playback | SPDIF | crossFeed | probe | 0.75 | Off | (none) |
| playback | SPDIF | crossFeed | restore | 0 | Off | (none) |
| playback | SPDIF | volume | probe | 0.75 | -2.2 dB | 4028:8c69 4069:8c69 |
| playback | SPDIF | volume | restore | 0 | -oo | 4028:0000 4069:0000 |
| playback | SPDIF | pan | probe | 0.75 | R50 | (none) |
| playback | SPDIF | pan | restore | 0.5 | 0 | (none) |
| playback | SPDIF | mute | press | 1 |  | 41a8:0000 41e9:0000 4228:0000 4269:0000 |
| playback | SPDIF | mute | press back | 1 |  | 41a8:9000 41e9:9000 4228:9000 4269:9000 |
| playback | SPDIF | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| playback | SPDIF | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | SPDIF | reverbSend | probe | 0.75 | -16.5 dB | 47a8:1326 47e9:1326 |
| playback | SPDIF | reverbSend | restore | 0 | -oo | 47a8:0000 47e9:0000 |
| playback | SPDIF | reverbReturn | probe | 0.75 | -oo | (none) |
| playback | SPDIF | reverbReturn | restore | 0 | -oo | (none) |
| playback | AS 1/2 | gain | probe | 0.75 | 0 | (none) |
| playback | AS 1/2 | gain | restore | 0 | 0 | (none) |
| playback | AS 1/2 | gainRight | probe | 0.75 | 0 | (none) |
| playback | AS 1/2 | gainRight | restore | 0 | 0 | (none) |
| playback | AS 1/2 | pad | press | 1 |  | (none) |
| playback | AS 1/2 | pad | press back | 1 |  | (none) |
| playback | AS 1/2 | instrument | press | 1 |  | (none) |
| playback | AS 1/2 | instrument | press back | 1 |  | (none) |
| playback | AS 1/2 | phase | press | 1 |  | 41aa:f000 42aa:f000 |
| playback | AS 1/2 | phase | press back | 1 |  | 41aa:9000 42aa:9000 |
| playback | AS 1/2 | phaseRight | press | 1 |  | 41eb:f000 42eb:f000 |
| playback | AS 1/2 | phaseRight | press back | 1 |  | 41eb:9000 42eb:9000 |
| playback | AS 1/2 | refLevel | probe | 0.75 | 1 | (none) |
| playback | AS 1/2 | refLevel | restore | 1 | 1 | (none) |
| playback | AS 1/2 | autoset | press | 1 |  | (none) |
| playback | AS 1/2 | autoset | press back | 1 |  | (none) |
| playback | AS 1/2 | msProc | press | 1 |  | 41aa:8800 41ab:8800 41ea:8800 41eb:4000 42aa:8800 42ab:8800 42ea:8800 42eb:4000 |
| playback | AS 1/2 | msProc | press back | 1 |  | 41aa:9000 41ab:0000 41ea:0000 41eb:9000 42aa:9000 42ab:0000 42ea:0000 42eb:9000 |
| playback | AS 1/2 | width | probe | 0.75 | 0.50 | 41aa:8c00 41ab:2000 41ea:2000 41eb:8c00 42aa:8c00 42ab:2000 42ea:2000 42eb:8c00 |
| playback | AS 1/2 | width | restore | 1 | 1.00 | 41aa:9000 41ab:0000 41ea:0000 41eb:9000 42aa:9000 42ab:0000 42ea:0000 42eb:9000 |
| playback | AS 1/2 | loopback | press | 1 |  | (none) |
| playback | AS 1/2 | loopback | press back | 1 |  | (none) |
| playback | AS 1/2 | alevEnable | press | 1 |  | (none) |
| playback | AS 1/2 | alevEnable | press back | 1 |  | (none) |
| playback | AS 1/2 | crossFeed | probe | 0.75 | Off | (none) |
| playback | AS 1/2 | crossFeed | restore | 0 | Off | (none) |
| playback | AS 1/2 | volume | probe | 0.75 | -2.2 dB | 402a:8c69 406b:8c69 |
| playback | AS 1/2 | volume | restore | 0 | -oo | 402a:0000 406b:0000 |
| playback | AS 1/2 | pan | probe | 0.75 | R50 | (none) |
| playback | AS 1/2 | pan | restore | 0.5 | 0 | (none) |
| playback | AS 1/2 | mute | press | 1 |  | 41aa:0000 41eb:0000 42aa:0000 42eb:0000 |
| playback | AS 1/2 | mute | press back | 1 |  | 41aa:9000 41eb:9000 42aa:9000 42eb:9000 |
| playback | AS 1/2 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| playback | AS 1/2 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | AS 1/2 | reverbSend | probe | 0.75 | -16.5 dB | 47aa:1326 47eb:1326 |
| playback | AS 1/2 | reverbSend | restore | 0 | -oo | 47aa:0000 47eb:0000 |
| playback | AS 1/2 | reverbReturn | probe | 0.75 | -oo | (none) |
| playback | AS 1/2 | reverbReturn | restore | 0 | -oo | (none) |
| playback | ADAT 3/4 | gain | probe | 0.75 | 0 | (none) |
| playback | ADAT 3/4 | gain | restore | 0 | 0 | (none) |
| playback | ADAT 3/4 | gainRight | probe | 0.75 | 0 | (none) |
| playback | ADAT 3/4 | gainRight | restore | 0 | 0 | (none) |
| playback | ADAT 3/4 | pad | press | 1 |  | (none) |
| playback | ADAT 3/4 | pad | press back | 1 |  | (none) |
| playback | ADAT 3/4 | instrument | press | 1 |  | (none) |
| playback | ADAT 3/4 | instrument | press back | 1 |  | (none) |
| playback | ADAT 3/4 | phase | press | 1 |  | 41ac:f000 432c:f000 |
| playback | ADAT 3/4 | phase | press back | 1 |  | 41ac:9000 432c:9000 |
| playback | ADAT 3/4 | phaseRight | press | 1 |  | 41ed:f000 436d:f000 |
| playback | ADAT 3/4 | phaseRight | press back | 1 |  | 41ed:9000 436d:9000 |
| playback | ADAT 3/4 | refLevel | probe | 0.75 | 1 | (none) |
| playback | ADAT 3/4 | refLevel | restore | 1 | 1 | (none) |
| playback | ADAT 3/4 | autoset | press | 1 |  | (none) |
| playback | ADAT 3/4 | autoset | press back | 1 |  | (none) |
| playback | ADAT 3/4 | msProc | press | 1 |  | 41ac:8800 41ad:8800 41ec:8800 41ed:4000 432c:8800 432d:8800 436c:8800 436d:4000 |
| playback | ADAT 3/4 | msProc | press back | 1 |  | 41ac:9000 41ad:0000 41ec:0000 41ed:9000 432c:9000 432d:0000 436c:0000 436d:9000 |
| playback | ADAT 3/4 | width | probe | 0.75 | 0.50 | 41ac:8c00 41ad:2000 41ec:2000 41ed:8c00 432c:8c00 432d:2000 436c:2000 436d:8c00 |
| playback | ADAT 3/4 | width | restore | 1 | 1.00 | 41ac:9000 41ad:0000 41ec:0000 41ed:9000 432c:9000 432d:0000 436c:0000 436d:9000 |
| playback | ADAT 3/4 | loopback | press | 1 |  | (none) |
| playback | ADAT 3/4 | loopback | press back | 1 |  | (none) |
| playback | ADAT 3/4 | alevEnable | press | 1 |  | (none) |
| playback | ADAT 3/4 | alevEnable | press back | 1 |  | (none) |
| playback | ADAT 3/4 | crossFeed | probe | 0.75 | Off | (none) |
| playback | ADAT 3/4 | crossFeed | restore | 0 | Off | (none) |
| playback | ADAT 3/4 | volume | probe | 0.75 | -2.2 dB | 402c:8c69 406d:8c69 |
| playback | ADAT 3/4 | volume | restore | 0 | -oo | 402c:0000 406d:0000 |
| playback | ADAT 3/4 | pan | probe | 0.75 | R50 | (none) |
| playback | ADAT 3/4 | pan | restore | 0.5 | 0 | (none) |
| playback | ADAT 3/4 | mute | press | 1 |  | 41ac:0000 41ed:0000 432c:0000 436d:0000 |
| playback | ADAT 3/4 | mute | press back | 1 |  | 41ac:9000 41ed:9000 432c:9000 436d:9000 |
| playback | ADAT 3/4 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| playback | ADAT 3/4 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | ADAT 3/4 | reverbSend | probe | 0.75 | -16.5 dB | 47ac:1326 47ed:1326 |
| playback | ADAT 3/4 | reverbSend | restore | 0 | -oo | 47ac:0000 47ed:0000 |
| playback | ADAT 3/4 | reverbReturn | probe | 0.75 | -oo | (none) |
| playback | ADAT 3/4 | reverbReturn | restore | 0 | -oo | (none) |
| playback | ADAT 5/6 | gain | probe | 0.75 | 0 | (none) |
| playback | ADAT 5/6 | gain | restore | 0 | 0 | (none) |
| playback | ADAT 5/6 | gainRight | probe | 0.75 | 0 | (none) |
| playback | ADAT 5/6 | gainRight | restore | 0 | 0 | (none) |
| playback | ADAT 5/6 | pad | press | 1 |  | (none) |
| playback | ADAT 5/6 | pad | press back | 1 |  | (none) |
| playback | ADAT 5/6 | instrument | press | 1 |  | (none) |
| playback | ADAT 5/6 | instrument | press back | 1 |  | (none) |
| playback | ADAT 5/6 | phase | press | 1 |  | 41ae:f000 43ae:f000 |
| playback | ADAT 5/6 | phase | press back | 1 |  | 41ae:9000 43ae:9000 |
| playback | ADAT 5/6 | phaseRight | press | 1 |  | 41ef:f000 43ef:f000 |
| playback | ADAT 5/6 | phaseRight | press back | 1 |  | 41ef:9000 43ef:9000 |
| playback | ADAT 5/6 | refLevel | probe | 0.75 | 1 | (none) |
| playback | ADAT 5/6 | refLevel | restore | 1 | 1 | (none) |
| playback | ADAT 5/6 | autoset | press | 1 |  | (none) |
| playback | ADAT 5/6 | autoset | press back | 1 |  | (none) |
| playback | ADAT 5/6 | msProc | press | 1 |  | 41ae:8800 41af:8800 41ee:8800 41ef:4000 43ae:8800 43af:8800 43ee:8800 43ef:4000 |
| playback | ADAT 5/6 | msProc | press back | 1 |  | 41ae:9000 41af:0000 41ee:0000 41ef:9000 43ae:9000 43af:0000 43ee:0000 43ef:9000 |
| playback | ADAT 5/6 | width | probe | 0.75 | 0.50 | 41ae:8c00 41af:2000 41ee:2000 41ef:8c00 43ae:8c00 43af:2000 43ee:2000 43ef:8c00 |
| playback | ADAT 5/6 | width | restore | 1 | 1.00 | 41ae:9000 41af:0000 41ee:0000 41ef:9000 43ae:9000 43af:0000 43ee:0000 43ef:9000 |
| playback | ADAT 5/6 | loopback | press | 1 |  | (none) |
| playback | ADAT 5/6 | loopback | press back | 1 |  | (none) |
| playback | ADAT 5/6 | alevEnable | press | 1 |  | (none) |
| playback | ADAT 5/6 | alevEnable | press back | 1 |  | (none) |
| playback | ADAT 5/6 | crossFeed | probe | 0.75 | Off | (none) |
| playback | ADAT 5/6 | crossFeed | restore | 0 | Off | (none) |
| playback | ADAT 5/6 | volume | probe | 0.75 | -2.2 dB | 402e:8c69 406f:8c69 |
| playback | ADAT 5/6 | volume | restore | 0 | -oo | 402e:0000 406f:0000 |
| playback | ADAT 5/6 | pan | probe | 0.75 | R50 | (none) |
| playback | ADAT 5/6 | pan | restore | 0.5 | 0 | (none) |
| playback | ADAT 5/6 | mute | press | 1 |  | 41ae:0000 41ef:0000 43ae:0000 43ef:0000 |
| playback | ADAT 5/6 | mute | press back | 1 |  | 41ae:9000 41ef:9000 43ae:9000 43ef:9000 |
| playback | ADAT 5/6 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| playback | ADAT 5/6 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | ADAT 5/6 | reverbSend | probe | 0.75 | -16.5 dB | 47ae:1326 47ef:1326 |
| playback | ADAT 5/6 | reverbSend | restore | 0 | -oo | 47ae:0000 47ef:0000 |
| playback | ADAT 5/6 | reverbReturn | probe | 0.75 | -oo | (none) |
| playback | ADAT 5/6 | reverbReturn | restore | 0 | -oo | (none) |
| playback | ADAT 7/8 | gain | probe | 0.75 | 0 | (none) |
| playback | ADAT 7/8 | gain | restore | 0 | 0 | (none) |
| playback | ADAT 7/8 | gainRight | probe | 0.75 | 0 | (none) |
| playback | ADAT 7/8 | gainRight | restore | 0 | 0 | (none) |
| playback | ADAT 7/8 | pad | press | 1 |  | (none) |
| playback | ADAT 7/8 | pad | press back | 1 |  | (none) |
| playback | ADAT 7/8 | instrument | press | 1 |  | (none) |
| playback | ADAT 7/8 | instrument | press back | 1 |  | (none) |
| playback | ADAT 7/8 | phase | press | 1 |  | 41b0:f000 4430:f000 |
| playback | ADAT 7/8 | phase | press back | 1 |  | 41b0:9000 4430:9000 |
| playback | ADAT 7/8 | phaseRight | press | 1 |  | 41f1:f000 4471:f000 |
| playback | ADAT 7/8 | phaseRight | press back | 1 |  | 41f1:9000 4471:9000 |
| playback | ADAT 7/8 | refLevel | probe | 0.75 | 1 | (none) |
| playback | ADAT 7/8 | refLevel | restore | 1 | 1 | (none) |
| playback | ADAT 7/8 | autoset | press | 1 |  | (none) |
| playback | ADAT 7/8 | autoset | press back | 1 |  | (none) |
| playback | ADAT 7/8 | msProc | press | 1 |  | 41b0:8800 41b1:8800 41f0:8800 41f1:4000 4430:8800 4431:8800 4470:8800 4471:4000 |
| playback | ADAT 7/8 | msProc | press back | 1 |  | 41b0:9000 41b1:0000 41f0:0000 41f1:9000 4430:9000 4431:0000 4470:0000 4471:9000 |
| playback | ADAT 7/8 | width | probe | 0.75 | 0.50 | 41b0:8c00 41b1:2000 41f0:2000 41f1:8c00 4430:8c00 4431:2000 4470:2000 4471:8c00 |
| playback | ADAT 7/8 | width | restore | 1 | 1.00 | 41b0:9000 41b1:0000 41f0:0000 41f1:9000 4430:9000 4431:0000 4470:0000 4471:9000 |
| playback | ADAT 7/8 | loopback | press | 1 |  | (none) |
| playback | ADAT 7/8 | loopback | press back | 1 |  | (none) |
| playback | ADAT 7/8 | alevEnable | press | 1 |  | (none) |
| playback | ADAT 7/8 | alevEnable | press back | 1 |  | (none) |
| playback | ADAT 7/8 | crossFeed | probe | 0.75 | Off | (none) |
| playback | ADAT 7/8 | crossFeed | restore | 0 | Off | (none) |
| playback | ADAT 7/8 | volume | probe | 0.75 | -2.2 dB | 4030:8c69 4071:8c69 |
| playback | ADAT 7/8 | volume | restore | 0 | -oo | 4030:0000 4071:0000 |
| playback | ADAT 7/8 | pan | probe | 0.75 | R50 | (none) |
| playback | ADAT 7/8 | pan | restore | 0.5 | 0 | (none) |
| playback | ADAT 7/8 | mute | press | 1 |  | 41b0:0000 41f1:0000 4430:0000 4471:0000 |
| playback | ADAT 7/8 | mute | press back | 1 |  | 41b0:9000 41f1:9000 4430:9000 4471:9000 |
| playback | ADAT 7/8 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| playback | ADAT 7/8 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| playback | ADAT 7/8 | reverbSend | probe | 0.75 | -16.5 dB | 47b0:1326 47f1:1326 |
| playback | ADAT 7/8 | reverbSend | restore | 0 | -oo | 47b0:0000 47f1:0000 |
| playback | ADAT 7/8 | reverbReturn | probe | 0.75 | -oo | (none) |
| playback | ADAT 7/8 | reverbReturn | restore | 0 | -oo | (none) |
| output | AN 1/2 | gain | sweep | 0.25 | 0 | (none) |
| output | AN 1/2 | gain | sweep | 0.5 | 0 | (none) |
| output | AN 1/2 | gain | sweep | 0.75 | 0 | (none) |
| output | AN 1/2 | gain | sweep | 1 | 0 | (none) |
| output | AN 1/2 | gain | restore | 0 | 0 | (none) |
| output | AN 1/2 | gainRight | sweep | 0.25 | 0 | (none) |
| output | AN 1/2 | gainRight | sweep | 0.5 | 0 | (none) |
| output | AN 1/2 | gainRight | sweep | 0.75 | 0 | (none) |
| output | AN 1/2 | gainRight | sweep | 1 | 0 | (none) |
| output | AN 1/2 | gainRight | restore | 0 | 0 | (none) |
| output | AN 1/2 | pad | press | 1 |  | (none) |
| output | AN 1/2 | pad | press back | 1 |  | (none) |
| output | AN 1/2 | instrument | press | 1 |  | (none) |
| output | AN 1/2 | instrument | press back | 1 |  | (none) |
| output | AN 1/2 | phase | press | 1 |  | 1207:0001 |
| output | AN 1/2 | phase | press back | 1 |  | 1207:0000 |
| output | AN 1/2 | phaseRight | press | 1 |  | 1307:0001 |
| output | AN 1/2 | phaseRight | press back | 1 |  | 1307:0000 |
| output | AN 1/2 | refLevel | sweep | 0 | 0 | 1208:0000 1308:0000 1408:0000 1508:0000 1608:0000 1708:0000 |
| output | AN 1/2 | refLevel | sweep | 0.25 | 0 | (none) |
| output | AN 1/2 | refLevel | sweep | 0.5 | 1 | 1208:0001 1308:0001 1408:0001 1508:0001 1608:0001 1708:0001 |
| output | AN 1/2 | refLevel | sweep | 0.75 | 1 | (none) |
| output | AN 1/2 | refLevel | restore | 1 | 1 | (none) |
| output | AN 1/2 | autoset | press | 1 |  | (none) |
| output | AN 1/2 | autoset | press back | 1 |  | (none) |
| output | AN 1/2 | msProc | press | 1 |  | (none) |
| output | AN 1/2 | msProc | press back | 1 |  | (none) |
| output | AN 1/2 | width | sweep | 0 | -1.00 | (none) |
| output | AN 1/2 | width | sweep | 0.25 | -0.50 | (none) |
| output | AN 1/2 | width | sweep | 0.5 | 0.00 | (none) |
| output | AN 1/2 | width | sweep | 0.75 | 0.50 | (none) |
| output | AN 1/2 | width | restore | 1 | 1.00 | (none) |
| output | AN 1/2 | loopback | press | 1 |  | req21 v=0x0001 i=0x0000 req21 v=0x0001 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 1/2 | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 1/2 | alevEnable | press | 1 |  | 1280:0001 1380:0001 |
| output | AN 1/2 | alevEnable | press back | 1 |  | 1280:0000 1380:0000 |
| output | AN 1/2 | crossFeed | sweep | 0.25 | 1 | (none) |
| output | AN 1/2 | crossFeed | sweep | 0.5 | 3 | (none) |
| output | AN 1/2 | crossFeed | sweep | 0.75 | 4 | (none) |
| output | AN 1/2 | crossFeed | sweep | 1 | 5 | (none) |
| output | AN 1/2 | crossFeed | restore | 0 | Off | (none) |
| output | AN 1/2 | volume | sweep | 0 | -oo | 1200:fd76 1300:fd76 |
| output | AN 1/2 | volume | sweep | 0.25 | -32.6 dB | 1200:feba 1300:feba |
| output | AN 1/2 | volume | sweep | 0.5 | -12.1 dB | 1200:ff87 1300:ff87 |
| output | AN 1/2 | volume | sweep | 0.75 | -2.2 dB | 1200:ffea 1300:ffea |
| output | AN 1/2 | volume | sweep | 1 | 6.0 dB | 1200:003c 1300:003c |
| output | AN 1/2 | volume | restore | 0.3501 | -23.0 dB | 1200:ff1a 1300:ff1a |
| output | AN 1/2 | pan | sweep | 0 | L100 | 1201:ff9c 1301:ff9c |
| output | AN 1/2 | pan | sweep | 0.25 | L50 | 1201:ffce 1301:ffce |
| output | AN 1/2 | pan | sweep | 0.5 | 0 | 1201:0000 1301:0000 |
| output | AN 1/2 | pan | sweep | 0.75 | R50 | 1201:0032 1301:0032 |
| output | AN 1/2 | pan | sweep | 1 | R100 | 1201:0064 1301:0064 |
| output | AN 1/2 | pan | restore | 0.475 | L5 | 1201:fffb 1301:fffb |
| output | AN 1/2 | mute | press | 1 |  | 1202:0001 1302:0001 |
| output | AN 1/2 | mute | press back | 1 |  | 1202:0000 1302:0000 |
| output | AN 1/2 | reverbSend | sweep | 0.25 | -49.0 dB | (none) |
| output | AN 1/2 | reverbSend | sweep | 0.5 | -32.5 dB | (none) |
| output | AN 1/2 | reverbSend | sweep | 0.75 | -16.5 dB | (none) |
| output | AN 1/2 | reverbSend | sweep | 1 | 0.0 dB | (none) |
| output | AN 1/2 | reverbSend | restore | 0 | -oo | (none) |
| output | AN 1/2 | reverbReturn | sweep | 0.25 | -49.0 dB | 1203:fe16 1303:fe16 |
| output | AN 1/2 | reverbReturn | sweep | 0.5 | -32.5 dB | 1203:febb 1303:febb |
| output | AN 1/2 | reverbReturn | sweep | 0.75 | -16.5 dB | 1203:ff5b 1303:ff5b |
| output | AN 1/2 | reverbReturn | sweep | 1 | 0.0 dB | 1203:0000 1303:0000 |
| output | AN 1/2 | reverbReturn | restore | 0 | -oo | 1203:fd76 1303:fd76 |
| output | AN 3/4 | gain | probe | 0.75 | 0 | (none) |
| output | AN 3/4 | gain | restore | 0 | 0 | (none) |
| output | AN 3/4 | gainRight | probe | 0.75 | 0 | (none) |
| output | AN 3/4 | gainRight | restore | 0 | 0 | (none) |
| output | AN 3/4 | pad | press | 1 |  | (none) |
| output | AN 3/4 | pad | press back | 1 |  | (none) |
| output | AN 3/4 | instrument | press | 1 |  | (none) |
| output | AN 3/4 | instrument | press back | 1 |  | (none) |
| output | AN 3/4 | phase | press | 1 |  | 1407:0001 |
| output | AN 3/4 | phase | press back | 1 |  | 1407:0000 |
| output | AN 3/4 | phaseRight | press | 1 |  | 1507:0001 |
| output | AN 3/4 | phaseRight | press back | 1 |  | 1507:0000 |
| output | AN 3/4 | refLevel | probe | 0.75 | 1 | (none) |
| output | AN 3/4 | refLevel | restore | 1 | 1 | (none) |
| output | AN 3/4 | autoset | press | 1 |  | (none) |
| output | AN 3/4 | autoset | press back | 1 |  | (none) |
| output | AN 3/4 | msProc | press | 1 |  | (none) |
| output | AN 3/4 | msProc | press back | 1 |  | (none) |
| output | AN 3/4 | width | probe | 0.75 | 0.50 | (none) |
| output | AN 3/4 | width | restore | 1 | 1.00 | (none) |
| output | AN 3/4 | loopback | press | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0001 i=0x0002 req21 v=0x0001 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 3/4 | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 3/4 | alevEnable | press | 1 |  | 1480:0001 1580:0001 |
| output | AN 3/4 | alevEnable | press back | 1 |  | 1480:0000 1580:0000 |
| output | AN 3/4 | crossFeed | probe | 0.75 | 4 | (none) |
| output | AN 3/4 | crossFeed | restore | 0 | Off | (none) |
| output | AN 3/4 | volume | probe | 0.75 | -2.2 dB | 1400:ffea 1500:ffea |
| output | AN 3/4 | volume | restore | 0.8172 | 0.0 dB | 1400:0000 1500:0000 |
| output | AN 3/4 | pan | probe | 0.75 | R50 | 1401:0032 1501:0032 |
| output | AN 3/4 | pan | restore | 0.5 | 0 | 1401:0000 1501:0000 |
| output | AN 3/4 | mute | press | 1 |  | 1402:0001 1502:0001 |
| output | AN 3/4 | mute | press back | 1 |  | 1402:0000 1502:0000 |
| output | AN 3/4 | reverbSend | probe | 0.75 | -16.5 dB | (none) |
| output | AN 3/4 | reverbSend | restore | 0 | -oo | (none) |
| output | AN 3/4 | reverbReturn | probe | 0.75 | -16.5 dB | 1403:ff5b 1503:ff5b |
| output | AN 3/4 | reverbReturn | restore | 0 | -oo | 1403:fd76 1503:fd76 |
| output | AN 5/6 | gain | probe | 0.75 | 0 | (none) |
| output | AN 5/6 | gain | restore | 0 | 0 | (none) |
| output | AN 5/6 | gainRight | probe | 0.75 | 0 | (none) |
| output | AN 5/6 | gainRight | restore | 0 | 0 | (none) |
| output | AN 5/6 | pad | press | 1 |  | (none) |
| output | AN 5/6 | pad | press back | 1 |  | (none) |
| output | AN 5/6 | instrument | press | 1 |  | (none) |
| output | AN 5/6 | instrument | press back | 1 |  | (none) |
| output | AN 5/6 | phase | press | 1 |  | 1607:0001 |
| output | AN 5/6 | phase | press back | 1 |  | 1607:0000 |
| output | AN 5/6 | phaseRight | press | 1 |  | 1707:0001 |
| output | AN 5/6 | phaseRight | press back | 1 |  | 1707:0000 |
| output | AN 5/6 | refLevel | probe | 0.75 | 1 | (none) |
| output | AN 5/6 | refLevel | restore | 1 | 1 | (none) |
| output | AN 5/6 | autoset | press | 1 |  | (none) |
| output | AN 5/6 | autoset | press back | 1 |  | (none) |
| output | AN 5/6 | msProc | press | 1 |  | (none) |
| output | AN 5/6 | msProc | press back | 1 |  | (none) |
| output | AN 5/6 | width | probe | 0.75 | 0.50 | (none) |
| output | AN 5/6 | width | restore | 1 | 1.00 | (none) |
| output | AN 5/6 | loopback | press | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0001 i=0x0004 req21 v=0x0001 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 5/6 | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AN 5/6 | alevEnable | press | 1 |  | 1680:0001 1780:0001 |
| output | AN 5/6 | alevEnable | press back | 1 |  | 1680:0000 1780:0000 |
| output | AN 5/6 | crossFeed | probe | 0.75 | 4 | (none) |
| output | AN 5/6 | crossFeed | restore | 0 | Off | (none) |
| output | AN 5/6 | volume | probe | 0.75 | -2.2 dB | 1600:ffea 1700:ffea |
| output | AN 5/6 | volume | restore | 0.8172 | 0.0 dB | 1600:0000 1700:0000 |
| output | AN 5/6 | pan | probe | 0.75 | R50 | 1601:0032 1701:0032 |
| output | AN 5/6 | pan | restore | 0.5 | 0 | 1601:0000 1701:0000 |
| output | AN 5/6 | mute | press | 1 |  | 1602:0001 1702:0001 |
| output | AN 5/6 | mute | press back | 1 |  | 1602:0000 1702:0000 |
| output | AN 5/6 | reverbSend | probe | 0.75 | -16.5 dB | (none) |
| output | AN 5/6 | reverbSend | restore | 0 | -oo | (none) |
| output | AN 5/6 | reverbReturn | probe | 0.75 | -16.5 dB | 1603:ff5b 1703:ff5b |
| output | AN 5/6 | reverbReturn | restore | 0 | -oo | 1603:fd76 1703:fd76 |
| output | Main | gain | probe | 0.75 | 0 | (none) |
| output | Main | gain | restore | 0 | 0 | (none) |
| output | Main | gainRight | probe | 0.75 | 0 | (none) |
| output | Main | gainRight | restore | 0 | 0 | (none) |
| output | Main | pad | press | 1 |  | (none) |
| output | Main | pad | press back | 1 |  | (none) |
| output | Main | instrument | press | 1 |  | (none) |
| output | Main | instrument | press back | 1 |  | (none) |
| output | Main | phase | press | 1 |  | 1807:0001 |
| output | Main | phase | press back | 1 |  | 1807:0000 |
| output | Main | phaseRight | press | 1 |  | 1907:0001 |
| output | Main | phaseRight | press back | 1 |  | 1907:0000 |
| output | Main | refLevel | probe | 0.75 | 1 | (none) |
| output | Main | refLevel | restore | 1 | 1 | (none) |
| output | Main | autoset | press | 1 |  | (none) |
| output | Main | autoset | press back | 1 |  | (none) |
| output | Main | msProc | press | 1 |  | (none) |
| output | Main | msProc | press back | 1 |  | (none) |
| output | Main | width | probe | 0.75 | 0.50 | (none) |
| output | Main | width | restore | 1 | 1.00 | (none) |
| output | Main | loopback | press | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0001 i=0x0006 req21 v=0x0001 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | Main | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | Main | alevEnable | press | 1 |  | 1880:0001 1980:0001 |
| output | Main | alevEnable | press back | 1 |  | 1880:0000 1980:0000 |
| output | Main | crossFeed | probe | 0.75 | 4 | (none) |
| output | Main | crossFeed | restore | 0 | Off | (none) |
| output | Main | volume | probe | 0.75 | -2.2 dB | 1800:ffea 1900:ffea |
| output | Main | volume | restore | 0 | -oo | 1800:fd76 1900:fd76 |
| output | Main | pan | probe | 0.75 | R50 | 1801:0032 1901:0032 |
| output | Main | pan | restore | 0.5 | 0 | 1801:0000 1901:0000 |
| output | Main | mute | press | 1 |  | 1802:0001 1902:0001 3f9e:0004 3f9e:0004 |
| output | Main | mute | press back | 1 |  | 1802:0000 1902:0000 |
| output | Main | reverbSend | probe | 0.75 | -16.5 dB | (none) |
| output | Main | reverbSend | restore | 0 | -oo | (none) |
| output | Main | reverbReturn | probe | 0.75 | -16.5 dB | 1803:ff5b 1903:ff5b |
| output | Main | reverbReturn | restore | 0 | -oo | 1803:fd76 1903:fd76 |
| output | SPDIF | gain | probe | 0.75 | 0 | (none) |
| output | SPDIF | gain | restore | 0 | 0 | (none) |
| output | SPDIF | gainRight | probe | 0.75 | 0 | (none) |
| output | SPDIF | gainRight | restore | 0 | 0 | (none) |
| output | SPDIF | pad | press | 1 |  | (none) |
| output | SPDIF | pad | press back | 1 |  | (none) |
| output | SPDIF | instrument | press | 1 |  | (none) |
| output | SPDIF | instrument | press back | 1 |  | (none) |
| output | SPDIF | phase | press | 1 |  | 1a07:0001 |
| output | SPDIF | phase | press back | 1 |  | 1a07:0000 |
| output | SPDIF | phaseRight | press | 1 |  | 1b07:0001 |
| output | SPDIF | phaseRight | press back | 1 |  | 1b07:0000 |
| output | SPDIF | refLevel | probe | 0.75 | 1 | (none) |
| output | SPDIF | refLevel | restore | 1 | 1 | (none) |
| output | SPDIF | autoset | press | 1 |  | (none) |
| output | SPDIF | autoset | press back | 1 |  | (none) |
| output | SPDIF | msProc | press | 1 |  | (none) |
| output | SPDIF | msProc | press back | 1 |  | (none) |
| output | SPDIF | width | probe | 0.75 | 0.50 | (none) |
| output | SPDIF | width | restore | 1 | 1.00 | (none) |
| output | SPDIF | loopback | press | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0001 i=0x0008 req21 v=0x0001 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | SPDIF | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | SPDIF | alevEnable | press | 1 |  | 1a80:0001 1b80:0001 |
| output | SPDIF | alevEnable | press back | 1 |  | 1a80:0000 1b80:0000 |
| output | SPDIF | crossFeed | probe | 0.75 | 4 | (none) |
| output | SPDIF | crossFeed | restore | 0 | Off | (none) |
| output | SPDIF | volume | probe | 0.75 | -2.2 dB | 1a00:ffea 1b00:ffea |
| output | SPDIF | volume | restore | 0.8172 | 0.0 dB | 1a00:0000 1b00:0000 |
| output | SPDIF | pan | probe | 0.75 | R50 | 1a01:0032 1b01:0032 |
| output | SPDIF | pan | restore | 0.5 | 0 | 1a01:0000 1b01:0000 |
| output | SPDIF | mute | press | 1 |  | 1a02:0001 1b02:0001 |
| output | SPDIF | mute | press back | 1 |  | 1a02:0000 1b02:0000 |
| output | SPDIF | reverbSend | probe | 0.75 | -16.5 dB | (none) |
| output | SPDIF | reverbSend | restore | 0 | -oo | (none) |
| output | SPDIF | reverbReturn | probe | 0.75 | -16.5 dB | 1a03:ff5b 1b03:ff5b |
| output | SPDIF | reverbReturn | restore | 0 | -oo | 1a03:fd76 1b03:fd76 |
| output | AS 1/2 | gain | probe | 0.75 | 0 | (none) |
| output | AS 1/2 | gain | restore | 0 | 0 | (none) |
| output | AS 1/2 | gainRight | probe | 0.75 | 0 | (none) |
| output | AS 1/2 | gainRight | restore | 0 | 0 | (none) |
| output | AS 1/2 | pad | press | 1 |  | (none) |
| output | AS 1/2 | pad | press back | 1 |  | (none) |
| output | AS 1/2 | instrument | press | 1 |  | (none) |
| output | AS 1/2 | instrument | press back | 1 |  | (none) |
| output | AS 1/2 | phase | press | 1 |  | 1c07:0001 |
| output | AS 1/2 | phase | press back | 1 |  | 1c07:0000 |
| output | AS 1/2 | phaseRight | press | 1 |  | 1d07:0001 |
| output | AS 1/2 | phaseRight | press back | 1 |  | 1d07:0000 |
| output | AS 1/2 | refLevel | probe | 0.75 | 1 | (none) |
| output | AS 1/2 | refLevel | restore | 1 | 1 | (none) |
| output | AS 1/2 | autoset | press | 1 |  | (none) |
| output | AS 1/2 | autoset | press back | 1 |  | (none) |
| output | AS 1/2 | msProc | press | 1 |  | (none) |
| output | AS 1/2 | msProc | press back | 1 |  | (none) |
| output | AS 1/2 | width | probe | 0.75 | 0.50 | (none) |
| output | AS 1/2 | width | restore | 1 | 1.00 | (none) |
| output | AS 1/2 | loopback | press | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0001 i=0x000a req21 v=0x0001 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AS 1/2 | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | AS 1/2 | alevEnable | press | 1 |  | 1c80:0001 1d80:0001 |
| output | AS 1/2 | alevEnable | press back | 1 |  | 1c80:0000 1d80:0000 |
| output | AS 1/2 | crossFeed | probe | 0.75 | 4 | (none) |
| output | AS 1/2 | crossFeed | restore | 0 | Off | (none) |
| output | AS 1/2 | volume | probe | 0.75 | -2.2 dB | 1c00:ffea 1d00:ffea |
| output | AS 1/2 | volume | restore | 0.8172 | 0.0 dB | 1c00:0000 1d00:0000 |
| output | AS 1/2 | pan | probe | 0.75 | R50 | 1c01:0032 1d01:0032 |
| output | AS 1/2 | pan | restore | 0.5 | 0 | 1c01:0000 1d01:0000 |
| output | AS 1/2 | mute | press | 1 |  | 1c02:0001 1d02:0001 |
| output | AS 1/2 | mute | press back | 1 |  | 1c02:0000 1d02:0000 |
| output | AS 1/2 | reverbSend | probe | 0.75 | -16.5 dB | (none) |
| output | AS 1/2 | reverbSend | restore | 0 | -oo | (none) |
| output | AS 1/2 | reverbReturn | probe | 0.75 | -16.5 dB | 1c03:ff5b 1d03:ff5b |
| output | AS 1/2 | reverbReturn | restore | 0 | -oo | 1c03:fd76 1d03:fd76 |
| output | ADAT 3/4 | gain | probe | 0.75 | 0 | (none) |
| output | ADAT 3/4 | gain | restore | 0 | 0 | (none) |
| output | ADAT 3/4 | gainRight | probe | 0.75 | 0 | (none) |
| output | ADAT 3/4 | gainRight | restore | 0 | 0 | (none) |
| output | ADAT 3/4 | pad | press | 1 |  | (none) |
| output | ADAT 3/4 | pad | press back | 1 |  | (none) |
| output | ADAT 3/4 | instrument | press | 1 |  | (none) |
| output | ADAT 3/4 | instrument | press back | 1 |  | (none) |
| output | ADAT 3/4 | phase | press | 1 |  | 1e07:0001 |
| output | ADAT 3/4 | phase | press back | 1 |  | 1e07:0000 |
| output | ADAT 3/4 | phaseRight | press | 1 |  | 1f07:0001 |
| output | ADAT 3/4 | phaseRight | press back | 1 |  | 1f07:0000 |
| output | ADAT 3/4 | refLevel | probe | 0.75 | 1 | (none) |
| output | ADAT 3/4 | refLevel | restore | 1 | 1 | (none) |
| output | ADAT 3/4 | autoset | press | 1 |  | (none) |
| output | ADAT 3/4 | autoset | press back | 1 |  | (none) |
| output | ADAT 3/4 | msProc | press | 1 |  | (none) |
| output | ADAT 3/4 | msProc | press back | 1 |  | (none) |
| output | ADAT 3/4 | width | probe | 0.75 | 0.50 | (none) |
| output | ADAT 3/4 | width | restore | 1 | 1.00 | (none) |
| output | ADAT 3/4 | loopback | press | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0001 i=0x000c req21 v=0x0001 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 3/4 | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 3/4 | alevEnable | press | 1 |  | 1e80:0001 1f80:0001 |
| output | ADAT 3/4 | alevEnable | press back | 1 |  | 1e80:0000 1f80:0000 |
| output | ADAT 3/4 | crossFeed | probe | 0.75 | 4 | (none) |
| output | ADAT 3/4 | crossFeed | restore | 0 | Off | (none) |
| output | ADAT 3/4 | volume | probe | 0.75 | -2.2 dB | 1e00:ffea 1f00:ffea |
| output | ADAT 3/4 | volume | restore | 0.8172 | 0.0 dB | 1e00:0000 1f00:0000 |
| output | ADAT 3/4 | pan | probe | 0.75 | R50 | 1e01:0032 1f01:0032 |
| output | ADAT 3/4 | pan | restore | 0.5 | 0 | 1e01:0000 1f01:0000 |
| output | ADAT 3/4 | mute | press | 1 |  | 1e02:0001 1f02:0001 |
| output | ADAT 3/4 | mute | press back | 1 |  | 1e02:0000 1f02:0000 |
| output | ADAT 3/4 | reverbSend | probe | 0.75 | -16.5 dB | (none) |
| output | ADAT 3/4 | reverbSend | restore | 0 | -oo | (none) |
| output | ADAT 3/4 | reverbReturn | probe | 0.75 | -16.5 dB | 1e03:ff5b 1f03:ff5b |
| output | ADAT 3/4 | reverbReturn | restore | 0 | -oo | 1e03:fd76 1f03:fd76 |
| output | ADAT 5/6 | gain | probe | 0.75 | 0 | (none) |
| output | ADAT 5/6 | gain | restore | 0 | 0 | (none) |
| output | ADAT 5/6 | gainRight | probe | 0.75 | 0 | (none) |
| output | ADAT 5/6 | gainRight | restore | 0 | 0 | (none) |
| output | ADAT 5/6 | pad | press | 1 |  | (none) |
| output | ADAT 5/6 | pad | press back | 1 |  | (none) |
| output | ADAT 5/6 | instrument | press | 1 |  | (none) |
| output | ADAT 5/6 | instrument | press back | 1 |  | (none) |
| output | ADAT 5/6 | phase | press | 1 |  | 2007:0001 |
| output | ADAT 5/6 | phase | press back | 1 |  | 2007:0000 |
| output | ADAT 5/6 | phaseRight | press | 1 |  | 2107:0001 |
| output | ADAT 5/6 | phaseRight | press back | 1 |  | 2107:0000 |
| output | ADAT 5/6 | refLevel | probe | 0.75 | 1 | (none) |
| output | ADAT 5/6 | refLevel | restore | 1 | 1 | (none) |
| output | ADAT 5/6 | autoset | press | 1 |  | (none) |
| output | ADAT 5/6 | autoset | press back | 1 |  | (none) |
| output | ADAT 5/6 | msProc | press | 1 |  | (none) |
| output | ADAT 5/6 | msProc | press back | 1 |  | (none) |
| output | ADAT 5/6 | width | probe | 0.75 | 0.50 | (none) |
| output | ADAT 5/6 | width | restore | 1 | 1.00 | (none) |
| output | ADAT 5/6 | loopback | press | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0001 i=0x000e req21 v=0x0001 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 5/6 | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 5/6 | alevEnable | press | 1 |  | 2080:0001 2180:0001 |
| output | ADAT 5/6 | alevEnable | press back | 1 |  | 2080:0000 2180:0000 |
| output | ADAT 5/6 | crossFeed | probe | 0.75 | 4 | (none) |
| output | ADAT 5/6 | crossFeed | restore | 0 | Off | (none) |
| output | ADAT 5/6 | volume | probe | 0.75 | -2.2 dB | 2000:ffea 2100:ffea |
| output | ADAT 5/6 | volume | restore | 0.8172 | 0.0 dB | 2000:0000 2100:0000 |
| output | ADAT 5/6 | pan | probe | 0.75 | R50 | 2001:0032 2101:0032 |
| output | ADAT 5/6 | pan | restore | 0.5 | 0 | 2001:0000 2101:0000 |
| output | ADAT 5/6 | mute | press | 1 |  | 2002:0001 2102:0001 |
| output | ADAT 5/6 | mute | press back | 1 |  | 2002:0000 2102:0000 |
| output | ADAT 5/6 | reverbSend | probe | 0.75 | -16.5 dB | (none) |
| output | ADAT 5/6 | reverbSend | restore | 0 | -oo | (none) |
| output | ADAT 5/6 | reverbReturn | probe | 0.75 | -16.5 dB | 2003:ff5b 2103:ff5b |
| output | ADAT 5/6 | reverbReturn | restore | 0 | -oo | 2003:fd76 2103:fd76 |
| output | ADAT 7/8 | gain | probe | 0.75 |  | (none) |
| output | ADAT 7/8 | gain | restore | 0 | 0 | (none) |
| output | ADAT 7/8 | gainRight | probe | 0.75 | 0 | (none) |
| output | ADAT 7/8 | gainRight | restore | 0 | 0 | (none) |
| output | ADAT 7/8 | pad | press | 1 |  | (none) |
| output | ADAT 7/8 | pad | press back | 1 |  | (none) |
| output | ADAT 7/8 | instrument | press | 1 |  | (none) |
| output | ADAT 7/8 | instrument | press back | 1 |  | (none) |
| output | ADAT 7/8 | phase | press | 1 |  | 2207:0001 |
| output | ADAT 7/8 | phase | press back | 1 |  | 2207:0000 |
| output | ADAT 7/8 | phaseRight | press | 1 |  | 2307:0001 |
| output | ADAT 7/8 | phaseRight | press back | 1 |  | 2307:0000 |
| output | ADAT 7/8 | refLevel | probe | 0.75 | 1 | (none) |
| output | ADAT 7/8 | refLevel | restore | 1 | 1 | (none) |
| output | ADAT 7/8 | autoset | press | 1 |  | (none) |
| output | ADAT 7/8 | autoset | press back | 1 |  | (none) |
| output | ADAT 7/8 | msProc | press | 1 |  | (none) |
| output | ADAT 7/8 | msProc | press back | 1 |  | (none) |
| output | ADAT 7/8 | width | probe | 0.75 | 0.50 | (none) |
| output | ADAT 7/8 | width | restore | 1 | 1.00 | (none) |
| output | ADAT 7/8 | loopback | press | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0001 i=0x0010 req21 v=0x0001 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 7/8 | loopback | press back | 1 |  | req21 v=0x0000 i=0x0000 req21 v=0x0000 i=0x0001 req21 v=0x0000 i=0x0002 req21 v=0x0000 i=0x0003 req21 v=0x0000 i=0x0004 req21 v=0x0000 i=0x0005 req21 v=0x0000 i=0x0006 req21 v=0x0000 i=0x0007 req21 v=0x0000 i=0x0008 req21 v=0x0000 i=0x0009 req21 v=0x0000 i=0x000a req21 v=0x0000 i=0x000b req21 v=0x0000 i=0x000c req21 v=0x0000 i=0x000d req21 v=0x0000 i=0x000e req21 v=0x0000 i=0x000f req21 v=0x0000 i=0x0010 req21 v=0x0000 i=0x0011 req21 v=0x0000 i=0x0012 req21 v=0x0000 i=0x0013 req21 v=0x0000 i=0x0014 req21 v=0x0000 i=0x0015 req21 v=0x0000 i=0x0016 req21 v=0x0000 i=0x0017 req21 v=0x0000 i=0x0018 req21 v=0x0000 i=0x0019 req21 v=0x0000 i=0x001a req21 v=0x0000 i=0x001b req21 v=0x0000 i=0x001c req21 v=0x0000 i=0x001d |
| output | ADAT 7/8 | alevEnable | press | 1 |  | 2280:0001 2380:0001 |
| output | ADAT 7/8 | alevEnable | press back | 1 |  | 2280:0000 2380:0000 |
| output | ADAT 7/8 | crossFeed | probe | 0.75 | 4 | (none) |
| output | ADAT 7/8 | crossFeed | restore | 0 | Off | (none) |
| output | ADAT 7/8 | volume | probe | 0.75 | -2.2 dB | 2200:ffea 2300:ffea |
| output | ADAT 7/8 | volume | restore | 0.8172 | 0.0 dB | 2200:0000 2300:0000 |
| output | ADAT 7/8 | pan | probe | 0.75 | R50 | 2201:0032 2301:0032 |
| output | ADAT 7/8 | pan | restore | 0.5 | 0 | 2201:0000 2301:0000 |
| output | ADAT 7/8 | mute | press | 1 |  | 2202:0001 2302:0001 |
| output | ADAT 7/8 | mute | press back | 1 |  | 2202:0000 2302:0000 |
| output | ADAT 7/8 | reverbSend | probe | 0.75 | -16.5 dB | (none) |
| output | ADAT 7/8 | reverbSend | restore | 0 | -oo | (none) |
| output | ADAT 7/8 | reverbReturn | probe | 0.75 | -16.5 dB | 2203:ff5b 2303:ff5b |
| output | ADAT 7/8 | reverbReturn | restore | 0 | -oo | 2203:fd76 2303:fd76 |
