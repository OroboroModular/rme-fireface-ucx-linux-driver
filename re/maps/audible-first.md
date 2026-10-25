# Channel settings → register writes

From `05-audible-first.pcap` (198 changes). Words are `address:value` (hex, bits 30–16 : 15–0).

## Per parameter

| Row | Channel | Parameter | Addresses | OSC value → display → words |
|---|---|---|---|---|
| input | Mic 1 | gain | `0007` | 0.25 → 16 → 0007:00a0<br>0.5 → 33 → 0007:014a<br>0.75 → 49 → 0007:01ea<br>1 → 65 → 0007:028a<br>0 → 0 → 0007:0000 |
| input | Mic 1 | gainRight | `0007` | 0.25 → 16 → 0007:00a0<br>0.5 → 33 → 0007:014a<br>0.75 → 49 → 0007:01ea<br>1 → 65 → 0007:028a<br>0 → 0 → 0007:0000 |
| input | Mic 1 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | Mic 1 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | Mic 1 | phase | `0006` | 1 → ? → 0006:0001<br>1 → ? → 0006:0000 |
| input | Mic 1 | phaseRight | `0006` | 1 → ? → 0006:0001<br>1 → ? → 0006:0000 |
| input | Mic 1 | refLevel | none | 0 → 0 → (none)<br>0.25 → 0 → (none)<br>0.5 → 1 → (none)<br>0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | Mic 1 | autoset | `000a` | 1 → ? → 000a:0001<br>1 → ? → 000a:0000 |
| input | Mic 1 | msProc | `0005` | 1 → ? → 0005:0001<br>1 → ? → 0005:0000 |
| input | Mic 1 | width | none | 0 → -1.00 → (none)<br>0.25 → -0.50 → (none)<br>0.5 → 0.00 → (none)<br>0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | Mic 1 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | Mic 1 | alevEnable | `0080` | 1 → ? → 0080:0001<br>1 → ? → 0080:0000 |
| input | Mic 1 | crossFeed | none | 0.25 → Off → (none)<br>0.5 → Off → (none)<br>0.75 → Off → (none)<br>1 → Off → (none)<br>0 → Off → (none) |
| input | Mic 1 | volume | `0001` `12e0` `13e0` `4000` `4040` | 0.25 → -32.6 dB → 0001:feba 12e0:0008 12e0:eba0 13e0:0008 13e0:eba0 4000:021d 4040:021d<br>0.5 → -12.1 dB → 0001:ff87 12e0:0008 12e0:f870 13e0:0008 13e0:f870 4000:1668 4040:1668<br>0.75 → -2.2 dB → 0001:ffea 12e0:0008 12e0:fea0 13e0:0008 13e0:fea0 4000:88c6 4040:88c6<br>1 → 6.0 dB → 0001:0000 12e0:0008 12e0:03c0 13e0:0008 13e0:03c0 4000:9692 4040:9692<br>0 → -oo → 0001:fd76 12e0:0008 12e0:d760 13e0:0008 13e0:d760 4000:0000 4040:0000 |
| input | Mic 1 | pan | none | 0 → L100 → (none)<br>0.25 → L50 → (none)<br>0.75 → R50 → (none)<br>1 → R100 → (none)<br>0.5 → 0 → (none) |
| input | Mic 1 | mute | `0000` | 1 → ? → 0000:0001<br>1 → ? → 0000:0000 |
| input | Mic 1 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | Mic 1 | reverbSend | `0001` | 0.25 → -49.0 dB → 0001:fe16<br>0.5 → -32.5 dB → 0001:febb<br>0.75 → -16.5 dB → 0001:ff5b<br>1 → 0.0 dB → 0001:0000<br>0 → -oo → 0001:fd76 |
| input | Mic 1 | reverbReturn | none | 0.25 → ? → (none)<br>0.5 → ? → (none)<br>0.75 → ? → (none)<br>1 → ? → (none)<br>0 → ? → (none) |
| playback | AN 1/2 | gain | none | 0.25 → ? → (none)<br>0.5 → 0 → (none)<br>0.75 → 0 → (none)<br>1 → 0 → (none)<br>0 → 0 → (none) |
| playback | AN 1/2 | gainRight | none | 0.25 → 0 → (none)<br>0.5 → 0 → (none)<br>0.75 → ? → (none)<br>1 → ? → (none)<br>0 → ? → (none) |
| playback | AN 1/2 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | phase | `4020` `41a0` | 1 → ? → 4020:f000 41a0:f000<br>1 → ? → 4020:9000 41a0:9000 |
| playback | AN 1/2 | phaseRight | `4061` `41e1` | 1 → ? → 4061:f000 41e1:f000<br>1 → ? → 4061:9000 41e1:9000 |
| playback | AN 1/2 | refLevel | none | 0 → 0 → (none)<br>0.25 → ? → (none)<br>0.5 → 1 → (none)<br>0.75 → 1 → (none)<br>1 → 1 → (none) |
| playback | AN 1/2 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | msProc | `4020` `4021` `4060` `4061` `41a0` `41a1` `41e0` `41e1` | 1 → ? → 4020:8800 4021:8800 4060:8800 4061:4000 41a0:8800 41a1:8800 41e0:8800 41e1:4000<br>1 → ? → 4020:9000 4021:0000 4060:0000 4061:9000 41a0:9000 41a1:0000 41e0:0000 41e1:9000 |
| playback | AN 1/2 | width | `4020` `4021` `4060` `4061` `41a0` `41a1` `41e0` `41e1` | 0 → -1.00 → 4020:0000 4021:9000 4060:9000 4061:0000 41a0:0000 41a1:9000 41e0:9000 41e1:0000<br>0.25 → -0.50 → 4020:2000 4021:8c00 4060:8c00 4061:2000 41a0:2000 41a1:8c00 41e0:8c00 41e1:2000<br>0.5 → 0.00 → 4020:8800 4021:8800 4060:8800 4061:8800 41a0:8800 41a1:8800 41e0:8800 41e1:8800<br>0.75 → 0.50 → 4020:8c00 4021:2000 4060:2000 4061:8c00 41a0:8c00 41a1:2000 41e0:2000 41e1:8c00<br>1 → 1.00 → 4020:9000 4021:0000 4060:0000 4061:9000 41a0:9000 41a1:0000 41e0:0000 41e1:9000 |
| playback | AN 1/2 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | alevEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | crossFeed | none | 0.25 → Off → (none)<br>0.5 → Off → (none)<br>0.75 → Off → (none)<br>1 → Off → (none)<br>0 → Off → (none) |
| playback | AN 1/2 | volume | `4020` `4061` | 0 → -oo → 4020:0000 4061:0000<br>0.25 → -32.6 dB → 4020:02fd 4061:02fd<br>0.5 → -12.1 dB → 4020:1fb1 4061:1fb1<br>0.75 → -2.2 dB → 4020:8c69 4061:8c69<br>1 → 6.0 dB → 4020:9fec 4061:9fec<br>0.8172 → 0.0 dB → 4020:9000 4061:9000 |
| playback | AN 1/2 | pan | `4061` `4020` | 0 → L100 → 4061:0000<br>0.25 → L50 → 4061:8800<br>0.75 → R50 → 4020:8800 4061:9000<br>1 → R100 → 4020:0000<br>0.5 → 0 → 4020:9000 |
| playback | AN 1/2 | mute | `4020` `4061` `41a0` `41e1` | 1 → ? → 4020:0000 4061:0000 41a0:0000 41e1:0000<br>1 → ? → 4020:9000 4061:9000 41a0:9000 41e1:9000 |
| playback | AN 1/2 | solo | `12e6` `13e7` `4006` `4047` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4047:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f |
| playback | AN 1/2 | reverbSend | `47a0` `47e1` | 0 → -oo → (none)<br>0.25 → -49.0 dB → 47a0:0074 47e1:0074<br>0.5 → -32.5 dB → 47a0:0309 47e1:0309<br>0.75 → -16.5 dB → 47a0:1326 47e1:1326<br>1 → 0.0 dB → 47a0:9000 47e1:9000<br>0 → -oo → 47a0:0000 47e1:0000 |
| playback | AN 1/2 | reverbReturn | none | 0.25 → -oo → (none)<br>0.5 → -oo → (none)<br>0.75 → -oo → (none)<br>1 → -oo → (none)<br>0 → -oo → (none) |
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
| output | AN 1/2 | reverbSend | none | 0 → -oo → (none)<br>0.25 → -49.0 dB → (none)<br>0.5 → -32.5 dB → (none)<br>0.75 → -16.5 dB → (none)<br>1 → 0.0 dB → (none)<br>0 → -oo → (none) |
| output | AN 1/2 | reverbReturn | `1203` `1303` | 0.25 → -49.0 dB → 1203:fe16 1303:fe16<br>0.5 → -32.5 dB → 1203:febb 1303:febb<br>0.75 → -16.5 dB → 1203:ff5b 1303:ff5b<br>1 → 0.0 dB → 1203:0000 1303:0000<br>0 → -oo → 1203:fd76 1303:fd76 |

