# Channel settings → register writes

From `05-audible-all.pcap` (217 changes). Words are `address:value` (hex, bits 30–16 : 15–0).

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
| input | Mic 1 | reverbReturn | none | 0.25 → -oo → (none)<br>0.5 → -oo → (none)<br>0.75 → -oo → (none)<br>1 → -oo → (none)<br>0 → -oo → (none) |
| input | Mic 2 | gain | `0107` | 0.75 → 49 → 0107:01ea<br>0 → 0 → 0107:0000 |
| input | Mic 2 | gainRight | `0107` | 0.75 → 49 → 0107:01ea<br>0 → 0 → 0107:0000 |
| input | Mic 2 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | Mic 2 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | Mic 2 | phase | `0106` | 1 → ? → 0106:0001<br>1 → ? → 0106:0000 |
| input | Mic 2 | phaseRight | `0106` | 1 → ? → 0106:0001<br>1 → ? → 0106:0000 |
| input | Mic 2 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | Mic 2 | autoset | `010a` | 1 → ? → 010a:0001<br>1 → ? → 010a:0000 |
| input | Mic 2 | msProc | `0105` | 1 → ? → 0105:0001<br>1 → ? → 0105:0000 |
| input | Mic 2 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | Mic 2 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | Mic 2 | alevEnable | `0180` | 1 → ? → 0180:0001<br>1 → ? → 0180:0000 |
| input | Mic 2 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | Mic 2 | volume | `12e1` `13e1` `4001` `4041` | 0.75 → -2.2 dB → 12e1:0008 12e1:fea0 13e1:0008 13e1:fea0 4001:88c6 4041:88c6<br>0 → -oo → 12e1:0008 12e1:d760 13e1:0008 13e1:d760 4001:0000 4041:0000 |
| input | Mic 2 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | Mic 2 | mute | `0100` | 1 → ? → 0100:0001<br>1 → ? → 0100:0000 |
| input | Mic 2 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | Mic 2 | reverbSend | `0101` | 0.75 → -16.5 dB → 0101:ff5b<br>0 → -oo → 0101:fd76 |
| input | Mic 2 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| input | AN 3 | gain | `0207` | 0.75 → 12 → 0207:0078<br>0 → 0 → 0207:0000 |
| input | AN 3 | gainRight | `0207` | 0.75 → 12 → 0207:0078<br>0 → 0 → 0207:0000 |
| input | AN 3 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 3 | instrument | `0209` | 1 → ? → 0209:0001<br>1 → ? → 0209:0000 |
| input | AN 3 | phase | `0206` | 1 → ? → 0206:0001<br>1 → ? → 0206:0000 |
| input | AN 3 | phaseRight | `0206` | 1 → ? → 0206:0001<br>1 → ? → 0206:0000 |
| input | AN 3 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | AN 3 | autoset | `020a` | 1 → ? → 020a:0001<br>1 → ? → 020a:0000 |
| input | AN 3 | msProc | `0205` | 1 → ? → 0205:0001<br>1 → ? → 0205:0000 |
| input | AN 3 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | AN 3 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 3 | alevEnable | `0280` | 1 → ? → 0280:0001<br>1 → ? → 0280:0000 |
| input | AN 3 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | AN 3 | volume | `12e2` `13e2` `4002` `4042` | 0.75 → -2.2 dB → 12e2:0008 12e2:fea0 13e2:0008 13e2:fea0 4002:88c6 4042:88c6<br>0 → -oo → 12e2:0008 12e2:d760 13e2:0008 13e2:d760 4002:0000 4042:0000 |
| input | AN 3 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | AN 3 | mute | `0200` | 1 → ? → 0200:0001<br>1 → ? → 0200:0000 |
| input | AN 3 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | AN 3 | reverbSend | `0201` | 0.75 → -16.5 dB → 0201:ff5b<br>0 → -oo → 0201:fd76 |
| input | AN 3 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| input | AN 4 | gain | `0307` | 0.75 → 12 → 0307:0078<br>0 → 0 → 0307:0000 |
| input | AN 4 | gainRight | `0307` | 0.75 → 12 → 0307:0078<br>0 → 0 → 0307:0000 |
| input | AN 4 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 4 | instrument | `0309` | 1 → ? → 0309:0001<br>1 → ? → 0309:0000 |
| input | AN 4 | phase | `0306` | 1 → ? → 0306:0001<br>1 → ? → 0306:0000 |
| input | AN 4 | phaseRight | `0306` | 1 → ? → 0306:0001<br>1 → ? → 0306:0000 |
| input | AN 4 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | AN 4 | autoset | `030a` | 1 → ? → 030a:0001<br>1 → ? → 030a:0000 |
| input | AN 4 | msProc | `0305` | 1 → ? → 0305:0001<br>1 → ? → 0305:0000 |
| input | AN 4 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | AN 4 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 4 | alevEnable | `0380` | 1 → ? → 0380:0001<br>1 → ? → 0380:0000 |
| input | AN 4 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | AN 4 | volume | `12e3` `13e3` `4003` `4043` | 0.75 → -2.2 dB → 12e3:0008 12e3:fea0 13e3:0008 13e3:fea0 4003:88c6 4043:88c6<br>0 → -oo → 12e3:0008 12e3:d760 13e3:0008 13e3:d760 4003:0000 4043:0000 |
| input | AN 4 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | AN 4 | mute | `0300` | 1 → ? → 0300:0001<br>1 → ? → 0300:0000 |
| input | AN 4 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | AN 4 | reverbSend | `0301` | 0.75 → -16.5 dB → 0301:ff5b<br>0 → -oo → 0301:fd76 |
| input | AN 4 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |
| input | AN 5/6 | gain | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | AN 5/6 | gainRight | none | 0.75 → 49 → (none)<br>0 → 0 → (none) |
| input | AN 5/6 | pad | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 5/6 | instrument | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 5/6 | phase | `0406` | 1 → ? → 0406:0001<br>1 → ? → 0406:0000 |
| input | AN 5/6 | phaseRight | `0506` | 1 → ? → 0506:0001<br>1 → ? → 0506:0000 |
| input | AN 5/6 | refLevel | none | 0.75 → 1 → (none)<br>1 → 1 → (none) |
| input | AN 5/6 | autoset | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 5/6 | msProc | `0405` `0505` | 1 → ? → 0405:0001 0505:0001<br>1 → ? → 0405:0000 0505:0000 |
| input | AN 5/6 | width | none | 0.75 → 0.50 → (none)<br>1 → 1.00 → (none) |
| input | AN 5/6 | loopback | none | 1 → ? → (none)<br>1 → ? → (none) |
| input | AN 5/6 | alevEnable | `0480` `0580` | 1 → ? → 0480:0001 0580:0001<br>1 → ? → 0480:0000 0580:0000 |
| input | AN 5/6 | crossFeed | none | 0.75 → Off → (none)<br>0 → Off → (none) |
| input | AN 5/6 | volume | `12e4` `13e5` `4004` `4045` | 0.75 → -2.2 dB → 12e4:f9c8 12e4:fea0 13e5:0648 13e5:fea0 4004:8c69 4045:8c69<br>0 → -oo → 12e4:d760 13e5:d760 4004:0000 4045:0000 |
| input | AN 5/6 | pan | none | 0.75 → R50 → (none)<br>0.5 → 0 → (none) |
| input | AN 5/6 | mute | `0400` `0500` | 1 → ? → 0400:0001 0500:0001<br>1 → ? → 0400:0000 0500:0000 |
| input | AN 5/6 | solo | `12e6` `13e7` `4006` `4020` `4047` `4061` | 1 → ? → 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000<br>1 → ? → 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | AN 5/6 | reverbSend | `0401` `0501` | 0.75 → -16.5 dB → 0401:ff5b 0501:ff5b req16 v=0x0800 i=0x0800 req16 v=0x0800 i=0x0800 req16 v=0x8000 i=0x8000 req29 v=0x0000 i=0x0000<br>0 → -oo → 0401:fd76 0501:fd76 |
| input | AN 5/6 | reverbReturn | none | 0.75 → -oo → (none)<br>0 → -oo → (none) |

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
| input | Mic 1 | reverbReturn | sweep | 0.25 | -oo | (none) |
| input | Mic 1 | reverbReturn | sweep | 0.5 | -oo | (none) |
| input | Mic 1 | reverbReturn | sweep | 0.75 | -oo | (none) |
| input | Mic 1 | reverbReturn | sweep | 1 | -oo | (none) |
| input | Mic 1 | reverbReturn | restore | 0 | -oo | (none) |
| input | Mic 2 | gain | probe | 0.75 | 49 | 0107:01ea |
| input | Mic 2 | gain | restore | 0 | 0 | 0107:0000 |
| input | Mic 2 | gainRight | probe | 0.75 | 49 | 0107:01ea |
| input | Mic 2 | gainRight | restore | 0 | 0 | 0107:0000 |
| input | Mic 2 | pad | press | 1 |  | (none) |
| input | Mic 2 | pad | press back | 1 |  | (none) |
| input | Mic 2 | instrument | press | 1 |  | (none) |
| input | Mic 2 | instrument | press back | 1 |  | (none) |
| input | Mic 2 | phase | press | 1 |  | 0106:0001 |
| input | Mic 2 | phase | press back | 1 |  | 0106:0000 |
| input | Mic 2 | phaseRight | press | 1 |  | 0106:0001 |
| input | Mic 2 | phaseRight | press back | 1 |  | 0106:0000 |
| input | Mic 2 | refLevel | probe | 0.75 | 1 | (none) |
| input | Mic 2 | refLevel | restore | 1 | 1 | (none) |
| input | Mic 2 | autoset | press | 1 |  | 010a:0001 |
| input | Mic 2 | autoset | press back | 1 |  | 010a:0000 |
| input | Mic 2 | msProc | press | 1 |  | 0105:0001 |
| input | Mic 2 | msProc | press back | 1 |  | 0105:0000 |
| input | Mic 2 | width | probe | 0.75 | 0.50 | (none) |
| input | Mic 2 | width | restore | 1 | 1.00 | (none) |
| input | Mic 2 | loopback | press | 1 |  | (none) |
| input | Mic 2 | loopback | press back | 1 |  | (none) |
| input | Mic 2 | alevEnable | press | 1 |  | 0180:0001 |
| input | Mic 2 | alevEnable | press back | 1 |  | 0180:0000 |
| input | Mic 2 | crossFeed | probe | 0.75 | Off | (none) |
| input | Mic 2 | crossFeed | restore | 0 | Off | (none) |
| input | Mic 2 | volume | probe | 0.75 | -2.2 dB | 12e1:0008 12e1:fea0 13e1:0008 13e1:fea0 4001:88c6 4041:88c6 |
| input | Mic 2 | volume | restore | 0 | -oo | 12e1:0008 12e1:d760 13e1:0008 13e1:d760 4001:0000 4041:0000 |
| input | Mic 2 | pan | probe | 0.75 | R50 | (none) |
| input | Mic 2 | pan | restore | 0.5 | 0 | (none) |
| input | Mic 2 | mute | press | 1 |  | 0100:0001 |
| input | Mic 2 | mute | press back | 1 |  | 0100:0000 |
| input | Mic 2 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | Mic 2 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | Mic 2 | reverbSend | probe | 0.75 | -16.5 dB | 0101:ff5b |
| input | Mic 2 | reverbSend | restore | 0 | -oo | 0101:fd76 |
| input | Mic 2 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | Mic 2 | reverbReturn | restore | 0 | -oo | (none) |
| input | AN 3 | gain | probe | 0.75 | 12 | 0207:0078 |
| input | AN 3 | gain | restore | 0 | 0 | 0207:0000 |
| input | AN 3 | gainRight | probe | 0.75 | 12 | 0207:0078 |
| input | AN 3 | gainRight | restore | 0 | 0 | 0207:0000 |
| input | AN 3 | pad | press | 1 |  | (none) |
| input | AN 3 | pad | press back | 1 |  | (none) |
| input | AN 3 | instrument | press | 1 |  | 0209:0001 |
| input | AN 3 | instrument | press back | 1 |  | 0209:0000 |
| input | AN 3 | phase | press | 1 |  | 0206:0001 |
| input | AN 3 | phase | press back | 1 |  | 0206:0000 |
| input | AN 3 | phaseRight | press | 1 |  | 0206:0001 |
| input | AN 3 | phaseRight | press back | 1 |  | 0206:0000 |
| input | AN 3 | refLevel | probe | 0.75 | 1 | (none) |
| input | AN 3 | refLevel | restore | 1 | 1 | (none) |
| input | AN 3 | autoset | press | 1 |  | 020a:0001 |
| input | AN 3 | autoset | press back | 1 |  | 020a:0000 |
| input | AN 3 | msProc | press | 1 |  | 0205:0001 |
| input | AN 3 | msProc | press back | 1 |  | 0205:0000 |
| input | AN 3 | width | probe | 0.75 | 0.50 | (none) |
| input | AN 3 | width | restore | 1 | 1.00 | (none) |
| input | AN 3 | loopback | press | 1 |  | (none) |
| input | AN 3 | loopback | press back | 1 |  | (none) |
| input | AN 3 | alevEnable | press | 1 |  | 0280:0001 |
| input | AN 3 | alevEnable | press back | 1 |  | 0280:0000 |
| input | AN 3 | crossFeed | probe | 0.75 | Off | (none) |
| input | AN 3 | crossFeed | restore | 0 | Off | (none) |
| input | AN 3 | volume | probe | 0.75 | -2.2 dB | 12e2:0008 12e2:fea0 13e2:0008 13e2:fea0 4002:88c6 4042:88c6 |
| input | AN 3 | volume | restore | 0 | -oo | 12e2:0008 12e2:d760 13e2:0008 13e2:d760 4002:0000 4042:0000 |
| input | AN 3 | pan | probe | 0.75 | R50 | (none) |
| input | AN 3 | pan | restore | 0.5 | 0 | (none) |
| input | AN 3 | mute | press | 1 |  | 0200:0001 |
| input | AN 3 | mute | press back | 1 |  | 0200:0000 |
| input | AN 3 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | AN 3 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | AN 3 | reverbSend | probe | 0.75 | -16.5 dB | 0201:ff5b |
| input | AN 3 | reverbSend | restore | 0 | -oo | 0201:fd76 |
| input | AN 3 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | AN 3 | reverbReturn | restore | 0 | -oo | (none) |
| input | AN 4 | gain | probe | 0.75 | 12 | 0307:0078 |
| input | AN 4 | gain | restore | 0 | 0 | 0307:0000 |
| input | AN 4 | gainRight | probe | 0.75 | 12 | 0307:0078 |
| input | AN 4 | gainRight | restore | 0 | 0 | 0307:0000 |
| input | AN 4 | pad | press | 1 |  | (none) |
| input | AN 4 | pad | press back | 1 |  | (none) |
| input | AN 4 | instrument | press | 1 |  | 0309:0001 |
| input | AN 4 | instrument | press back | 1 |  | 0309:0000 |
| input | AN 4 | phase | press | 1 |  | 0306:0001 |
| input | AN 4 | phase | press back | 1 |  | 0306:0000 |
| input | AN 4 | phaseRight | press | 1 |  | 0306:0001 |
| input | AN 4 | phaseRight | press back | 1 |  | 0306:0000 |
| input | AN 4 | refLevel | probe | 0.75 | 1 | (none) |
| input | AN 4 | refLevel | restore | 1 | 1 | (none) |
| input | AN 4 | autoset | press | 1 |  | 030a:0001 |
| input | AN 4 | autoset | press back | 1 |  | 030a:0000 |
| input | AN 4 | msProc | press | 1 |  | 0305:0001 |
| input | AN 4 | msProc | press back | 1 |  | 0305:0000 |
| input | AN 4 | width | probe | 0.75 | 0.50 | (none) |
| input | AN 4 | width | restore | 1 | 1.00 | (none) |
| input | AN 4 | loopback | press | 1 |  | (none) |
| input | AN 4 | loopback | press back | 1 |  | (none) |
| input | AN 4 | alevEnable | press | 1 |  | 0380:0001 |
| input | AN 4 | alevEnable | press back | 1 |  | 0380:0000 |
| input | AN 4 | crossFeed | probe | 0.75 | Off | (none) |
| input | AN 4 | crossFeed | restore | 0 | Off | (none) |
| input | AN 4 | volume | probe | 0.75 | -2.2 dB | 12e3:0008 12e3:fea0 13e3:0008 13e3:fea0 4003:88c6 4043:88c6 |
| input | AN 4 | volume | restore | 0 | -oo | 12e3:0008 12e3:d760 13e3:0008 13e3:d760 4003:0000 4043:0000 |
| input | AN 4 | pan | probe | 0.75 | R50 | (none) |
| input | AN 4 | pan | restore | 0.5 | 0 | (none) |
| input | AN 4 | mute | press | 1 |  | 0300:0001 |
| input | AN 4 | mute | press back | 1 |  | 0300:0000 |
| input | AN 4 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | AN 4 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | AN 4 | reverbSend | probe | 0.75 | -16.5 dB | 0301:ff5b |
| input | AN 4 | reverbSend | restore | 0 | -oo | 0301:fd76 |
| input | AN 4 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | AN 4 | reverbReturn | restore | 0 | -oo | (none) |
| input | AN 5/6 | gain | probe | 0.75 | 49 | (none) |
| input | AN 5/6 | gain | restore | 0 | 0 | (none) |
| input | AN 5/6 | gainRight | probe | 0.75 | 49 | (none) |
| input | AN 5/6 | gainRight | restore | 0 | 0 | (none) |
| input | AN 5/6 | pad | press | 1 |  | (none) |
| input | AN 5/6 | pad | press back | 1 |  | (none) |
| input | AN 5/6 | instrument | press | 1 |  | (none) |
| input | AN 5/6 | instrument | press back | 1 |  | (none) |
| input | AN 5/6 | phase | press | 1 |  | 0406:0001 |
| input | AN 5/6 | phase | press back | 1 |  | 0406:0000 |
| input | AN 5/6 | phaseRight | press | 1 |  | 0506:0001 |
| input | AN 5/6 | phaseRight | press back | 1 |  | 0506:0000 |
| input | AN 5/6 | refLevel | probe | 0.75 | 1 | (none) |
| input | AN 5/6 | refLevel | restore | 1 | 1 | (none) |
| input | AN 5/6 | autoset | press | 1 |  | (none) |
| input | AN 5/6 | autoset | press back | 1 |  | (none) |
| input | AN 5/6 | msProc | press | 1 |  | 0405:0001 0505:0001 |
| input | AN 5/6 | msProc | press back | 1 |  | 0405:0000 0505:0000 |
| input | AN 5/6 | width | probe | 0.75 | 0.50 | (none) |
| input | AN 5/6 | width | restore | 1 | 1.00 | (none) |
| input | AN 5/6 | loopback | press | 1 |  | (none) |
| input | AN 5/6 | loopback | press back | 1 |  | (none) |
| input | AN 5/6 | alevEnable | press | 1 |  | 0480:0001 0580:0001 |
| input | AN 5/6 | alevEnable | press back | 1 |  | 0480:0000 0580:0000 |
| input | AN 5/6 | crossFeed | probe | 0.75 | Off | (none) |
| input | AN 5/6 | crossFeed | restore | 0 | Off | (none) |
| input | AN 5/6 | volume | probe | 0.75 | -2.2 dB | 12e4:f9c8 12e4:fea0 13e5:0648 13e5:fea0 4004:8c69 4045:8c69 |
| input | AN 5/6 | volume | restore | 0 | -oo | 12e4:d760 13e5:d760 4004:0000 4045:0000 |
| input | AN 5/6 | pan | probe | 0.75 | R50 | (none) |
| input | AN 5/6 | pan | restore | 0.5 | 0 | (none) |
| input | AN 5/6 | mute | press | 1 |  | 0400:0001 0500:0001 |
| input | AN 5/6 | mute | press back | 1 |  | 0400:0000 0500:0000 |
| input | AN 5/6 | solo | press | 1 |  | 12e6:d760 12e6:f9c8 13e7:0648 13e7:d760 4006:0000 4020:0000 4047:0000 4061:0000 |
| input | AN 5/6 | solo | press back | 1 |  | 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4020:9000 4047:258f 4061:9000 |
| input | AN 5/6 | reverbSend | probe | 0.75 | -16.5 dB | 0401:ff5b 0501:ff5b req16 v=0x0800 i=0x0800 req16 v=0x0800 i=0x0800 req16 v=0x8000 i=0x8000 req29 v=0x0000 i=0x0000 |
| input | AN 5/6 | reverbSend | restore | 0 | -oo | 0401:fd76 0501:fd76 |
| input | AN 5/6 | reverbReturn | probe | 0.75 | -oo | (none) |
| input | AN 5/6 | reverbReturn | restore | 0 | -oo | (none) |
