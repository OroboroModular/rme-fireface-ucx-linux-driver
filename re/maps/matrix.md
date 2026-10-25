# Channel settings → register writes

From `06-matrix-all.pcap` (360 changes). Words are `address:value` (hex, bits 30–16 : 15–0).

## Per parameter

| Row | Channel | Parameter | Addresses | OSC value → display → words |
|---|---|---|---|---|
| input@AN 1/2 | Mic 1 | volume | `0001` `12e0` `13e0` `4000` `4040` | 0.5 → -12.1 dB → 0001:ff87 12e0:0008 12e0:f870 13e0:0008 13e0:f870 4000:1668 4040:1668<br>0 → -oo → 0001:fd76 12e0:0008 12e0:d760 13e0:0008 13e0:d760 4000:0000 4040:0000 |
| input@AN 1/2 | Mic 2 | volume | `0101` `12e1` `13e1` `4001` `4041` | 0.5 → -12.1 dB → 0101:ff87 12e1:0008 12e1:f870 13e1:0008 13e1:f870 4001:1668 4041:1668<br>0 → -oo → 0101:fd76 12e1:0008 12e1:d760 13e1:0008 13e1:d760 4001:0000 4041:0000 |
| input@AN 1/2 | AN 3 | volume | `0201` `12e2` `13e2` `4002` `4042` | 0.5 → -12.1 dB → 0201:ff87 12e2:0008 12e2:f870 13e2:0008 13e2:f870 4002:1668 4042:1668<br>0 → -oo → 0201:fd76 12e2:0008 12e2:d760 13e2:0008 13e2:d760 4002:0000 4042:0000 |
| input@AN 1/2 | AN 4 | volume | `0301` `12e3` `13e3` `4003` `4043` | 0.5 → -12.1 dB → 0301:ff87 12e3:0008 12e3:f870 13e3:0008 13e3:f870 4003:1668 4043:1668<br>0 → -oo → 0301:fd76 12e3:0008 12e3:d760 13e3:0008 13e3:d760 4003:0000 4043:0000 |
| input@AN 1/2 | AN 5/6 | volume | `0401` `0501` `12e4` `13e5` `4004` `4045` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 12e4:f870 12e4:f9c8 13e5:0648 13e5:f870 4004:1fb1 4045:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 12e4:d760 13e5:d760 4004:0000 4045:0000 |
| input@AN 1/2 | AN 7/8 | volume | `0601` `0701` `12e6` `13e7` `4006` `4047` | 0.5 → -12.1 dB → req16 v=0x8000 i=0x8000 req29 v=0x0000 i=0x0000 req20 v=0x0000 i=0x000c 0601:fd76 0701:fd76 12e6:f870 12e6:f9c8 13e7:0648 13e7:f870 4006:1fb1 4047:1fb1<br>0.5264 → -10.6 dB → 0601:fd76 0701:fd76 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f req19 v=0x0000 i=0x000c |
| input@AN 1/2 | SPDIF | volume | `0801` `0901` `12e8` `13e9` `4008` `4049` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 12e8:f870 12e8:f9c8 13e9:0648 13e9:f870 4008:1fb1 4049:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 12e8:d760 13e9:d760 4008:0000 4049:0000 |
| input@AN 1/2 | AS 1/2 | volume | `0a01` `0b01` `12ea` `13eb` `400a` `404b` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 12ea:f870 12ea:f9c8 13eb:0648 13eb:f870 400a:1fb1 404b:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 12ea:d760 13eb:d760 400a:0000 404b:0000 |
| input@AN 1/2 | ADAT 3/4 | volume | `0c01` `0d01` `12ec` `13ed` `400c` `404d` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 12ec:f870 12ec:f9c8 13ed:0648 13ed:f870 400c:1fb1 404d:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 12ec:d760 13ed:d760 400c:0000 404d:0000 |
| input@AN 1/2 | ADAT 5/6 | volume | `0e01` `0f01` `12ee` `13ef` `400e` `404f` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 12ee:f870 12ee:f9c8 13ef:0648 13ef:f870 400e:1fb1 404f:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 12ee:d760 13ef:d760 400e:0000 404f:0000 |
| input@AN 1/2 | ADAT 7/8 | volume | `1001` `1101` `12f0` `13f1` `4010` `4051` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 12f0:f870 12f0:f9c8 13f1:0648 13f1:f870 4010:1fb1 4051:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 12f0:d760 13f1:d760 4010:0000 4051:0000 |
| playback@AN 1/2 | AN 1/2 | volume | `4020` `4061` | 0.5 → -12.1 dB → 4020:1fb1 4061:1fb1<br>0.8172 → 0.0 dB → 4020:9000 4061:9000 |
| playback@AN 1/2 | AN 3/4 | volume | `4022` `4063` | 0.5 → -12.1 dB → 4022:1fb1 4063:1fb1<br>0 → -oo → 4022:0000 4063:0000 |
| playback@AN 1/2 | AN 5/6 | volume | `4024` `4065` | 0.5 → -12.1 dB → 4024:1fb1 4065:1fb1<br>0 → -oo → 4024:0000 4065:0000 |
| playback@AN 1/2 | PH 7/8 | volume | `4026` `4067` | 0.5 → -12.1 dB → 4026:1fb1 4067:1fb1<br>0 → -oo → 4026:0000 4067:0000 |
| playback@AN 1/2 | SPDIF | volume | `4028` `4069` | 0.5 → -12.1 dB → 4028:1fb1 4069:1fb1<br>0 → -oo → 4028:0000 4069:0000 |
| playback@AN 1/2 | AS 1/2 | volume | `402a` `406b` | 0.5 → -12.1 dB → 402a:1fb1 406b:1fb1<br>0 → -oo → 402a:0000 406b:0000 |
| playback@AN 1/2 | ADAT 3/4 | volume | `402c` `406d` | 0.5 → -12.1 dB → 402c:1fb1 406d:1fb1<br>0 → -oo → 402c:0000 406d:0000 |
| playback@AN 1/2 | ADAT 5/6 | volume | `402e` `406f` | 0.5 → -12.1 dB → 402e:1fb1 406f:1fb1<br>0 → -oo → 402e:0000 406f:0000 |
| playback@AN 1/2 | ADAT 7/8 | volume | `4030` `4071` | 0.5 → -12.1 dB → 4030:1fb1 4071:1fb1<br>0 → -oo → 4030:0000 4071:0000 |
| input@AN 3/4 | Mic 1 | volume | `0001` `14e0` `15e0` `4080` `40c0` | 0.5 → -12.1 dB → 0001:ff87 14e0:0008 14e0:f870 15e0:0008 15e0:f870 4080:1668 40c0:1668<br>0 → -oo → 0001:fd76 14e0:0008 14e0:d760 15e0:0008 15e0:d760 4080:0000 40c0:0000 |
| input@AN 3/4 | Mic 2 | volume | `0101` `14e1` `15e1` `4081` `40c1` | 0.5 → -12.1 dB → 0101:ff87 14e1:0008 14e1:f870 15e1:0008 15e1:f870 4081:1668 40c1:1668<br>0 → -oo → 0101:fd76 14e1:0008 14e1:d760 15e1:0008 15e1:d760 4081:0000 40c1:0000 |
| input@AN 3/4 | AN 3 | volume | `0201` `14e2` `15e2` `4082` `40c2` | 0.5 → -12.1 dB → 0201:ff87 14e2:0008 14e2:f870 15e2:0008 15e2:f870 4082:1668 40c2:1668<br>0 → -oo → 0201:fd76 14e2:0008 14e2:d760 15e2:0008 15e2:d760 4082:0000 40c2:0000 |
| input@AN 3/4 | AN 4 | volume | `0301` `14e3` `15e3` `4083` `40c3` | 0.5 → -12.1 dB → 0301:ff87 14e3:0008 14e3:f870 15e3:0008 15e3:f870 4083:1668 40c3:1668<br>0 → -oo → 0301:fd76 14e3:0008 14e3:d760 15e3:0008 15e3:d760 4083:0000 40c3:0000 |
| input@AN 3/4 | AN 5/6 | volume | `0401` `0501` `14e4` `15e5` `4084` `40c5` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 14e4:f870 14e4:f9c8 15e5:0648 15e5:f870 4084:1fb1 40c5:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 14e4:d760 15e5:d760 4084:0000 40c5:0000 |
| input@AN 3/4 | AN 7/8 | volume | `14e6` `15e7` `4086` `40c7` | 0.5 → -12.1 dB → 14e6:f870 14e6:f9c8 15e7:0648 15e7:f870 4086:1fb1 40c7:1fb1<br>0 → -oo → 14e6:d760 15e7:d760 4086:0000 40c7:0000 |
| input@AN 3/4 | SPDIF | volume | `0801` `0901` `14e8` `15e9` `4088` `40c9` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 14e8:f870 14e8:f9c8 15e9:0648 15e9:f870 4088:1fb1 40c9:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 14e8:d760 15e9:d760 4088:0000 40c9:0000 |
| input@AN 3/4 | AS 1/2 | volume | `0a01` `0b01` `14ea` `15eb` `408a` `40cb` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 14ea:f870 14ea:f9c8 15eb:0648 15eb:f870 408a:1fb1 40cb:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 14ea:d760 15eb:d760 408a:0000 40cb:0000 |
| input@AN 3/4 | ADAT 3/4 | volume | `0c01` `0d01` `14ec` `15ed` `408c` `40cd` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 14ec:f870 14ec:f9c8 15ed:0648 15ed:f870 408c:1fb1 40cd:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 14ec:d760 15ed:d760 408c:0000 40cd:0000 |
| input@AN 3/4 | ADAT 5/6 | volume | `0e01` `0f01` `14ee` `15ef` `408e` `40cf` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 14ee:f870 14ee:f9c8 15ef:0648 15ef:f870 408e:1fb1 40cf:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 14ee:d760 15ef:d760 408e:0000 40cf:0000 |
| input@AN 3/4 | ADAT 7/8 | volume | `1001` `1101` `14f0` `15f1` `4090` `40d1` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 14f0:f870 14f0:f9c8 15f1:0648 15f1:f870 4090:1fb1 40d1:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 14f0:d760 15f1:d760 4090:0000 40d1:0000 |
| playback@AN 3/4 | AN 1/2 | volume | `40a0` `40e1` | 0.5 → -12.1 dB → 40a0:1fb1 40e1:1fb1<br>0 → -oo → 40a0:0000 40e1:0000 |
| playback@AN 3/4 | AN 3/4 | volume | `40a2` `40e3` | 0.5 → -12.1 dB → 40a2:1fb1 40e3:1fb1<br>0.8172 → 0.0 dB → 40a2:9000 40e3:9000 |
| playback@AN 3/4 | AN 5/6 | volume | `40a4` `40e5` | 0.5 → -12.1 dB → 40a4:1fb1 40e5:1fb1<br>0 → -oo → 40a4:0000 40e5:0000 |
| playback@AN 3/4 | PH 7/8 | volume | `40a6` `40e7` | 0.5 → -12.1 dB → 40a6:1fb1 40e7:1fb1<br>0 → -oo → 40a6:0000 40e7:0000 |
| playback@AN 3/4 | SPDIF | volume | `40a8` `40e9` | 0.5 → -12.1 dB → 40a8:1fb1 40e9:1fb1<br>0 → -oo → 40a8:0000 40e9:0000 |
| playback@AN 3/4 | AS 1/2 | volume | `40aa` `40eb` | 0.5 → -12.1 dB → 40aa:1fb1 40eb:1fb1<br>0 → -oo → 40aa:0000 40eb:0000 |
| playback@AN 3/4 | ADAT 3/4 | volume | `40ac` `40ed` | 0.5 → -12.1 dB → 40ac:1fb1 40ed:1fb1<br>0 → -oo → 40ac:0000 40ed:0000 |
| playback@AN 3/4 | ADAT 5/6 | volume | `40ae` `40ef` | 0.5 → -12.1 dB → 40ae:1fb1 40ef:1fb1<br>0 → -oo → 40ae:0000 40ef:0000 |
| playback@AN 3/4 | ADAT 7/8 | volume | `40b0` `40f1` | 0.5 → -12.1 dB → 40b0:1fb1 40f1:1fb1<br>0 → -oo → 40b0:0000 40f1:0000 |
| input@AN 5/6 | Mic 1 | volume | `0001` `16e0` `17e0` `4100` `4140` | 0.5 → -12.1 dB → 0001:ff87 16e0:0008 16e0:f870 17e0:0008 17e0:f870 4100:1668 4140:1668<br>0 → -oo → 0001:fd76 16e0:0008 16e0:d760 17e0:0008 17e0:d760 4100:0000 4140:0000 |
| input@AN 5/6 | Mic 2 | volume | `0101` `16e1` `17e1` `4101` `4141` | 0.5 → -12.1 dB → 0101:ff87 16e1:0008 16e1:f870 17e1:0008 17e1:f870 4101:1668 4141:1668<br>0 → -oo → 0101:fd76 16e1:0008 16e1:d760 17e1:0008 17e1:d760 4101:0000 4141:0000 |
| input@AN 5/6 | AN 3 | volume | `0201` `16e2` `17e2` `4102` `4142` | 0.5 → -12.1 dB → 0201:ff87 16e2:0008 16e2:f870 17e2:0008 17e2:f870 4102:1668 4142:1668<br>0 → -oo → 0201:fd76 16e2:0008 16e2:d760 17e2:0008 17e2:d760 4102:0000 4142:0000 |
| input@AN 5/6 | AN 4 | volume | `0301` `16e3` `17e3` `4103` `4143` | 0.5 → -12.1 dB → 0301:ff87 16e3:0008 16e3:f870 17e3:0008 17e3:f870 4103:1668 4143:1668<br>0 → -oo → 0301:fd76 16e3:0008 16e3:d760 17e3:0008 17e3:d760 4103:0000 4143:0000 |
| input@AN 5/6 | AN 5/6 | volume | `0401` `0501` `16e4` `17e5` `4104` `4145` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 16e4:f870 16e4:f9c8 17e5:0648 17e5:f870 4104:1fb1 4145:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 16e4:d760 17e5:d760 4104:0000 4145:0000 |
| input@AN 5/6 | AN 7/8 | volume | `16e6` `17e7` `4106` `4147` | 0.5 → -12.1 dB → 16e6:f870 16e6:f9c8 17e7:0648 17e7:f870 4106:1fb1 4147:1fb1<br>0 → -oo → 16e6:d760 17e7:d760 4106:0000 4147:0000 |
| input@AN 5/6 | SPDIF | volume | `0801` `0901` `16e8` `17e9` `4108` `4149` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 16e8:f870 16e8:f9c8 17e9:0648 17e9:f870 4108:1fb1 4149:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 16e8:d760 17e9:d760 4108:0000 4149:0000 |
| input@AN 5/6 | AS 1/2 | volume | `0a01` `0b01` `16ea` `17eb` `410a` `414b` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 16ea:f870 16ea:f9c8 17eb:0648 17eb:f870 410a:1fb1 414b:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 16ea:d760 17eb:d760 410a:0000 414b:0000 |
| input@AN 5/6 | ADAT 3/4 | volume | `0c01` `0d01` `16ec` `17ed` `410c` `414d` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 16ec:f870 16ec:f9c8 17ed:0648 17ed:f870 410c:1fb1 414d:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 16ec:d760 17ed:d760 410c:0000 414d:0000 |
| input@AN 5/6 | ADAT 5/6 | volume | `0e01` `0f01` `16ee` `17ef` `410e` `414f` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 16ee:f870 16ee:f9c8 17ef:0648 17ef:f870 410e:1fb1 414f:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 16ee:d760 17ef:d760 410e:0000 414f:0000 |
| input@AN 5/6 | ADAT 7/8 | volume | `1001` `1101` `16f0` `17f1` `4110` `4151` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 16f0:f870 16f0:f9c8 17f1:0648 17f1:f870 4110:1fb1 4151:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 16f0:d760 17f1:d760 4110:0000 4151:0000 |
| playback@AN 5/6 | AN 1/2 | volume | `4120` `4161` | 0.5 → -12.1 dB → 4120:1fb1 4161:1fb1<br>0 → -oo → 4120:0000 4161:0000 |
| playback@AN 5/6 | AN 3/4 | volume | `4122` `4163` | 0.5 → -12.1 dB → 4122:1fb1 4163:1fb1<br>0 → -oo → 4122:0000 4163:0000 |
| playback@AN 5/6 | AN 5/6 | volume | `4124` `4165` | 0.5 → -12.1 dB → 4124:1fb1 4165:1fb1<br>0.8172 → 0.0 dB → 4124:9000 4165:9000 |
| playback@AN 5/6 | PH 7/8 | volume | `4126` `4167` | 0.5 → -12.1 dB → 4126:1fb1 4167:1fb1<br>0 → -oo → 4126:0000 4167:0000 |
| playback@AN 5/6 | SPDIF | volume | `4128` `4169` | 0.5 → -12.1 dB → 4128:1fb1 4169:1fb1<br>0 → -oo → 4128:0000 4169:0000 |
| playback@AN 5/6 | AS 1/2 | volume | `412a` `416b` | 0.5 → -12.1 dB → 412a:1fb1 416b:1fb1<br>0 → -oo → 412a:0000 416b:0000 |
| playback@AN 5/6 | ADAT 3/4 | volume | `412c` `416d` | 0.5 → -12.1 dB → 412c:1fb1 416d:1fb1<br>0 → -oo → 412c:0000 416d:0000 |
| playback@AN 5/6 | ADAT 5/6 | volume | `412e` `416f` | 0.5 → -12.1 dB → 412e:1fb1 416f:1fb1<br>0 → -oo → 412e:0000 416f:0000 |
| playback@AN 5/6 | ADAT 7/8 | volume | `4130` `4171` | 0.5 → -12.1 dB → 4130:1fb1 4171:1fb1<br>0 → -oo → 4130:0000 4171:0000 |
| input@Main | Mic 1 | volume | `0001` `18e0` `19e0` `4180` `41c0` | 0.5 → -12.1 dB → 0001:ff87 18e0:0008 18e0:f870 19e0:0008 19e0:f870 4180:1668 41c0:1668<br>0 → -oo → 0001:fd76 18e0:0008 18e0:d760 19e0:0008 19e0:d760 4180:0000 41c0:0000 |
| input@Main | Mic 2 | volume | `0101` `18e1` `19e1` `4181` `41c1` | 0.5 → -12.1 dB → 0101:ff87 18e1:0008 18e1:f870 19e1:0008 19e1:f870 4181:1668 41c1:1668<br>0 → -oo → 0101:fd76 18e1:0008 18e1:d760 19e1:0008 19e1:d760 4181:0000 41c1:0000 |
| input@Main | AN 3 | volume | `0201` `18e2` `19e2` `4182` `41c2` | 0.5 → -12.1 dB → 0201:ff87 18e2:0008 18e2:f870 19e2:0008 19e2:f870 4182:1668 41c2:1668<br>0 → -oo → 0201:fd76 18e2:0008 18e2:d760 19e2:0008 19e2:d760 4182:0000 41c2:0000 |
| input@Main | AN 4 | volume | `0301` `18e3` `19e3` `4183` `41c3` | 0.5 → -12.1 dB → 0301:ff87 18e3:0008 18e3:f870 19e3:0008 19e3:f870 4183:1668 41c3:1668<br>0 → -oo → 0301:fd76 18e3:0008 18e3:d760 19e3:0008 19e3:d760 4183:0000 41c3:0000 |
| input@Main | AN 5/6 | volume | `0401` `0501` `18e4` `19e5` `4184` `41c5` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 18e4:f870 18e4:f9c8 19e5:0648 19e5:f870 4184:1fb1 41c5:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 18e4:d760 19e5:d760 4184:0000 41c5:0000 |
| input@Main | AN 7/8 | volume | `18e6` `19e7` `4186` `41c7` | 0.5 → -12.1 dB → 18e6:f870 18e6:f9c8 19e7:0648 19e7:f870 4186:1fb1 41c7:1fb1<br>0 → -oo → 18e6:d760 19e7:d760 4186:0000 41c7:0000 |
| input@Main | SPDIF | volume | `0801` `0901` `18e8` `19e9` `4188` `41c9` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 18e8:f870 18e8:f9c8 19e9:0648 19e9:f870 4188:1fb1 41c9:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 18e8:d760 19e9:d760 4188:0000 41c9:0000 |
| input@Main | AS 1/2 | volume | `0a01` `0b01` `18ea` `19eb` `418a` `41cb` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 18ea:f870 18ea:f9c8 19eb:0648 19eb:f870 418a:1fb1 41cb:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 18ea:d760 19eb:d760 418a:0000 41cb:0000 |
| input@Main | ADAT 3/4 | volume | `0c01` `0d01` `18ec` `19ed` `418c` `41cd` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 18ec:f870 18ec:f9c8 19ed:0648 19ed:f870 418c:1fb1 41cd:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 18ec:d760 19ed:d760 418c:0000 41cd:0000 |
| input@Main | ADAT 5/6 | volume | `0e01` `0f01` `18ee` `19ef` `418e` `41cf` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 18ee:f870 18ee:f9c8 19ef:0648 19ef:f870 418e:1fb1 41cf:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 18ee:d760 19ef:d760 418e:0000 41cf:0000 |
| input@Main | ADAT 7/8 | volume | `1001` `1101` `18f0` `19f1` `4190` `41d1` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 18f0:f870 18f0:f9c8 19f1:0648 19f1:f870 4190:1fb1 41d1:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 18f0:d760 19f1:d760 4190:0000 41d1:0000 |
| playback@Main | AN 1/2 | volume | `41a0` `41e1` | 0.5 → -12.1 dB → 41a0:1fb1 41e1:1fb1<br>0.8172 → 0.0 dB → 41a0:9000 41e1:9000 |
| playback@Main | AN 3/4 | volume | `41a2` `41e3` | 0.5 → -12.1 dB → 41a2:1fb1 41e3:1fb1<br>0.8172 → 0.0 dB → 41a2:9000 41e3:9000 |
| playback@Main | AN 5/6 | volume | `41a4` `41e5` | 0.5 → -12.1 dB → 41a4:1fb1 41e5:1fb1<br>0.8172 → 0.0 dB → 41a4:9000 41e5:9000 |
| playback@Main | PH 7/8 | volume | `41a6` `41e7` | 0.5 → -12.1 dB → 41a6:1fb1 41e7:1fb1<br>0.8172 → 0.0 dB → 41a6:9000 41e7:9000 |
| playback@Main | SPDIF | volume | `41a8` `41e9` | 0.5 → -12.1 dB → 41a8:1fb1 41e9:1fb1<br>0.8172 → 0.0 dB → 41a8:9000 41e9:9000 |
| playback@Main | AS 1/2 | volume | `41aa` `41eb` | 0.5 → -12.1 dB → 41aa:1fb1 41eb:1fb1<br>0.8172 → 0.0 dB → 41aa:9000 41eb:9000 |
| playback@Main | ADAT 3/4 | volume | `41ac` `41ed` | 0.5 → -12.1 dB → 41ac:1fb1 41ed:1fb1<br>0.8172 → 0.0 dB → 41ac:9000 41ed:9000 |
| playback@Main | ADAT 5/6 | volume | `41ae` `41ef` | 0.5 → -12.1 dB → 41ae:1fb1 41ef:1fb1<br>0.8172 → 0.0 dB → 41ae:9000 41ef:9000 |
| playback@Main | ADAT 7/8 | volume | `41b0` `41f1` | 0.5 → -12.1 dB → 41b0:1fb1 41f1:1fb1<br>0.8172 → 0.0 dB → 41b0:9000 41f1:9000 |
| input@SPDIF | Mic 1 | volume | `0001` `1ae0` `1be0` `4200` `4240` | 0.5 → -12.1 dB → 0001:ff87 1ae0:0008 1ae0:f870 1be0:0008 1be0:f870 4200:1668 4240:1668<br>0 → -oo → 0001:fd76 1ae0:0008 1ae0:d760 1be0:0008 1be0:d760 4200:0000 4240:0000 |
| input@SPDIF | Mic 2 | volume | `0101` `1ae1` `1be1` `4201` `4241` | 0.5 → -12.1 dB → 0101:ff87 1ae1:0008 1ae1:f870 1be1:0008 1be1:f870 4201:1668 4241:1668<br>0 → -oo → 0101:fd76 1ae1:0008 1ae1:d760 1be1:0008 1be1:d760 4201:0000 4241:0000 |
| input@SPDIF | AN 3 | volume | `0201` `1ae2` `1be2` `4202` `4242` | 0.5 → -12.1 dB → 0201:ff87 1ae2:0008 1ae2:f870 1be2:0008 1be2:f870 4202:1668 4242:1668<br>0 → -oo → 0201:fd76 1ae2:0008 1ae2:d760 1be2:0008 1be2:d760 4202:0000 4242:0000 |
| input@SPDIF | AN 4 | volume | `0301` `1ae3` `1be3` `4203` `4243` | 0.5 → -12.1 dB → 0301:ff87 1ae3:0008 1ae3:f870 1be3:0008 1be3:f870 4203:1668 4243:1668<br>0 → -oo → 0301:fd76 1ae3:0008 1ae3:d760 1be3:0008 1be3:d760 4203:0000 4243:0000 |
| input@SPDIF | AN 5/6 | volume | `0401` `0501` `1ae4` `1be5` `4204` `4245` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 1ae4:f870 1ae4:f9c8 1be5:0648 1be5:f870 4204:1fb1 4245:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 1ae4:d760 1be5:d760 4204:0000 4245:0000 |
| input@SPDIF | AN 7/8 | volume | `1ae6` `1be7` `4206` `4247` | 0.5 → -12.1 dB → 1ae6:f870 1ae6:f9c8 1be7:0648 1be7:f870 4206:1fb1 4247:1fb1<br>0 → -oo → 1ae6:d760 1be7:d760 4206:0000 4247:0000 |
| input@SPDIF | SPDIF | volume | `0801` `0901` `1ae8` `1be9` `4208` `4249` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 1ae8:f870 1ae8:f9c8 1be9:0648 1be9:f870 4208:1fb1 4249:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 1ae8:d760 1be9:d760 4208:0000 4249:0000 |
| input@SPDIF | AS 1/2 | volume | `0a01` `0b01` `1aea` `1beb` `420a` `424b` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 1aea:f870 1aea:f9c8 1beb:0648 1beb:f870 420a:1fb1 424b:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 1aea:d760 1beb:d760 420a:0000 424b:0000 |
| input@SPDIF | ADAT 3/4 | volume | `0c01` `0d01` `1aec` `1bed` `420c` `424d` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 1aec:f870 1aec:f9c8 1bed:0648 1bed:f870 420c:1fb1 424d:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 1aec:d760 1bed:d760 420c:0000 424d:0000 |
| input@SPDIF | ADAT 5/6 | volume | `0e01` `0f01` `1aee` `1bef` `420e` `424f` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 1aee:f870 1aee:f9c8 1bef:0648 1bef:f870 420e:1fb1 424f:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 1aee:d760 1bef:d760 420e:0000 424f:0000 |
| input@SPDIF | ADAT 7/8 | volume | `1001` `1101` `1af0` `1bf1` `4210` `4251` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 1af0:f870 1af0:f9c8 1bf1:0648 1bf1:f870 4210:1fb1 4251:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 1af0:d760 1bf1:d760 4210:0000 4251:0000 |
| playback@SPDIF | AN 1/2 | volume | `4220` `4261` | 0.5 → -12.1 dB → 4220:1fb1 4261:1fb1<br>0 → -oo → 4220:0000 4261:0000 |
| playback@SPDIF | AN 3/4 | volume | `4222` `4263` | 0.5 → -12.1 dB → 4222:1fb1 4263:1fb1<br>0 → -oo → 4222:0000 4263:0000 |
| playback@SPDIF | AN 5/6 | volume | `4224` `4265` | 0.5 → -12.1 dB → 4224:1fb1 4265:1fb1<br>0 → -oo → 4224:0000 4265:0000 req16 v=0x8000 i=0x8000 req29 v=0x0000 i=0x0000 req20 v=0x0000 i=0x000c req19 v=0x0000 i=0x000c |
| playback@SPDIF | PH 7/8 | volume | `4226` `4267` | 0.5 → -12.1 dB → 4226:1fb1 4267:1fb1<br>0 → -oo → 4226:0000 4267:0000 |
| playback@SPDIF | SPDIF | volume | `4228` `4269` | 0.5 → -12.1 dB → 4228:1fb1 4269:1fb1<br>0.8172 → 0.0 dB → 4228:9000 4269:9000 |
| playback@SPDIF | AS 1/2 | volume | `422a` `426b` | 0.5 → -12.1 dB → 422a:1fb1 426b:1fb1<br>0 → -oo → 422a:0000 426b:0000 |
| playback@SPDIF | ADAT 3/4 | volume | `422c` `426d` | 0.5 → -12.1 dB → 422c:1fb1 426d:1fb1<br>0 → -oo → 422c:0000 426d:0000 |
| playback@SPDIF | ADAT 5/6 | volume | `422e` `426f` | 0.5 → -12.1 dB → 422e:1fb1 426f:1fb1<br>0 → -oo → 422e:0000 426f:0000 |
| playback@SPDIF | ADAT 7/8 | volume | `4230` `4271` | 0.5 → -12.1 dB → 4230:1fb1 4271:1fb1<br>0 → -oo → 4230:0000 4271:0000 |
| input@AS 1/2 | Mic 1 | volume | `0001` `1ce0` `1de0` `4280` `42c0` | 0.5 → -12.1 dB → 0001:ff87 1ce0:0008 1ce0:f870 1de0:0008 1de0:f870 4280:1668 42c0:1668<br>0 → -oo → 0001:fd76 1ce0:0008 1ce0:d760 1de0:0008 1de0:d760 4280:0000 42c0:0000 |
| input@AS 1/2 | Mic 2 | volume | `0101` `1ce1` `1de1` `4281` `42c1` | 0.5 → -12.1 dB → 0101:ff87 1ce1:0008 1ce1:f870 1de1:0008 1de1:f870 4281:1668 42c1:1668<br>0 → -oo → 0101:fd76 1ce1:0008 1ce1:d760 1de1:0008 1de1:d760 4281:0000 42c1:0000 |
| input@AS 1/2 | AN 3 | volume | `0201` `1ce2` `1de2` `4282` `42c2` | 0.5 → -12.1 dB → 0201:ff87 1ce2:0008 1ce2:f870 1de2:0008 1de2:f870 4282:1668 42c2:1668<br>0 → -oo → 0201:fd76 1ce2:0008 1ce2:d760 1de2:0008 1de2:d760 4282:0000 42c2:0000 |
| input@AS 1/2 | AN 4 | volume | `0301` `1ce3` `1de3` `4283` `42c3` | 0.5 → -12.1 dB → 0301:ff87 1ce3:0008 1ce3:f870 1de3:0008 1de3:f870 4283:1668 42c3:1668<br>0 → -oo → 0301:fd76 1ce3:0008 1ce3:d760 1de3:0008 1de3:d760 4283:0000 42c3:0000 |
| input@AS 1/2 | AN 5/6 | volume | `0401` `0501` `1ce4` `1de5` `4284` `42c5` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 1ce4:f870 1ce4:f9c8 1de5:0648 1de5:f870 4284:1fb1 42c5:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 1ce4:d760 1de5:d760 4284:0000 42c5:0000 |
| input@AS 1/2 | AN 7/8 | volume | `1ce6` `1de7` `4286` `42c7` | 0.5 → -12.1 dB → 1ce6:f870 1ce6:f9c8 1de7:0648 1de7:f870 4286:1fb1 42c7:1fb1<br>0 → -oo → 1ce6:d760 1de7:d760 4286:0000 42c7:0000 |
| input@AS 1/2 | SPDIF | volume | `0801` `0901` `1ce8` `1de9` `4288` `42c9` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 1ce8:f870 1ce8:f9c8 1de9:0648 1de9:f870 4288:1fb1 42c9:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 1ce8:d760 1de9:d760 4288:0000 42c9:0000 |
| input@AS 1/2 | AS 1/2 | volume | `0a01` `0b01` `1cea` `1deb` `428a` `42cb` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 1cea:f870 1cea:f9c8 1deb:0648 1deb:f870 428a:1fb1 42cb:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 1cea:d760 1deb:d760 428a:0000 42cb:0000 |
| input@AS 1/2 | ADAT 3/4 | volume | `0c01` `0d01` `1cec` `1ded` `428c` `42cd` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 1cec:f870 1cec:f9c8 1ded:0648 1ded:f870 428c:1fb1 42cd:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 1cec:d760 1ded:d760 428c:0000 42cd:0000 |
| input@AS 1/2 | ADAT 5/6 | volume | `0e01` `0f01` `1cee` `1def` `428e` `42cf` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 1cee:f870 1cee:f9c8 1def:0648 1def:f870 428e:1fb1 42cf:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 1cee:d760 1def:d760 428e:0000 42cf:0000 |
| input@AS 1/2 | ADAT 7/8 | volume | `1001` `1101` `1cf0` `1df1` `4290` `42d1` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 1cf0:f870 1cf0:f9c8 1df1:0648 1df1:f870 4290:1fb1 42d1:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 1cf0:d760 1df1:d760 4290:0000 42d1:0000 |
| playback@AS 1/2 | AN 1/2 | volume | `42a0` `42e1` | 0.5 → -12.1 dB → 42a0:1fb1 42e1:1fb1<br>0 → -oo → 42a0:0000 42e1:0000 |
| playback@AS 1/2 | AN 3/4 | volume | `42a2` `42e3` | 0.5 → -12.1 dB → 42a2:1fb1 42e3:1fb1<br>0 → -oo → 42a2:0000 42e3:0000 |
| playback@AS 1/2 | AN 5/6 | volume | `42a4` `42e5` | 0.5 → -12.1 dB → 42a4:1fb1 42e5:1fb1<br>0 → -oo → 42a4:0000 42e5:0000 |
| playback@AS 1/2 | PH 7/8 | volume | `42a6` `42e7` | 0.5 → -12.1 dB → 42a6:1fb1 42e7:1fb1<br>0 → -oo → 42a6:0000 42e7:0000 |
| playback@AS 1/2 | SPDIF | volume | `42a8` `42e9` | 0.5 → -12.1 dB → 42a8:1fb1 42e9:1fb1<br>0 → -oo → 42a8:0000 42e9:0000 |
| playback@AS 1/2 | AS 1/2 | volume | `42aa` `42eb` | 0.5 → -12.1 dB → 42aa:1fb1 42eb:1fb1<br>0.8172 → 0.0 dB → 42aa:9000 42eb:9000 |
| playback@AS 1/2 | ADAT 3/4 | volume | `42ac` `42ed` | 0.5 → -12.1 dB → 42ac:1fb1 42ed:1fb1<br>0 → -oo → 42ac:0000 42ed:0000 |
| playback@AS 1/2 | ADAT 5/6 | volume | `42ae` `42ef` | 0.5 → -12.1 dB → 42ae:1fb1 42ef:1fb1<br>0 → -oo → 42ae:0000 42ef:0000 |
| playback@AS 1/2 | ADAT 7/8 | volume | `42b0` `42f1` | 0.5 → -12.1 dB → 42b0:1fb1 42f1:1fb1<br>0 → -oo → 42b0:0000 42f1:0000 |
| input@ADAT 3/4 | Mic 1 | volume | `0001` `1ee0` `1fe0` `4300` `4340` | 0.5 → -12.1 dB → 0001:ff87 1ee0:0008 1ee0:f870 1fe0:0008 1fe0:f870 4300:1668 4340:1668<br>0 → -oo → 0001:fd76 1ee0:0008 1ee0:d760 1fe0:0008 1fe0:d760 4300:0000 4340:0000 |
| input@ADAT 3/4 | Mic 2 | volume | `0101` `1ee1` `1fe1` `4301` `4341` | 0.5 → -12.1 dB → 0101:ff87 1ee1:0008 1ee1:f870 1fe1:0008 1fe1:f870 4301:1668 4341:1668<br>0 → -oo → 0101:fd76 1ee1:0008 1ee1:d760 1fe1:0008 1fe1:d760 4301:0000 4341:0000 |
| input@ADAT 3/4 | AN 3 | volume | `0201` `1ee2` `1fe2` `4302` `4342` | 0.5 → -12.1 dB → 0201:ff87 1ee2:0008 1ee2:f870 1fe2:0008 1fe2:f870 4302:1668 4342:1668<br>0 → -oo → 0201:fd76 1ee2:0008 1ee2:d760 1fe2:0008 1fe2:d760 4302:0000 4342:0000 |
| input@ADAT 3/4 | AN 4 | volume | `0301` `1ee3` `1fe3` `4303` `4343` | 0.5 → -12.1 dB → 0301:ff87 1ee3:0008 1ee3:f870 1fe3:0008 1fe3:f870 4303:1668 4343:1668<br>0 → -oo → 0301:fd76 1ee3:0008 1ee3:d760 1fe3:0008 1fe3:d760 4303:0000 4343:0000 |
| input@ADAT 3/4 | AN 5/6 | volume | `0401` `0501` `1ee4` `1fe5` `4304` `4345` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 1ee4:f870 1ee4:f9c8 1fe5:0648 1fe5:f870 4304:1fb1 4345:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 1ee4:d760 1fe5:d760 4304:0000 4345:0000 |
| input@ADAT 3/4 | AN 7/8 | volume | `1ee6` `1fe7` `4306` `4347` | 0.5 → -12.1 dB → 1ee6:f870 1ee6:f9c8 1fe7:0648 1fe7:f870 4306:1fb1 4347:1fb1<br>0 → -oo → 1ee6:d760 1fe7:d760 4306:0000 4347:0000 |
| input@ADAT 3/4 | SPDIF | volume | `0801` `0901` `1ee8` `1fe9` `4308` `4349` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 1ee8:f870 1ee8:f9c8 1fe9:0648 1fe9:f870 4308:1fb1 4349:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 1ee8:d760 1fe9:d760 4308:0000 4349:0000 |
| input@ADAT 3/4 | AS 1/2 | volume | `0a01` `0b01` `1eea` `1feb` `430a` `434b` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 1eea:f870 1eea:f9c8 1feb:0648 1feb:f870 430a:1fb1 434b:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 1eea:d760 1feb:d760 430a:0000 434b:0000 |
| input@ADAT 3/4 | ADAT 3/4 | volume | `0c01` `0d01` `1eec` `1fed` `430c` `434d` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 1eec:f870 1eec:f9c8 1fed:0648 1fed:f870 430c:1fb1 434d:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 1eec:d760 1fed:d760 430c:0000 434d:0000 |
| input@ADAT 3/4 | ADAT 5/6 | volume | `0e01` `0f01` `1eee` `1fef` `430e` `434f` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 1eee:f870 1eee:f9c8 1fef:0648 1fef:f870 430e:1fb1 434f:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 1eee:d760 1fef:d760 430e:0000 434f:0000 |
| input@ADAT 3/4 | ADAT 7/8 | volume | `1001` `1101` `1ef0` `1ff1` `4310` `4351` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 1ef0:f870 1ef0:f9c8 1ff1:0648 1ff1:f870 4310:1fb1 4351:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 1ef0:d760 1ff1:d760 4310:0000 4351:0000 |
| playback@ADAT 3/4 | AN 1/2 | volume | `4320` `4361` | 0.5 → -12.1 dB → 4320:1fb1 4361:1fb1<br>0 → -oo → 4320:0000 4361:0000 |
| playback@ADAT 3/4 | AN 3/4 | volume | `4322` `4363` | 0.5 → -12.1 dB → 4322:1fb1 4363:1fb1<br>0 → -oo → 4322:0000 4363:0000 |
| playback@ADAT 3/4 | AN 5/6 | volume | `4324` `4365` | 0.5 → -12.1 dB → 4324:1fb1 4365:1fb1<br>0 → -oo → 4324:0000 4365:0000 |
| playback@ADAT 3/4 | PH 7/8 | volume | `4326` `4367` | 0.5 → -12.1 dB → 4326:1fb1 4367:1fb1<br>0 → -oo → 4326:0000 4367:0000 |
| playback@ADAT 3/4 | SPDIF | volume | `4328` `4369` | 0.5 → -12.1 dB → 4328:1fb1 4369:1fb1<br>0 → -oo → 4328:0000 4369:0000 |
| playback@ADAT 3/4 | AS 1/2 | volume | `432a` `436b` | 0.5 → -12.1 dB → 432a:1fb1 436b:1fb1<br>0 → -oo → 432a:0000 436b:0000 |
| playback@ADAT 3/4 | ADAT 3/4 | volume | `432c` `436d` | 0.5 → -12.1 dB → 432c:1fb1 436d:1fb1<br>0.8172 → 0.0 dB → 432c:9000 436d:9000 |
| playback@ADAT 3/4 | ADAT 5/6 | volume | `432e` `436f` | 0.5 → -12.1 dB → 432e:1fb1 436f:1fb1<br>0 → -oo → 432e:0000 436f:0000 |
| playback@ADAT 3/4 | ADAT 7/8 | volume | `4330` `4371` | 0.5 → -12.1 dB → 4330:1fb1 4371:1fb1<br>0 → -oo → 4330:0000 4371:0000 |
| input@ADAT 5/6 | Mic 1 | volume | `0001` `20e0` `21e0` `4380` `43c0` | 0.5 → -12.1 dB → 0001:ff87 20e0:0008 20e0:f870 21e0:0008 21e0:f870 4380:1668 43c0:1668<br>0 → -oo → 0001:fd76 20e0:0008 20e0:d760 21e0:0008 21e0:d760 4380:0000 43c0:0000 |
| input@ADAT 5/6 | Mic 2 | volume | `0101` `20e1` `21e1` `4381` `43c1` | 0.5 → -12.1 dB → 0101:ff87 20e1:0008 20e1:f870 21e1:0008 21e1:f870 4381:1668 43c1:1668<br>0 → -oo → 0101:fd76 20e1:0008 20e1:d760 21e1:0008 21e1:d760 4381:0000 43c1:0000 |
| input@ADAT 5/6 | AN 3 | volume | `0201` `20e2` `21e2` `4382` `43c2` | 0.5 → -12.1 dB → 0201:ff87 20e2:0008 20e2:f870 21e2:0008 21e2:f870 4382:1668 43c2:1668<br>0 → -oo → 0201:fd76 20e2:0008 20e2:d760 21e2:0008 21e2:d760 4382:0000 43c2:0000 |
| input@ADAT 5/6 | AN 4 | volume | `0301` `20e3` `21e3` `4383` `43c3` | 0.5 → -12.1 dB → 0301:ff87 20e3:0008 20e3:f870 21e3:0008 21e3:f870 4383:1668 43c3:1668<br>0 → -oo → 0301:fd76 20e3:0008 20e3:d760 21e3:0008 21e3:d760 4383:0000 43c3:0000 |
| input@ADAT 5/6 | AN 5/6 | volume | `0401` `0501` `20e4` `21e5` `4384` `43c5` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 20e4:f870 20e4:f9c8 21e5:0648 21e5:f870 4384:1fb1 43c5:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 20e4:d760 21e5:d760 4384:0000 43c5:0000 |
| input@ADAT 5/6 | AN 7/8 | volume | `20e6` `21e7` `4386` `43c7` | 0.5 → -12.1 dB → 20e6:f870 20e6:f9c8 21e7:0648 21e7:f870 4386:1fb1 43c7:1fb1<br>0 → -oo → 20e6:d760 21e7:d760 4386:0000 43c7:0000 |
| input@ADAT 5/6 | SPDIF | volume | `0801` `0901` `20e8` `21e9` `4388` `43c9` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 20e8:f870 20e8:f9c8 21e9:0648 21e9:f870 4388:1fb1 43c9:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 20e8:d760 21e9:d760 4388:0000 43c9:0000 |
| input@ADAT 5/6 | AS 1/2 | volume | `0a01` `0b01` `20ea` `21eb` `438a` `43cb` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 20ea:f870 20ea:f9c8 21eb:0648 21eb:f870 438a:1fb1 43cb:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 20ea:d760 21eb:d760 438a:0000 43cb:0000 |
| input@ADAT 5/6 | ADAT 3/4 | volume | `0c01` `0d01` `20ec` `21ed` `438c` `43cd` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 20ec:f870 20ec:f9c8 21ed:0648 21ed:f870 438c:1fb1 43cd:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 20ec:d760 21ed:d760 438c:0000 43cd:0000 |
| input@ADAT 5/6 | ADAT 5/6 | volume | `0e01` `0f01` `20ee` `21ef` `438e` `43cf` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 20ee:f870 20ee:f9c8 21ef:0648 21ef:f870 438e:1fb1 43cf:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 20ee:d760 21ef:d760 438e:0000 43cf:0000 |
| input@ADAT 5/6 | ADAT 7/8 | volume | `1001` `1101` `20f0` `21f1` `4390` `43d1` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 20f0:f870 20f0:f9c8 21f1:0648 21f1:f870 4390:1fb1 43d1:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 20f0:d760 21f1:d760 4390:0000 43d1:0000 |
| playback@ADAT 5/6 | AN 1/2 | volume | `43a0` `43e1` | 0.5 → -12.1 dB → 43a0:1fb1 43e1:1fb1<br>0 → -oo → 43a0:0000 43e1:0000 |
| playback@ADAT 5/6 | AN 3/4 | volume | `43a2` `43e3` | 0.5 → -12.1 dB → 43a2:1fb1 43e3:1fb1<br>0 → -oo → 43a2:0000 43e3:0000 |
| playback@ADAT 5/6 | AN 5/6 | volume | `43a4` `43e5` | 0.5 → -12.1 dB → 43a4:1fb1 43e5:1fb1<br>0 → -oo → 43a4:0000 43e5:0000 |
| playback@ADAT 5/6 | PH 7/8 | volume | `43a6` `43e7` | 0.5 → -12.1 dB → 43a6:1fb1 43e7:1fb1<br>0 → -oo → 43a6:0000 43e7:0000 |
| playback@ADAT 5/6 | SPDIF | volume | `43a8` `43e9` | 0.5 → -12.1 dB → 43a8:1fb1 43e9:1fb1<br>0 → -oo → 43a8:0000 43e9:0000 |
| playback@ADAT 5/6 | AS 1/2 | volume | `43aa` `43eb` | 0.5 → -12.1 dB → 43aa:1fb1 43eb:1fb1<br>0 → -oo → 43aa:0000 43eb:0000 |
| playback@ADAT 5/6 | ADAT 3/4 | volume | `43ac` `43ed` | 0.5 → -12.1 dB → 43ac:1fb1 43ed:1fb1<br>0 → -oo → 43ac:0000 43ed:0000 |
| playback@ADAT 5/6 | ADAT 5/6 | volume | `43ae` `43ef` | 0.5 → -12.1 dB → 43ae:1fb1 43ef:1fb1<br>0.8172 → 0.0 dB → 43ae:9000 43ef:9000 |
| playback@ADAT 5/6 | ADAT 7/8 | volume | `43b0` `43f1` | 0.5 → -12.1 dB → 43b0:1fb1 43f1:1fb1<br>0 → -oo → 43b0:0000 43f1:0000 |
| input@ADAT 7/8 | Mic 1 | volume | `0001` `22e0` `23e0` `4400` `4440` | 0.5 → -12.1 dB → 0001:ff87 22e0:0008 22e0:f870 23e0:0008 23e0:f870 4400:1668 4440:1668<br>0 → -oo → 0001:fd76 22e0:0008 22e0:d760 23e0:0008 23e0:d760 4400:0000 4440:0000 |
| input@ADAT 7/8 | Mic 2 | volume | `0101` `22e1` `23e1` `4401` `4441` | 0.5 → -12.1 dB → 0101:ff87 22e1:0008 22e1:f870 23e1:0008 23e1:f870 4401:1668 4441:1668<br>0 → -oo → 0101:fd76 22e1:0008 22e1:d760 23e1:0008 23e1:d760 4401:0000 4441:0000 |
| input@ADAT 7/8 | AN 3 | volume | `0201` `22e2` `23e2` `4402` `4442` | 0.5 → -12.1 dB → 0201:ff87 22e2:0008 22e2:f870 23e2:0008 23e2:f870 4402:1668 4442:1668<br>0 → -oo → 0201:fd76 22e2:0008 22e2:d760 23e2:0008 23e2:d760 4402:0000 4442:0000 |
| input@ADAT 7/8 | AN 4 | volume | `0301` `22e3` `23e3` `4403` `4443` | 0.5 → -12.1 dB → 0301:ff87 22e3:0008 22e3:f870 23e3:0008 23e3:f870 4403:1668 4443:1668<br>0 → -oo → 0301:fd76 22e3:0008 22e3:d760 23e3:0008 23e3:d760 4403:0000 4443:0000 |
| input@ADAT 7/8 | AN 5/6 | volume | `0401` `0501` `22e4` `23e5` `4404` `4445` | 0.5 → -12.1 dB → 0401:ff87 0501:ff87 22e4:f870 22e4:f9c8 23e5:0648 23e5:f870 4404:1fb1 4445:1fb1<br>0 → -oo → 0401:fd76 0501:fd76 22e4:d760 23e5:d760 4404:0000 4445:0000 |
| input@ADAT 7/8 | AN 7/8 | volume | `22e6` `23e7` `4406` `4447` | 0.5 → -12.1 dB → 22e6:f870 22e6:f9c8 23e7:0648 23e7:f870 4406:1fb1 4447:1fb1<br>0 → -oo → 22e6:d760 23e7:d760 4406:0000 4447:0000 |
| input@ADAT 7/8 | SPDIF | volume | `0801` `0901` `22e8` `23e9` `4408` `4449` | 0.5 → -12.1 dB → 0801:ff87 0901:ff87 22e8:f870 22e8:f9c8 23e9:0648 23e9:f870 4408:1fb1 4449:1fb1<br>0 → -oo → 0801:fd76 0901:fd76 22e8:d760 23e9:d760 4408:0000 4449:0000 |
| input@ADAT 7/8 | AS 1/2 | volume | `0a01` `0b01` `22ea` `23eb` `440a` `444b` | 0.5 → -12.1 dB → 0a01:ff87 0b01:ff87 22ea:f870 22ea:f9c8 23eb:0648 23eb:f870 440a:1fb1 444b:1fb1<br>0 → -oo → 0a01:fd76 0b01:fd76 22ea:d760 23eb:d760 440a:0000 444b:0000 |
| input@ADAT 7/8 | ADAT 3/4 | volume | `0c01` `0d01` `22ec` `23ed` `440c` `444d` | 0.5 → -12.1 dB → 0c01:ff87 0d01:ff87 22ec:f870 22ec:f9c8 23ed:0648 23ed:f870 440c:1fb1 444d:1fb1<br>0 → -oo → 0c01:fd76 0d01:fd76 22ec:d760 23ed:d760 440c:0000 444d:0000 |
| input@ADAT 7/8 | ADAT 5/6 | volume | `0e01` `0f01` `22ee` `23ef` `440e` `444f` | 0.5 → -12.1 dB → 0e01:ff87 0f01:ff87 22ee:f870 22ee:f9c8 23ef:0648 23ef:f870 440e:1fb1 444f:1fb1<br>0 → -oo → 0e01:fd76 0f01:fd76 22ee:d760 23ef:d760 440e:0000 444f:0000 |
| input@ADAT 7/8 | ADAT 7/8 | volume | `1001` `1101` `22f0` `23f1` `4410` `4451` | 0.5 → -12.1 dB → 1001:ff87 1101:ff87 22f0:f870 22f0:f9c8 23f1:0648 23f1:f870 4410:1fb1 4451:1fb1<br>0 → -oo → 1001:fd76 1101:fd76 22f0:d760 23f1:d760 4410:0000 4451:0000 |
| playback@ADAT 7/8 | AN 1/2 | volume | `4420` `4461` | 0.5 → -12.1 dB → 4420:1fb1 4461:1fb1<br>0 → -oo → 4420:0000 4461:0000 |
| playback@ADAT 7/8 | AN 3/4 | volume | `4422` `4463` | 0.5 → -12.1 dB → 4422:1fb1 4463:1fb1<br>0 → -oo → 4422:0000 4463:0000 |
| playback@ADAT 7/8 | AN 5/6 | volume | `4424` `4465` | 0.5 → -12.1 dB → 4424:1fb1 4465:1fb1<br>0 → -oo → 4424:0000 4465:0000 |
| playback@ADAT 7/8 | PH 7/8 | volume | `4426` `4467` | 0.5 → -12.1 dB → 4426:1fb1 4467:1fb1<br>0 → -oo → 4426:0000 4467:0000 |
| playback@ADAT 7/8 | SPDIF | volume | `4428` `4469` | 0.5 → -12.1 dB → 4428:1fb1 4469:1fb1<br>0 → -oo → 4428:0000 4469:0000 |
| playback@ADAT 7/8 | AS 1/2 | volume | `442a` `446b` | 0.5 → -12.1 dB → 442a:1fb1 446b:1fb1<br>0 → -oo → 442a:0000 446b:0000 |
| playback@ADAT 7/8 | ADAT 3/4 | volume | `442c` `446d` | 0.5 → -12.1 dB → 442c:1fb1 446d:1fb1<br>0 → -oo → 442c:0000 446d:0000 |
| playback@ADAT 7/8 | ADAT 5/6 | volume | `442e` `446f` | 0.5 → -12.1 dB → 442e:1fb1 446f:1fb1<br>0 → -oo → 442e:0000 446f:0000 |
| playback@ADAT 7/8 | ADAT 7/8 | volume | `4430` `4471` | 0.5 → -12.1 dB → 4430:1fb1 4471:1fb1<br>0.8172 → 0.0 dB → 4430:9000 4471:9000 |

