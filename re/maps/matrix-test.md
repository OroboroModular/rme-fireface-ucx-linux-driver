# Channel settings → register writes

From `06-matrix-test.pcap` (40 changes). Words are `address:value` (hex, bits 30–16 : 15–0).

## Per parameter

| Row | Channel | Parameter | Addresses | OSC value → display → words |
|---|---|---|---|---|
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

## Every change in order

| Row | Channel | Parameter | Kind | OSC value | Display | Writes |
|---|---|---|---|---|---|---|
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