## Every change in order

| Row | Channel | Parameter | Kind | OSC value | Display | Writes |
|---|---|---|---|---|---|---|
| input | Mic 1 | gain | sweep | 0.25 | 16 | 0007:00a0 |
| input | Mic 1 | gain | sweep | 0.5 | 33 | 0007:014a |
| input | Mic 1 | gain | sweep | 0.75 | 49 | 0007:01ea |
| input | Mic 1 | gain | sweep | 1 | 65 | 0007:028a |
| input | Mic 1 | gain | restore | 0 | 0 | 0007:0000 |
| input | Mic 1 | gainRight | sweep | 0.25 | 16 | 0007:00a0 |
| input | Mic 1 | gainRight | sweep | 0.5 | 33 | 0007:014a |
| input | Mic 1 | gainRight | sweep | 0.75 | 49 | 0007:01ea |
| input | Mic 1 | gainRight | sweep | 1 | 65 | 0007:028a |
| input | Mic 1 | gainRight | restore | 0 | 0 | 0007:0000 |
| input | Mic 1 | pad | press | 1 |  | (none) |
| input | Mic 1 | pad | press back | 1 |  | (none) |
| input | Mic 1 | instrument | press | 1 |  | (none) |
| input | Mic 1 | instrument | press back | 1 |  | (none) |
| input | Mic 1 | phase | press | 1 |  | 0006:0001 |
| input | Mic 1 | phase | press back | 1 |  | 0006:0000 |
| input | Mic 1 | phaseRight | press | 1 |  | 0006:0001 |
| input | Mic 1 | phaseRight | press back | 1 |  | 0006:0000 |
| input | Mic 1 | refLevel | sweep | 0 | 0 | (none) |
| input | Mic 1 | refLevel | sweep | 0.25 | 0 | (none) |
| input | Mic 1 | refLevel | sweep | 0.5 | 1 | (none) |
| input | Mic 1 | refLevel | sweep | 0.75 | 1 | (none) |
| input | Mic 1 | refLevel | restore | 1 | 1 | (none) |
| input | Mic 1 | autoset | press | 1 |  | 000a:0001 |
| input | Mic 1 | autoset | press back | 1 |  | 000a:0000 |
| input | Mic 1 | msProc | press | 1 |  | 0005:0001 |
| input | Mic 1 | msProc | press back | 1 |  | 0005:0000 |
| input | Mic 1 | width | sweep | 0 | -1.00 | (none) |
| input | Mic 1 | width | sweep | 0.25 | -0.50 | (none) |
| input | Mic 1 | width | sweep | 0.5 | 0.00 | (none) |
| input | Mic 1 | width | sweep | 0.75 | 0.50 | (none) |
| input | Mic 1 | width | restore | 1 | 1.00 | (none) |
| input | Mic 1 | loopback | press | 1 |  | (none) |
| input | Mic 1 | loopback | press back | 1 |  | (none) |
| input | Mic 1 | alevEnable | press | 1 |  | 0080:0001 |
| input | Mic 1 | alevEnable | press back | 1 |  | 0080:0000 |
| input | Mic 1 | crossFeed | sweep | 0.25 | Off | (none) |
| input | Mic 1 | crossFeed | sweep | 0.5 | Off | (none) |
| input | Mic 1 | crossFeed | sweep | 0.75 | Off | (none) |
| input | Mic 1 | crossFeed | sweep | 1 | Off | (none) |
| input | Mic 1 | crossFeed | restore | 0 | Off | (none) |
| input | Mic 1 | volume | sweep | 0.25 | -32.6 dB | 0001:feba 12e0:0008 12e0:eba0 13e0:0008 13e0:eba0 4000:021d 4040:021d |
| input | Mic 1 | volume | sweep | 0.5 | -12.1 dB | 0001:ff87 12e0:0008 12e0:f870 13e0:0008 13e0:f870 4000:1668 4040:1668 |
| input | Mic 1 | volume | sweep | 0.75 | -2.2 dB | 0001:ffea 12e0:0008 12e0:fea0 13e0:0008 13e0:fea0 4000:88c6 4040:88c6 |
| input | Mic 1 | volume | sweep | 1 | 6.0 dB | 0001:0000 12e0:0008 12e0:03c0 13e0:0008 13e0:03c0 4000:9692 4040:9692 |
| input | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 12e0:0008 12e0:d760 13e0:0008 13e0:d760 4000:0000 4040:0000 |
| input | Mic 1 | pan | sweep | 0 | L100 | (none) |
| input | Mic 1 | pan | sweep | 0.25 | L50 | (none) |
| input | Mic 1 | pan | sweep | 0.75 | R50 | (none) |
| input | Mic 1 | pan | sweep | 1 | R100 | (none) |
| input | Mic 1 | pan | restore | 0.5 | 0 | (none) |
| input | Mic 1 | mute | press | 1 |  | 0000:0001 |
| input | Mic 1 | mute | press back | 1 |  | 0000:0000 |
| input | Mic 1 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | Mic 1 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | Mic 1 | reverbSend | sweep | 0.25 | -49.0 dB | 0001:fe16 |
| input | Mic 1 | reverbSend | sweep | 0.5 | -32.5 dB | 0001:febb |
| input | Mic 1 | reverbSend | sweep | 0.75 | -16.5 dB | 0001:ff5b |
| input | Mic 1 | reverbSend | sweep | 1 | 0.0 dB | 0001:0000 |
| input | Mic 1 | reverbSend | restore | 0 | -oo | 0001:fd76 |
| input | Mic 1 | reverbReturn | sweep | 0.25 |  | (none) |
| input | Mic 1 | reverbReturn | sweep | 0.5 |  | (none) |
| input | Mic 1 | reverbReturn | sweep | 0.75 |  | (none) |
| input | Mic 1 | reverbReturn | sweep | 1 |  | (none) |
| input | Mic 1 | reverbReturn | restore | 0 |  | (none) |
| playback | AN 1/2 | gain | sweep | 0.25 |  | (none) |
| playback | AN 1/2 | gain | sweep | 0.5 | 0 | (none) |
| playback | AN 1/2 | gain | sweep | 0.75 | 0 | (none) |
| playback | AN 1/2 | gain | sweep | 1 | 0 | (none) |
| playback | AN 1/2 | gain | restore | 0 | 0 | (none) |
| playback | AN 1/2 | gainRight | sweep | 0.25 | 0 | (none) |
| playback | AN 1/2 | gainRight | sweep | 0.5 | 0 | (none) |
| playback | AN 1/2 | gainRight | sweep | 0.75 |  | (none) |
| playback | AN 1/2 | gainRight | sweep | 1 |  | (none) |
| playback | AN 1/2 | gainRight | restore | 0 |  | (none) |
| playback | AN 1/2 | pad | press | 1 |  | (none) |
| playback | AN 1/2 | pad | press back | 1 |  | (none) |
| playback | AN 1/2 | instrument | press | 1 |  | (none) |
| playback | AN 1/2 | instrument | press back | 1 |  | (none) |
| playback | AN 1/2 | phase | press | 1 |  | 4020:f000 41a0:f000 |
| playback | AN 1/2 | phase | press back | 1 |  | 4020:9000 41a0:9000 |
| playback | AN 1/2 | phaseRight | press | 1 |  | 4061:f000 41e1:f000 |
| playback | AN 1/2 | phaseRight | press back | 1 |  | 4061:9000 41e1:9000 |
| playback | AN 1/2 | refLevel | sweep | 0 | 0 | (none) |
| playback | AN 1/2 | refLevel | sweep | 0.25 |  | (none) |
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
| playback | AN 1/2 | volume | sweep | 1 | 6.0 dB | 4020:9fec 4061:9fec |
| playback | AN 1/2 | volume | restore | 0.8172 | 0.0 dB | 4020:9000 4061:9000 |
| playback | AN 1/2 | pan | sweep | 0 | L100 | 4061:0000 |
| playback | AN 1/2 | pan | sweep | 0.25 | L50 | 4061:8800 |
| playback | AN 1/2 | pan | sweep | 0.75 | R50 | 4020:8800 4061:9000 |
| playback | AN 1/2 | pan | sweep | 1 | R100 | 4020:0000 |
| playback | AN 1/2 | pan | restore | 0.5 | 0 | 4020:9000 |
| playback | AN 1/2 | mute | press | 1 |  | 4020:0000 4061:0000 41a0:0000 41e1:0000 |
| playback | AN 1/2 | mute | press back | 1 |  | 4020:9000 4061:9000 41a0:9000 41e1:9000 |
| playback | AN 1/2 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4047:0000 |
| playback | AN 1/2 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f |
| playback | AN 1/2 | reverbSend | sweep | 0 | -oo | (none) |
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
| output | AN 1/2 | reverbSend | sweep | 0 | -oo | (none) |
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