## Every change in order

| Row | Channel | Parameter | Kind | OSC value | Display | Writes |
|---|---|---|---|---|---|---|
| input@AN 1/2 | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 12e0:0008 12e0:f870 13e0:0008 13e0:f870 4000:1668 4040:1668 |
| input@AN 1/2 | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 12e0:0008 12e0:d760 13e0:0008 13e0:d760 4000:0000 4040:0000 |
| input@AN 1/2 | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 12e1:0008 12e1:f870 13e1:0008 13e1:f870 4001:1668 4041:1668 |
| input@AN 1/2 | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 12e1:0008 12e1:d760 13e1:0008 13e1:d760 4001:0000 4041:0000 |
| input@AN 1/2 | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 12e2:0008 12e2:f870 13e2:0008 13e2:f870 4002:1668 4042:1668 |
| input@AN 1/2 | AN 3 | volume | restore | 0 | -oo | 0201:fd76 12e2:0008 12e2:d760 13e2:0008 13e2:d760 4002:0000 4042:0000 |
| input@AN 1/2 | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 12e3:0008 12e3:f870 13e3:0008 13e3:f870 4003:1668 4043:1668 |
| input@AN 1/2 | AN 4 | volume | restore | 0 | -oo | 0301:fd76 12e3:0008 12e3:d760 13e3:0008 13e3:d760 4003:0000 4043:0000 |
| input@AN 1/2 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 12e4:f870 12e4:f9c8 13e5:0648 13e5:f870 4004:1fb1 4045:1fb1 |
| input@AN 1/2 | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 12e4:d760 13e5:d760 4004:0000 4045:0000 |
| input@AN 1/2 | AN 7/8 | volume | probe | 0.5 | -12.1 dB | req16 v=0x8000 i=0x8000 req29 v=0x0000 i=0x0000 req20 v=0x0000 i=0x000c 0601:fd76 0701:fd76 12e6:f870 12e6:f9c8 13e7:0648 13e7:f870 4006:1fb1 4047:1fb1 |
| input@AN 1/2 | AN 7/8 | volume | restore | 0.5264 | -10.6 dB | 0601:fd76 0701:fd76 12e6:f960 12e6:f9c8 13e7:0648 13e7:f960 4006:258f 4047:258f req19 v=0x0000 i=0x000c |
| input@AN 1/2 | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 12e8:f870 12e8:f9c8 13e9:0648 13e9:f870 4008:1fb1 4049:1fb1 |
| input@AN 1/2 | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 12e8:d760 13e9:d760 4008:0000 4049:0000 |
| input@AN 1/2 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 12ea:f870 12ea:f9c8 13eb:0648 13eb:f870 400a:1fb1 404b:1fb1 |
| input@AN 1/2 | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 12ea:d760 13eb:d760 400a:0000 404b:0000 |
| input@AN 1/2 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 12ec:f870 12ec:f9c8 13ed:0648 13ed:f870 400c:1fb1 404d:1fb1 |
| input@AN 1/2 | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 12ec:d760 13ed:d760 400c:0000 404d:0000 |
| input@AN 1/2 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 12ee:f870 12ee:f9c8 13ef:0648 13ef:f870 400e:1fb1 404f:1fb1 |
| input@AN 1/2 | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 12ee:d760 13ef:d760 400e:0000 404f:0000 |
| input@AN 1/2 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 12f0:f870 12f0:f9c8 13f1:0648 13f1:f870 4010:1fb1 4051:1fb1 |
| input@AN 1/2 | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 12f0:d760 13f1:d760 4010:0000 4051:0000 |
| playback@AN 1/2 | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 4020:1fb1 4061:1fb1 |
| playback@AN 1/2 | AN 1/2 | volume | restore | 0.8172 | 0.0 dB | 4020:9000 4061:9000 |
| playback@AN 1/2 | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 4022:1fb1 4063:1fb1 |
| playback@AN 1/2 | AN 3/4 | volume | restore | 0 | -oo | 4022:0000 4063:0000 |
| playback@AN 1/2 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 4024:1fb1 4065:1fb1 |
| playback@AN 1/2 | AN 5/6 | volume | restore | 0 | -oo | 4024:0000 4065:0000 |
| playback@AN 1/2 | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 4026:1fb1 4067:1fb1 |
| playback@AN 1/2 | PH 7/8 | volume | restore | 0 | -oo | 4026:0000 4067:0000 |
| playback@AN 1/2 | SPDIF | volume | probe | 0.5 | -12.1 dB | 4028:1fb1 4069:1fb1 |
| playback@AN 1/2 | SPDIF | volume | restore | 0 | -oo | 4028:0000 4069:0000 |
| playback@AN 1/2 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 402a:1fb1 406b:1fb1 |
| playback@AN 1/2 | AS 1/2 | volume | restore | 0 | -oo | 402a:0000 406b:0000 |
| playback@AN 1/2 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 402c:1fb1 406d:1fb1 |
| playback@AN 1/2 | ADAT 3/4 | volume | restore | 0 | -oo | 402c:0000 406d:0000 |
| playback@AN 1/2 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 402e:1fb1 406f:1fb1 |
| playback@AN 1/2 | ADAT 5/6 | volume | restore | 0 | -oo | 402e:0000 406f:0000 |
| playback@AN 1/2 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 4030:1fb1 4071:1fb1 |
| playback@AN 1/2 | ADAT 7/8 | volume | restore | 0 | -oo | 4030:0000 4071:0000 |
| input@AN 3/4 | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 14e0:0008 14e0:f870 15e0:0008 15e0:f870 4080:1668 40c0:1668 |
| input@AN 3/4 | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 14e0:0008 14e0:d760 15e0:0008 15e0:d760 4080:0000 40c0:0000 |
| input@AN 3/4 | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 14e1:0008 14e1:f870 15e1:0008 15e1:f870 4081:1668 40c1:1668 |
| input@AN 3/4 | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 14e1:0008 14e1:d760 15e1:0008 15e1:d760 4081:0000 40c1:0000 |
| input@AN 3/4 | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 14e2:0008 14e2:f870 15e2:0008 15e2:f870 4082:1668 40c2:1668 |
| input@AN 3/4 | AN 3 | volume | restore | 0 | -oo | 0201:fd76 14e2:0008 14e2:d760 15e2:0008 15e2:d760 4082:0000 40c2:0000 |
| input@AN 3/4 | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 14e3:0008 14e3:f870 15e3:0008 15e3:f870 4083:1668 40c3:1668 |
| input@AN 3/4 | AN 4 | volume | restore | 0 | -oo | 0301:fd76 14e3:0008 14e3:d760 15e3:0008 15e3:d760 4083:0000 40c3:0000 |
| input@AN 3/4 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 14e4:f870 14e4:f9c8 15e5:0648 15e5:f870 4084:1fb1 40c5:1fb1 |
| input@AN 3/4 | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 14e4:d760 15e5:d760 4084:0000 40c5:0000 |
| input@AN 3/4 | AN 7/8 | volume | probe | 0.5 | -12.1 dB | 14e6:f870 14e6:f9c8 15e7:0648 15e7:f870 4086:1fb1 40c7:1fb1 |
| input@AN 3/4 | AN 7/8 | volume | restore | 0 | -oo | 14e6:d760 15e7:d760 4086:0000 40c7:0000 |
| input@AN 3/4 | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 14e8:f870 14e8:f9c8 15e9:0648 15e9:f870 4088:1fb1 40c9:1fb1 |
| input@AN 3/4 | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 14e8:d760 15e9:d760 4088:0000 40c9:0000 |
| input@AN 3/4 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 14ea:f870 14ea:f9c8 15eb:0648 15eb:f870 408a:1fb1 40cb:1fb1 |
| input@AN 3/4 | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 14ea:d760 15eb:d760 408a:0000 40cb:0000 |
| input@AN 3/4 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 14ec:f870 14ec:f9c8 15ed:0648 15ed:f870 408c:1fb1 40cd:1fb1 |
| input@AN 3/4 | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 14ec:d760 15ed:d760 408c:0000 40cd:0000 |
| input@AN 3/4 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 14ee:f870 14ee:f9c8 15ef:0648 15ef:f870 408e:1fb1 40cf:1fb1 |
| input@AN 3/4 | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 14ee:d760 15ef:d760 408e:0000 40cf:0000 |
| input@AN 3/4 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 14f0:f870 14f0:f9c8 15f1:0648 15f1:f870 4090:1fb1 40d1:1fb1 |
| input@AN 3/4 | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 14f0:d760 15f1:d760 4090:0000 40d1:0000 |
| playback@AN 3/4 | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 40a0:1fb1 40e1:1fb1 |
| playback@AN 3/4 | AN 1/2 | volume | restore | 0 | -oo | 40a0:0000 40e1:0000 |
| playback@AN 3/4 | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 40a2:1fb1 40e3:1fb1 |
| playback@AN 3/4 | AN 3/4 | volume | restore | 0.8172 | 0.0 dB | 40a2:9000 40e3:9000 |
| playback@AN 3/4 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 40a4:1fb1 40e5:1fb1 |
| playback@AN 3/4 | AN 5/6 | volume | restore | 0 | -oo | 40a4:0000 40e5:0000 |
| playback@AN 3/4 | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 40a6:1fb1 40e7:1fb1 |
| playback@AN 3/4 | PH 7/8 | volume | restore | 0 | -oo | 40a6:0000 40e7:0000 |
| playback@AN 3/4 | SPDIF | volume | probe | 0.5 | -12.1 dB | 40a8:1fb1 40e9:1fb1 |
| playback@AN 3/4 | SPDIF | volume | restore | 0 | -oo | 40a8:0000 40e9:0000 |
| playback@AN 3/4 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 40aa:1fb1 40eb:1fb1 |
| playback@AN 3/4 | AS 1/2 | volume | restore | 0 | -oo | 40aa:0000 40eb:0000 |
| playback@AN 3/4 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 40ac:1fb1 40ed:1fb1 |
| playback@AN 3/4 | ADAT 3/4 | volume | restore | 0 | -oo | 40ac:0000 40ed:0000 |
| playback@AN 3/4 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 40ae:1fb1 40ef:1fb1 |
| playback@AN 3/4 | ADAT 5/6 | volume | restore | 0 | -oo | 40ae:0000 40ef:0000 |
| playback@AN 3/4 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 40b0:1fb1 40f1:1fb1 |
| playback@AN 3/4 | ADAT 7/8 | volume | restore | 0 | -oo | 40b0:0000 40f1:0000 |
| input@AN 5/6 | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 16e0:0008 16e0:f870 17e0:0008 17e0:f870 4100:1668 4140:1668 |
| input@AN 5/6 | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 16e0:0008 16e0:d760 17e0:0008 17e0:d760 4100:0000 4140:0000 |
| input@AN 5/6 | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 16e1:0008 16e1:f870 17e1:0008 17e1:f870 4101:1668 4141:1668 |
| input@AN 5/6 | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 16e1:0008 16e1:d760 17e1:0008 17e1:d760 4101:0000 4141:0000 |
| input@AN 5/6 | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 16e2:0008 16e2:f870 17e2:0008 17e2:f870 4102:1668 4142:1668 |
| input@AN 5/6 | AN 3 | volume | restore | 0 | -oo | 0201:fd76 16e2:0008 16e2:d760 17e2:0008 17e2:d760 4102:0000 4142:0000 |
| input@AN 5/6 | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 16e3:0008 16e3:f870 17e3:0008 17e3:f870 4103:1668 4143:1668 |
| input@AN 5/6 | AN 4 | volume | restore | 0 | -oo | 0301:fd76 16e3:0008 16e3:d760 17e3:0008 17e3:d760 4103:0000 4143:0000 |
| input@AN 5/6 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 16e4:f870 16e4:f9c8 17e5:0648 17e5:f870 4104:1fb1 4145:1fb1 |
| input@AN 5/6 | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 16e4:d760 17e5:d760 4104:0000 4145:0000 |
| input@AN 5/6 | AN 7/8 | volume | probe | 0.5 | -12.1 dB | 16e6:f870 16e6:f9c8 17e7:0648 17e7:f870 4106:1fb1 4147:1fb1 |
| input@AN 5/6 | AN 7/8 | volume | restore | 0 | -oo | 16e6:d760 17e7:d760 4106:0000 4147:0000 |
| input@AN 5/6 | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 16e8:f870 16e8:f9c8 17e9:0648 17e9:f870 4108:1fb1 4149:1fb1 |
| input@AN 5/6 | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 16e8:d760 17e9:d760 4108:0000 4149:0000 |
| input@AN 5/6 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 16ea:f870 16ea:f9c8 17eb:0648 17eb:f870 410a:1fb1 414b:1fb1 |
| input@AN 5/6 | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 16ea:d760 17eb:d760 410a:0000 414b:0000 |
| input@AN 5/6 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 16ec:f870 16ec:f9c8 17ed:0648 17ed:f870 410c:1fb1 414d:1fb1 |
| input@AN 5/6 | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 16ec:d760 17ed:d760 410c:0000 414d:0000 |
| input@AN 5/6 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 16ee:f870 16ee:f9c8 17ef:0648 17ef:f870 410e:1fb1 414f:1fb1 |
| input@AN 5/6 | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 16ee:d760 17ef:d760 410e:0000 414f:0000 |
| input@AN 5/6 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 16f0:f870 16f0:f9c8 17f1:0648 17f1:f870 4110:1fb1 4151:1fb1 |
| input@AN 5/6 | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 16f0:d760 17f1:d760 4110:0000 4151:0000 |
| playback@AN 5/6 | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 4120:1fb1 4161:1fb1 |
| playback@AN 5/6 | AN 1/2 | volume | restore | 0 | -oo | 4120:0000 4161:0000 |
| playback@AN 5/6 | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 4122:1fb1 4163:1fb1 |
| playback@AN 5/6 | AN 3/4 | volume | restore | 0 | -oo | 4122:0000 4163:0000 |
| playback@AN 5/6 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 4124:1fb1 4165:1fb1 |
| playback@AN 5/6 | AN 5/6 | volume | restore | 0.8172 | 0.0 dB | 4124:9000 4165:9000 |
| playback@AN 5/6 | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 4126:1fb1 4167:1fb1 |
| playback@AN 5/6 | PH 7/8 | volume | restore | 0 | -oo | 4126:0000 4167:0000 |
| playback@AN 5/6 | SPDIF | volume | probe | 0.5 | -12.1 dB | 4128:1fb1 4169:1fb1 |
| playback@AN 5/6 | SPDIF | volume | restore | 0 | -oo | 4128:0000 4169:0000 |
| playback@AN 5/6 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 412a:1fb1 416b:1fb1 |
| playback@AN 5/6 | AS 1/2 | volume | restore | 0 | -oo | 412a:0000 416b:0000 |
| playback@AN 5/6 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 412c:1fb1 416d:1fb1 |
| playback@AN 5/6 | ADAT 3/4 | volume | restore | 0 | -oo | 412c:0000 416d:0000 |
| playback@AN 5/6 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 412e:1fb1 416f:1fb1 |
| playback@AN 5/6 | ADAT 5/6 | volume | restore | 0 | -oo | 412e:0000 416f:0000 |
| playback@AN 5/6 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 4130:1fb1 4171:1fb1 |
| playback@AN 5/6 | ADAT 7/8 | volume | restore | 0 | -oo | 4130:0000 4171:0000 |
| input@Main | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 18e0:0008 18e0:f870 19e0:0008 19e0:f870 4180:1668 41c0:1668 |
| input@Main | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 18e0:0008 18e0:d760 19e0:0008 19e0:d760 4180:0000 41c0:0000 |
| input@Main | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 18e1:0008 18e1:f870 19e1:0008 19e1:f870 4181:1668 41c1:1668 |
| input@Main | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 18e1:0008 18e1:d760 19e1:0008 19e1:d760 4181:0000 41c1:0000 |
| input@Main | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 18e2:0008 18e2:f870 19e2:0008 19e2:f870 4182:1668 41c2:1668 |
| input@Main | AN 3 | volume | restore | 0 | -oo | 0201:fd76 18e2:0008 18e2:d760 19e2:0008 19e2:d760 4182:0000 41c2:0000 |
| input@Main | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 18e3:0008 18e3:f870 19e3:0008 19e3:f870 4183:1668 41c3:1668 |
| input@Main | AN 4 | volume | restore | 0 | -oo | 0301:fd76 18e3:0008 18e3:d760 19e3:0008 19e3:d760 4183:0000 41c3:0000 |
| input@Main | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 18e4:f870 18e4:f9c8 19e5:0648 19e5:f870 4184:1fb1 41c5:1fb1 |
| input@Main | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 18e4:d760 19e5:d760 4184:0000 41c5:0000 |
| input@Main | AN 7/8 | volume | probe | 0.5 | -12.1 dB | 18e6:f870 18e6:f9c8 19e7:0648 19e7:f870 4186:1fb1 41c7:1fb1 |
| input@Main | AN 7/8 | volume | restore | 0 | -oo | 18e6:d760 19e7:d760 4186:0000 41c7:0000 |
| input@Main | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 18e8:f870 18e8:f9c8 19e9:0648 19e9:f870 4188:1fb1 41c9:1fb1 |
| input@Main | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 18e8:d760 19e9:d760 4188:0000 41c9:0000 |
| input@Main | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 18ea:f870 18ea:f9c8 19eb:0648 19eb:f870 418a:1fb1 41cb:1fb1 |
| input@Main | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 18ea:d760 19eb:d760 418a:0000 41cb:0000 |
| input@Main | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 18ec:f870 18ec:f9c8 19ed:0648 19ed:f870 418c:1fb1 41cd:1fb1 |
| input@Main | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 18ec:d760 19ed:d760 418c:0000 41cd:0000 |
| input@Main | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 18ee:f870 18ee:f9c8 19ef:0648 19ef:f870 418e:1fb1 41cf:1fb1 |
| input@Main | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 18ee:d760 19ef:d760 418e:0000 41cf:0000 |
| input@Main | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 18f0:f870 18f0:f9c8 19f1:0648 19f1:f870 4190:1fb1 41d1:1fb1 |
| input@Main | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 18f0:d760 19f1:d760 4190:0000 41d1:0000 |
| playback@Main | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 41a0:1fb1 41e1:1fb1 |
| playback@Main | AN 1/2 | volume | restore | 0.8172 | 0.0 dB | 41a0:9000 41e1:9000 |
| playback@Main | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 41a2:1fb1 41e3:1fb1 |
| playback@Main | AN 3/4 | volume | restore | 0.8172 | 0.0 dB | 41a2:9000 41e3:9000 |
| playback@Main | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 41a4:1fb1 41e5:1fb1 |
| playback@Main | AN 5/6 | volume | restore | 0.8172 | 0.0 dB | 41a4:9000 41e5:9000 |
| playback@Main | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 41a6:1fb1 41e7:1fb1 |
| playback@Main | PH 7/8 | volume | restore | 0.8172 | 0.0 dB | 41a6:9000 41e7:9000 |
| playback@Main | SPDIF | volume | probe | 0.5 | -12.1 dB | 41a8:1fb1 41e9:1fb1 |
| playback@Main | SPDIF | volume | restore | 0.8172 | 0.0 dB | 41a8:9000 41e9:9000 |
| playback@Main | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 41aa:1fb1 41eb:1fb1 |
| playback@Main | AS 1/2 | volume | restore | 0.8172 | 0.0 dB | 41aa:9000 41eb:9000 |
| playback@Main | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 41ac:1fb1 41ed:1fb1 |
| playback@Main | ADAT 3/4 | volume | restore | 0.8172 | 0.0 dB | 41ac:9000 41ed:9000 |
| playback@Main | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 41ae:1fb1 41ef:1fb1 |
| playback@Main | ADAT 5/6 | volume | restore | 0.8172 | 0.0 dB | 41ae:9000 41ef:9000 |
| playback@Main | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 41b0:1fb1 41f1:1fb1 |
| playback@Main | ADAT 7/8 | volume | restore | 0.8172 | 0.0 dB | 41b0:9000 41f1:9000 |
| input@SPDIF | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 1ae0:0008 1ae0:f870 1be0:0008 1be0:f870 4200:1668 4240:1668 |
| input@SPDIF | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 1ae0:0008 1ae0:d760 1be0:0008 1be0:d760 4200:0000 4240:0000 |
| input@SPDIF | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 1ae1:0008 1ae1:f870 1be1:0008 1be1:f870 4201:1668 4241:1668 |
| input@SPDIF | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 1ae1:0008 1ae1:d760 1be1:0008 1be1:d760 4201:0000 4241:0000 |
| input@SPDIF | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 1ae2:0008 1ae2:f870 1be2:0008 1be2:f870 4202:1668 4242:1668 |
| input@SPDIF | AN 3 | volume | restore | 0 | -oo | 0201:fd76 1ae2:0008 1ae2:d760 1be2:0008 1be2:d760 4202:0000 4242:0000 |
| input@SPDIF | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 1ae3:0008 1ae3:f870 1be3:0008 1be3:f870 4203:1668 4243:1668 |
| input@SPDIF | AN 4 | volume | restore | 0 | -oo | 0301:fd76 1ae3:0008 1ae3:d760 1be3:0008 1be3:d760 4203:0000 4243:0000 |
| input@SPDIF | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 1ae4:f870 1ae4:f9c8 1be5:0648 1be5:f870 4204:1fb1 4245:1fb1 |
| input@SPDIF | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 1ae4:d760 1be5:d760 4204:0000 4245:0000 |
| input@SPDIF | AN 7/8 | volume | probe | 0.5 | -12.1 dB | 1ae6:f870 1ae6:f9c8 1be7:0648 1be7:f870 4206:1fb1 4247:1fb1 |
| input@SPDIF | AN 7/8 | volume | restore | 0 | -oo | 1ae6:d760 1be7:d760 4206:0000 4247:0000 |
| input@SPDIF | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 1ae8:f870 1ae8:f9c8 1be9:0648 1be9:f870 4208:1fb1 4249:1fb1 |
| input@SPDIF | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 1ae8:d760 1be9:d760 4208:0000 4249:0000 |
| input@SPDIF | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 1aea:f870 1aea:f9c8 1beb:0648 1beb:f870 420a:1fb1 424b:1fb1 |
| input@SPDIF | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 1aea:d760 1beb:d760 420a:0000 424b:0000 |
| input@SPDIF | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 1aec:f870 1aec:f9c8 1bed:0648 1bed:f870 420c:1fb1 424d:1fb1 |
| input@SPDIF | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 1aec:d760 1bed:d760 420c:0000 424d:0000 |
| input@SPDIF | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 1aee:f870 1aee:f9c8 1bef:0648 1bef:f870 420e:1fb1 424f:1fb1 |
| input@SPDIF | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 1aee:d760 1bef:d760 420e:0000 424f:0000 |
| input@SPDIF | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 1af0:f870 1af0:f9c8 1bf1:0648 1bf1:f870 4210:1fb1 4251:1fb1 |
| input@SPDIF | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 1af0:d760 1bf1:d760 4210:0000 4251:0000 |
| playback@SPDIF | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 4220:1fb1 4261:1fb1 |
| playback@SPDIF | AN 1/2 | volume | restore | 0 | -oo | 4220:0000 4261:0000 |
| playback@SPDIF | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 4222:1fb1 4263:1fb1 |
| playback@SPDIF | AN 3/4 | volume | restore | 0 | -oo | 4222:0000 4263:0000 |
| playback@SPDIF | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 4224:1fb1 4265:1fb1 |
| playback@SPDIF | AN 5/6 | volume | restore | 0 | -oo | 4224:0000 4265:0000 req16 v=0x8000 i=0x8000 req29 v=0x0000 i=0x0000 req20 v=0x0000 i=0x000c req19 v=0x0000 i=0x000c |
| playback@SPDIF | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 4226:1fb1 4267:1fb1 |
| playback@SPDIF | PH 7/8 | volume | restore | 0 | -oo | 4226:0000 4267:0000 |
| playback@SPDIF | SPDIF | volume | probe | 0.5 | -12.1 dB | 4228:1fb1 4269:1fb1 |
| playback@SPDIF | SPDIF | volume | restore | 0.8172 | 0.0 dB | 4228:9000 4269:9000 |
| playback@SPDIF | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 422a:1fb1 426b:1fb1 |
| playback@SPDIF | AS 1/2 | volume | restore | 0 | -oo | 422a:0000 426b:0000 |
| playback@SPDIF | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 422c:1fb1 426d:1fb1 |
| playback@SPDIF | ADAT 3/4 | volume | restore | 0 | -oo | 422c:0000 426d:0000 |
| playback@SPDIF | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 422e:1fb1 426f:1fb1 |
| playback@SPDIF | ADAT 5/6 | volume | restore | 0 | -oo | 422e:0000 426f:0000 |
| playback@SPDIF | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 4230:1fb1 4271:1fb1 |
| playback@SPDIF | ADAT 7/8 | volume | restore | 0 | -oo | 4230:0000 4271:0000 |
| input@AS 1/2 | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 1ce0:0008 1ce0:f870 1de0:0008 1de0:f870 4280:1668 42c0:1668 |
| input@AS 1/2 | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 1ce0:0008 1ce0:d760 1de0:0008 1de0:d760 4280:0000 42c0:0000 |
| input@AS 1/2 | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 1ce1:0008 1ce1:f870 1de1:0008 1de1:f870 4281:1668 42c1:1668 |
| input@AS 1/2 | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 1ce1:0008 1ce1:d760 1de1:0008 1de1:d760 4281:0000 42c1:0000 |
| input@AS 1/2 | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 1ce2:0008 1ce2:f870 1de2:0008 1de2:f870 4282:1668 42c2:1668 |
| input@AS 1/2 | AN 3 | volume | restore | 0 | -oo | 0201:fd76 1ce2:0008 1ce2:d760 1de2:0008 1de2:d760 4282:0000 42c2:0000 |
| input@AS 1/2 | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 1ce3:0008 1ce3:f870 1de3:0008 1de3:f870 4283:1668 42c3:1668 |
| input@AS 1/2 | AN 4 | volume | restore | 0 | -oo | 0301:fd76 1ce3:0008 1ce3:d760 1de3:0008 1de3:d760 4283:0000 42c3:0000 |
| input@AS 1/2 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 1ce4:f870 1ce4:f9c8 1de5:0648 1de5:f870 4284:1fb1 42c5:1fb1 |
| input@AS 1/2 | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 1ce4:d760 1de5:d760 4284:0000 42c5:0000 |
| input@AS 1/2 | AN 7/8 | volume | probe | 0.5 | -12.1 dB | 1ce6:f870 1ce6:f9c8 1de7:0648 1de7:f870 4286:1fb1 42c7:1fb1 |
| input@AS 1/2 | AN 7/8 | volume | restore | 0 | -oo | 1ce6:d760 1de7:d760 4286:0000 42c7:0000 |
| input@AS 1/2 | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 1ce8:f870 1ce8:f9c8 1de9:0648 1de9:f870 4288:1fb1 42c9:1fb1 |
| input@AS 1/2 | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 1ce8:d760 1de9:d760 4288:0000 42c9:0000 |
| input@AS 1/2 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 1cea:f870 1cea:f9c8 1deb:0648 1deb:f870 428a:1fb1 42cb:1fb1 |
| input@AS 1/2 | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 1cea:d760 1deb:d760 428a:0000 42cb:0000 |
| input@AS 1/2 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 1cec:f870 1cec:f9c8 1ded:0648 1ded:f870 428c:1fb1 42cd:1fb1 |
| input@AS 1/2 | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 1cec:d760 1ded:d760 428c:0000 42cd:0000 |
| input@AS 1/2 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 1cee:f870 1cee:f9c8 1def:0648 1def:f870 428e:1fb1 42cf:1fb1 |
| input@AS 1/2 | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 1cee:d760 1def:d760 428e:0000 42cf:0000 |
| input@AS 1/2 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 1cf0:f870 1cf0:f9c8 1df1:0648 1df1:f870 4290:1fb1 42d1:1fb1 |
| input@AS 1/2 | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 1cf0:d760 1df1:d760 4290:0000 42d1:0000 |
| playback@AS 1/2 | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 42a0:1fb1 42e1:1fb1 |
| playback@AS 1/2 | AN 1/2 | volume | restore | 0 | -oo | 42a0:0000 42e1:0000 |
| playback@AS 1/2 | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 42a2:1fb1 42e3:1fb1 |
| playback@AS 1/2 | AN 3/4 | volume | restore | 0 | -oo | 42a2:0000 42e3:0000 |
| playback@AS 1/2 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 42a4:1fb1 42e5:1fb1 |
| playback@AS 1/2 | AN 5/6 | volume | restore | 0 | -oo | 42a4:0000 42e5:0000 |
| playback@AS 1/2 | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 42a6:1fb1 42e7:1fb1 |
| playback@AS 1/2 | PH 7/8 | volume | restore | 0 | -oo | 42a6:0000 42e7:0000 |
| playback@AS 1/2 | SPDIF | volume | probe | 0.5 | -12.1 dB | 42a8:1fb1 42e9:1fb1 |
| playback@AS 1/2 | SPDIF | volume | restore | 0 | -oo | 42a8:0000 42e9:0000 |
| playback@AS 1/2 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 42aa:1fb1 42eb:1fb1 |
| playback@AS 1/2 | AS 1/2 | volume | restore | 0.8172 | 0.0 dB | 42aa:9000 42eb:9000 |
| playback@AS 1/2 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 42ac:1fb1 42ed:1fb1 |
| playback@AS 1/2 | ADAT 3/4 | volume | restore | 0 | -oo | 42ac:0000 42ed:0000 |
| playback@AS 1/2 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 42ae:1fb1 42ef:1fb1 |
| playback@AS 1/2 | ADAT 5/6 | volume | restore | 0 | -oo | 42ae:0000 42ef:0000 |
| playback@AS 1/2 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 42b0:1fb1 42f1:1fb1 |
| playback@AS 1/2 | ADAT 7/8 | volume | restore | 0 | -oo | 42b0:0000 42f1:0000 |
| input@ADAT 3/4 | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 1ee0:0008 1ee0:f870 1fe0:0008 1fe0:f870 4300:1668 4340:1668 |
| input@ADAT 3/4 | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 1ee0:0008 1ee0:d760 1fe0:0008 1fe0:d760 4300:0000 4340:0000 |
| input@ADAT 3/4 | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 1ee1:0008 1ee1:f870 1fe1:0008 1fe1:f870 4301:1668 4341:1668 |
| input@ADAT 3/4 | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 1ee1:0008 1ee1:d760 1fe1:0008 1fe1:d760 4301:0000 4341:0000 |
| input@ADAT 3/4 | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 1ee2:0008 1ee2:f870 1fe2:0008 1fe2:f870 4302:1668 4342:1668 |
| input@ADAT 3/4 | AN 3 | volume | restore | 0 | -oo | 0201:fd76 1ee2:0008 1ee2:d760 1fe2:0008 1fe2:d760 4302:0000 4342:0000 |
| input@ADAT 3/4 | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 1ee3:0008 1ee3:f870 1fe3:0008 1fe3:f870 4303:1668 4343:1668 |
| input@ADAT 3/4 | AN 4 | volume | restore | 0 | -oo | 0301:fd76 1ee3:0008 1ee3:d760 1fe3:0008 1fe3:d760 4303:0000 4343:0000 |
| input@ADAT 3/4 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 1ee4:f870 1ee4:f9c8 1fe5:0648 1fe5:f870 4304:1fb1 4345:1fb1 |
| input@ADAT 3/4 | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 1ee4:d760 1fe5:d760 4304:0000 4345:0000 |
| input@ADAT 3/4 | AN 7/8 | volume | probe | 0.5 | -12.1 dB | 1ee6:f870 1ee6:f9c8 1fe7:0648 1fe7:f870 4306:1fb1 4347:1fb1 |
| input@ADAT 3/4 | AN 7/8 | volume | restore | 0 | -oo | 1ee6:d760 1fe7:d760 4306:0000 4347:0000 |
| input@ADAT 3/4 | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 1ee8:f870 1ee8:f9c8 1fe9:0648 1fe9:f870 4308:1fb1 4349:1fb1 |
| input@ADAT 3/4 | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 1ee8:d760 1fe9:d760 4308:0000 4349:0000 |
| input@ADAT 3/4 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 1eea:f870 1eea:f9c8 1feb:0648 1feb:f870 430a:1fb1 434b:1fb1 |
| input@ADAT 3/4 | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 1eea:d760 1feb:d760 430a:0000 434b:0000 |
| input@ADAT 3/4 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 1eec:f870 1eec:f9c8 1fed:0648 1fed:f870 430c:1fb1 434d:1fb1 |
| input@ADAT 3/4 | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 1eec:d760 1fed:d760 430c:0000 434d:0000 |
| input@ADAT 3/4 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 1eee:f870 1eee:f9c8 1fef:0648 1fef:f870 430e:1fb1 434f:1fb1 |
| input@ADAT 3/4 | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 1eee:d760 1fef:d760 430e:0000 434f:0000 |
| input@ADAT 3/4 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 1ef0:f870 1ef0:f9c8 1ff1:0648 1ff1:f870 4310:1fb1 4351:1fb1 |
| input@ADAT 3/4 | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 1ef0:d760 1ff1:d760 4310:0000 4351:0000 |
| playback@ADAT 3/4 | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 4320:1fb1 4361:1fb1 |
| playback@ADAT 3/4 | AN 1/2 | volume | restore | 0 | -oo | 4320:0000 4361:0000 |
| playback@ADAT 3/4 | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 4322:1fb1 4363:1fb1 |
| playback@ADAT 3/4 | AN 3/4 | volume | restore | 0 | -oo | 4322:0000 4363:0000 |
| playback@ADAT 3/4 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 4324:1fb1 4365:1fb1 |
| playback@ADAT 3/4 | AN 5/6 | volume | restore | 0 | -oo | 4324:0000 4365:0000 |
| playback@ADAT 3/4 | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 4326:1fb1 4367:1fb1 |
| playback@ADAT 3/4 | PH 7/8 | volume | restore | 0 | -oo | 4326:0000 4367:0000 |
| playback@ADAT 3/4 | SPDIF | volume | probe | 0.5 | -12.1 dB | 4328:1fb1 4369:1fb1 |
| playback@ADAT 3/4 | SPDIF | volume | restore | 0 | -oo | 4328:0000 4369:0000 |
| playback@ADAT 3/4 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 432a:1fb1 436b:1fb1 |
| playback@ADAT 3/4 | AS 1/2 | volume | restore | 0 | -oo | 432a:0000 436b:0000 |
| playback@ADAT 3/4 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 432c:1fb1 436d:1fb1 |
| playback@ADAT 3/4 | ADAT 3/4 | volume | restore | 0.8172 | 0.0 dB | 432c:9000 436d:9000 |
| playback@ADAT 3/4 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 432e:1fb1 436f:1fb1 |
| playback@ADAT 3/4 | ADAT 5/6 | volume | restore | 0 | -oo | 432e:0000 436f:0000 |
| playback@ADAT 3/4 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 4330:1fb1 4371:1fb1 |
| playback@ADAT 3/4 | ADAT 7/8 | volume | restore | 0 | -oo | 4330:0000 4371:0000 |
| input@ADAT 5/6 | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 20e0:0008 20e0:f870 21e0:0008 21e0:f870 4380:1668 43c0:1668 |
| input@ADAT 5/6 | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 20e0:0008 20e0:d760 21e0:0008 21e0:d760 4380:0000 43c0:0000 |
| input@ADAT 5/6 | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 20e1:0008 20e1:f870 21e1:0008 21e1:f870 4381:1668 43c1:1668 |
| input@ADAT 5/6 | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 20e1:0008 20e1:d760 21e1:0008 21e1:d760 4381:0000 43c1:0000 |
| input@ADAT 5/6 | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 20e2:0008 20e2:f870 21e2:0008 21e2:f870 4382:1668 43c2:1668 |
| input@ADAT 5/6 | AN 3 | volume | restore | 0 | -oo | 0201:fd76 20e2:0008 20e2:d760 21e2:0008 21e2:d760 4382:0000 43c2:0000 |
| input@ADAT 5/6 | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 20e3:0008 20e3:f870 21e3:0008 21e3:f870 4383:1668 43c3:1668 |
| input@ADAT 5/6 | AN 4 | volume | restore | 0 | -oo | 0301:fd76 20e3:0008 20e3:d760 21e3:0008 21e3:d760 4383:0000 43c3:0000 |
| input@ADAT 5/6 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 20e4:f870 20e4:f9c8 21e5:0648 21e5:f870 4384:1fb1 43c5:1fb1 |
| input@ADAT 5/6 | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 20e4:d760 21e5:d760 4384:0000 43c5:0000 |
| input@ADAT 5/6 | AN 7/8 | volume | probe | 0.5 | -12.1 dB | 20e6:f870 20e6:f9c8 21e7:0648 21e7:f870 4386:1fb1 43c7:1fb1 |
| input@ADAT 5/6 | AN 7/8 | volume | restore | 0 | -oo | 20e6:d760 21e7:d760 4386:0000 43c7:0000 |
| input@ADAT 5/6 | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 20e8:f870 20e8:f9c8 21e9:0648 21e9:f870 4388:1fb1 43c9:1fb1 |
| input@ADAT 5/6 | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 20e8:d760 21e9:d760 4388:0000 43c9:0000 |
| input@ADAT 5/6 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 20ea:f870 20ea:f9c8 21eb:0648 21eb:f870 438a:1fb1 43cb:1fb1 |
| input@ADAT 5/6 | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 20ea:d760 21eb:d760 438a:0000 43cb:0000 |
| input@ADAT 5/6 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 20ec:f870 20ec:f9c8 21ed:0648 21ed:f870 438c:1fb1 43cd:1fb1 |
| input@ADAT 5/6 | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 20ec:d760 21ed:d760 438c:0000 43cd:0000 |
| input@ADAT 5/6 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 20ee:f870 20ee:f9c8 21ef:0648 21ef:f870 438e:1fb1 43cf:1fb1 |
| input@ADAT 5/6 | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 20ee:d760 21ef:d760 438e:0000 43cf:0000 |
| input@ADAT 5/6 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 20f0:f870 20f0:f9c8 21f1:0648 21f1:f870 4390:1fb1 43d1:1fb1 |
| input@ADAT 5/6 | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 20f0:d760 21f1:d760 4390:0000 43d1:0000 |
| playback@ADAT 5/6 | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 43a0:1fb1 43e1:1fb1 |
| playback@ADAT 5/6 | AN 1/2 | volume | restore | 0 | -oo | 43a0:0000 43e1:0000 |
| playback@ADAT 5/6 | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 43a2:1fb1 43e3:1fb1 |
| playback@ADAT 5/6 | AN 3/4 | volume | restore | 0 | -oo | 43a2:0000 43e3:0000 |
| playback@ADAT 5/6 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 43a4:1fb1 43e5:1fb1 |
| playback@ADAT 5/6 | AN 5/6 | volume | restore | 0 | -oo | 43a4:0000 43e5:0000 |
| playback@ADAT 5/6 | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 43a6:1fb1 43e7:1fb1 |
| playback@ADAT 5/6 | PH 7/8 | volume | restore | 0 | -oo | 43a6:0000 43e7:0000 |
| playback@ADAT 5/6 | SPDIF | volume | probe | 0.5 | -12.1 dB | 43a8:1fb1 43e9:1fb1 |
| playback@ADAT 5/6 | SPDIF | volume | restore | 0 | -oo | 43a8:0000 43e9:0000 |
| playback@ADAT 5/6 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 43aa:1fb1 43eb:1fb1 |
| playback@ADAT 5/6 | AS 1/2 | volume | restore | 0 | -oo | 43aa:0000 43eb:0000 |
| playback@ADAT 5/6 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 43ac:1fb1 43ed:1fb1 |
| playback@ADAT 5/6 | ADAT 3/4 | volume | restore | 0 | -oo | 43ac:0000 43ed:0000 |
| playback@ADAT 5/6 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 43ae:1fb1 43ef:1fb1 |
| playback@ADAT 5/6 | ADAT 5/6 | volume | restore | 0.8172 | 0.0 dB | 43ae:9000 43ef:9000 |
| playback@ADAT 5/6 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 43b0:1fb1 43f1:1fb1 |
| playback@ADAT 5/6 | ADAT 7/8 | volume | restore | 0 | -oo | 43b0:0000 43f1:0000 |
| input@ADAT 7/8 | Mic 1 | volume | probe | 0.5 | -12.1 dB | 0001:ff87 22e0:0008 22e0:f870 23e0:0008 23e0:f870 4400:1668 4440:1668 |
| input@ADAT 7/8 | Mic 1 | volume | restore | 0 | -oo | 0001:fd76 22e0:0008 22e0:d760 23e0:0008 23e0:d760 4400:0000 4440:0000 |
| input@ADAT 7/8 | Mic 2 | volume | probe | 0.5 | -12.1 dB | 0101:ff87 22e1:0008 22e1:f870 23e1:0008 23e1:f870 4401:1668 4441:1668 |
| input@ADAT 7/8 | Mic 2 | volume | restore | 0 | -oo | 0101:fd76 22e1:0008 22e1:d760 23e1:0008 23e1:d760 4401:0000 4441:0000 |
| input@ADAT 7/8 | AN 3 | volume | probe | 0.5 | -12.1 dB | 0201:ff87 22e2:0008 22e2:f870 23e2:0008 23e2:f870 4402:1668 4442:1668 |
| input@ADAT 7/8 | AN 3 | volume | restore | 0 | -oo | 0201:fd76 22e2:0008 22e2:d760 23e2:0008 23e2:d760 4402:0000 4442:0000 |
| input@ADAT 7/8 | AN 4 | volume | probe | 0.5 | -12.1 dB | 0301:ff87 22e3:0008 22e3:f870 23e3:0008 23e3:f870 4403:1668 4443:1668 |
| input@ADAT 7/8 | AN 4 | volume | restore | 0 | -oo | 0301:fd76 22e3:0008 22e3:d760 23e3:0008 23e3:d760 4403:0000 4443:0000 |
| input@ADAT 7/8 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 0401:ff87 0501:ff87 22e4:f870 22e4:f9c8 23e5:0648 23e5:f870 4404:1fb1 4445:1fb1 |
| input@ADAT 7/8 | AN 5/6 | volume | restore | 0 | -oo | 0401:fd76 0501:fd76 22e4:d760 23e5:d760 4404:0000 4445:0000 |
| input@ADAT 7/8 | AN 7/8 | volume | probe | 0.5 | -12.1 dB | 22e6:f870 22e6:f9c8 23e7:0648 23e7:f870 4406:1fb1 4447:1fb1 |
| input@ADAT 7/8 | AN 7/8 | volume | restore | 0 | -oo | 22e6:d760 23e7:d760 4406:0000 4447:0000 |
| input@ADAT 7/8 | SPDIF | volume | probe | 0.5 | -12.1 dB | 0801:ff87 0901:ff87 22e8:f870 22e8:f9c8 23e9:0648 23e9:f870 4408:1fb1 4449:1fb1 |
| input@ADAT 7/8 | SPDIF | volume | restore | 0 | -oo | 0801:fd76 0901:fd76 22e8:d760 23e9:d760 4408:0000 4449:0000 |
| input@ADAT 7/8 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 0a01:ff87 0b01:ff87 22ea:f870 22ea:f9c8 23eb:0648 23eb:f870 440a:1fb1 444b:1fb1 |
| input@ADAT 7/8 | AS 1/2 | volume | restore | 0 | -oo | 0a01:fd76 0b01:fd76 22ea:d760 23eb:d760 440a:0000 444b:0000 |
| input@ADAT 7/8 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 0c01:ff87 0d01:ff87 22ec:f870 22ec:f9c8 23ed:0648 23ed:f870 440c:1fb1 444d:1fb1 |
| input@ADAT 7/8 | ADAT 3/4 | volume | restore | 0 | -oo | 0c01:fd76 0d01:fd76 22ec:d760 23ed:d760 440c:0000 444d:0000 |
| input@ADAT 7/8 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 0e01:ff87 0f01:ff87 22ee:f870 22ee:f9c8 23ef:0648 23ef:f870 440e:1fb1 444f:1fb1 |
| input@ADAT 7/8 | ADAT 5/6 | volume | restore | 0 | -oo | 0e01:fd76 0f01:fd76 22ee:d760 23ef:d760 440e:0000 444f:0000 |
| input@ADAT 7/8 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 1001:ff87 1101:ff87 22f0:f870 22f0:f9c8 23f1:0648 23f1:f870 4410:1fb1 4451:1fb1 |
| input@ADAT 7/8 | ADAT 7/8 | volume | restore | 0 | -oo | 1001:fd76 1101:fd76 22f0:d760 23f1:d760 4410:0000 4451:0000 |
| playback@ADAT 7/8 | AN 1/2 | volume | probe | 0.5 | -12.1 dB | 4420:1fb1 4461:1fb1 |
| playback@ADAT 7/8 | AN 1/2 | volume | restore | 0 | -oo | 4420:0000 4461:0000 |
| playback@ADAT 7/8 | AN 3/4 | volume | probe | 0.5 | -12.1 dB | 4422:1fb1 4463:1fb1 |
| playback@ADAT 7/8 | AN 3/4 | volume | restore | 0 | -oo | 4422:0000 4463:0000 |
| playback@ADAT 7/8 | AN 5/6 | volume | probe | 0.5 | -12.1 dB | 4424:1fb1 4465:1fb1 |
| playback@ADAT 7/8 | AN 5/6 | volume | restore | 0 | -oo | 4424:0000 4465:0000 |
| playback@ADAT 7/8 | PH 7/8 | volume | probe | 0.5 | -12.1 dB | 4426:1fb1 4467:1fb1 |
| playback@ADAT 7/8 | PH 7/8 | volume | restore | 0 | -oo | 4426:0000 4467:0000 |
| playback@ADAT 7/8 | SPDIF | volume | probe | 0.5 | -12.1 dB | 4428:1fb1 4469:1fb1 |
| playback@ADAT 7/8 | SPDIF | volume | restore | 0 | -oo | 4428:0000 4469:0000 |
| playback@ADAT 7/8 | AS 1/2 | volume | probe | 0.5 | -12.1 dB | 442a:1fb1 446b:1fb1 |
| playback@ADAT 7/8 | AS 1/2 | volume | restore | 0 | -oo | 442a:0000 446b:0000 |
| playback@ADAT 7/8 | ADAT 3/4 | volume | probe | 0.5 | -12.1 dB | 442c:1fb1 446d:1fb1 |
| playback@ADAT 7/8 | ADAT 3/4 | volume | restore | 0 | -oo | 442c:0000 446d:0000 |
| playback@ADAT 7/8 | ADAT 5/6 | volume | probe | 0.5 | -12.1 dB | 442e:1fb1 446f:1fb1 |
| playback@ADAT 7/8 | ADAT 5/6 | volume | restore | 0 | -oo | 442e:0000 446f:0000 |
| playback@ADAT 7/8 | ADAT 7/8 | volume | probe | 0.5 | -12.1 dB | 4430:1fb1 4471:1fb1 |
| playback@ADAT 7/8 | ADAT 7/8 | volume | restore | 0.8172 | 0.0 dB | 4430:9000 4471:9000 |
