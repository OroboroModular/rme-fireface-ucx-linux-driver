# Channel settings → register writes

From `08-settings-all.pcap` (1751 changes). Words are `address:value` (hex, bits 30–16 : 15–0).

## Per parameter

| Row | Channel | Parameter | Addresses | OSC value → display → words |
|---|---|---|---|---|
| input | Mic 1 | eqType1 | `0041` | 0 → Bell → 0041:0000<br>0.25 → Bell → (none)<br>0.5 → Shelf → 0041:0001<br>0.75 → ? → (none)<br>1 → ? → (none) |
| input | Mic 1 | eqGain1 | `0042` | 0 → -20.0 dB → 0042:ff38<br>0.25 → -10.0 dB → 0042:ff9c<br>0.75 → 10.0 dB → 0042:0064<br>1 → 20.0 dB → 0042:00c8<br>0.5 → 0.0 dB → 0042:0000 |
| input | Mic 1 | eqFreq1 | `0043` | 0 → 20 Hz → 0043:0014<br>0.25 → 112 Hz → 0043:0070<br>0.5 → 632 Hz → 0043:0278<br>0.75 → 3557 Hz → 0043:0de5<br>1 → 20000 Hz → 0043:4e20<br>0.2007 → 80 Hz → 0043:0050 |
| input | Mic 1 | eqQ1 | `0044` | 0 → 0.7 → 0044:0007<br>0.25 → 1.7 → 0044:0011<br>0.5 → 2.8 → 0044:001c<br>0.75 → 3.9 → 0044:0027<br>1 → 5.0 → 0044:0032<br>0.06977 → 1.0 → 0044:000a |
| input | Mic 1 | eqGain2 | `0045` | 0 → -20.0 dB → 0045:ff38<br>0.25 → -10.0 dB → 0045:ff9c<br>0.75 → 10.0 dB → 0045:0064<br>1 → 20.0 dB → 0045:00c8<br>0.5 → 0.0 dB → 0045:0000 |
| input | Mic 1 | eqFreq2 | `0046` | 0 → 20 Hz → 0046:0014<br>0.25 → 112 Hz → 0046:0070<br>0.5 → 632 Hz → 0046:0278<br>0.75 → 3557 Hz → 0046:0de5<br>1 → 20000 Hz → 0046:4e20<br>0.5663 → 1000 Hz → 0046:03e8 |
| input | Mic 1 | eqQ2 | `0047` | 0 → 0.7 → 0047:0007<br>0.25 → 1.7 → 0047:0011<br>0.5 → 2.8 → 0047:001c<br>0.75 → 3.9 → 0047:0027<br>1 → 5.0 → 0047:0032<br>0.06977 → 1.0 → 0047:000a |
| input | Mic 1 | eqType3 | `0048` | 0 → Bell → 0048:0000<br>0.25 → Shelf → 0048:0001<br>0.75 → Low Pass → 0048:0002<br>1 → Low Pass → (none)<br>0.5 → Shelf → 0048:0001 |
| input | Mic 1 | eqGain3 | `0049` | 0 → -20.0 dB → 0049:ff38<br>0.25 → -10.0 dB → 0049:ff9c<br>0.75 → 10.0 dB → 0049:0064<br>1 → 20.0 dB → 0049:00c8<br>0.5 → 0.0 dB → 0049:0000 |
| input | Mic 1 | eqFreq3 | `004a` | 0 → 20 Hz → 004a:0014<br>0.25 → 112 Hz → 004a:0070<br>0.5 → 632 Hz → 004a:0278<br>0.75 → 3557 Hz → 004a:0de5<br>1 → 20000 Hz → 004a:4e20<br>0.7993 → 5000 Hz → 004a:1388 |
| input | Mic 1 | eqQ3 | `004b` | 0 → 0.7 → 004b:0007<br>0.25 → 1.7 → 004b:0011<br>0.5 → 2.8 → 004b:001c<br>0.75 → 3.9 → 004b:0027<br>1 → 5.0 → 004b:0032<br>0.06977 → 1.0 → 004b:000a |
| input | Mic 1 | eqEnable | `0040` | 1 → ? → 0040:0001<br>1 → ? → 0040:0000 |
| input | Mic 1 | lowcutFreq | `0021` | 0.25 → 45 Hz → 0021:002d<br>0.5 → 100 Hz → 0021:0064<br>0.75 → 224 Hz → 0021:00e0<br>1 → 500 Hz → 0021:01f4<br>0 → 20 Hz → 0021:0014 |
| input | Mic 1 | lowcutGrade | `0022` | 0 → 6 dB/oct → 0022:0000<br>0.25 → 12 dB/oct → 0022:0001<br>0.5 → 18 dB/oct → 0022:0002<br>0.75 → 18 dB/oct → (none)<br>1 → 24 dB/oct → 0022:0003<br>0.3333 → 12 dB/oct → 0022:0001 |
| input | Mic 1 | lowcutEnable | `0020` | 1 → ? → 0020:0001<br>1 → ? → 0020:0000 |
| input | Mic 1 | compexpGain | `0061` | 0 → -30.0 dB → 0061:fed4<br>0.25 → -15.0 dB → 0061:ff6a<br>0.75 → 15.0 dB → 0061:0096<br>1 → 30.0 dB → 0061:012c<br>0.5 → 0.0 dB → 0061:0000 |
| input | Mic 1 | compexpAttack | `0062` | 0 → 0 ms → 0062:0000<br>0.25 → 50 ms → 0062:0032<br>0.5 → 100 ms → 0062:0064<br>0.75 → 150 ms → 0062:0096<br>1 → 200 ms → 0062:00c8<br>0.05 → 10 ms → 0062:000a |
| input | Mic 1 | compexpRelease | `0063` | 0 → 100 ms → 0063:0064<br>0.25 → 325 ms → 0063:0145<br>0.5 → 550 ms → 0063:0226<br>0.75 → 774 ms → 0063:0306<br>1 → 999 ms → 0063:03e7<br>0.2225 → 300 ms → 0063:012c |
| input | Mic 1 | compTrsh | `0064` | 0 → -60.0 dB → 0064:fda8<br>0.25 → -45.0 dB → 0064:fe3e<br>0.75 → -15.0 dB → 0064:ff6a<br>1 → 0.0 dB → 0064:0000<br>0.5 → -30.0 dB → 0064:fed4 |
| input | Mic 1 | compRatio | `0065` | 0.25 → 3.2 → 0065:0020<br>0.5 → 5.5 → 0065:0037<br>0.75 → 7.7 → 0065:004d<br>1 → 10.0 → 0065:0064<br>0 → 1.0 → 0065:000a |
| input | Mic 1 | expTrsh | `0066` | 0 → -99.0 dB → 0066:fc22<br>0.25 → -82.0 dB → 0066:fccc<br>0.5 → -64.5 dB → 0066:fd7b<br>0.75 → -47.5 dB → 0066:fe25<br>1 → -30.0 dB → 0066:fed4<br>0.5652 → -60.0 dB → 0066:fda8 |
| input | Mic 1 | expRatio | `0067` | 0.25 → 3.2 → 0067:0020<br>0.5 → 5.5 → 0067:0037<br>0.75 → 7.7 → 0067:004d<br>1 → 10.0 → 0067:0064<br>0 → 1.0 → 0067:000a |
| input | Mic 1 | compexpEnable | `0060` | 1 → ? → 0060:0001<br>1 → ? → 0060:0000 |
| input | Mic 1 | alevMaxgain | `0081` | 0 → 0.0 dB → 0081:0000<br>0.25 → 4.5 dB → 0081:002d<br>0.5 → 9.0 dB → 0081:005a<br>0.75 → 13.5 dB → 0081:0087<br>1 → 18.0 dB → 0081:00b4<br>0.3333 → 6.0 dB → 0081:003c |
| input | Mic 1 | alevHeadroom | `0082` | 0 → 3.0 dB → 0082:001e<br>0.25 → 5.0 dB → 0082:0032<br>0.5 → 7.5 dB → 0082:004b<br>0.75 → 9.5 dB → 0082:005f<br>1 → 12.0 dB → 0082:0078<br>0.3333 → 6.0 dB → 0082:003c |
| input | Mic 1 | alevRisetime | `0083` | 0 → 0.1 s → 0083:0001<br>0.25 → 2.5 s → 0083:0019<br>0.75 → 7.4 s → 0083:004a<br>1 → 9.9 s → 0083:0063<br>0.5 → 5.0 s → 0083:0032 |
| input | Mic 2 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | Mic 2 | eqGain1 | `0142` | 0.75 → 10.0 dB → 0142:0064<br>0.5 → 0.0 dB → 0142:0000 |
| input | Mic 2 | eqFreq1 | `0143` | 0.75 → 3557 Hz → 0143:0de5<br>0.2007 → 80 Hz → 0143:0050 |
| input | Mic 2 | eqQ1 | `0144` | 0.75 → 3.9 → 0144:0027<br>0.06977 → 1.0 → 0144:000a |
| input | Mic 2 | eqGain2 | `0145` | 0.75 → 10.0 dB → 0145:0064<br>0.5 → 0.0 dB → 0145:0000 |
| input | Mic 2 | eqFreq2 | `0146` | 0.75 → 3557 Hz → 0146:0de5<br>0.5663 → 1000 Hz → 0146:03e8 |
| input | Mic 2 | eqQ2 | `0147` | 0.75 → 3.9 → 0147:0027<br>0.06977 → 1.0 → 0147:000a |
| input | Mic 2 | eqType3 | `0148` | 0.75 → Low Pass → 0148:0002<br>0.5 → Shelf → 0148:0001 |
| input | Mic 2 | eqGain3 | `0149` | 0.75 → 10.0 dB → 0149:0064<br>0.5 → 0.0 dB → 0149:0000 |
| input | Mic 2 | eqFreq3 | `014a` | 0.75 → 3557 Hz → 014a:0de5<br>0.7993 → 5000 Hz → 014a:1388 |
| input | Mic 2 | eqQ3 | `014b` | 0.75 → 3.9 → 014b:0027<br>0.06977 → 1.0 → 014b:000a |
| input | Mic 2 | eqEnable | `0140` | 1 → ? → 0140:0001<br>1 → ? → 0140:0000 |
| input | Mic 2 | lowcutFreq | `0121` | 0.75 → 224 Hz → 0121:00e0<br>0 → 20 Hz → 0121:0014 |
| input | Mic 2 | lowcutGrade | `0122` | 0.75 → 18 dB/oct → 0122:0002<br>0.3333 → 12 dB/oct → 0122:0001 |
| input | Mic 2 | lowcutEnable | `0120` | 1 → ? → 0120:0001<br>1 → ? → 0120:0000 |
| input | Mic 2 | compexpGain | `0161` | 0.75 → 15.0 dB → 0161:0096<br>0.5 → 0.0 dB → 0161:0000 |
| input | Mic 2 | compexpAttack | `0162` | 0.75 → 150 ms → 0162:0096<br>0.05 → 10 ms → 0162:000a |
| input | Mic 2 | compexpRelease | `0163` | 0.75 → 774 ms → 0163:0306<br>0.2225 → 300 ms → 0163:012c |
| input | Mic 2 | compTrsh | `0164` | 0.75 → -15.0 dB → 0164:ff6a<br>0.5 → -30.0 dB → 0164:fed4 |
| input | Mic 2 | compRatio | `0165` | 0.75 → 7.7 → 0165:004d<br>0 → 1.0 → 0165:000a |
| input | Mic 2 | expTrsh | `0166` | 0.75 → -47.5 dB → 0166:fe25<br>0.5652 → -60.0 dB → 0166:fda8 |
| input | Mic 2 | expRatio | `0167` | 0.75 → 7.7 → 0167:004d<br>0 → 1.0 → 0167:000a |
| input | Mic 2 | compexpEnable | `0160` | 1 → ? → 0160:0001<br>1 → ? → 0160:0000 |
| input | Mic 2 | alevMaxgain | `0181` | 0.75 → 13.5 dB → 0181:0087<br>0.3333 → 6.0 dB → 0181:003c |
| input | Mic 2 | alevHeadroom | `0182` | 0.75 → 9.5 dB → 0182:005f<br>0.3333 → 6.0 dB → 0182:003c |
| input | Mic 2 | alevRisetime | `0183` | 0.75 → 7.4 s → 0183:004a<br>0.5 → 5.0 s → 0183:0032 |
| input | AN 3 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | AN 3 | eqGain1 | `0242` | 0.75 → 10.0 dB → 0242:0064<br>0.5 → 0.0 dB → 0242:0000 |
| input | AN 3 | eqFreq1 | `0243` | 0.75 → 3557 Hz → 0243:0de5<br>0.2007 → 80 Hz → 0243:0050 |
| input | AN 3 | eqQ1 | `0244` | 0.75 → 3.9 → 0244:0027<br>0.06977 → 1.0 → 0244:000a |
| input | AN 3 | eqGain2 | `0245` | 0.75 → 10.0 dB → 0245:0064<br>0.5 → 0.0 dB → 0245:0000 |
| input | AN 3 | eqFreq2 | `0246` | 0.75 → 3557 Hz → 0246:0de5<br>0.5663 → 1000 Hz → 0246:03e8 |
| input | AN 3 | eqQ2 | `0247` | 0.75 → 3.9 → 0247:0027<br>0.06977 → 1.0 → 0247:000a |
| input | AN 3 | eqType3 | `0248` | 0.75 → Low Pass → 0248:0002<br>0.5 → Shelf → 0248:0001 |
| input | AN 3 | eqGain3 | `0249` | 0.75 → 10.0 dB → 0249:0064<br>0.5 → 0.0 dB → 0249:0000 |
| input | AN 3 | eqFreq3 | `024a` | 0.75 → 3557 Hz → 024a:0de5<br>0.7993 → 5000 Hz → 024a:1388 |
| input | AN 3 | eqQ3 | `024b` | 0.75 → 3.9 → 024b:0027<br>0.06977 → 1.0 → 024b:000a |
| input | AN 3 | eqEnable | `0240` | 1 → ? → 0240:0001<br>1 → ? → 0240:0000 |
| input | AN 3 | lowcutFreq | `0221` | 0.75 → 224 Hz → 0221:00e0<br>0 → 20 Hz → 0221:0014 |
| input | AN 3 | lowcutGrade | `0222` | 0.75 → 18 dB/oct → 0222:0002<br>0.3333 → 12 dB/oct → 0222:0001 |
| input | AN 3 | lowcutEnable | `0220` | 1 → ? → 0220:0001<br>1 → ? → 0220:0000 |
| input | AN 3 | compexpGain | `0261` | 0.75 → 15.0 dB → 0261:0096<br>0.5 → 0.0 dB → 0261:0000 |
| input | AN 3 | compexpAttack | `0262` | 0.75 → 150 ms → 0262:0096<br>0.05 → 10 ms → 0262:000a |
| input | AN 3 | compexpRelease | `0263` | 0.75 → 774 ms → 0263:0306<br>0.2225 → 300 ms → 0263:012c |
| input | AN 3 | compTrsh | `0264` | 0.75 → -15.0 dB → 0264:ff6a<br>0.5 → -30.0 dB → 0264:fed4 |
| input | AN 3 | compRatio | `0265` | 0.75 → 7.7 → 0265:004d<br>0 → 1.0 → 0265:000a |
| input | AN 3 | expTrsh | `0266` | 0.75 → -47.5 dB → 0266:fe25<br>0.5652 → -60.0 dB → 0266:fda8 |
| input | AN 3 | expRatio | `0267` | 0.75 → 7.7 → 0267:004d<br>0 → 1.0 → 0267:000a |
| input | AN 3 | compexpEnable | `0260` | 1 → ? → 0260:0001<br>1 → ? → 0260:0000 |
| input | AN 3 | alevMaxgain | `0281` | 0.75 → 13.5 dB → 0281:0087<br>0.3333 → 6.0 dB → 0281:003c |
| input | AN 3 | alevHeadroom | `0282` | 0.75 → 9.5 dB → 0282:005f<br>0.3333 → 6.0 dB → 0282:003c |
| input | AN 3 | alevRisetime | `0283` | 0.75 → 7.4 s → 0283:004a<br>0.5 → 5.0 s → 0283:0032 |
| input | AN 4 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | AN 4 | eqGain1 | `0342` | 0.75 → 10.0 dB → 0342:0064<br>0.5 → 0.0 dB → 0342:0000 |
| input | AN 4 | eqFreq1 | `0343` | 0.75 → 3557 Hz → 0343:0de5<br>0.2007 → 80 Hz → 0343:0050 |
| input | AN 4 | eqQ1 | `0344` | 0.75 → 3.9 → 0344:0027<br>0.06977 → 1.0 → 0344:000a |
| input | AN 4 | eqGain2 | `0345` | 0.75 → 10.0 dB → 0345:0064<br>0.5 → 0.0 dB → 0345:0000 |
| input | AN 4 | eqFreq2 | `0346` | 0.75 → 3557 Hz → 0346:0de5<br>0.5663 → 1000 Hz → 0346:03e8 |
| input | AN 4 | eqQ2 | `0347` | 0.75 → 3.9 → 0347:0027<br>0.06977 → 1.0 → 0347:000a |
| input | AN 4 | eqType3 | `0348` | 0.75 → Low Pass → 0348:0002<br>0.5 → Shelf → 0348:0001 |
| input | AN 4 | eqGain3 | `0349` | 0.75 → 10.0 dB → 0349:0064<br>0.5 → 0.0 dB → 0349:0000 |
| input | AN 4 | eqFreq3 | `034a` | 0.75 → 3557 Hz → 034a:0de5<br>0.7993 → 5000 Hz → 034a:1388 |
| input | AN 4 | eqQ3 | `034b` | 0.75 → 3.9 → 034b:0027<br>0.06977 → 1.0 → 034b:000a |
| input | AN 4 | eqEnable | `0340` | 1 → ? → 0340:0001<br>1 → ? → 0340:0000 |
| input | AN 4 | lowcutFreq | `0321` | 0.75 → 224 Hz → 0321:00e0<br>0 → 20 Hz → 0321:0014 |
| input | AN 4 | lowcutGrade | `0322` | 0.75 → 18 dB/oct → 0322:0002<br>0.3333 → 12 dB/oct → 0322:0001 |
| input | AN 4 | lowcutEnable | `0320` | 1 → ? → 0320:0001<br>1 → ? → 0320:0000 |
| input | AN 4 | compexpGain | `0361` | 0.75 → 15.0 dB → 0361:0096<br>0.5 → 0.0 dB → 0361:0000 |
| input | AN 4 | compexpAttack | `0362` | 0.75 → 150 ms → 0362:0096<br>0.05 → 10 ms → 0362:000a |
| input | AN 4 | compexpRelease | `0363` | 0.75 → 774 ms → 0363:0306<br>0.2225 → 300 ms → 0363:012c |
| input | AN 4 | compTrsh | `0364` | 0.75 → -15.0 dB → 0364:ff6a<br>0.5 → -30.0 dB → 0364:fed4 |
| input | AN 4 | compRatio | `0365` | 0.75 → 7.7 → 0365:004d<br>0 → 1.0 → 0365:000a |
| input | AN 4 | expTrsh | `0366` | 0.75 → -47.5 dB → 0366:fe25<br>0.5652 → -60.0 dB → 0366:fda8 |
| input | AN 4 | expRatio | `0367` | 0.75 → 7.7 → 0367:004d<br>0 → 1.0 → 0367:000a |
| input | AN 4 | compexpEnable | `0360` | 1 → ? → 0360:0001<br>1 → ? → 0360:0000 |
| input | AN 4 | alevMaxgain | `0381` | 0.75 → 13.5 dB → 0381:0087<br>0.3333 → 6.0 dB → 0381:003c |
| input | AN 4 | alevHeadroom | `0382` | 0.75 → 9.5 dB → 0382:005f<br>0.3333 → 6.0 dB → 0382:003c |
| input | AN 4 | alevRisetime | `0383` | 0.75 → 7.4 s → 0383:004a<br>0.5 → 5.0 s → 0383:0032 |
| input | AN 5/6 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | AN 5/6 | eqGain1 | `0442` `0542` | 0.75 → 10.0 dB → 0442:0064 0542:0064<br>0.5 → 0.0 dB → 0442:0000 0542:0000 |
| input | AN 5/6 | eqFreq1 | `0443` `0543` | 0.75 → 3557 Hz → 0443:0de5 0543:0de5<br>0.2007 → 80 Hz → 0443:0050 0543:0050 |
| input | AN 5/6 | eqQ1 | `0444` `0544` | 0.75 → 3.9 → 0444:0027 0544:0027<br>0.06977 → 1.0 → 0444:000a 0544:000a |
| input | AN 5/6 | eqGain2 | `0445` `0545` | 0.75 → 10.0 dB → 0445:0064 0545:0064<br>0.5 → 0.0 dB → 0445:0000 0545:0000 |
| input | AN 5/6 | eqFreq2 | `0446` `0546` | 0.75 → 3557 Hz → 0446:0de5 0546:0de5<br>0.5663 → 1000 Hz → 0446:03e8 0546:03e8 |
| input | AN 5/6 | eqQ2 | `0447` `0547` | 0.75 → 3.9 → 0447:0027 0547:0027<br>0.06977 → 1.0 → 0447:000a 0547:000a |
| input | AN 5/6 | eqType3 | `0448` `0548` | 0.75 → Low Pass → 0448:0002 0548:0002<br>0.5 → Shelf → 0448:0001 0548:0001 |
| input | AN 5/6 | eqGain3 | `0449` `0549` | 0.75 → 10.0 dB → 0449:0064 0549:0064<br>0.5 → 0.0 dB → 0449:0000 0549:0000 |
| input | AN 5/6 | eqFreq3 | `044a` `054a` | 0.75 → 3557 Hz → 044a:0de5 054a:0de5<br>0.7993 → 5000 Hz → 044a:1388 054a:1388 |
| input | AN 5/6 | eqQ3 | `044b` `054b` | 0.75 → 3.9 → 044b:0027 054b:0027<br>0.06977 → 1.0 → 044b:000a 054b:000a |
| input | AN 5/6 | eqEnable | `0440` `0540` | 1 → ? → 0440:0001 0540:0001<br>1 → ? → 0440:0000 0540:0000 |
| input | AN 5/6 | lowcutFreq | `0421` `0521` | 0.75 → 224 Hz → 0421:00e0 0521:00e0<br>0 → 20 Hz → 0421:0014 0521:0014 |
| input | AN 5/6 | lowcutGrade | `0422` `0522` | 0.75 → 18 dB/oct → 0422:0002 0522:0002<br>0.3333 → 12 dB/oct → 0422:0001 0522:0001 |
| input | AN 5/6 | lowcutEnable | `0420` `0520` | 1 → ? → 0420:0001 0520:0001<br>1 → ? → 0420:0000 0520:0000 |
| input | AN 5/6 | compexpGain | `0461` `0561` | 0.75 → 15.0 dB → 0461:0096 0561:0096<br>0.5 → 0.0 dB → 0461:0000 0561:0000 |
| input | AN 5/6 | compexpAttack | `0462` `0562` | 0.75 → 150 ms → 0462:0096 0562:0096<br>0.05 → 10 ms → 0462:000a 0562:000a |
| input | AN 5/6 | compexpRelease | `0463` `0563` | 0.75 → 774 ms → 0463:0306 0563:0306<br>0.2225 → 300 ms → 0463:012c 0563:012c |
| input | AN 5/6 | compTrsh | `0464` `0564` | 0.75 → -15.0 dB → 0464:ff6a 0564:ff6a<br>0.5 → -30.0 dB → 0464:fed4 0564:fed4 |
| input | AN 5/6 | compRatio | `0465` `0565` | 0.75 → 7.7 → 0465:004d 0565:004d<br>0 → 1.0 → 0465:000a 0565:000a |
| input | AN 5/6 | expTrsh | `0466` `0566` | 0.75 → -47.5 dB → 0466:fe25 0566:fe25<br>0.5652 → -60.0 dB → 0466:fda8 0566:fda8 |
| input | AN 5/6 | expRatio | `0467` `0567` | 0.75 → 7.7 → 0467:004d 0567:004d<br>0 → 1.0 → 0467:000a 0567:000a |
| input | AN 5/6 | compexpEnable | `0460` `0560` | 1 → ? → 0460:0001 0560:0001<br>1 → ? → 0460:0000 0560:0000 |
| input | AN 5/6 | alevMaxgain | `0481` `0581` | 0.75 → 13.5 dB → 0481:0087 0581:0087<br>0.3333 → 6.0 dB → 0481:003c 0581:003c |
| input | AN 5/6 | alevHeadroom | `0482` `0582` | 0.75 → 9.5 dB → 0482:005f 0582:005f<br>0.3333 → 6.0 dB → 0482:003c 0582:003c |
| input | AN 5/6 | alevRisetime | `0483` `0583` | 0.75 → 7.4 s → 0483:004a 0583:004a<br>0.5 → 5.0 s → 0483:0032 0583:0032 |
| input | AN 7/8 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | AN 7/8 | eqGain1 | `0642` `0742` | 0.75 → 10.0 dB → 0642:0064 0742:0064<br>0.5 → 0.0 dB → 0642:0000 0742:0000 |
| input | AN 7/8 | eqFreq1 | `0643` `0743` | 0.75 → 3557 Hz → 0643:0de5 0743:0de5<br>0.2007 → 80 Hz → 0643:0050 0743:0050 |
| input | AN 7/8 | eqQ1 | `0644` `0744` | 0.75 → 3.9 → 0644:0027 0744:0027<br>0.06977 → 1.0 → 0644:000a 0744:000a |
| input | AN 7/8 | eqGain2 | `0645` `0745` | 0.75 → 10.0 dB → 0645:0064 0745:0064<br>0.5 → 0.0 dB → 0645:0000 0745:0000 |
| input | AN 7/8 | eqFreq2 | `0646` `0746` | 0.75 → 3557 Hz → 0646:0de5 0746:0de5<br>0.5663 → 1000 Hz → 0646:03e8 0746:03e8 |
| input | AN 7/8 | eqQ2 | `0647` `0747` | 0.75 → 3.9 → 0647:0027 0747:0027<br>0.06977 → 1.0 → 0647:000a 0747:000a |
| input | AN 7/8 | eqType3 | `0648` `0748` | 0.75 → Low Pass → 0648:0002 0748:0002<br>0.5 → Shelf → 0648:0001 0748:0001 |
| input | AN 7/8 | eqGain3 | `0649` `0749` | 0.75 → 10.0 dB → 0649:0064 0749:0064<br>0.5 → 0.0 dB → 0649:0000 0749:0000 |
| input | AN 7/8 | eqFreq3 | `064a` `074a` | 0.75 → 3557 Hz → 064a:0de5 074a:0de5<br>0.7993 → 5000 Hz → 064a:1388 074a:1388 |
| input | AN 7/8 | eqQ3 | `064b` `074b` | 0.75 → 3.9 → 064b:0027 074b:0027<br>0.06977 → 1.0 → 064b:000a 074b:000a |
| input | AN 7/8 | eqEnable | `0640` `0740` | 1 → ? → 0640:0001 0740:0001<br>1 → ? → 0640:0000 0740:0000 |
| input | AN 7/8 | lowcutFreq | `0621` `0721` | 0.75 → 224 Hz → 0621:00e0 0721:00e0<br>0 → 20 Hz → 0621:0014 0721:0014 |
| input | AN 7/8 | lowcutGrade | `0622` `0722` | 0.75 → 18 dB/oct → 0622:0002 0722:0002<br>0.3333 → 12 dB/oct → 0622:0001 0722:0001 |
| input | AN 7/8 | lowcutEnable | `0620` `0720` | 1 → ? → 0620:0001 0720:0001<br>1 → ? → 0620:0000 0720:0000 |
| input | AN 7/8 | compexpGain | `0661` `0761` | 0.75 → 15.0 dB → 0661:0096 0761:0096<br>0.5 → 0.0 dB → 0661:0000 0761:0000 |
| input | AN 7/8 | compexpAttack | `0662` `0762` | 0.75 → 150 ms → 0662:0096 0762:0096<br>0.05 → 10 ms → 0662:000a 0762:000a |
| input | AN 7/8 | compexpRelease | `0663` `0763` | 0.75 → 774 ms → 0663:0306 0763:0306<br>0.2225 → 300 ms → 0663:012c 0763:012c |
| input | AN 7/8 | compTrsh | `0664` `0764` | 0.75 → -15.0 dB → 0664:ff6a 0764:ff6a<br>0.5 → -30.0 dB → 0664:fed4 0764:fed4 |
| input | AN 7/8 | compRatio | `0665` `0765` | 0.75 → 7.7 → 0665:004d 0765:004d<br>0 → 1.0 → 0665:000a 0765:000a |
| input | AN 7/8 | expTrsh | `0666` `0766` | 0.75 → -47.5 dB → 0666:fe25 0766:fe25<br>0.5652 → -60.0 dB → 0666:fda8 0766:fda8 |
| input | AN 7/8 | expRatio | `0667` `0767` | 0.75 → 7.7 → 0667:004d 0767:004d<br>0 → 1.0 → 0667:000a 0767:000a |
| input | AN 7/8 | compexpEnable | `0660` `0760` | 1 → ? → 0660:0001 0760:0001<br>1 → ? → 0660:0000 0760:0000 |
| input | AN 7/8 | alevMaxgain | `0681` `0781` | 0.75 → 13.5 dB → 0681:0087 0781:0087<br>0.3333 → 6.0 dB → 0681:003c 0781:003c |
| input | AN 7/8 | alevHeadroom | `0682` `0782` | 0.75 → 9.5 dB → 0682:005f 0782:005f<br>0.3333 → 6.0 dB → 0682:003c 0782:003c |
| input | AN 7/8 | alevRisetime | `0683` `0783` | 0.75 → 7.4 s → 0683:004a 0783:004a<br>0.5 → 5.0 s → 0683:0032 0783:0032 |
| input | SPDIF | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | SPDIF | eqGain1 | `0842` `0942` | 0.75 → 10.0 dB → 0842:0064 0942:0064<br>0.5 → 0.0 dB → 0842:0000 0942:0000 |
| input | SPDIF | eqFreq1 | `0843` `0943` | 0.75 → 3557 Hz → 0843:0de5 0943:0de5<br>0.2007 → 80 Hz → 0843:0050 0943:0050 |
| input | SPDIF | eqQ1 | `0844` `0944` | 0.75 → 3.9 → 0844:0027 0944:0027<br>0.06977 → 1.0 → 0844:000a 0944:000a |
| input | SPDIF | eqGain2 | `0845` `0945` | 0.75 → 10.0 dB → 0845:0064 0945:0064<br>0.5 → 0.0 dB → 0845:0000 0945:0000 |
| input | SPDIF | eqFreq2 | `0846` `0946` | 0.75 → 3557 Hz → 0846:0de5 0946:0de5<br>0.5663 → 1000 Hz → 0846:03e8 0946:03e8 |
| input | SPDIF | eqQ2 | `0847` `0947` | 0.75 → 3.9 → 0847:0027 0947:0027<br>0.06977 → 1.0 → 0847:000a 0947:000a |
| input | SPDIF | eqType3 | `0848` `0948` | 0.75 → Low Pass → 0848:0002 0948:0002<br>0.5 → Shelf → 0848:0001 0948:0001 |
| input | SPDIF | eqGain3 | `0849` `0949` | 0.75 → 10.0 dB → 0849:0064 0949:0064<br>0.5 → 0.0 dB → 0849:0000 0949:0000 |
| input | SPDIF | eqFreq3 | `084a` `094a` | 0.75 → 3557 Hz → 084a:0de5 094a:0de5<br>0.7993 → 5000 Hz → 084a:1388 094a:1388 |
| input | SPDIF | eqQ3 | `084b` `094b` | 0.75 → 3.9 → 084b:0027 094b:0027<br>0.06977 → 1.0 → 084b:000a 094b:000a |
| input | SPDIF | eqEnable | `0840` `0940` | 1 → ? → 0840:0001 0940:0001<br>1 → ? → 0840:0000 0940:0000 |
| input | SPDIF | lowcutFreq | `0821` `0921` | 0.75 → 224 Hz → 0821:00e0 0921:00e0<br>0 → 20 Hz → 0821:0014 0921:0014 |
| input | SPDIF | lowcutGrade | `0822` `0922` | 0.75 → 18 dB/oct → 0822:0002 0922:0002<br>0.3333 → 12 dB/oct → 0822:0001 0922:0001 |
| input | SPDIF | lowcutEnable | `0820` `0920` | 1 → ? → 0820:0001 0920:0001<br>1 → ? → 0820:0000 0920:0000 |
| input | SPDIF | compexpGain | `0861` `0961` | 0.75 → 15.0 dB → 0861:0096 0961:0096<br>0.5 → 0.0 dB → 0861:0000 0961:0000 |
| input | SPDIF | compexpAttack | `0862` `0962` | 0.75 → 150 ms → 0862:0096 0962:0096<br>0.05 → 10 ms → 0862:000a 0962:000a |
| input | SPDIF | compexpRelease | `0863` `0963` | 0.75 → 774 ms → 0863:0306 0963:0306<br>0.2225 → 300 ms → 0863:012c 0963:012c |
| input | SPDIF | compTrsh | `0864` `0964` | 0.75 → -15.0 dB → 0864:ff6a 0964:ff6a<br>0.5 → -30.0 dB → 0864:fed4 0964:fed4 |
| input | SPDIF | compRatio | `0865` `0965` | 0.75 → 7.7 → 0865:004d 0965:004d<br>0 → 1.0 → 0865:000a 0965:000a |
| input | SPDIF | expTrsh | `0866` `0966` | 0.75 → -47.5 dB → 0866:fe25 0966:fe25<br>0.5652 → -60.0 dB → 0866:fda8 0966:fda8 |
| input | SPDIF | expRatio | `0867` `0967` | 0.75 → 7.7 → 0867:004d 0967:004d<br>0 → 1.0 → 0867:000a 0967:000a |
| input | SPDIF | compexpEnable | `0860` `0960` | 1 → ? → 0860:0001 0960:0001<br>1 → ? → 0860:0000 0960:0000 |
| input | SPDIF | alevMaxgain | `0881` `0981` | 0.75 → 13.5 dB → 0881:0087 0981:0087<br>0.3333 → 6.0 dB → 0881:003c 0981:003c |
| input | SPDIF | alevHeadroom | `0882` `0982` | 0.75 → 9.5 dB → 0882:005f 0982:005f<br>0.3333 → 6.0 dB → 0882:003c 0982:003c |
| input | SPDIF | alevRisetime | `0883` `0983` | 0.75 → 7.4 s → 0883:004a 0983:004a<br>0.5 → 5.0 s → 0883:0032 0983:0032 |
| input | AS 1/2 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | AS 1/2 | eqGain1 | `0a42` `0b42` | 0.75 → 10.0 dB → 0a42:0064 0b42:0064<br>0.5 → 0.0 dB → 0a42:0000 0b42:0000 |
| input | AS 1/2 | eqFreq1 | `0a43` `0b43` | 0.75 → 3557 Hz → 0a43:0de5 0b43:0de5<br>0.2007 → 80 Hz → 0a43:0050 0b43:0050 |
| input | AS 1/2 | eqQ1 | `0a44` `0b44` | 0.75 → 3.9 → 0a44:0027 0b44:0027<br>0.06977 → 1.0 → 0a44:000a 0b44:000a |
| input | AS 1/2 | eqGain2 | `0a45` `0b45` | 0.75 → 10.0 dB → 0a45:0064 0b45:0064<br>0.5 → 0.0 dB → 0a45:0000 0b45:0000 |
| input | AS 1/2 | eqFreq2 | `0a46` `0b46` | 0.75 → 3557 Hz → 0a46:0de5 0b46:0de5<br>0.5663 → 1000 Hz → 0a46:03e8 0b46:03e8 |
| input | AS 1/2 | eqQ2 | `0a47` `0b47` | 0.75 → 3.9 → 0a47:0027 0b47:0027<br>0.06977 → 1.0 → 0a47:000a 0b47:000a |
| input | AS 1/2 | eqType3 | `0a48` `0b48` | 0.75 → Low Pass → 0a48:0002 0b48:0002<br>0.5 → Shelf → 0a48:0001 0b48:0001 |
| input | AS 1/2 | eqGain3 | `0a49` `0b49` | 0.75 → 10.0 dB → 0a49:0064 0b49:0064<br>0.5 → 0.0 dB → 0a49:0000 0b49:0000 |
| input | AS 1/2 | eqFreq3 | `0a4a` `0b4a` | 0.75 → 3557 Hz → 0a4a:0de5 0b4a:0de5<br>0.7993 → 5000 Hz → 0a4a:1388 0b4a:1388 |
| input | AS 1/2 | eqQ3 | `0a4b` `0b4b` | 0.75 → 3.9 → 0a4b:0027 0b4b:0027<br>0.06977 → 1.0 → 0a4b:000a 0b4b:000a |
| input | AS 1/2 | eqEnable | `0a40` `0b40` | 1 → ? → 0a40:0001 0b40:0001<br>1 → ? → 0a40:0000 0b40:0000 |
| input | AS 1/2 | lowcutFreq | `0a21` `0b21` | 0.75 → 224 Hz → 0a21:00e0 0b21:00e0<br>0 → 20 Hz → 0a21:0014 0b21:0014 |
| input | AS 1/2 | lowcutGrade | `0a22` `0b22` | 0.75 → 18 dB/oct → 0a22:0002 0b22:0002<br>0.3333 → 12 dB/oct → 0a22:0001 0b22:0001 |
| input | AS 1/2 | lowcutEnable | `0a20` `0b20` | 1 → ? → 0a20:0001 0b20:0001<br>1 → ? → 0a20:0000 0b20:0000 |
| input | AS 1/2 | compexpGain | `0a61` `0b61` | 0.75 → 15.0 dB → 0a61:0096 0b61:0096<br>0.5 → 0.0 dB → 0a61:0000 0b61:0000 |
| input | AS 1/2 | compexpAttack | `0a62` `0b62` | 0.75 → 150 ms → 0a62:0096 0b62:0096<br>0.05 → 10 ms → 0a62:000a 0b62:000a |
| input | AS 1/2 | compexpRelease | `0a63` `0b63` | 0.75 → 774 ms → 0a63:0306 0b63:0306<br>0.2225 → 300 ms → 0a63:012c 0b63:012c |
| input | AS 1/2 | compTrsh | `0a64` `0b64` | 0.75 → -15.0 dB → 0a64:ff6a 0b64:ff6a<br>0.5 → -30.0 dB → 0a64:fed4 0b64:fed4 |
| input | AS 1/2 | compRatio | `0a65` `0b65` | 0.75 → 7.7 → 0a65:004d 0b65:004d<br>0 → 1.0 → 0a65:000a 0b65:000a |
| input | AS 1/2 | expTrsh | `0a66` `0b66` | 0.75 → -47.5 dB → 0a66:fe25 0b66:fe25<br>0.5652 → -60.0 dB → 0a66:fda8 0b66:fda8 |
| input | AS 1/2 | expRatio | `0a67` `0b67` | 0.75 → 7.7 → 0a67:004d 0b67:004d<br>0 → 1.0 → 0a67:000a 0b67:000a |
| input | AS 1/2 | compexpEnable | `0a60` `0b60` | 1 → ? → 0a60:0001 0b60:0001<br>1 → ? → 0a60:0000 0b60:0000 |
| input | AS 1/2 | alevMaxgain | `0a81` `0b81` | 0.75 → 13.5 dB → 0a81:0087 0b81:0087<br>0.3333 → 6.0 dB → 0a81:003c 0b81:003c |
| input | AS 1/2 | alevHeadroom | `0a82` `0b82` | 0.75 → 9.5 dB → 0a82:005f 0b82:005f<br>0.3333 → 6.0 dB → 0a82:003c 0b82:003c |
| input | AS 1/2 | alevRisetime | `0a83` `0b83` | 0.75 → 7.4 s → 0a83:004a 0b83:004a<br>0.5 → 5.0 s → 0a83:0032 0b83:0032 |
| input | ADAT 3/4 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | ADAT 3/4 | eqGain1 | `0c42` `0d42` | 0.75 → 10.0 dB → 0c42:0064 0d42:0064<br>0.5 → 0.0 dB → 0c42:0000 0d42:0000 |
| input | ADAT 3/4 | eqFreq1 | `0c43` `0d43` | 0.75 → 3557 Hz → 0c43:0de5 0d43:0de5<br>0.2007 → 80 Hz → 0c43:0050 0d43:0050 |
| input | ADAT 3/4 | eqQ1 | `0c44` `0d44` | 0.75 → 3.9 → 0c44:0027 0d44:0027<br>0.06977 → 1.0 → 0c44:000a 0d44:000a |
| input | ADAT 3/4 | eqGain2 | `0c45` `0d45` | 0.75 → 10.0 dB → 0c45:0064 0d45:0064<br>0.5 → 0.0 dB → 0c45:0000 0d45:0000 |
| input | ADAT 3/4 | eqFreq2 | `0c46` `0d46` | 0.75 → 3557 Hz → 0c46:0de5 0d46:0de5<br>0.5663 → 1000 Hz → 0c46:03e8 0d46:03e8 |
| input | ADAT 3/4 | eqQ2 | `0c47` `0d47` | 0.75 → 3.9 → 0c47:0027 0d47:0027<br>0.06977 → 1.0 → 0c47:000a 0d47:000a |
| input | ADAT 3/4 | eqType3 | `0c48` `0d48` | 0.75 → Low Pass → 0c48:0002 0d48:0002<br>0.5 → Shelf → 0c48:0001 0d48:0001 |
| input | ADAT 3/4 | eqGain3 | `0c49` `0d49` | 0.75 → 10.0 dB → 0c49:0064 0d49:0064<br>0.5 → 0.0 dB → 0c49:0000 0d49:0000 |
| input | ADAT 3/4 | eqFreq3 | `0c4a` `0d4a` | 0.75 → 3557 Hz → 0c4a:0de5 0d4a:0de5<br>0.7993 → 5000 Hz → 0c4a:1388 0d4a:1388 |
| input | ADAT 3/4 | eqQ3 | `0c4b` `0d4b` | 0.75 → 3.9 → 0c4b:0027 0d4b:0027<br>0.06977 → 1.0 → 0c4b:000a 0d4b:000a |
| input | ADAT 3/4 | eqEnable | `0c40` `0d40` | 1 → ? → 0c40:0001 0d40:0001<br>1 → ? → 0c40:0000 0d40:0000 |
| input | ADAT 3/4 | lowcutFreq | `0c21` `0d21` | 0.75 → 224 Hz → 0c21:00e0 0d21:00e0<br>0 → 20 Hz → 0c21:0014 0d21:0014 |
| input | ADAT 3/4 | lowcutGrade | `0c22` `0d22` | 0.75 → 18 dB/oct → 0c22:0002 0d22:0002<br>0.3333 → 12 dB/oct → 0c22:0001 0d22:0001 |
| input | ADAT 3/4 | lowcutEnable | `0c20` `0d20` | 1 → ? → 0c20:0001 0d20:0001<br>1 → ? → 0c20:0000 0d20:0000 |
| input | ADAT 3/4 | compexpGain | `0c61` `0d61` | 0.75 → 15.0 dB → 0c61:0096 0d61:0096<br>0.5 → 0.0 dB → 0c61:0000 0d61:0000 |
| input | ADAT 3/4 | compexpAttack | `0c62` `0d62` | 0.75 → 150 ms → 0c62:0096 0d62:0096<br>0.05 → 10 ms → 0c62:000a 0d62:000a |
| input | ADAT 3/4 | compexpRelease | `0c63` `0d63` | 0.75 → 774 ms → 0c63:0306 0d63:0306<br>0.2225 → 300 ms → 0c63:012c 0d63:012c |
| input | ADAT 3/4 | compTrsh | `0c64` `0d64` | 0.75 → -15.0 dB → 0c64:ff6a 0d64:ff6a<br>0.5 → -30.0 dB → 0c64:fed4 0d64:fed4 |
| input | ADAT 3/4 | compRatio | `0c65` `0d65` | 0.75 → 7.7 → 0c65:004d 0d65:004d<br>0 → 1.0 → 0c65:000a 0d65:000a |
| input | ADAT 3/4 | expTrsh | `0c66` `0d66` | 0.75 → -47.5 dB → 0c66:fe25 0d66:fe25<br>0.5652 → -60.0 dB → 0c66:fda8 0d66:fda8 |
| input | ADAT 3/4 | expRatio | `0c67` `0d67` | 0.75 → 7.7 → 0c67:004d 0d67:004d<br>0 → 1.0 → 0c67:000a 0d67:000a |
| input | ADAT 3/4 | compexpEnable | `0c60` `0d60` | 1 → ? → 0c60:0001 0d60:0001<br>1 → ? → 0c60:0000 0d60:0000 |
| input | ADAT 3/4 | alevMaxgain | `0c81` `0d81` | 0.75 → 13.5 dB → 0c81:0087 0d81:0087<br>0.3333 → 6.0 dB → 0c81:003c 0d81:003c |
| input | ADAT 3/4 | alevHeadroom | `0c82` `0d82` | 0.75 → 9.5 dB → 0c82:005f 0d82:005f<br>0.3333 → 6.0 dB → 0c82:003c 0d82:003c |
| input | ADAT 3/4 | alevRisetime | `0c83` `0d83` | 0.75 → 7.4 s → 0c83:004a 0d83:004a<br>0.5 → 5.0 s → 0c83:0032 0d83:0032 |
| input | ADAT 5/6 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | ADAT 5/6 | eqGain1 | `0e42` `0f42` | 0.75 → 10.0 dB → 0e42:0064 0f42:0064<br>0.5 → 0.0 dB → 0e42:0000 0f42:0000 |
| input | ADAT 5/6 | eqFreq1 | `0e43` `0f43` | 0.75 → 3557 Hz → 0e43:0de5 0f43:0de5<br>0.2007 → 80 Hz → 0e43:0050 0f43:0050 |
| input | ADAT 5/6 | eqQ1 | `0e44` `0f44` | 0.75 → 3.9 → 0e44:0027 0f44:0027<br>0.06977 → 1.0 → 0e44:000a 0f44:000a |
| input | ADAT 5/6 | eqGain2 | `0e45` `0f45` | 0.75 → 10.0 dB → 0e45:0064 0f45:0064<br>0.5 → 0.0 dB → 0e45:0000 0f45:0000 |
| input | ADAT 5/6 | eqFreq2 | `0e46` `0f46` | 0.75 → 3557 Hz → 0e46:0de5 0f46:0de5<br>0.5663 → 1000 Hz → 0e46:03e8 0f46:03e8 |
| input | ADAT 5/6 | eqQ2 | `0e47` `0f47` | 0.75 → 3.9 → 0e47:0027 0f47:0027<br>0.06977 → 1.0 → 0e47:000a 0f47:000a |
| input | ADAT 5/6 | eqType3 | `0e48` `0f48` | 0.75 → Low Pass → 0e48:0002 0f48:0002<br>0.5 → Shelf → 0e48:0001 0f48:0001 |
| input | ADAT 5/6 | eqGain3 | `0e49` `0f49` | 0.75 → 10.0 dB → 0e49:0064 0f49:0064<br>0.5 → 0.0 dB → 0e49:0000 0f49:0000 |
| input | ADAT 5/6 | eqFreq3 | `0e4a` `0f4a` | 0.75 → 3557 Hz → 0e4a:0de5 0f4a:0de5<br>0.7993 → 5000 Hz → 0e4a:1388 0f4a:1388 |
| input | ADAT 5/6 | eqQ3 | `0e4b` `0f4b` | 0.75 → 3.9 → 0e4b:0027 0f4b:0027<br>0.06977 → 1.0 → 0e4b:000a 0f4b:000a |
| input | ADAT 5/6 | eqEnable | `0e40` `0f40` | 1 → ? → 0e40:0001 0f40:0001<br>1 → ? → 0e40:0000 0f40:0000 |
| input | ADAT 5/6 | lowcutFreq | `0e21` `0f21` | 0.75 → 224 Hz → 0e21:00e0 0f21:00e0<br>0 → 20 Hz → 0e21:0014 0f21:0014 |
| input | ADAT 5/6 | lowcutGrade | `0e22` `0f22` | 0.75 → 18 dB/oct → 0e22:0002 0f22:0002<br>0.3333 → 12 dB/oct → 0e22:0001 0f22:0001 |
| input | ADAT 5/6 | lowcutEnable | `0e20` `0f20` | 1 → ? → 0e20:0001 0f20:0001<br>1 → ? → 0e20:0000 0f20:0000 |
| input | ADAT 5/6 | compexpGain | `0e61` `0f61` | 0.75 → 15.0 dB → 0e61:0096 0f61:0096<br>0.5 → 0.0 dB → 0e61:0000 0f61:0000 |
| input | ADAT 5/6 | compexpAttack | `0e62` `0f62` | 0.75 → 150 ms → 0e62:0096 0f62:0096<br>0.05 → 10 ms → 0e62:000a 0f62:000a |
| input | ADAT 5/6 | compexpRelease | `0e63` `0f63` | 0.75 → 774 ms → 0e63:0306 0f63:0306<br>0.2225 → 300 ms → 0e63:012c 0f63:012c |
| input | ADAT 5/6 | compTrsh | `0e64` `0f64` | 0.75 → -15.0 dB → 0e64:ff6a 0f64:ff6a<br>0.5 → -30.0 dB → 0e64:fed4 0f64:fed4 |
| input | ADAT 5/6 | compRatio | `0e65` `0f65` | 0.75 → 7.7 → 0e65:004d 0f65:004d<br>0 → 1.0 → 0e65:000a 0f65:000a |
| input | ADAT 5/6 | expTrsh | `0e66` `0f66` | 0.75 → -47.5 dB → 0e66:fe25 0f66:fe25<br>0.5652 → -60.0 dB → 0e66:fda8 0f66:fda8 |
| input | ADAT 5/6 | expRatio | `0e67` `0f67` | 0.75 → 7.7 → 0e67:004d 0f67:004d<br>0 → 1.0 → 0e67:000a 0f67:000a |
| input | ADAT 5/6 | compexpEnable | `0e60` `0f60` | 1 → ? → 0e60:0001 0f60:0001<br>1 → ? → 0e60:0000 0f60:0000 |
| input | ADAT 5/6 | alevMaxgain | `0e81` `0f81` | 0.75 → 13.5 dB → 0e81:0087 0f81:0087<br>0.3333 → 6.0 dB → 0e81:003c 0f81:003c |
| input | ADAT 5/6 | alevHeadroom | `0e82` `0f82` | 0.75 → 9.5 dB → 0e82:005f 0f82:005f<br>0.3333 → 6.0 dB → 0e82:003c 0f82:003c |
| input | ADAT 5/6 | alevRisetime | `0e83` `0f83` | 0.75 → 7.4 s → 0e83:004a 0f83:004a<br>0.5 → 5.0 s → 0e83:0032 0f83:0032 |
| input | ADAT 7/8 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| input | ADAT 7/8 | eqGain1 | `1042` `1142` | 0.75 → 10.0 dB → 1042:0064 1142:0064<br>0.5 → 0.0 dB → 1042:0000 1142:0000 |
| input | ADAT 7/8 | eqFreq1 | `1043` `1143` | 0.75 → 3557 Hz → 1043:0de5 1143:0de5<br>0.2007 → 80 Hz → 1043:0050 1143:0050 |
| input | ADAT 7/8 | eqQ1 | `1044` `1144` | 0.75 → 3.9 → 1044:0027 1144:0027<br>0.06977 → 1.0 → 1044:000a 1144:000a |
| input | ADAT 7/8 | eqGain2 | `1045` `1145` | 0.75 → 10.0 dB → 1045:0064 1145:0064<br>0.5 → 0.0 dB → 1045:0000 1145:0000 |
| input | ADAT 7/8 | eqFreq2 | `1046` `1146` | 0.75 → 3557 Hz → 1046:0de5 1146:0de5<br>0.5663 → 1000 Hz → 1046:03e8 1146:03e8 |
| input | ADAT 7/8 | eqQ2 | `1047` `1147` | 0.75 → 3.9 → 1047:0027 1147:0027<br>0.06977 → 1.0 → 1047:000a 1147:000a |
| input | ADAT 7/8 | eqType3 | `1048` `1148` | 0.75 → Low Pass → 1048:0002 1148:0002<br>0.5 → Shelf → 1048:0001 1148:0001 |
| input | ADAT 7/8 | eqGain3 | `1049` `1149` | 0.75 → 10.0 dB → 1049:0064 1149:0064<br>0.5 → 0.0 dB → 1049:0000 1149:0000 |
| input | ADAT 7/8 | eqFreq3 | `104a` `114a` | 0.75 → 3557 Hz → 104a:0de5 114a:0de5<br>0.7993 → 5000 Hz → 104a:1388 114a:1388 |
| input | ADAT 7/8 | eqQ3 | `104b` `114b` | 0.75 → 3.9 → 104b:0027 114b:0027<br>0.06977 → 1.0 → 104b:000a 114b:000a |
| input | ADAT 7/8 | eqEnable | `1040` `1140` | 1 → ? → 1040:0001 1140:0001<br>1 → ? → 1040:0000 1140:0000 |
| input | ADAT 7/8 | lowcutFreq | `1021` `1121` | 0.75 → 224 Hz → 1021:00e0 1121:00e0<br>0 → 20 Hz → 1021:0014 1121:0014 |
| input | ADAT 7/8 | lowcutGrade | `1022` `1122` | 0.75 → 18 dB/oct → 1022:0002 1122:0002<br>0.3333 → 12 dB/oct → 1022:0001 1122:0001 |
| input | ADAT 7/8 | lowcutEnable | `1020` `1120` | 1 → ? → 1020:0001 1120:0001<br>1 → ? → 1020:0000 1120:0000 |
| input | ADAT 7/8 | compexpGain | `1061` `1161` | 0.75 → 15.0 dB → 1061:0096 1161:0096<br>0.5 → 0.0 dB → 1061:0000 1161:0000 |
| input | ADAT 7/8 | compexpAttack | `1062` `1162` | 0.75 → 150 ms → 1062:0096 1162:0096<br>0.05 → 10 ms → 1062:000a 1162:000a |
| input | ADAT 7/8 | compexpRelease | `1063` `1163` | 0.75 → 774 ms → 1063:0306 1163:0306<br>0.2225 → 300 ms → 1063:012c 1163:012c |
| input | ADAT 7/8 | compTrsh | `1064` `1164` | 0.75 → -15.0 dB → 1064:ff6a 1164:ff6a<br>0.5 → -30.0 dB → 1064:fed4 1164:fed4 |
| input | ADAT 7/8 | compRatio | `1065` `1165` | 0.75 → 7.7 → 1065:004d 1165:004d<br>0 → 1.0 → 1065:000a 1165:000a |
| input | ADAT 7/8 | expTrsh | `1066` `1166` | 0.75 → -47.5 dB → 1066:fe25 1166:fe25<br>0.5652 → -60.0 dB → 1066:fda8 1166:fda8 |
| input | ADAT 7/8 | expRatio | `1067` `1167` | 0.75 → 7.7 → 1067:004d 1167:004d<br>0 → 1.0 → 1067:000a 1167:000a |
| input | ADAT 7/8 | compexpEnable | `1060` `1160` | 1 → ? → 1060:0001 1160:0001<br>1 → ? → 1060:0000 1160:0000 |
| input | ADAT 7/8 | alevMaxgain | `1081` `1181` | 0.75 → 13.5 dB → 1081:0087 1181:0087<br>0.3333 → 6.0 dB → 1081:003c 1181:003c |
| input | ADAT 7/8 | alevHeadroom | `1082` `1182` | 0.75 → 9.5 dB → 1082:005f 1182:005f<br>0.3333 → 6.0 dB → 1082:003c 1182:003c |
| input | ADAT 7/8 | alevRisetime | `1083` `1183` | 0.75 → 7.4 s → 1083:004a 1183:004a<br>0.5 → 5.0 s → 1083:0032 1183:0032 |
| playback | AN 1/2 | eqType1 | none | 0 → Bell → (none)<br>0.25 → Bell → (none)<br>0.5 → Shelf → (none)<br>0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| playback | AN 1/2 | eqGain1 | none | 0 → -20.0 dB → (none)<br>0.25 → -10.0 dB → (none)<br>0.75 → 10.0 dB → (none)<br>1 → 20.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 1/2 | eqFreq1 | none | 0 → 20 Hz → (none)<br>0.25 → 112 Hz → (none)<br>0.5 → 632 Hz → (none)<br>0.75 → 3557 Hz → (none)<br>1 → 20000 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | AN 1/2 | eqQ1 | none | 0 → 0.7 → (none)<br>0.25 → 1.7 → (none)<br>0.5 → 2.8 → (none)<br>0.75 → 3.9 → (none)<br>1 → 5.0 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 1/2 | eqGain2 | none | 0 → -20.0 dB → (none)<br>0.25 → -10.0 dB → (none)<br>0.75 → 10.0 dB → (none)<br>1 → 20.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 1/2 | eqFreq2 | none | 0 → 20 Hz → (none)<br>0.25 → 112 Hz → (none)<br>0.5 → 632 Hz → (none)<br>0.75 → 3557 Hz → (none)<br>1 → 20000 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | AN 1/2 | eqQ2 | none | 0 → 0.7 → (none)<br>0.25 → 1.7 → (none)<br>0.5 → 2.8 → (none)<br>0.75 → 3.9 → (none)<br>1 → 5.0 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 1/2 | eqType3 | none | 0 → Bell → (none)<br>0.25 → Shelf → (none)<br>0.75 → Low Pass → (none)<br>1 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | AN 1/2 | eqGain3 | none | 0 → -20.0 dB → (none)<br>0.25 → -10.0 dB → (none)<br>0.75 → 10.0 dB → (none)<br>1 → 20.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 1/2 | eqFreq3 | none | 0 → 20 Hz → (none)<br>0.25 → 112 Hz → (none)<br>0.5 → 632 Hz → (none)<br>0.75 → 3557 Hz → (none)<br>1 → 20000 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | AN 1/2 | eqQ3 | none | 0 → 0.7 → (none)<br>0.25 → 1.7 → (none)<br>0.5 → 2.8 → (none)<br>0.75 → 3.9 → (none)<br>1 → 5.0 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 1/2 | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | lowcutFreq | none | 0.25 → 45 Hz → (none)<br>0.5 → 100 Hz → (none)<br>0.75 → 224 Hz → (none)<br>1 → 500 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | AN 1/2 | lowcutGrade | none | 0 → 6 dB/oct → (none)<br>0.25 → 12 dB/oct → (none)<br>0.5 → 18 dB/oct → (none)<br>0.75 → 18 dB/oct → (none)<br>1 → 24 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | AN 1/2 | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | compexpGain | none | 0 → -30.0 dB → (none)<br>0.25 → -15.0 dB → (none)<br>0.75 → 15.0 dB → (none)<br>1 → 30.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 1/2 | compexpAttack | none | 0 → 0 ms → (none)<br>0.25 → 50 ms → (none)<br>0.5 → 100 ms → (none)<br>0.75 → 150 ms → (none)<br>1 → 200 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | AN 1/2 | compexpRelease | none | 0 → 100 ms → (none)<br>0.25 → 325 ms → (none)<br>0.5 → 550 ms → (none)<br>0.75 → 774 ms → (none)<br>1 → 999 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | AN 1/2 | compTrsh | none | 0 → -60.0 dB → (none)<br>0.25 → -45.0 dB → (none)<br>0.75 → -15.0 dB → (none)<br>1 → 0.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | AN 1/2 | compRatio | none | 0.25 → 3.2 → (none)<br>0.5 → 5.5 → (none)<br>0.75 → 7.7 → (none)<br>1 → 10.0 → (none)<br>0 → 1.0 → (none) |
| playback | AN 1/2 | expTrsh | none | 0 → -99.0 dB → (none)<br>0.25 → -82.0 dB → (none)<br>0.5 → -64.5 dB → (none)<br>0.75 → -47.5 dB → (none)<br>1 → -30.0 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | AN 1/2 | expRatio | none | 0.25 → 3.2 → (none)<br>0.5 → 5.5 → (none)<br>0.75 → 7.7 → (none)<br>1 → 10.0 → (none)<br>0 → 1.0 → (none) |
| playback | AN 1/2 | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 1/2 | alevMaxgain | none | 0 → 0.0 dB → (none)<br>0.25 → 4.5 dB → (none)<br>0.5 → 9.0 dB → (none)<br>0.75 → 13.5 dB → (none)<br>1 → 18.0 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | AN 1/2 | alevHeadroom | none | 0 → 3.0 dB → (none)<br>0.25 → 5.0 dB → (none)<br>0.5 → 7.5 dB → (none)<br>0.75 → 9.5 dB → (none)<br>1 → 12.0 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | AN 1/2 | alevRisetime | none | 0 → 0.1 s → (none)<br>0.25 → 2.5 s → (none)<br>0.75 → 7.4 s → (none)<br>1 → 9.9 s → (none)<br>0.5 → 5.0 s → (none) |
| playback | AN 3/4 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| playback | AN 3/4 | eqGain1 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 3/4 | eqFreq1 | none | 0.75 → 3557 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | AN 3/4 | eqQ1 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 3/4 | eqGain2 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 3/4 | eqFreq2 | none | 0.75 → 3557 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | AN 3/4 | eqQ2 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 3/4 | eqType3 | none | 0.75 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | AN 3/4 | eqGain3 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 3/4 | eqFreq3 | none | 0.75 → 3557 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | AN 3/4 | eqQ3 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 3/4 | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 3/4 | lowcutFreq | none | 0.75 → 224 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | AN 3/4 | lowcutGrade | none | 0.75 → 18 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | AN 3/4 | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 3/4 | compexpGain | none | 0.75 → 15.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 3/4 | compexpAttack | none | 0.75 → 150 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | AN 3/4 | compexpRelease | none | 0.75 → 774 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | AN 3/4 | compTrsh | none | 0.75 → -15.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | AN 3/4 | compRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | AN 3/4 | expTrsh | none | 0.75 → -47.5 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | AN 3/4 | expRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | AN 3/4 | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 3/4 | alevMaxgain | none | 0.75 → 13.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | AN 3/4 | alevHeadroom | none | 0.75 → 9.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | AN 3/4 | alevRisetime | none | 0.75 → 7.4 s → (none)<br>0.5 → 5.0 s → (none) |
| playback | AN 5/6 | eqType1 | none | 0.75 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | eqGain1 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 5/6 | eqFreq1 | none | 0.75 → 3557 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | AN 5/6 | eqQ1 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 5/6 | eqGain2 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 5/6 | eqFreq2 | none | 0.75 → 3557 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | AN 5/6 | eqQ2 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 5/6 | eqType3 | none | 0.75 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | AN 5/6 | eqGain3 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 5/6 | eqFreq3 | none | 0.75 → 3557 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | AN 5/6 | eqQ3 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AN 5/6 | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | lowcutFreq | none | 0.75 → 224 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | AN 5/6 | lowcutGrade | none | 0.75 → 18 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | AN 5/6 | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | compexpGain | none | 0.75 → 15.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AN 5/6 | compexpAttack | none | 0.75 → 150 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | AN 5/6 | compexpRelease | none | 0.75 → 774 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | AN 5/6 | compTrsh | none | 0.75 → -15.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | AN 5/6 | compRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | AN 5/6 | expTrsh | none | 0.75 → -47.5 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | AN 5/6 | expRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | AN 5/6 | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AN 5/6 | alevMaxgain | none | 0.75 → 13.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | AN 5/6 | alevHeadroom | none | 0.75 → 9.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | AN 5/6 | alevRisetime | none | 0.75 → 7.4 s → (none)<br>0.5 → 5.0 s → (none) |
| playback | PH 7/8 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| playback | PH 7/8 | eqGain1 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | PH 7/8 | eqFreq1 | none | 0.75 → 3557 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | PH 7/8 | eqQ1 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | PH 7/8 | eqGain2 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | PH 7/8 | eqFreq2 | none | 0.75 → 3557 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | PH 7/8 | eqQ2 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | PH 7/8 | eqType3 | none | 0.75 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | PH 7/8 | eqGain3 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | PH 7/8 | eqFreq3 | none | 0.75 → 3557 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | PH 7/8 | eqQ3 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | PH 7/8 | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | PH 7/8 | lowcutFreq | none | 0.75 → 224 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | PH 7/8 | lowcutGrade | none | 0.75 → 18 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | PH 7/8 | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | PH 7/8 | compexpGain | none | 0.75 → 15.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | PH 7/8 | compexpAttack | none | 0.75 → 150 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | PH 7/8 | compexpRelease | none | 0.75 → 774 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | PH 7/8 | compTrsh | none | 0.75 → -15.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | PH 7/8 | compRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | PH 7/8 | expTrsh | none | 0.75 → -47.5 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | PH 7/8 | expRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | PH 7/8 | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | PH 7/8 | alevMaxgain | none | 0.75 → 13.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | PH 7/8 | alevHeadroom | none | 0.75 → 9.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | PH 7/8 | alevRisetime | none | 0.75 → 7.4 s → (none)<br>0.5 → 5.0 s → (none) |
| playback | SPDIF | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| playback | SPDIF | eqGain1 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | SPDIF | eqFreq1 | none | 0.75 → 3557 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | SPDIF | eqQ1 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | SPDIF | eqGain2 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | SPDIF | eqFreq2 | none | 0.75 → 3557 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | SPDIF | eqQ2 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | SPDIF | eqType3 | none | 0.75 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | SPDIF | eqGain3 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | SPDIF | eqFreq3 | none | 0.75 → 3557 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | SPDIF | eqQ3 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | SPDIF | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | SPDIF | lowcutFreq | none | 0.75 → 224 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | SPDIF | lowcutGrade | none | 0.75 → 18 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | SPDIF | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | SPDIF | compexpGain | none | 0.75 → 15.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | SPDIF | compexpAttack | none | 0.75 → 150 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | SPDIF | compexpRelease | none | 0.75 → 774 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | SPDIF | compTrsh | none | 0.75 → -15.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | SPDIF | compRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | SPDIF | expTrsh | none | 0.75 → -47.5 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | SPDIF | expRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | SPDIF | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | SPDIF | alevMaxgain | none | 0.75 → 13.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | SPDIF | alevHeadroom | none | 0.75 → 9.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | SPDIF | alevRisetime | none | 0.75 → 7.4 s → (none)<br>0.5 → 5.0 s → (none) |
| playback | AS 1/2 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| playback | AS 1/2 | eqGain1 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AS 1/2 | eqFreq1 | none | 0.75 → 3557 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | AS 1/2 | eqQ1 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AS 1/2 | eqGain2 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AS 1/2 | eqFreq2 | none | 0.75 → 3557 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | AS 1/2 | eqQ2 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AS 1/2 | eqType3 | none | 0.75 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | AS 1/2 | eqGain3 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AS 1/2 | eqFreq3 | none | 0.75 → 3557 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | AS 1/2 | eqQ3 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | AS 1/2 | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AS 1/2 | lowcutFreq | none | 0.75 → 224 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | AS 1/2 | lowcutGrade | none | 0.75 → 18 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | AS 1/2 | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AS 1/2 | compexpGain | none | 0.75 → 15.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | AS 1/2 | compexpAttack | none | 0.75 → 150 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | AS 1/2 | compexpRelease | none | 0.75 → 774 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | AS 1/2 | compTrsh | none | 0.75 → -15.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | AS 1/2 | compRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | AS 1/2 | expTrsh | none | 0.75 → -47.5 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | AS 1/2 | expRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | AS 1/2 | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | AS 1/2 | alevMaxgain | none | 0.75 → 13.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | AS 1/2 | alevHeadroom | none | 0.75 → 9.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | AS 1/2 | alevRisetime | none | 0.75 → 7.4 s → (none)<br>0.5 → 5.0 s → (none) |
| playback | ADAT 3/4 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| playback | ADAT 3/4 | eqGain1 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 3/4 | eqFreq1 | none | 0.75 → 3557 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | ADAT 3/4 | eqQ1 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 3/4 | eqGain2 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 3/4 | eqFreq2 | none | 0.75 → 3557 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | ADAT 3/4 | eqQ2 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 3/4 | eqType3 | none | 0.75 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | ADAT 3/4 | eqGain3 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 3/4 | eqFreq3 | none | 0.75 → 3557 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | ADAT 3/4 | eqQ3 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 3/4 | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 3/4 | lowcutFreq | none | 0.75 → 224 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | ADAT 3/4 | lowcutGrade | none | 0.75 → 18 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | ADAT 3/4 | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 3/4 | compexpGain | none | 0.75 → 15.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 3/4 | compexpAttack | none | 0.75 → 150 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | ADAT 3/4 | compexpRelease | none | 0.75 → 774 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | ADAT 3/4 | compTrsh | none | 0.75 → -15.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | ADAT 3/4 | compRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | ADAT 3/4 | expTrsh | none | 0.75 → -47.5 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | ADAT 3/4 | expRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | ADAT 3/4 | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 3/4 | alevMaxgain | none | 0.75 → 13.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | ADAT 3/4 | alevHeadroom | none | 0.75 → 9.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | ADAT 3/4 | alevRisetime | none | 0.75 → 7.4 s → (none)<br>0.5 → 5.0 s → (none) |
| playback | ADAT 5/6 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| playback | ADAT 5/6 | eqGain1 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 5/6 | eqFreq1 | none | 0.75 → 3557 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | ADAT 5/6 | eqQ1 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 5/6 | eqGain2 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 5/6 | eqFreq2 | none | 0.75 → 3557 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | ADAT 5/6 | eqQ2 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 5/6 | eqType3 | none | 0.75 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | ADAT 5/6 | eqGain3 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 5/6 | eqFreq3 | none | 0.75 → 3557 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | ADAT 5/6 | eqQ3 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 5/6 | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 5/6 | lowcutFreq | none | 0.75 → 224 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | ADAT 5/6 | lowcutGrade | none | 0.75 → 18 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | ADAT 5/6 | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 5/6 | compexpGain | none | 0.75 → 15.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 5/6 | compexpAttack | none | 0.75 → 150 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | ADAT 5/6 | compexpRelease | none | 0.75 → 774 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | ADAT 5/6 | compTrsh | none | 0.75 → -15.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | ADAT 5/6 | compRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | ADAT 5/6 | expTrsh | none | 0.75 → -47.5 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | ADAT 5/6 | expRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | ADAT 5/6 | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 5/6 | alevMaxgain | none | 0.75 → 13.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | ADAT 5/6 | alevHeadroom | none | 0.75 → 9.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | ADAT 5/6 | alevRisetime | none | 0.75 → 7.4 s → (none)<br>0.5 → 5.0 s → (none) |
| playback | ADAT 7/8 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| playback | ADAT 7/8 | eqGain1 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 7/8 | eqFreq1 | none | 0.75 → 3557 Hz → (none)<br>0.2007 → 80 Hz → (none) |
| playback | ADAT 7/8 | eqQ1 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 7/8 | eqGain2 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 7/8 | eqFreq2 | none | 0.75 → 3557 Hz → (none)<br>0.5663 → 1000 Hz → (none) |
| playback | ADAT 7/8 | eqQ2 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 7/8 | eqType3 | none | 0.75 → Low Pass → (none)<br>0.5 → Shelf → (none) |
| playback | ADAT 7/8 | eqGain3 | none | 0.75 → 10.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 7/8 | eqFreq3 | none | 0.75 → 3557 Hz → (none)<br>0.7993 → 5000 Hz → (none) |
| playback | ADAT 7/8 | eqQ3 | none | 0.75 → 3.9 → (none)<br>0.06977 → 1.0 → (none) |
| playback | ADAT 7/8 | eqEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 7/8 | lowcutFreq | none | 0.75 → 224 Hz → (none)<br>0 → 20 Hz → (none) |
| playback | ADAT 7/8 | lowcutGrade | none | 0.75 → 18 dB/oct → (none)<br>0.3333 → 12 dB/oct → (none) |
| playback | ADAT 7/8 | lowcutEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 7/8 | compexpGain | none | 0.75 → 15.0 dB → (none)<br>0.5 → 0.0 dB → (none) |
| playback | ADAT 7/8 | compexpAttack | none | 0.75 → 150 ms → (none)<br>0.05 → 10 ms → (none) |
| playback | ADAT 7/8 | compexpRelease | none | 0.75 → 774 ms → (none)<br>0.2225 → 300 ms → (none) |
| playback | ADAT 7/8 | compTrsh | none | 0.75 → -15.0 dB → (none)<br>0.5 → -30.0 dB → (none) |
| playback | ADAT 7/8 | compRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | ADAT 7/8 | expTrsh | none | 0.75 → -47.5 dB → (none)<br>0.5652 → -60.0 dB → (none) |
| playback | ADAT 7/8 | expRatio | none | 0.75 → 7.7 → (none)<br>0 → 1.0 → (none) |
| playback | ADAT 7/8 | compexpEnable | none | 1 → ? → (none)<br>1 → ? → (none) |
| playback | ADAT 7/8 | alevMaxgain | none | 0.75 → 13.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | ADAT 7/8 | alevHeadroom | none | 0.75 → 9.5 dB → (none)<br>0.3333 → 6.0 dB → (none) |
| playback | ADAT 7/8 | alevRisetime | none | 0.75 → 7.4 s → (none)<br>0.5 → 5.0 s → (none) |
| output | AN 1/2 | eqType1 | `1241` `1341` | 0 → Bell → 1241:0000 1341:0000<br>0.25 → Bell → (none)<br>0.5 → Shelf → 1241:0001 1341:0001<br>0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| output | AN 1/2 | eqGain1 | `1242` `1342` | 0 → -20.0 dB → 1242:ff38 1342:ff38<br>0.25 → -10.0 dB → 1242:ff9c 1342:ff9c<br>0.75 → 10.0 dB → 1242:0064 1342:0064<br>1 → 20.0 dB → 1242:00c8 1342:00c8<br>0.5 → 0.0 dB → 1242:0000 1342:0000 |
| output | AN 1/2 | eqFreq1 | `1243` `1343` | 0 → 20 Hz → 1243:0014 1343:0014<br>0.25 → 112 Hz → 1243:0070 1343:0070<br>0.5 → 632 Hz → 1243:0278 1343:0278<br>0.75 → 3557 Hz → 1243:0de5 1343:0de5<br>1 → 20000 Hz → 1243:4e20 1343:4e20<br>0.2007 → 80 Hz → 1243:0050 1343:0050 |
| output | AN 1/2 | eqQ1 | `1244` `1344` | 0 → 0.7 → 1244:0007 1344:0007<br>0.25 → 1.7 → 1244:0011 1344:0011<br>0.5 → 2.8 → 1244:001c 1344:001c<br>0.75 → 3.9 → 1244:0027 1344:0027<br>1 → 5.0 → 1244:0032 1344:0032<br>0.06977 → 1.0 → 1244:000a 1344:000a |
| output | AN 1/2 | eqGain2 | `1245` `1345` | 0 → -20.0 dB → 1245:ff38 1345:ff38<br>0.25 → -10.0 dB → 1245:ff9c 1345:ff9c<br>0.75 → 10.0 dB → 1245:0064 1345:0064<br>1 → 20.0 dB → 1245:00c8 1345:00c8<br>0.5 → 0.0 dB → 1245:0000 1345:0000 |
| output | AN 1/2 | eqFreq2 | `1246` `1346` | 0 → 20 Hz → 1246:0014 1346:0014<br>0.25 → 112 Hz → 1246:0070 1346:0070<br>0.5 → 632 Hz → 1246:0278 1346:0278<br>0.75 → 3557 Hz → 1246:0de5 1346:0de5<br>1 → 20000 Hz → 1246:4e20 1346:4e20<br>0.5663 → 1000 Hz → 1246:03e8 1346:03e8 |
| output | AN 1/2 | eqQ2 | `1247` `1347` | 0 → 0.7 → 1247:0007 1347:0007<br>0.25 → 1.7 → 1247:0011 1347:0011<br>0.5 → 2.8 → 1247:001c 1347:001c<br>0.75 → 3.9 → 1247:0027 1347:0027<br>1 → 5.0 → 1247:0032 1347:0032<br>0.06977 → 1.0 → 1247:000a 1347:000a |
| output | AN 1/2 | eqType3 | `1248` `1348` | 0 → Bell → 1248:0000 1348:0000<br>0.25 → Shelf → 1248:0001 1348:0001<br>0.75 → Low Pass → 1248:0002 1348:0002<br>1 → Low Pass → (none)<br>0.5 → Shelf → 1248:0001 1348:0001 |
| output | AN 1/2 | eqGain3 | `1249` `1349` | 0 → -20.0 dB → 1249:ff38 1349:ff38<br>0.25 → -10.0 dB → 1249:ff9c 1349:ff9c<br>0.75 → 10.0 dB → 1249:0064 1349:0064<br>1 → 20.0 dB → 1249:00c8 1349:00c8<br>0.5 → 0.0 dB → 1249:0000 1349:0000 |
| output | AN 1/2 | eqFreq3 | `124a` `134a` | 0 → 20 Hz → 124a:0014 134a:0014<br>0.25 → 112 Hz → 124a:0070 134a:0070<br>0.5 → 632 Hz → 124a:0278 134a:0278<br>0.75 → 3557 Hz → 124a:0de5 134a:0de5<br>1 → 20000 Hz → 124a:4e20 134a:4e20<br>0.7993 → 5000 Hz → 124a:1388 134a:1388 |
| output | AN 1/2 | eqQ3 | `124b` `134b` | 0 → 0.7 → 124b:0007 134b:0007<br>0.25 → 1.7 → 124b:0011 134b:0011<br>0.5 → 2.8 → 124b:001c 134b:001c<br>0.75 → 3.9 → 124b:0027 134b:0027<br>1 → 5.0 → 124b:0032 134b:0032<br>0.06977 → 1.0 → 124b:000a 134b:000a |
| output | AN 1/2 | eqEnable | `1240` `1340` | 1 → ? → 1240:0001 1340:0001<br>1 → ? → 1240:0000 1340:0000 |
| output | AN 1/2 | lowcutFreq | `1221` `1321` | 0.25 → 45 Hz → 1221:002d 1321:002d<br>0.5 → 100 Hz → 1221:0064 1321:0064<br>0.75 → 224 Hz → 1221:00e0 1321:00e0<br>1 → 500 Hz → 1221:01f4 1321:01f4<br>0 → 20 Hz → 1221:0014 1321:0014 |
| output | AN 1/2 | lowcutGrade | `1222` `1322` | 0 → 6 dB/oct → 1222:0000 1322:0000<br>0.25 → 12 dB/oct → 1222:0001 1322:0001<br>0.5 → 18 dB/oct → 1222:0002 1322:0002<br>0.75 → 18 dB/oct → (none)<br>1 → 24 dB/oct → 1222:0003 1322:0003<br>0.3333 → 12 dB/oct → 1222:0001 1322:0001 |
| output | AN 1/2 | lowcutEnable | `1220` `1320` | 1 → ? → 1220:0001 1320:0001<br>1 → ? → 1220:0000 1320:0000 |
| output | AN 1/2 | compexpGain | `1261` `1361` | 0 → -30.0 dB → 1261:fed4 1361:fed4<br>0.25 → -15.0 dB → 1261:ff6a 1361:ff6a<br>0.75 → 15.0 dB → 1261:0096 1361:0096<br>1 → 30.0 dB → 1261:012c 1361:012c<br>0.5 → 0.0 dB → 1261:0000 1361:0000 |
| output | AN 1/2 | compexpAttack | `1262` `1362` | 0 → 0 ms → 1262:0000 1362:0000<br>0.25 → 50 ms → 1262:0032 1362:0032<br>0.5 → 100 ms → 1262:0064 1362:0064<br>0.75 → 150 ms → 1262:0096 1362:0096<br>1 → 200 ms → 1262:00c8 1362:00c8<br>0.05 → 10 ms → 1262:000a 1362:000a |
| output | AN 1/2 | compexpRelease | `1263` `1363` | 0 → 100 ms → 1263:0064 1363:0064<br>0.25 → 325 ms → 1263:0145 1363:0145<br>0.5 → 550 ms → 1263:0226 1363:0226<br>0.75 → 774 ms → 1263:0306 1363:0306<br>1 → 999 ms → 1263:03e7 1363:03e7<br>0.2225 → 300 ms → 1263:012c 1363:012c |
| output | AN 1/2 | compTrsh | `1264` `1364` | 0 → -60.0 dB → 1264:fda8 1364:fda8<br>0.25 → -45.0 dB → 1264:fe3e 1364:fe3e<br>0.75 → -15.0 dB → 1264:ff6a 1364:ff6a<br>1 → 0.0 dB → 1264:0000 1364:0000<br>0.5 → -30.0 dB → 1264:fed4 1364:fed4 |
| output | AN 1/2 | compRatio | `1265` `1365` | 0.25 → 3.2 → 1265:0020 1365:0020<br>0.5 → 5.5 → 1265:0037 1365:0037<br>0.75 → 7.7 → 1265:004d 1365:004d<br>1 → 10.0 → 1265:0064 1365:0064<br>0 → 1.0 → 1265:000a 1365:000a |
| output | AN 1/2 | expTrsh | `1266` `1366` | 0 → -99.0 dB → 1266:fc22 1366:fc22<br>0.25 → -82.0 dB → 1266:fccc 1366:fccc<br>0.5 → -64.5 dB → 1266:fd7b 1366:fd7b<br>0.75 → -47.5 dB → 1266:fe25 1366:fe25<br>1 → -30.0 dB → 1266:fed4 1366:fed4<br>0.5652 → -60.0 dB → 1266:fda8 1366:fda8 |
| output | AN 1/2 | expRatio | `1267` `1367` | 0.25 → 3.2 → 1267:0020 1367:0020<br>0.5 → 5.5 → 1267:0037 1367:0037<br>0.75 → 7.7 → 1267:004d 1367:004d<br>1 → 10.0 → 1267:0064 1367:0064<br>0 → 1.0 → 1267:000a 1367:000a |
| output | AN 1/2 | compexpEnable | `1260` `1360` | 1 → ? → 1260:0001 1360:0001<br>1 → ? → 1260:0000 1360:0000 |
| output | AN 1/2 | alevMaxgain | `1281` `1381` | 0 → 0.0 dB → 1281:0000 1381:0000<br>0.25 → 4.5 dB → 1281:002d 1381:002d<br>0.5 → 9.0 dB → 1281:005a 1381:005a<br>0.75 → 13.5 dB → 1281:0087 1381:0087<br>1 → 18.0 dB → 1281:00b4 1381:00b4<br>0.3333 → 6.0 dB → 1281:003c 1381:003c |
| output | AN 1/2 | alevHeadroom | `1282` `1382` | 0 → 3.0 dB → 1282:001e 1382:001e<br>0.25 → 5.0 dB → 1282:0032 1382:0032<br>0.5 → 7.5 dB → 1282:004b 1382:004b<br>0.75 → 9.5 dB → 1282:005f 1382:005f<br>1 → 12.0 dB → 1282:0078 1382:0078<br>0.3333 → 6.0 dB → 1282:003c 1382:003c |
| output | AN 1/2 | alevRisetime | `1283` `1383` | 0 → 0.1 s → 1283:0001 1383:0001<br>0.25 → 2.5 s → 1283:0019 1383:0019<br>0.75 → 7.4 s → 1283:004a 1383:004a<br>1 → 9.9 s → 1283:0063 1383:0063<br>0.5 → 5.0 s → 1283:0032 1383:0032 |
| output | AN 3/4 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| output | AN 3/4 | eqGain1 | `1442` `1542` | 0.75 → 10.0 dB → 1442:0064 1542:0064<br>0.5 → 0.0 dB → 1442:0000 1542:0000 |
| output | AN 3/4 | eqFreq1 | `1443` `1543` | 0.75 → 3557 Hz → 1443:0de5 1543:0de5<br>0.2007 → 80 Hz → 1443:0050 1543:0050 |
| output | AN 3/4 | eqQ1 | `1444` `1544` | 0.75 → 3.9 → 1444:0027 1544:0027<br>0.06977 → 1.0 → 1444:000a 1544:000a |
| output | AN 3/4 | eqGain2 | `1445` `1545` | 0.75 → 10.0 dB → 1445:0064 1545:0064<br>0.5 → 0.0 dB → 1445:0000 1545:0000 |
| output | AN 3/4 | eqFreq2 | `1446` `1546` | 0.75 → 3557 Hz → 1446:0de5 1546:0de5<br>0.5663 → 1000 Hz → 1446:03e8 1546:03e8 |
| output | AN 3/4 | eqQ2 | `1447` `1547` | 0.75 → 3.9 → 1447:0027 1547:0027<br>0.06977 → 1.0 → 1447:000a 1547:000a |
| output | AN 3/4 | eqType3 | `1448` `1548` | 0.75 → Low Pass → 1448:0002 1548:0002<br>0.5 → Shelf → 1448:0001 1548:0001 |
| output | AN 3/4 | eqGain3 | `1449` `1549` | 0.75 → 10.0 dB → 1449:0064 1549:0064<br>0.5 → 0.0 dB → 1449:0000 1549:0000 |
| output | AN 3/4 | eqFreq3 | `144a` `154a` | 0.75 → 3557 Hz → 144a:0de5 154a:0de5<br>0.7993 → 5000 Hz → 144a:1388 154a:1388 |
| output | AN 3/4 | eqQ3 | `144b` `154b` | 0.75 → 3.9 → 144b:0027 154b:0027<br>0.06977 → 1.0 → 144b:000a 154b:000a |
| output | AN 3/4 | eqEnable | `1440` `1540` | 1 → ? → 1440:0001 1540:0001<br>1 → ? → 1440:0000 1540:0000 |
| output | AN 3/4 | lowcutFreq | `1421` `1521` | 0.75 → 224 Hz → 1421:00e0 1521:00e0<br>0 → 20 Hz → 1421:0014 1521:0014 |
| output | AN 3/4 | lowcutGrade | `1422` `1522` | 0.75 → 18 dB/oct → 1422:0002 1522:0002<br>0.3333 → 12 dB/oct → 1422:0001 1522:0001 |
| output | AN 3/4 | lowcutEnable | `1420` `1520` | 1 → ? → 1420:0001 1520:0001<br>1 → ? → 1420:0000 1520:0000 |
| output | AN 3/4 | compexpGain | `1461` `1561` | 0.75 → 15.0 dB → 1461:0096 1561:0096<br>0.5 → 0.0 dB → 1461:0000 1561:0000 |
| output | AN 3/4 | compexpAttack | `1462` `1562` | 0.75 → 150 ms → 1462:0096 1562:0096<br>0.05 → 10 ms → 1462:000a 1562:000a |
| output | AN 3/4 | compexpRelease | `1463` `1563` | 0.75 → 774 ms → 1463:0306 1563:0306<br>0.2225 → 300 ms → 1463:012c 1563:012c |
| output | AN 3/4 | compTrsh | `1464` `1564` | 0.75 → -15.0 dB → 1464:ff6a 1564:ff6a<br>0.5 → -30.0 dB → 1464:fed4 1564:fed4 |
| output | AN 3/4 | compRatio | `1465` `1565` | 0.75 → 7.7 → 1465:004d 1565:004d<br>0 → 1.0 → 1465:000a 1565:000a |
| output | AN 3/4 | expTrsh | `1466` `1566` | 0.75 → -47.5 dB → 1466:fe25 1566:fe25<br>0.5652 → -60.0 dB → 1466:fda8 1566:fda8 |
| output | AN 3/4 | expRatio | `1467` `1567` | 0.75 → 7.7 → 1467:004d 1567:004d<br>0 → 1.0 → 1467:000a 1567:000a |
| output | AN 3/4 | compexpEnable | `1460` `1560` | 1 → ? → 1460:0001 1560:0001<br>1 → ? → 1460:0000 1560:0000 |
| output | AN 3/4 | alevMaxgain | `1481` `1581` | 0.75 → 13.5 dB → 1481:0087 1581:0087<br>0.3333 → 6.0 dB → 1481:003c 1581:003c |
| output | AN 3/4 | alevHeadroom | `1482` `1582` | 0.75 → 9.5 dB → 1482:005f 1582:005f<br>0.3333 → 6.0 dB → 1482:003c 1582:003c |
| output | AN 3/4 | alevRisetime | `1483` `1583` | 0.75 → 7.4 s → 1483:004a 1583:004a<br>0.5 → 5.0 s → 1483:0032 1583:0032 |
| output | AN 5/6 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| output | AN 5/6 | eqGain1 | `1642` `1742` | 0.75 → 10.0 dB → 1642:0064 1742:0064<br>0.5 → 0.0 dB → 1642:0000 1742:0000 |
| output | AN 5/6 | eqFreq1 | `1643` `1743` | 0.75 → 3557 Hz → 1643:0de5 1743:0de5<br>0.2007 → 80 Hz → 1643:0050 1743:0050 |
| output | AN 5/6 | eqQ1 | `1644` `1744` | 0.75 → 3.9 → 1644:0027 1744:0027<br>0.06977 → 1.0 → 1644:000a 1744:000a |
| output | AN 5/6 | eqGain2 | `1645` `1745` | 0.75 → 10.0 dB → 1645:0064 1745:0064<br>0.5 → 0.0 dB → 1645:0000 1745:0000 |
| output | AN 5/6 | eqFreq2 | `1646` `1746` | 0.75 → 3557 Hz → 1646:0de5 1746:0de5<br>0.5663 → 1000 Hz → 1646:03e8 1746:03e8 |
| output | AN 5/6 | eqQ2 | `1647` `1747` | 0.75 → 3.9 → 1647:0027 1747:0027<br>0.06977 → 1.0 → 1647:000a 1747:000a |
| output | AN 5/6 | eqType3 | `1648` `1748` | 0.75 → Low Pass → 1648:0002 1748:0002<br>0.5 → Shelf → 1648:0001 1748:0001 |
| output | AN 5/6 | eqGain3 | `1649` `1749` | 0.75 → 10.0 dB → 1649:0064 1749:0064<br>0.5 → 0.0 dB → 1649:0000 1749:0000 |
| output | AN 5/6 | eqFreq3 | `164a` `174a` | 0.75 → 3557 Hz → 164a:0de5 174a:0de5<br>0.7993 → 5000 Hz → 164a:1388 174a:1388 |
| output | AN 5/6 | eqQ3 | `164b` `174b` | 0.75 → 3.9 → 164b:0027 174b:0027<br>0.06977 → 1.0 → 164b:000a 174b:000a |
| output | AN 5/6 | eqEnable | `1640` `1740` | 1 → ? → 1640:0001 1740:0001<br>1 → ? → 1640:0000 1740:0000 |
| output | AN 5/6 | lowcutFreq | `1621` `1721` | 0.75 → 224 Hz → 1621:00e0 1721:00e0<br>0 → 20 Hz → 1621:0014 1721:0014 |
| output | AN 5/6 | lowcutGrade | `1622` `1722` | 0.75 → 18 dB/oct → 1622:0002 1722:0002<br>0.3333 → 12 dB/oct → 1622:0001 1722:0001 |
| output | AN 5/6 | lowcutEnable | `1620` `1720` | 1 → ? → 1620:0001 1720:0001<br>1 → ? → 1620:0000 1720:0000 |
| output | AN 5/6 | compexpGain | `1661` `1761` | 0.75 → 15.0 dB → 1661:0096 1761:0096<br>0.5 → 0.0 dB → 1661:0000 1761:0000 |
| output | AN 5/6 | compexpAttack | `1662` `1762` | 0.75 → 150 ms → 1662:0096 1762:0096<br>0.05 → 10 ms → 1662:000a 1762:000a |
| output | AN 5/6 | compexpRelease | `1663` `1763` | 0.75 → 774 ms → 1663:0306 1763:0306<br>0.2225 → 300 ms → 1663:012c 1763:012c |
| output | AN 5/6 | compTrsh | `1664` `1764` | 0.75 → -15.0 dB → 1664:ff6a 1764:ff6a<br>0.5 → -30.0 dB → 1664:fed4 1764:fed4 |
| output | AN 5/6 | compRatio | `1665` `1765` | 0.75 → 7.7 → 1665:004d 1765:004d<br>0 → 1.0 → 1665:000a 1765:000a |
| output | AN 5/6 | expTrsh | `1666` `1766` | 0.75 → -47.5 dB → 1666:fe25 1766:fe25<br>0.5652 → -60.0 dB → 1666:fda8 1766:fda8 |
| output | AN 5/6 | expRatio | `1667` `1767` | 0.75 → 7.7 → 1667:004d 1767:004d<br>0 → 1.0 → 1667:000a 1767:000a |
| output | AN 5/6 | compexpEnable | `1660` `1760` | 1 → ? → 1660:0001 1760:0001<br>1 → ? → 1660:0000 1760:0000 |
| output | AN 5/6 | alevMaxgain | `1681` `1781` | 0.75 → 13.5 dB → 1681:0087 1781:0087<br>0.3333 → 6.0 dB → 1681:003c 1781:003c |
| output | AN 5/6 | alevHeadroom | `1682` `1782` | 0.75 → 9.5 dB → 1682:005f 1782:005f<br>0.3333 → 6.0 dB → 1682:003c 1782:003c |
| output | AN 5/6 | alevRisetime | `1683` `1783` | 0.75 → 7.4 s → 1683:004a 1783:004a<br>0.5 → 5.0 s → 1683:0032 1783:0032 |
| output | Main | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| output | Main | eqGain1 | `1842` `1942` | 0.75 → 10.0 dB → 1842:0064 1942:0064<br>0.5 → 0.0 dB → 1842:0000 1942:0000 |
| output | Main | eqFreq1 | `1843` `1943` | 0.75 → 3557 Hz → 1843:0de5 1943:0de5<br>0.2007 → 80 Hz → 1843:0050 1943:0050 |
| output | Main | eqQ1 | `1844` `1944` | 0.75 → 3.9 → 1844:0027 1944:0027<br>0.06977 → 1.0 → 1844:000a 1944:000a |
| output | Main | eqGain2 | `1845` `1945` | 0.75 → 10.0 dB → 1845:0064 1945:0064<br>0.5 → 0.0 dB → 1845:0000 1945:0000 |
| output | Main | eqFreq2 | `1846` `1946` | 0.75 → 3557 Hz → 1846:0de5 1946:0de5<br>0.5663 → 1000 Hz → 1846:03e8 1946:03e8 |
| output | Main | eqQ2 | `1847` `1947` | 0.75 → 3.9 → 1847:0027 1947:0027<br>0.06977 → 1.0 → 1847:000a 1947:000a |
| output | Main | eqType3 | `1848` `1948` | 0.75 → Low Pass → 1848:0002 1948:0002<br>0.5 → Shelf → 1848:0001 1948:0001 |
| output | Main | eqGain3 | `1849` `1949` | 0.75 → 10.0 dB → 1849:0064 1949:0064<br>0.5 → 0.0 dB → 1849:0000 1949:0000 |
| output | Main | eqFreq3 | `184a` `194a` | 0.75 → 3557 Hz → 184a:0de5 194a:0de5<br>0.7993 → 5000 Hz → 184a:1388 194a:1388 |
| output | Main | eqQ3 | `184b` `194b` | 0.75 → 3.9 → 184b:0027 194b:0027<br>0.06977 → 1.0 → 184b:000a 194b:000a |
| output | Main | eqEnable | `1840` `1940` | 1 → ? → 1840:0001 1940:0001<br>1 → ? → 1840:0000 1940:0000 |
| output | Main | lowcutFreq | `1821` `1921` | 0.75 → 224 Hz → 1821:00e0 1921:00e0<br>0 → 20 Hz → 1821:0014 1921:0014 |
| output | Main | lowcutGrade | `1822` `1922` | 0.75 → 18 dB/oct → 1822:0002 1922:0002<br>0.3333 → 12 dB/oct → 1822:0001 1922:0001 |
| output | Main | lowcutEnable | `1820` `1920` | 1 → ? → 1820:0001 1920:0001<br>1 → ? → 1820:0000 1920:0000 |
| output | Main | compexpGain | `1861` `1961` | 0.75 → 15.0 dB → 1861:0096 1961:0096<br>0.5 → 0.0 dB → 1861:0000 1961:0000 |
| output | Main | compexpAttack | `1862` `1962` | 0.75 → 150 ms → 1862:0096 1962:0096<br>0.05 → 10 ms → 1862:000a 1962:000a |
| output | Main | compexpRelease | `1863` `1963` | 0.75 → 774 ms → 1863:0306 1963:0306<br>0.2225 → 300 ms → 1863:012c 1963:012c |
| output | Main | compTrsh | `1864` `1964` | 0.75 → -15.0 dB → 1864:ff6a 1964:ff6a<br>0.5 → -30.0 dB → 1864:fed4 1964:fed4 |
| output | Main | compRatio | `1865` `1965` | 0.75 → 7.7 → 1865:004d 1965:004d<br>0 → 1.0 → 1865:000a 1965:000a |
| output | Main | expTrsh | `1866` `1966` | 0.75 → -47.5 dB → 1866:fe25 1966:fe25<br>0.5652 → -60.0 dB → 1866:fda8 1966:fda8 |
| output | Main | expRatio | `1867` `1967` | 0.75 → 7.7 → 1867:004d 1967:004d<br>0 → 1.0 → 1867:000a 1967:000a |
| output | Main | compexpEnable | `1860` `1960` | 1 → ? → 1860:0001 1960:0001<br>1 → ? → 1860:0000 1960:0000 |
| output | Main | alevMaxgain | `1881` `1981` | 0.75 → 13.5 dB → 1881:0087 1981:0087<br>0.3333 → 6.0 dB → 1881:003c 1981:003c |
| output | Main | alevHeadroom | `1882` `1982` | 0.75 → 9.5 dB → 1882:005f 1982:005f<br>0.3333 → 6.0 dB → 1882:003c 1982:003c |
| output | Main | alevRisetime | `1883` `1983` | 0.75 → 7.4 s → 1883:004a 1983:004a<br>0.5 → 5.0 s → 1883:0032 1983:0032 |
| output | SPDIF | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| output | SPDIF | eqGain1 | `1a42` `1b42` | 0.75 → 10.0 dB → 1a42:0064 1b42:0064<br>0.5 → 0.0 dB → 1a42:0000 1b42:0000 |
| output | SPDIF | eqFreq1 | `1a43` `1b43` | 0.75 → 3557 Hz → 1a43:0de5 1b43:0de5<br>0.2007 → 80 Hz → 1a43:0050 1b43:0050 |
| output | SPDIF | eqQ1 | `1a44` `1b44` | 0.75 → 3.9 → 1a44:0027 1b44:0027<br>0.06977 → 1.0 → 1a44:000a 1b44:000a |
| output | SPDIF | eqGain2 | `1a45` `1b45` | 0.75 → 10.0 dB → 1a45:0064 1b45:0064<br>0.5 → 0.0 dB → 1a45:0000 1b45:0000 |
| output | SPDIF | eqFreq2 | `1a46` `1b46` | 0.75 → 3557 Hz → 1a46:0de5 1b46:0de5<br>0.5663 → 1000 Hz → 1a46:03e8 1b46:03e8 |
| output | SPDIF | eqQ2 | `1a47` `1b47` | 0.75 → 3.9 → 1a47:0027 1b47:0027<br>0.06977 → 1.0 → 1a47:000a 1b47:000a |
| output | SPDIF | eqType3 | `1a48` `1b48` | 0.75 → Low Pass → 1a48:0002 1b48:0002<br>0.5 → Shelf → 1a48:0001 1b48:0001 |
| output | SPDIF | eqGain3 | `1a49` `1b49` | 0.75 → 10.0 dB → 1a49:0064 1b49:0064<br>0.5 → 0.0 dB → 1a49:0000 1b49:0000 |
| output | SPDIF | eqFreq3 | `1a4a` `1b4a` | 0.75 → 3557 Hz → 1a4a:0de5 1b4a:0de5<br>0.7993 → 5000 Hz → 1a4a:1388 1b4a:1388 |
| output | SPDIF | eqQ3 | `1a4b` `1b4b` | 0.75 → 3.9 → 1a4b:0027 1b4b:0027<br>0.06977 → 1.0 → 1a4b:000a 1b4b:000a |
| output | SPDIF | eqEnable | `1a40` `1b40` | 1 → ? → 1a40:0001 1b40:0001<br>1 → ? → 1a40:0000 1b40:0000 |
| output | SPDIF | lowcutFreq | `1a21` `1b21` | 0.75 → 224 Hz → 1a21:00e0 1b21:00e0<br>0 → 20 Hz → 1a21:0014 1b21:0014 |
| output | SPDIF | lowcutGrade | `1a22` `1b22` | 0.75 → 18 dB/oct → 1a22:0002 1b22:0002<br>0.3333 → 12 dB/oct → 1a22:0001 1b22:0001 |
| output | SPDIF | lowcutEnable | `1a20` `1b20` | 1 → ? → 1a20:0001 1b20:0001<br>1 → ? → 1a20:0000 1b20:0000 |
| output | SPDIF | compexpGain | `1a61` `1b61` | 0.75 → 15.0 dB → 1a61:0096 1b61:0096<br>0.5 → 0.0 dB → 1a61:0000 1b61:0000 |
| output | SPDIF | compexpAttack | `1a62` `1b62` | 0.75 → 150 ms → 1a62:0096 1b62:0096<br>0.05 → 10 ms → 1a62:000a 1b62:000a |
| output | SPDIF | compexpRelease | `1a63` `1b63` | 0.75 → 774 ms → 1a63:0306 1b63:0306<br>0.2225 → 300 ms → 1a63:012c 1b63:012c |
| output | SPDIF | compTrsh | `1a64` `1b64` | 0.75 → -15.0 dB → 1a64:ff6a 1b64:ff6a<br>0.5 → -30.0 dB → 1a64:fed4 1b64:fed4 |
| output | SPDIF | compRatio | `1a65` `1b65` | 0.75 → 7.7 → 1a65:004d 1b65:004d<br>0 → 1.0 → 1a65:000a 1b65:000a |
| output | SPDIF | expTrsh | `1a66` `1b66` | 0.75 → -47.5 dB → 1a66:fe25 1b66:fe25<br>0.5652 → -60.0 dB → 1a66:fda8 1b66:fda8 |
| output | SPDIF | expRatio | `1a67` `1b67` | 0.75 → 7.7 → 1a67:004d 1b67:004d<br>0 → 1.0 → 1a67:000a 1b67:000a |
| output | SPDIF | compexpEnable | `1a60` `1b60` | 1 → ? → 1a60:0001 1b60:0001<br>1 → ? → 1a60:0000 1b60:0000 |
| output | SPDIF | alevMaxgain | `1a81` `1b81` | 0.75 → 13.5 dB → 1a81:0087 1b81:0087<br>0.3333 → 6.0 dB → 1a81:003c 1b81:003c |
| output | SPDIF | alevHeadroom | `1a82` `1b82` | 0.75 → 9.5 dB → 1a82:005f 1b82:005f<br>0.3333 → 6.0 dB → 1a82:003c 1b82:003c |
| output | SPDIF | alevRisetime | `1a83` `1b83` | 0.75 → 7.4 s → 1a83:004a 1b83:004a<br>0.5 → 5.0 s → 1a83:0032 1b83:0032 |
| output | AS 1/2 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| output | AS 1/2 | eqGain1 | `1c42` `1d42` | 0.75 → 10.0 dB → 1c42:0064 1d42:0064<br>0.5 → 0.0 dB → 1c42:0000 1d42:0000 |
| output | AS 1/2 | eqFreq1 | `1c43` `1d43` | 0.75 → 3557 Hz → 1c43:0de5 1d43:0de5<br>0.2007 → 80 Hz → 1c43:0050 1d43:0050 |
| output | AS 1/2 | eqQ1 | `1c44` `1d44` | 0.75 → 3.9 → 1c44:0027 1d44:0027<br>0.06977 → 1.0 → 1c44:000a 1d44:000a |
| output | AS 1/2 | eqGain2 | `1c45` `1d45` | 0.75 → 10.0 dB → 1c45:0064 1d45:0064<br>0.5 → 0.0 dB → 1c45:0000 1d45:0000 |
| output | AS 1/2 | eqFreq2 | `1c46` `1d46` | 0.75 → 3557 Hz → 1c46:0de5 1d46:0de5<br>0.5663 → 1000 Hz → 1c46:03e8 1d46:03e8 |
| output | AS 1/2 | eqQ2 | `1c47` `1d47` | 0.75 → 3.9 → 1c47:0027 1d47:0027<br>0.06977 → 1.0 → 1c47:000a 1d47:000a |
| output | AS 1/2 | eqType3 | `1c48` `1d48` | 0.75 → Low Pass → 1c48:0002 1d48:0002<br>0.5 → Shelf → 1c48:0001 1d48:0001 |
| output | AS 1/2 | eqGain3 | `1c49` `1d49` | 0.75 → 10.0 dB → 1c49:0064 1d49:0064<br>0.5 → 0.0 dB → 1c49:0000 1d49:0000 |
| output | AS 1/2 | eqFreq3 | `1c4a` `1d4a` | 0.75 → 3557 Hz → 1c4a:0de5 1d4a:0de5<br>0.7993 → 5000 Hz → 1c4a:1388 1d4a:1388 |
| output | AS 1/2 | eqQ3 | `1c4b` `1d4b` | 0.75 → 3.9 → 1c4b:0027 1d4b:0027<br>0.06977 → 1.0 → 1c4b:000a 1d4b:000a |
| output | AS 1/2 | eqEnable | `1c40` `1d40` | 1 → ? → 1c40:0001 1d40:0001<br>1 → ? → 1c40:0000 1d40:0000 |
| output | AS 1/2 | lowcutFreq | `1c21` `1d21` | 0.75 → 224 Hz → 1c21:00e0 1d21:00e0<br>0 → 20 Hz → 1c21:0014 1d21:0014 |
| output | AS 1/2 | lowcutGrade | `1c22` `1d22` | 0.75 → 18 dB/oct → 1c22:0002 1d22:0002<br>0.3333 → 12 dB/oct → 1c22:0001 1d22:0001 |
| output | AS 1/2 | lowcutEnable | `1c20` `1d20` | 1 → ? → 1c20:0001 1d20:0001<br>1 → ? → 1c20:0000 1d20:0000 |
| output | AS 1/2 | compexpGain | `1c61` `1d61` | 0.75 → 15.0 dB → 1c61:0096 1d61:0096<br>0.5 → 0.0 dB → 1c61:0000 1d61:0000 |
| output | AS 1/2 | compexpAttack | `1c62` `1d62` | 0.75 → 150 ms → 1c62:0096 1d62:0096<br>0.05 → 10 ms → 1c62:000a 1d62:000a |
| output | AS 1/2 | compexpRelease | `1c63` `1d63` | 0.75 → 774 ms → 1c63:0306 1d63:0306<br>0.2225 → 300 ms → 1c63:012c 1d63:012c |
| output | AS 1/2 | compTrsh | `1c64` `1d64` | 0.75 → -15.0 dB → 1c64:ff6a 1d64:ff6a<br>0.5 → -30.0 dB → 1c64:fed4 1d64:fed4 |
| output | AS 1/2 | compRatio | `1c65` `1d65` | 0.75 → 7.7 → 1c65:004d 1d65:004d<br>0 → 1.0 → 1c65:000a 1d65:000a |
| output | AS 1/2 | expTrsh | `1c66` `1d66` | 0.75 → -47.5 dB → 1c66:fe25 1d66:fe25<br>0.5652 → -60.0 dB → 1c66:fda8 1d66:fda8 |
| output | AS 1/2 | expRatio | `1c67` `1d67` | 0.75 → 7.7 → 1c67:004d 1d67:004d<br>0 → 1.0 → 1c67:000a 1d67:000a |
| output | AS 1/2 | compexpEnable | `1c60` `1d60` | 1 → ? → 1c60:0001 1d60:0001<br>1 → ? → 1c60:0000 1d60:0000 |
| output | AS 1/2 | alevMaxgain | `1c81` `1d81` | 0.75 → 13.5 dB → 1c81:0087 1d81:0087<br>0.3333 → 6.0 dB → 1c81:003c 1d81:003c |
| output | AS 1/2 | alevHeadroom | `1c82` `1d82` | 0.75 → 9.5 dB → 1c82:005f 1d82:005f<br>0.3333 → 6.0 dB → 1c82:003c 1d82:003c |
| output | AS 1/2 | alevRisetime | `1c83` `1d83` | 0.75 → 7.4 s → 1c83:004a 1d83:004a<br>0.5 → 5.0 s → 1c83:0032 1d83:0032 |
| output | ADAT 3/4 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| output | ADAT 3/4 | eqGain1 | `1e42` `1f42` | 0.75 → 10.0 dB → 1e42:0064 1f42:0064<br>0.5 → 0.0 dB → 1e42:0000 1f42:0000 |
| output | ADAT 3/4 | eqFreq1 | `1e43` `1f43` | 0.75 → 3557 Hz → 1e43:0de5 1f43:0de5<br>0.2007 → 80 Hz → 1e43:0050 1f43:0050 |
| output | ADAT 3/4 | eqQ1 | `1e44` `1f44` | 0.75 → 3.9 → 1e44:0027 1f44:0027<br>0.06977 → 1.0 → 1e44:000a 1f44:000a |
| output | ADAT 3/4 | eqGain2 | `1e45` `1f45` | 0.75 → 10.0 dB → 1e45:0064 1f45:0064<br>0.5 → 0.0 dB → 1e45:0000 1f45:0000 |
| output | ADAT 3/4 | eqFreq2 | `1e46` `1f46` | 0.75 → 3557 Hz → 1e46:0de5 1f46:0de5<br>0.5663 → 1000 Hz → 1e46:03e8 1f46:03e8 |
| output | ADAT 3/4 | eqQ2 | `1e47` `1f47` | 0.75 → 3.9 → 1e47:0027 1f47:0027<br>0.06977 → 1.0 → 1e47:000a 1f47:000a |
| output | ADAT 3/4 | eqType3 | `1e48` `1f48` | 0.75 → Low Pass → 1e48:0002 1f48:0002<br>0.5 → Shelf → 1e48:0001 1f48:0001 |
| output | ADAT 3/4 | eqGain3 | `1e49` `1f49` | 0.75 → 10.0 dB → 1e49:0064 1f49:0064<br>0.5 → 0.0 dB → 1e49:0000 1f49:0000 |
| output | ADAT 3/4 | eqFreq3 | `1e4a` `1f4a` | 0.75 → 3557 Hz → 1e4a:0de5 1f4a:0de5<br>0.7993 → 5000 Hz → 1e4a:1388 1f4a:1388 |
| output | ADAT 3/4 | eqQ3 | `1e4b` `1f4b` | 0.75 → 3.9 → 1e4b:0027 1f4b:0027<br>0.06977 → 1.0 → 1e4b:000a 1f4b:000a |
| output | ADAT 3/4 | eqEnable | `1e40` `1f40` | 1 → ? → 1e40:0001 1f40:0001<br>1 → ? → 1e40:0000 1f40:0000 |
| output | ADAT 3/4 | lowcutFreq | `1e21` `1f21` | 0.75 → 224 Hz → 1e21:00e0 1f21:00e0<br>0 → 20 Hz → 1e21:0014 1f21:0014 |
| output | ADAT 3/4 | lowcutGrade | `1e22` `1f22` | 0.75 → 18 dB/oct → 1e22:0002 1f22:0002<br>0.3333 → 12 dB/oct → 1e22:0001 1f22:0001 |
| output | ADAT 3/4 | lowcutEnable | `1e20` `1f20` | 1 → ? → 1e20:0001 1f20:0001<br>1 → ? → 1e20:0000 1f20:0000 |
| output | ADAT 3/4 | compexpGain | `1e61` `1f61` | 0.75 → 15.0 dB → 1e61:0096 1f61:0096<br>0.5 → 0.0 dB → 1e61:0000 1f61:0000 |
| output | ADAT 3/4 | compexpAttack | `1e62` `1f62` | 0.75 → 150 ms → 1e62:0096 1f62:0096<br>0.05 → 10 ms → 1e62:000a 1f62:000a |
| output | ADAT 3/4 | compexpRelease | `1e63` `1f63` | 0.75 → 774 ms → 1e63:0306 1f63:0306<br>0.2225 → 300 ms → 1e63:012c 1f63:012c |
| output | ADAT 3/4 | compTrsh | `1e64` `1f64` | 0.75 → -15.0 dB → 1e64:ff6a 1f64:ff6a<br>0.5 → -30.0 dB → 1e64:fed4 1f64:fed4 |
| output | ADAT 3/4 | compRatio | `1e65` `1f65` | 0.75 → 7.7 → 1e65:004d 1f65:004d<br>0 → 1.0 → 1e65:000a 1f65:000a |
| output | ADAT 3/4 | expTrsh | `1e66` `1f66` | 0.75 → -47.5 dB → 1e66:fe25 1f66:fe25<br>0.5652 → -60.0 dB → 1e66:fda8 1f66:fda8 |
| output | ADAT 3/4 | expRatio | `1e67` `1f67` | 0.75 → 7.7 → 1e67:004d 1f67:004d<br>0 → 1.0 → 1e67:000a 1f67:000a |
| output | ADAT 3/4 | compexpEnable | `1e60` `1f60` | 1 → ? → 1e60:0001 1f60:0001<br>1 → ? → 1e60:0000 1f60:0000 |
| output | ADAT 3/4 | alevMaxgain | `1e81` `1f81` | 0.75 → 13.5 dB → 1e81:0087 1f81:0087<br>0.3333 → 6.0 dB → 1e81:003c 1f81:003c |
| output | ADAT 3/4 | alevHeadroom | `1e82` `1f82` | 0.75 → 9.5 dB → 1e82:005f 1f82:005f<br>0.3333 → 6.0 dB → 1e82:003c 1f82:003c |
| output | ADAT 3/4 | alevRisetime | `1e83` `1f83` | 0.75 → 7.4 s → 1e83:004a 1f83:004a<br>0.5 → 5.0 s → 1e83:0032 1f83:0032 |
| output | ADAT 5/6 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → Shelf → (none) |
| output | ADAT 5/6 | eqGain1 | `2042` `2142` | 0.75 → 10.0 dB → 2042:0064 2142:0064<br>0.5 → 0.0 dB → 2042:0000 2142:0000 |
| output | ADAT 5/6 | eqFreq1 | `2043` `2143` | 0.75 → 3557 Hz → 2043:0de5 2143:0de5<br>0.2007 → 80 Hz → 2043:0050 2143:0050 |
| output | ADAT 5/6 | eqQ1 | `2044` `2144` | 0.75 → 3.9 → 2044:0027 2144:0027<br>0.06977 → 1.0 → 2044:000a 2144:000a |
| output | ADAT 5/6 | eqGain2 | `2045` `2145` | 0.75 → 10.0 dB → 2045:0064 2145:0064<br>0.5 → 0.0 dB → 2045:0000 2145:0000 |
| output | ADAT 5/6 | eqFreq2 | `2046` `2146` | 0.75 → 3557 Hz → 2046:0de5 2146:0de5<br>0.5663 → 1000 Hz → 2046:03e8 2146:03e8 |
| output | ADAT 5/6 | eqQ2 | `2047` `2147` | 0.75 → 3.9 → 2047:0027 2147:0027<br>0.06977 → 1.0 → 2047:000a 2147:000a |
| output | ADAT 5/6 | eqType3 | `2048` `2148` | 0.75 → Low Pass → 2048:0002 2148:0002<br>0.5 → Shelf → 2048:0001 2148:0001 |
| output | ADAT 5/6 | eqGain3 | `2049` `2149` | 0.75 → 10.0 dB → 2049:0064 2149:0064<br>0.5 → 0.0 dB → 2049:0000 2149:0000 |
| output | ADAT 5/6 | eqFreq3 | `204a` `214a` | 0.75 → 3557 Hz → 204a:0de5 214a:0de5<br>0.7993 → 5000 Hz → 204a:1388 214a:1388 |
| output | ADAT 5/6 | eqQ3 | `204b` `214b` | 0.75 → 3.9 → 204b:0027 214b:0027<br>0.06977 → 1.0 → 204b:000a 214b:000a |
| output | ADAT 5/6 | eqEnable | `2040` `2140` | 1 → ? → 2040:0001 2140:0001<br>1 → ? → 2040:0000 2140:0000 |
| output | ADAT 5/6 | lowcutFreq | `2021` `2121` | 0.75 → 224 Hz → 2021:00e0 2121:00e0<br>0 → 20 Hz → 2021:0014 2121:0014 |
| output | ADAT 5/6 | lowcutGrade | `2022` `2122` | 0.75 → 18 dB/oct → 2022:0002 2122:0002<br>0.3333 → 12 dB/oct → 2022:0001 2122:0001 |
| output | ADAT 5/6 | lowcutEnable | `2020` `2120` | 1 → ? → 2020:0001 2120:0001<br>1 → ? → 2020:0000 2120:0000 |
| output | ADAT 5/6 | compexpGain | `2061` `2161` | 0.75 → 15.0 dB → 2061:0096 2161:0096<br>0.5 → 0.0 dB → 2061:0000 2161:0000 |
| output | ADAT 5/6 | compexpAttack | `2062` `2162` | 0.75 → 150 ms → 2062:0096 2162:0096<br>0.05 → 10 ms → 2062:000a 2162:000a |
| output | ADAT 5/6 | compexpRelease | `2063` `2163` | 0.75 → 774 ms → 2063:0306 2163:0306<br>0.2225 → 300 ms → 2063:012c 2163:012c |
| output | ADAT 5/6 | compTrsh | `2064` `2164` | 0.75 → -15.0 dB → 2064:ff6a 2164:ff6a<br>0.5 → -30.0 dB → 2064:fed4 2164:fed4 |
| output | ADAT 5/6 | compRatio | `2065` `2165` | 0.75 → 7.7 → 2065:004d 2165:004d<br>0 → 1.0 → 2065:000a 2165:000a |
| output | ADAT 5/6 | expTrsh | `2066` `2166` | 0.75 → -47.5 dB → 2066:fe25 2166:fe25<br>0.5652 → -60.0 dB → 2066:fda8 2166:fda8 |
| output | ADAT 5/6 | expRatio | `2067` `2167` | 0.75 → 7.7 → 2067:004d 2167:004d<br>0 → 1.0 → 2067:000a 2167:000a |
| output | ADAT 5/6 | compexpEnable | `2060` `2160` | 1 → ? → 2060:0001 2160:0001<br>1 → ? → 2060:0000 2160:0000 |
| output | ADAT 5/6 | alevMaxgain | `2081` `2181` | 0.75 → 13.5 dB → 2081:0087 2181:0087<br>0.3333 → 6.0 dB → 2081:003c 2181:003c |
| output | ADAT 5/6 | alevHeadroom | `2082` `2182` | 0.75 → 9.5 dB → 2082:005f 2182:005f<br>0.3333 → 6.0 dB → 2082:003c 2182:003c |
| output | ADAT 5/6 | alevRisetime | `2083` `2183` | 0.75 → 7.4 s → 2083:004a 2183:004a<br>0.5 → 5.0 s → 2083:0032 2183:0032 |
| output | ADAT 7/8 | eqType1 | none | 0.75 → Shelf → (none)<br>1 → ? → (none) |
| output | ADAT 7/8 | eqGain1 | `2242` `2342` | 0.75 → 10.0 dB → 2242:0064 2342:0064<br>0.5 → 0.0 dB → 2242:0000 2342:0000 |
| output | ADAT 7/8 | eqFreq1 | `2243` `2343` | 0.75 → 3557 Hz → 2243:0de5 2343:0de5<br>0.2007 → 80 Hz → 2243:0050 2343:0050 |
| output | ADAT 7/8 | eqQ1 | `2244` `2344` | 0.75 → 3.9 → 2244:0027 2344:0027<br>0.06977 → 1.0 → 2244:000a 2344:000a |
| output | ADAT 7/8 | eqGain2 | `2245` `2345` | 0.75 → 10.0 dB → 2245:0064 2345:0064<br>0.5 → 0.0 dB → 2245:0000 2345:0000 |
| output | ADAT 7/8 | eqFreq2 | `2246` `2346` | 0.75 → 3557 Hz → 2246:0de5 2346:0de5<br>0.5663 → 1000 Hz → 2246:03e8 2346:03e8 |
| output | ADAT 7/8 | eqQ2 | `2247` `2347` | 0.75 → 3.9 → 2247:0027 2347:0027<br>0.06977 → 1.0 → 2247:000a 2347:000a |
| output | ADAT 7/8 | eqType3 | `2248` `2348` | 0.75 → Low Pass → 2248:0002 2348:0002<br>0.5 → Shelf → 2248:0001 2348:0001 |
| output | ADAT 7/8 | eqGain3 | `2249` `2349` | 0.75 → 10.0 dB → 2249:0064 2349:0064<br>0.5 → 0.0 dB → 2249:0000 2349:0000 |
| output | ADAT 7/8 | eqFreq3 | `224a` `234a` | 0.75 → 3557 Hz → 224a:0de5 234a:0de5<br>0.7993 → 5000 Hz → 224a:1388 234a:1388 |
| output | ADAT 7/8 | eqQ3 | `224b` `234b` | 0.75 → 3.9 → 224b:0027 234b:0027<br>0.06977 → 1.0 → 224b:000a 234b:000a |
| output | ADAT 7/8 | eqEnable | `2240` `2340` | 1 → ? → 2240:0001 2340:0001<br>1 → ? → 2240:0000 2340:0000 |
| output | ADAT 7/8 | lowcutFreq | `2221` `2321` | 0.75 → 224 Hz → 2221:00e0 2321:00e0<br>0 → 20 Hz → 2221:0014 2321:0014 |
| output | ADAT 7/8 | lowcutGrade | `2222` `2322` | 0.75 → 18 dB/oct → 2222:0002 2322:0002<br>0.3333 → 12 dB/oct → 2222:0001 2322:0001 |
| output | ADAT 7/8 | lowcutEnable | `2220` `2320` | 1 → ? → 2220:0001 2320:0001<br>1 → ? → 2220:0000 2320:0000 |
| output | ADAT 7/8 | compexpGain | `2261` `2361` | 0.75 → 15.0 dB → 2261:0096 2361:0096<br>0.5 → 0.0 dB → 2261:0000 2361:0000 |
| output | ADAT 7/8 | compexpAttack | `2262` `2362` | 0.75 → 150 ms → 2262:0096 2362:0096<br>0.05 → 10 ms → 2262:000a 2362:000a |
| output | ADAT 7/8 | compexpRelease | `2263` `2363` | 0.75 → 774 ms → 2263:0306 2363:0306<br>0.2225 → 300 ms → 2263:012c 2363:012c |
| output | ADAT 7/8 | compTrsh | `2264` `2364` | 0.75 → -15.0 dB → 2264:ff6a 2364:ff6a<br>0.5 → -30.0 dB → 2264:fed4 2364:fed4 |
| output | ADAT 7/8 | compRatio | `2265` `2365` | 0.75 → 7.7 → 2265:004d 2365:004d<br>0 → 1.0 → 2265:000a 2365:000a |
| output | ADAT 7/8 | expTrsh | `2266` `2366` | 0.75 → -47.5 dB → 2266:fe25 2366:fe25<br>0.5652 → -60.0 dB → 2266:fda8 2366:fda8 |
| output | ADAT 7/8 | expRatio | `2267` `2367` | 0.75 → 7.7 → 2267:004d 2367:004d<br>0 → 1.0 → 2267:000a 2367:000a |
| output | ADAT 7/8 | compexpEnable | `2260` `2360` | 1 → ? → 2260:0001 2360:0001<br>1 → ? → 2260:0000 2360:0000 |
| output | ADAT 7/8 | alevMaxgain | `2281` `2381` | 0.75 → 13.5 dB → 2281:0087 2381:0087<br>0.3333 → 6.0 dB → 2281:003c 2381:003c |
| output | ADAT 7/8 | alevHeadroom | `2282` `2382` | 0.75 → 9.5 dB → 2282:005f 2382:005f<br>0.3333 → 6.0 dB → 2282:003c 2382:003c |
| output | ADAT 7/8 | alevRisetime | `2283` `2383` | 0.75 → 7.4 s → 2283:004a 2383:004a<br>0.5 → 5.0 s → 2283:0032 2383:0032 |

## Every change in order

| Row | Channel | Parameter | Kind | OSC value | Display | Writes |
|---|---|---|---|---|---|---|
| input | Mic 1 | eqType1 | sweep | 0 | Bell | 0041:0000 |
| input | Mic 1 | eqType1 | sweep | 0.25 | Bell | (none) |
| input | Mic 1 | eqType1 | sweep | 0.5 | Shelf | 0041:0001 |
| input | Mic 1 | eqType1 | sweep | 0.75 |  | (none) |
| input | Mic 1 | eqType1 | restore | 1 |  | (none) |
| input | Mic 1 | eqGain1 | sweep | 0 | -20.0 dB | 0042:ff38 |
| input | Mic 1 | eqGain1 | sweep | 0.25 | -10.0 dB | 0042:ff9c |
| input | Mic 1 | eqGain1 | sweep | 0.75 | 10.0 dB | 0042:0064 |
| input | Mic 1 | eqGain1 | sweep | 1 | 20.0 dB | 0042:00c8 |
| input | Mic 1 | eqGain1 | restore | 0.5 | 0.0 dB | 0042:0000 |
| input | Mic 1 | eqFreq1 | sweep | 0 | 20 Hz | 0043:0014 |
| input | Mic 1 | eqFreq1 | sweep | 0.25 | 112 Hz | 0043:0070 |
| input | Mic 1 | eqFreq1 | sweep | 0.5 | 632 Hz | 0043:0278 |
| input | Mic 1 | eqFreq1 | sweep | 0.75 | 3557 Hz | 0043:0de5 |
| input | Mic 1 | eqFreq1 | sweep | 1 | 20000 Hz | 0043:4e20 |
| input | Mic 1 | eqFreq1 | restore | 0.2007 | 80 Hz | 0043:0050 |
| input | Mic 1 | eqQ1 | sweep | 0 | 0.7 | 0044:0007 |
| input | Mic 1 | eqQ1 | sweep | 0.25 | 1.7 | 0044:0011 |
| input | Mic 1 | eqQ1 | sweep | 0.5 | 2.8 | 0044:001c |
| input | Mic 1 | eqQ1 | sweep | 0.75 | 3.9 | 0044:0027 |
| input | Mic 1 | eqQ1 | sweep | 1 | 5.0 | 0044:0032 |
| input | Mic 1 | eqQ1 | restore | 0.06977 | 1.0 | 0044:000a |
| input | Mic 1 | eqGain2 | sweep | 0 | -20.0 dB | 0045:ff38 |
| input | Mic 1 | eqGain2 | sweep | 0.25 | -10.0 dB | 0045:ff9c |
| input | Mic 1 | eqGain2 | sweep | 0.75 | 10.0 dB | 0045:0064 |
| input | Mic 1 | eqGain2 | sweep | 1 | 20.0 dB | 0045:00c8 |
| input | Mic 1 | eqGain2 | restore | 0.5 | 0.0 dB | 0045:0000 |
| input | Mic 1 | eqFreq2 | sweep | 0 | 20 Hz | 0046:0014 |
| input | Mic 1 | eqFreq2 | sweep | 0.25 | 112 Hz | 0046:0070 |
| input | Mic 1 | eqFreq2 | sweep | 0.5 | 632 Hz | 0046:0278 |
| input | Mic 1 | eqFreq2 | sweep | 0.75 | 3557 Hz | 0046:0de5 |
| input | Mic 1 | eqFreq2 | sweep | 1 | 20000 Hz | 0046:4e20 |
| input | Mic 1 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0046:03e8 |
| input | Mic 1 | eqQ2 | sweep | 0 | 0.7 | 0047:0007 |
| input | Mic 1 | eqQ2 | sweep | 0.25 | 1.7 | 0047:0011 |
| input | Mic 1 | eqQ2 | sweep | 0.5 | 2.8 | 0047:001c |
| input | Mic 1 | eqQ2 | sweep | 0.75 | 3.9 | 0047:0027 |
| input | Mic 1 | eqQ2 | sweep | 1 | 5.0 | 0047:0032 |
| input | Mic 1 | eqQ2 | restore | 0.06977 | 1.0 | 0047:000a |
| input | Mic 1 | eqType3 | sweep | 0 | Bell | 0048:0000 |
| input | Mic 1 | eqType3 | sweep | 0.25 | Shelf | 0048:0001 |
| input | Mic 1 | eqType3 | sweep | 0.75 | Low Pass | 0048:0002 |
| input | Mic 1 | eqType3 | sweep | 1 | Low Pass | (none) |
| input | Mic 1 | eqType3 | restore | 0.5 | Shelf | 0048:0001 |
| input | Mic 1 | eqGain3 | sweep | 0 | -20.0 dB | 0049:ff38 |
| input | Mic 1 | eqGain3 | sweep | 0.25 | -10.0 dB | 0049:ff9c |
| input | Mic 1 | eqGain3 | sweep | 0.75 | 10.0 dB | 0049:0064 |
| input | Mic 1 | eqGain3 | sweep | 1 | 20.0 dB | 0049:00c8 |
| input | Mic 1 | eqGain3 | restore | 0.5 | 0.0 dB | 0049:0000 |
| input | Mic 1 | eqFreq3 | sweep | 0 | 20 Hz | 004a:0014 |
| input | Mic 1 | eqFreq3 | sweep | 0.25 | 112 Hz | 004a:0070 |
| input | Mic 1 | eqFreq3 | sweep | 0.5 | 632 Hz | 004a:0278 |
| input | Mic 1 | eqFreq3 | sweep | 0.75 | 3557 Hz | 004a:0de5 |
| input | Mic 1 | eqFreq3 | sweep | 1 | 20000 Hz | 004a:4e20 |
| input | Mic 1 | eqFreq3 | restore | 0.7993 | 5000 Hz | 004a:1388 |
| input | Mic 1 | eqQ3 | sweep | 0 | 0.7 | 004b:0007 |
| input | Mic 1 | eqQ3 | sweep | 0.25 | 1.7 | 004b:0011 |
| input | Mic 1 | eqQ3 | sweep | 0.5 | 2.8 | 004b:001c |
| input | Mic 1 | eqQ3 | sweep | 0.75 | 3.9 | 004b:0027 |
| input | Mic 1 | eqQ3 | sweep | 1 | 5.0 | 004b:0032 |
| input | Mic 1 | eqQ3 | restore | 0.06977 | 1.0 | 004b:000a |
| input | Mic 1 | eqEnable | toggle on | 1 |  | 0040:0001 |
| input | Mic 1 | eqEnable | toggle off | 1 |  | 0040:0000 |
| input | Mic 1 | lowcutFreq | sweep | 0.25 | 45 Hz | 0021:002d |
| input | Mic 1 | lowcutFreq | sweep | 0.5 | 100 Hz | 0021:0064 |
| input | Mic 1 | lowcutFreq | sweep | 0.75 | 224 Hz | 0021:00e0 |
| input | Mic 1 | lowcutFreq | sweep | 1 | 500 Hz | 0021:01f4 |
| input | Mic 1 | lowcutFreq | restore | 0 | 20 Hz | 0021:0014 |
| input | Mic 1 | lowcutGrade | sweep | 0 | 6 dB/oct | 0022:0000 |
| input | Mic 1 | lowcutGrade | sweep | 0.25 | 12 dB/oct | 0022:0001 |
| input | Mic 1 | lowcutGrade | sweep | 0.5 | 18 dB/oct | 0022:0002 |
| input | Mic 1 | lowcutGrade | sweep | 0.75 | 18 dB/oct | (none) |
| input | Mic 1 | lowcutGrade | sweep | 1 | 24 dB/oct | 0022:0003 |
| input | Mic 1 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0022:0001 |
| input | Mic 1 | lowcutEnable | toggle on | 1 |  | 0020:0001 |
| input | Mic 1 | lowcutEnable | toggle off | 1 |  | 0020:0000 |
| input | Mic 1 | compexpGain | sweep | 0 | -30.0 dB | 0061:fed4 |
| input | Mic 1 | compexpGain | sweep | 0.25 | -15.0 dB | 0061:ff6a |
| input | Mic 1 | compexpGain | sweep | 0.75 | 15.0 dB | 0061:0096 |
| input | Mic 1 | compexpGain | sweep | 1 | 30.0 dB | 0061:012c |
| input | Mic 1 | compexpGain | restore | 0.5 | 0.0 dB | 0061:0000 |
| input | Mic 1 | compexpAttack | sweep | 0 | 0 ms | 0062:0000 |
| input | Mic 1 | compexpAttack | sweep | 0.25 | 50 ms | 0062:0032 |
| input | Mic 1 | compexpAttack | sweep | 0.5 | 100 ms | 0062:0064 |
| input | Mic 1 | compexpAttack | sweep | 0.75 | 150 ms | 0062:0096 |
| input | Mic 1 | compexpAttack | sweep | 1 | 200 ms | 0062:00c8 |
| input | Mic 1 | compexpAttack | restore | 0.05 | 10 ms | 0062:000a |
| input | Mic 1 | compexpRelease | sweep | 0 | 100 ms | 0063:0064 |
| input | Mic 1 | compexpRelease | sweep | 0.25 | 325 ms | 0063:0145 |
| input | Mic 1 | compexpRelease | sweep | 0.5 | 550 ms | 0063:0226 |
| input | Mic 1 | compexpRelease | sweep | 0.75 | 774 ms | 0063:0306 |
| input | Mic 1 | compexpRelease | sweep | 1 | 999 ms | 0063:03e7 |
| input | Mic 1 | compexpRelease | restore | 0.2225 | 300 ms | 0063:012c |
| input | Mic 1 | compTrsh | sweep | 0 | -60.0 dB | 0064:fda8 |
| input | Mic 1 | compTrsh | sweep | 0.25 | -45.0 dB | 0064:fe3e |
| input | Mic 1 | compTrsh | sweep | 0.75 | -15.0 dB | 0064:ff6a |
| input | Mic 1 | compTrsh | sweep | 1 | 0.0 dB | 0064:0000 |
| input | Mic 1 | compTrsh | restore | 0.5 | -30.0 dB | 0064:fed4 |
| input | Mic 1 | compRatio | sweep | 0.25 | 3.2 | 0065:0020 |
| input | Mic 1 | compRatio | sweep | 0.5 | 5.5 | 0065:0037 |
| input | Mic 1 | compRatio | sweep | 0.75 | 7.7 | 0065:004d |
| input | Mic 1 | compRatio | sweep | 1 | 10.0 | 0065:0064 |
| input | Mic 1 | compRatio | restore | 0 | 1.0 | 0065:000a |
| input | Mic 1 | expTrsh | sweep | 0 | -99.0 dB | 0066:fc22 |
| input | Mic 1 | expTrsh | sweep | 0.25 | -82.0 dB | 0066:fccc |
| input | Mic 1 | expTrsh | sweep | 0.5 | -64.5 dB | 0066:fd7b |
| input | Mic 1 | expTrsh | sweep | 0.75 | -47.5 dB | 0066:fe25 |
| input | Mic 1 | expTrsh | sweep | 1 | -30.0 dB | 0066:fed4 |
| input | Mic 1 | expTrsh | restore | 0.5652 | -60.0 dB | 0066:fda8 |
| input | Mic 1 | expRatio | sweep | 0.25 | 3.2 | 0067:0020 |
| input | Mic 1 | expRatio | sweep | 0.5 | 5.5 | 0067:0037 |
| input | Mic 1 | expRatio | sweep | 0.75 | 7.7 | 0067:004d |
| input | Mic 1 | expRatio | sweep | 1 | 10.0 | 0067:0064 |
| input | Mic 1 | expRatio | restore | 0 | 1.0 | 0067:000a |
| input | Mic 1 | compexpEnable | toggle on | 1 |  | 0060:0001 |
| input | Mic 1 | compexpEnable | toggle off | 1 |  | 0060:0000 |
| input | Mic 1 | alevMaxgain | sweep | 0 | 0.0 dB | 0081:0000 |
| input | Mic 1 | alevMaxgain | sweep | 0.25 | 4.5 dB | 0081:002d |
| input | Mic 1 | alevMaxgain | sweep | 0.5 | 9.0 dB | 0081:005a |
| input | Mic 1 | alevMaxgain | sweep | 0.75 | 13.5 dB | 0081:0087 |
| input | Mic 1 | alevMaxgain | sweep | 1 | 18.0 dB | 0081:00b4 |
| input | Mic 1 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0081:003c |
| input | Mic 1 | alevHeadroom | sweep | 0 | 3.0 dB | 0082:001e |
| input | Mic 1 | alevHeadroom | sweep | 0.25 | 5.0 dB | 0082:0032 |
| input | Mic 1 | alevHeadroom | sweep | 0.5 | 7.5 dB | 0082:004b |
| input | Mic 1 | alevHeadroom | sweep | 0.75 | 9.5 dB | 0082:005f |
| input | Mic 1 | alevHeadroom | sweep | 1 | 12.0 dB | 0082:0078 |
| input | Mic 1 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0082:003c |
| input | Mic 1 | alevRisetime | sweep | 0 | 0.1 s | 0083:0001 |
| input | Mic 1 | alevRisetime | sweep | 0.25 | 2.5 s | 0083:0019 |
| input | Mic 1 | alevRisetime | sweep | 0.75 | 7.4 s | 0083:004a |
| input | Mic 1 | alevRisetime | sweep | 1 | 9.9 s | 0083:0063 |
| input | Mic 1 | alevRisetime | restore | 0.5 | 5.0 s | 0083:0032 |
| input | Mic 2 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | Mic 2 | eqType1 | restore | 1 | Shelf | (none) |
| input | Mic 2 | eqGain1 | probe | 0.75 | 10.0 dB | 0142:0064 |
| input | Mic 2 | eqGain1 | restore | 0.5 | 0.0 dB | 0142:0000 |
| input | Mic 2 | eqFreq1 | probe | 0.75 | 3557 Hz | 0143:0de5 |
| input | Mic 2 | eqFreq1 | restore | 0.2007 | 80 Hz | 0143:0050 |
| input | Mic 2 | eqQ1 | probe | 0.75 | 3.9 | 0144:0027 |
| input | Mic 2 | eqQ1 | restore | 0.06977 | 1.0 | 0144:000a |
| input | Mic 2 | eqGain2 | probe | 0.75 | 10.0 dB | 0145:0064 |
| input | Mic 2 | eqGain2 | restore | 0.5 | 0.0 dB | 0145:0000 |
| input | Mic 2 | eqFreq2 | probe | 0.75 | 3557 Hz | 0146:0de5 |
| input | Mic 2 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0146:03e8 |
| input | Mic 2 | eqQ2 | probe | 0.75 | 3.9 | 0147:0027 |
| input | Mic 2 | eqQ2 | restore | 0.06977 | 1.0 | 0147:000a |
| input | Mic 2 | eqType3 | probe | 0.75 | Low Pass | 0148:0002 |
| input | Mic 2 | eqType3 | restore | 0.5 | Shelf | 0148:0001 |
| input | Mic 2 | eqGain3 | probe | 0.75 | 10.0 dB | 0149:0064 |
| input | Mic 2 | eqGain3 | restore | 0.5 | 0.0 dB | 0149:0000 |
| input | Mic 2 | eqFreq3 | probe | 0.75 | 3557 Hz | 014a:0de5 |
| input | Mic 2 | eqFreq3 | restore | 0.7993 | 5000 Hz | 014a:1388 |
| input | Mic 2 | eqQ3 | probe | 0.75 | 3.9 | 014b:0027 |
| input | Mic 2 | eqQ3 | restore | 0.06977 | 1.0 | 014b:000a |
| input | Mic 2 | eqEnable | toggle on | 1 |  | 0140:0001 |
| input | Mic 2 | eqEnable | toggle off | 1 |  | 0140:0000 |
| input | Mic 2 | lowcutFreq | probe | 0.75 | 224 Hz | 0121:00e0 |
| input | Mic 2 | lowcutFreq | restore | 0 | 20 Hz | 0121:0014 |
| input | Mic 2 | lowcutGrade | probe | 0.75 | 18 dB/oct | 0122:0002 |
| input | Mic 2 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0122:0001 |
| input | Mic 2 | lowcutEnable | toggle on | 1 |  | 0120:0001 |
| input | Mic 2 | lowcutEnable | toggle off | 1 |  | 0120:0000 |
| input | Mic 2 | compexpGain | probe | 0.75 | 15.0 dB | 0161:0096 |
| input | Mic 2 | compexpGain | restore | 0.5 | 0.0 dB | 0161:0000 |
| input | Mic 2 | compexpAttack | probe | 0.75 | 150 ms | 0162:0096 |
| input | Mic 2 | compexpAttack | restore | 0.05 | 10 ms | 0162:000a |
| input | Mic 2 | compexpRelease | probe | 0.75 | 774 ms | 0163:0306 |
| input | Mic 2 | compexpRelease | restore | 0.2225 | 300 ms | 0163:012c |
| input | Mic 2 | compTrsh | probe | 0.75 | -15.0 dB | 0164:ff6a |
| input | Mic 2 | compTrsh | restore | 0.5 | -30.0 dB | 0164:fed4 |
| input | Mic 2 | compRatio | probe | 0.75 | 7.7 | 0165:004d |
| input | Mic 2 | compRatio | restore | 0 | 1.0 | 0165:000a |
| input | Mic 2 | expTrsh | probe | 0.75 | -47.5 dB | 0166:fe25 |
| input | Mic 2 | expTrsh | restore | 0.5652 | -60.0 dB | 0166:fda8 |
| input | Mic 2 | expRatio | probe | 0.75 | 7.7 | 0167:004d |
| input | Mic 2 | expRatio | restore | 0 | 1.0 | 0167:000a |
| input | Mic 2 | compexpEnable | toggle on | 1 |  | 0160:0001 |
| input | Mic 2 | compexpEnable | toggle off | 1 |  | 0160:0000 |
| input | Mic 2 | alevMaxgain | probe | 0.75 | 13.5 dB | 0181:0087 |
| input | Mic 2 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0181:003c |
| input | Mic 2 | alevHeadroom | probe | 0.75 | 9.5 dB | 0182:005f |
| input | Mic 2 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0182:003c |
| input | Mic 2 | alevRisetime | probe | 0.75 | 7.4 s | 0183:004a |
| input | Mic 2 | alevRisetime | restore | 0.5 | 5.0 s | 0183:0032 |
| input | AN 3 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | AN 3 | eqType1 | restore | 1 | Shelf | (none) |
| input | AN 3 | eqGain1 | probe | 0.75 | 10.0 dB | 0242:0064 |
| input | AN 3 | eqGain1 | restore | 0.5 | 0.0 dB | 0242:0000 |
| input | AN 3 | eqFreq1 | probe | 0.75 | 3557 Hz | 0243:0de5 |
| input | AN 3 | eqFreq1 | restore | 0.2007 | 80 Hz | 0243:0050 |
| input | AN 3 | eqQ1 | probe | 0.75 | 3.9 | 0244:0027 |
| input | AN 3 | eqQ1 | restore | 0.06977 | 1.0 | 0244:000a |
| input | AN 3 | eqGain2 | probe | 0.75 | 10.0 dB | 0245:0064 |
| input | AN 3 | eqGain2 | restore | 0.5 | 0.0 dB | 0245:0000 |
| input | AN 3 | eqFreq2 | probe | 0.75 | 3557 Hz | 0246:0de5 |
| input | AN 3 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0246:03e8 |
| input | AN 3 | eqQ2 | probe | 0.75 | 3.9 | 0247:0027 |
| input | AN 3 | eqQ2 | restore | 0.06977 | 1.0 | 0247:000a |
| input | AN 3 | eqType3 | probe | 0.75 | Low Pass | 0248:0002 |
| input | AN 3 | eqType3 | restore | 0.5 | Shelf | 0248:0001 |
| input | AN 3 | eqGain3 | probe | 0.75 | 10.0 dB | 0249:0064 |
| input | AN 3 | eqGain3 | restore | 0.5 | 0.0 dB | 0249:0000 |
| input | AN 3 | eqFreq3 | probe | 0.75 | 3557 Hz | 024a:0de5 |
| input | AN 3 | eqFreq3 | restore | 0.7993 | 5000 Hz | 024a:1388 |
| input | AN 3 | eqQ3 | probe | 0.75 | 3.9 | 024b:0027 |
| input | AN 3 | eqQ3 | restore | 0.06977 | 1.0 | 024b:000a |
| input | AN 3 | eqEnable | toggle on | 1 |  | 0240:0001 |
| input | AN 3 | eqEnable | toggle off | 1 |  | 0240:0000 |
| input | AN 3 | lowcutFreq | probe | 0.75 | 224 Hz | 0221:00e0 |
| input | AN 3 | lowcutFreq | restore | 0 | 20 Hz | 0221:0014 |
| input | AN 3 | lowcutGrade | probe | 0.75 | 18 dB/oct | 0222:0002 |
| input | AN 3 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0222:0001 |
| input | AN 3 | lowcutEnable | toggle on | 1 |  | 0220:0001 |
| input | AN 3 | lowcutEnable | toggle off | 1 |  | 0220:0000 |
| input | AN 3 | compexpGain | probe | 0.75 | 15.0 dB | 0261:0096 |
| input | AN 3 | compexpGain | restore | 0.5 | 0.0 dB | 0261:0000 |
| input | AN 3 | compexpAttack | probe | 0.75 | 150 ms | 0262:0096 |
| input | AN 3 | compexpAttack | restore | 0.05 | 10 ms | 0262:000a |
| input | AN 3 | compexpRelease | probe | 0.75 | 774 ms | 0263:0306 |
| input | AN 3 | compexpRelease | restore | 0.2225 | 300 ms | 0263:012c |
| input | AN 3 | compTrsh | probe | 0.75 | -15.0 dB | 0264:ff6a |
| input | AN 3 | compTrsh | restore | 0.5 | -30.0 dB | 0264:fed4 |
| input | AN 3 | compRatio | probe | 0.75 | 7.7 | 0265:004d |
| input | AN 3 | compRatio | restore | 0 | 1.0 | 0265:000a |
| input | AN 3 | expTrsh | probe | 0.75 | -47.5 dB | 0266:fe25 |
| input | AN 3 | expTrsh | restore | 0.5652 | -60.0 dB | 0266:fda8 |
| input | AN 3 | expRatio | probe | 0.75 | 7.7 | 0267:004d |
| input | AN 3 | expRatio | restore | 0 | 1.0 | 0267:000a |
| input | AN 3 | compexpEnable | toggle on | 1 |  | 0260:0001 |
| input | AN 3 | compexpEnable | toggle off | 1 |  | 0260:0000 |
| input | AN 3 | alevMaxgain | probe | 0.75 | 13.5 dB | 0281:0087 |
| input | AN 3 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0281:003c |
| input | AN 3 | alevHeadroom | probe | 0.75 | 9.5 dB | 0282:005f |
| input | AN 3 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0282:003c |
| input | AN 3 | alevRisetime | probe | 0.75 | 7.4 s | 0283:004a |
| input | AN 3 | alevRisetime | restore | 0.5 | 5.0 s | 0283:0032 |
| input | AN 4 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | AN 4 | eqType1 | restore | 1 | Shelf | (none) |
| input | AN 4 | eqGain1 | probe | 0.75 | 10.0 dB | 0342:0064 |
| input | AN 4 | eqGain1 | restore | 0.5 | 0.0 dB | 0342:0000 |
| input | AN 4 | eqFreq1 | probe | 0.75 | 3557 Hz | 0343:0de5 |
| input | AN 4 | eqFreq1 | restore | 0.2007 | 80 Hz | 0343:0050 |
| input | AN 4 | eqQ1 | probe | 0.75 | 3.9 | 0344:0027 |
| input | AN 4 | eqQ1 | restore | 0.06977 | 1.0 | 0344:000a |
| input | AN 4 | eqGain2 | probe | 0.75 | 10.0 dB | 0345:0064 |
| input | AN 4 | eqGain2 | restore | 0.5 | 0.0 dB | 0345:0000 |
| input | AN 4 | eqFreq2 | probe | 0.75 | 3557 Hz | 0346:0de5 |
| input | AN 4 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0346:03e8 |
| input | AN 4 | eqQ2 | probe | 0.75 | 3.9 | 0347:0027 |
| input | AN 4 | eqQ2 | restore | 0.06977 | 1.0 | 0347:000a |
| input | AN 4 | eqType3 | probe | 0.75 | Low Pass | 0348:0002 |
| input | AN 4 | eqType3 | restore | 0.5 | Shelf | 0348:0001 |
| input | AN 4 | eqGain3 | probe | 0.75 | 10.0 dB | 0349:0064 |
| input | AN 4 | eqGain3 | restore | 0.5 | 0.0 dB | 0349:0000 |
| input | AN 4 | eqFreq3 | probe | 0.75 | 3557 Hz | 034a:0de5 |
| input | AN 4 | eqFreq3 | restore | 0.7993 | 5000 Hz | 034a:1388 |
| input | AN 4 | eqQ3 | probe | 0.75 | 3.9 | 034b:0027 |
| input | AN 4 | eqQ3 | restore | 0.06977 | 1.0 | 034b:000a |
| input | AN 4 | eqEnable | toggle on | 1 |  | 0340:0001 |
| input | AN 4 | eqEnable | toggle off | 1 |  | 0340:0000 |
| input | AN 4 | lowcutFreq | probe | 0.75 | 224 Hz | 0321:00e0 |
| input | AN 4 | lowcutFreq | restore | 0 | 20 Hz | 0321:0014 |
| input | AN 4 | lowcutGrade | probe | 0.75 | 18 dB/oct | 0322:0002 |
| input | AN 4 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0322:0001 |
| input | AN 4 | lowcutEnable | toggle on | 1 |  | 0320:0001 |
| input | AN 4 | lowcutEnable | toggle off | 1 |  | 0320:0000 |
| input | AN 4 | compexpGain | probe | 0.75 | 15.0 dB | 0361:0096 |
| input | AN 4 | compexpGain | restore | 0.5 | 0.0 dB | 0361:0000 |
| input | AN 4 | compexpAttack | probe | 0.75 | 150 ms | 0362:0096 |
| input | AN 4 | compexpAttack | restore | 0.05 | 10 ms | 0362:000a |
| input | AN 4 | compexpRelease | probe | 0.75 | 774 ms | 0363:0306 |
| input | AN 4 | compexpRelease | restore | 0.2225 | 300 ms | 0363:012c |
| input | AN 4 | compTrsh | probe | 0.75 | -15.0 dB | 0364:ff6a |
| input | AN 4 | compTrsh | restore | 0.5 | -30.0 dB | 0364:fed4 |
| input | AN 4 | compRatio | probe | 0.75 | 7.7 | 0365:004d |
| input | AN 4 | compRatio | restore | 0 | 1.0 | 0365:000a |
| input | AN 4 | expTrsh | probe | 0.75 | -47.5 dB | 0366:fe25 |
| input | AN 4 | expTrsh | restore | 0.5652 | -60.0 dB | 0366:fda8 |
| input | AN 4 | expRatio | probe | 0.75 | 7.7 | 0367:004d |
| input | AN 4 | expRatio | restore | 0 | 1.0 | 0367:000a |
| input | AN 4 | compexpEnable | toggle on | 1 |  | 0360:0001 |
| input | AN 4 | compexpEnable | toggle off | 1 |  | 0360:0000 |
| input | AN 4 | alevMaxgain | probe | 0.75 | 13.5 dB | 0381:0087 |
| input | AN 4 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0381:003c |
| input | AN 4 | alevHeadroom | probe | 0.75 | 9.5 dB | 0382:005f |
| input | AN 4 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0382:003c |
| input | AN 4 | alevRisetime | probe | 0.75 | 7.4 s | 0383:004a |
| input | AN 4 | alevRisetime | restore | 0.5 | 5.0 s | 0383:0032 |
| input | AN 5/6 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | AN 5/6 | eqType1 | restore | 1 | Shelf | (none) |
| input | AN 5/6 | eqGain1 | probe | 0.75 | 10.0 dB | 0442:0064 0542:0064 |
| input | AN 5/6 | eqGain1 | restore | 0.5 | 0.0 dB | 0442:0000 0542:0000 |
| input | AN 5/6 | eqFreq1 | probe | 0.75 | 3557 Hz | 0443:0de5 0543:0de5 |
| input | AN 5/6 | eqFreq1 | restore | 0.2007 | 80 Hz | 0443:0050 0543:0050 |
| input | AN 5/6 | eqQ1 | probe | 0.75 | 3.9 | 0444:0027 0544:0027 |
| input | AN 5/6 | eqQ1 | restore | 0.06977 | 1.0 | 0444:000a 0544:000a |
| input | AN 5/6 | eqGain2 | probe | 0.75 | 10.0 dB | 0445:0064 0545:0064 |
| input | AN 5/6 | eqGain2 | restore | 0.5 | 0.0 dB | 0445:0000 0545:0000 |
| input | AN 5/6 | eqFreq2 | probe | 0.75 | 3557 Hz | 0446:0de5 0546:0de5 |
| input | AN 5/6 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0446:03e8 0546:03e8 |
| input | AN 5/6 | eqQ2 | probe | 0.75 | 3.9 | 0447:0027 0547:0027 |
| input | AN 5/6 | eqQ2 | restore | 0.06977 | 1.0 | 0447:000a 0547:000a |
| input | AN 5/6 | eqType3 | probe | 0.75 | Low Pass | 0448:0002 0548:0002 |
| input | AN 5/6 | eqType3 | restore | 0.5 | Shelf | 0448:0001 0548:0001 |
| input | AN 5/6 | eqGain3 | probe | 0.75 | 10.0 dB | 0449:0064 0549:0064 |
| input | AN 5/6 | eqGain3 | restore | 0.5 | 0.0 dB | 0449:0000 0549:0000 |
| input | AN 5/6 | eqFreq3 | probe | 0.75 | 3557 Hz | 044a:0de5 054a:0de5 |
| input | AN 5/6 | eqFreq3 | restore | 0.7993 | 5000 Hz | 044a:1388 054a:1388 |
| input | AN 5/6 | eqQ3 | probe | 0.75 | 3.9 | 044b:0027 054b:0027 |
| input | AN 5/6 | eqQ3 | restore | 0.06977 | 1.0 | 044b:000a 054b:000a |
| input | AN 5/6 | eqEnable | toggle on | 1 |  | 0440:0001 0540:0001 |
| input | AN 5/6 | eqEnable | toggle off | 1 |  | 0440:0000 0540:0000 |
| input | AN 5/6 | lowcutFreq | probe | 0.75 | 224 Hz | 0421:00e0 0521:00e0 |
| input | AN 5/6 | lowcutFreq | restore | 0 | 20 Hz | 0421:0014 0521:0014 |
| input | AN 5/6 | lowcutGrade | probe | 0.75 | 18 dB/oct | 0422:0002 0522:0002 |
| input | AN 5/6 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0422:0001 0522:0001 |
| input | AN 5/6 | lowcutEnable | toggle on | 1 |  | 0420:0001 0520:0001 |
| input | AN 5/6 | lowcutEnable | toggle off | 1 |  | 0420:0000 0520:0000 |
| input | AN 5/6 | compexpGain | probe | 0.75 | 15.0 dB | 0461:0096 0561:0096 |
| input | AN 5/6 | compexpGain | restore | 0.5 | 0.0 dB | 0461:0000 0561:0000 |
| input | AN 5/6 | compexpAttack | probe | 0.75 | 150 ms | 0462:0096 0562:0096 |
| input | AN 5/6 | compexpAttack | restore | 0.05 | 10 ms | 0462:000a 0562:000a |
| input | AN 5/6 | compexpRelease | probe | 0.75 | 774 ms | 0463:0306 0563:0306 |
| input | AN 5/6 | compexpRelease | restore | 0.2225 | 300 ms | 0463:012c 0563:012c |
| input | AN 5/6 | compTrsh | probe | 0.75 | -15.0 dB | 0464:ff6a 0564:ff6a |
| input | AN 5/6 | compTrsh | restore | 0.5 | -30.0 dB | 0464:fed4 0564:fed4 |
| input | AN 5/6 | compRatio | probe | 0.75 | 7.7 | 0465:004d 0565:004d |
| input | AN 5/6 | compRatio | restore | 0 | 1.0 | 0465:000a 0565:000a |
| input | AN 5/6 | expTrsh | probe | 0.75 | -47.5 dB | 0466:fe25 0566:fe25 |
| input | AN 5/6 | expTrsh | restore | 0.5652 | -60.0 dB | 0466:fda8 0566:fda8 |
| input | AN 5/6 | expRatio | probe | 0.75 | 7.7 | 0467:004d 0567:004d |
| input | AN 5/6 | expRatio | restore | 0 | 1.0 | 0467:000a 0567:000a |
| input | AN 5/6 | compexpEnable | toggle on | 1 |  | 0460:0001 0560:0001 |
| input | AN 5/6 | compexpEnable | toggle off | 1 |  | 0460:0000 0560:0000 |
| input | AN 5/6 | alevMaxgain | probe | 0.75 | 13.5 dB | 0481:0087 0581:0087 |
| input | AN 5/6 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0481:003c 0581:003c |
| input | AN 5/6 | alevHeadroom | probe | 0.75 | 9.5 dB | 0482:005f 0582:005f |
| input | AN 5/6 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0482:003c 0582:003c |
| input | AN 5/6 | alevRisetime | probe | 0.75 | 7.4 s | 0483:004a 0583:004a |
| input | AN 5/6 | alevRisetime | restore | 0.5 | 5.0 s | 0483:0032 0583:0032 |
| input | AN 7/8 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | AN 7/8 | eqType1 | restore | 1 | Shelf | (none) |
| input | AN 7/8 | eqGain1 | probe | 0.75 | 10.0 dB | 0642:0064 0742:0064 |
| input | AN 7/8 | eqGain1 | restore | 0.5 | 0.0 dB | 0642:0000 0742:0000 |
| input | AN 7/8 | eqFreq1 | probe | 0.75 | 3557 Hz | 0643:0de5 0743:0de5 |
| input | AN 7/8 | eqFreq1 | restore | 0.2007 | 80 Hz | 0643:0050 0743:0050 |
| input | AN 7/8 | eqQ1 | probe | 0.75 | 3.9 | 0644:0027 0744:0027 |
| input | AN 7/8 | eqQ1 | restore | 0.06977 | 1.0 | 0644:000a 0744:000a |
| input | AN 7/8 | eqGain2 | probe | 0.75 | 10.0 dB | 0645:0064 0745:0064 |
| input | AN 7/8 | eqGain2 | restore | 0.5 | 0.0 dB | 0645:0000 0745:0000 |
| input | AN 7/8 | eqFreq2 | probe | 0.75 | 3557 Hz | 0646:0de5 0746:0de5 |
| input | AN 7/8 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0646:03e8 0746:03e8 |
| input | AN 7/8 | eqQ2 | probe | 0.75 | 3.9 | 0647:0027 0747:0027 |
| input | AN 7/8 | eqQ2 | restore | 0.06977 | 1.0 | 0647:000a 0747:000a |
| input | AN 7/8 | eqType3 | probe | 0.75 | Low Pass | 0648:0002 0748:0002 |
| input | AN 7/8 | eqType3 | restore | 0.5 | Shelf | 0648:0001 0748:0001 |
| input | AN 7/8 | eqGain3 | probe | 0.75 | 10.0 dB | 0649:0064 0749:0064 |
| input | AN 7/8 | eqGain3 | restore | 0.5 | 0.0 dB | 0649:0000 0749:0000 |
| input | AN 7/8 | eqFreq3 | probe | 0.75 | 3557 Hz | 064a:0de5 074a:0de5 |
| input | AN 7/8 | eqFreq3 | restore | 0.7993 | 5000 Hz | 064a:1388 074a:1388 |
| input | AN 7/8 | eqQ3 | probe | 0.75 | 3.9 | 064b:0027 074b:0027 |
| input | AN 7/8 | eqQ3 | restore | 0.06977 | 1.0 | 064b:000a 074b:000a |
| input | AN 7/8 | eqEnable | toggle on | 1 |  | 0640:0001 0740:0001 |
| input | AN 7/8 | eqEnable | toggle off | 1 |  | 0640:0000 0740:0000 |
| input | AN 7/8 | lowcutFreq | probe | 0.75 | 224 Hz | 0621:00e0 0721:00e0 |
| input | AN 7/8 | lowcutFreq | restore | 0 | 20 Hz | 0621:0014 0721:0014 |
| input | AN 7/8 | lowcutGrade | probe | 0.75 | 18 dB/oct | 0622:0002 0722:0002 |
| input | AN 7/8 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0622:0001 0722:0001 |
| input | AN 7/8 | lowcutEnable | toggle on | 1 |  | 0620:0001 0720:0001 |
| input | AN 7/8 | lowcutEnable | toggle off | 1 |  | 0620:0000 0720:0000 |
| input | AN 7/8 | compexpGain | probe | 0.75 | 15.0 dB | 0661:0096 0761:0096 |
| input | AN 7/8 | compexpGain | restore | 0.5 | 0.0 dB | 0661:0000 0761:0000 |
| input | AN 7/8 | compexpAttack | probe | 0.75 | 150 ms | 0662:0096 0762:0096 |
| input | AN 7/8 | compexpAttack | restore | 0.05 | 10 ms | 0662:000a 0762:000a |
| input | AN 7/8 | compexpRelease | probe | 0.75 | 774 ms | 0663:0306 0763:0306 |
| input | AN 7/8 | compexpRelease | restore | 0.2225 | 300 ms | 0663:012c 0763:012c |
| input | AN 7/8 | compTrsh | probe | 0.75 | -15.0 dB | 0664:ff6a 0764:ff6a |
| input | AN 7/8 | compTrsh | restore | 0.5 | -30.0 dB | 0664:fed4 0764:fed4 |
| input | AN 7/8 | compRatio | probe | 0.75 | 7.7 | 0665:004d 0765:004d |
| input | AN 7/8 | compRatio | restore | 0 | 1.0 | 0665:000a 0765:000a |
| input | AN 7/8 | expTrsh | probe | 0.75 | -47.5 dB | 0666:fe25 0766:fe25 |
| input | AN 7/8 | expTrsh | restore | 0.5652 | -60.0 dB | 0666:fda8 0766:fda8 |
| input | AN 7/8 | expRatio | probe | 0.75 | 7.7 | 0667:004d 0767:004d |
| input | AN 7/8 | expRatio | restore | 0 | 1.0 | 0667:000a 0767:000a |
| input | AN 7/8 | compexpEnable | toggle on | 1 |  | 0660:0001 0760:0001 |
| input | AN 7/8 | compexpEnable | toggle off | 1 |  | 0660:0000 0760:0000 |
| input | AN 7/8 | alevMaxgain | probe | 0.75 | 13.5 dB | 0681:0087 0781:0087 |
| input | AN 7/8 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0681:003c 0781:003c |
| input | AN 7/8 | alevHeadroom | probe | 0.75 | 9.5 dB | 0682:005f 0782:005f |
| input | AN 7/8 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0682:003c 0782:003c |
| input | AN 7/8 | alevRisetime | probe | 0.75 | 7.4 s | 0683:004a 0783:004a |
| input | AN 7/8 | alevRisetime | restore | 0.5 | 5.0 s | 0683:0032 0783:0032 |
| input | SPDIF | eqType1 | probe | 0.75 | Shelf | (none) |
| input | SPDIF | eqType1 | restore | 1 | Shelf | (none) |
| input | SPDIF | eqGain1 | probe | 0.75 | 10.0 dB | 0842:0064 0942:0064 |
| input | SPDIF | eqGain1 | restore | 0.5 | 0.0 dB | 0842:0000 0942:0000 |
| input | SPDIF | eqFreq1 | probe | 0.75 | 3557 Hz | 0843:0de5 0943:0de5 |
| input | SPDIF | eqFreq1 | restore | 0.2007 | 80 Hz | 0843:0050 0943:0050 |
| input | SPDIF | eqQ1 | probe | 0.75 | 3.9 | 0844:0027 0944:0027 |
| input | SPDIF | eqQ1 | restore | 0.06977 | 1.0 | 0844:000a 0944:000a |
| input | SPDIF | eqGain2 | probe | 0.75 | 10.0 dB | 0845:0064 0945:0064 |
| input | SPDIF | eqGain2 | restore | 0.5 | 0.0 dB | 0845:0000 0945:0000 |
| input | SPDIF | eqFreq2 | probe | 0.75 | 3557 Hz | 0846:0de5 0946:0de5 |
| input | SPDIF | eqFreq2 | restore | 0.5663 | 1000 Hz | 0846:03e8 0946:03e8 |
| input | SPDIF | eqQ2 | probe | 0.75 | 3.9 | 0847:0027 0947:0027 |
| input | SPDIF | eqQ2 | restore | 0.06977 | 1.0 | 0847:000a 0947:000a |
| input | SPDIF | eqType3 | probe | 0.75 | Low Pass | 0848:0002 0948:0002 |
| input | SPDIF | eqType3 | restore | 0.5 | Shelf | 0848:0001 0948:0001 |
| input | SPDIF | eqGain3 | probe | 0.75 | 10.0 dB | 0849:0064 0949:0064 |
| input | SPDIF | eqGain3 | restore | 0.5 | 0.0 dB | 0849:0000 0949:0000 |
| input | SPDIF | eqFreq3 | probe | 0.75 | 3557 Hz | 084a:0de5 094a:0de5 |
| input | SPDIF | eqFreq3 | restore | 0.7993 | 5000 Hz | 084a:1388 094a:1388 |
| input | SPDIF | eqQ3 | probe | 0.75 | 3.9 | 084b:0027 094b:0027 |
| input | SPDIF | eqQ3 | restore | 0.06977 | 1.0 | 084b:000a 094b:000a |
| input | SPDIF | eqEnable | toggle on | 1 |  | 0840:0001 0940:0001 |
| input | SPDIF | eqEnable | toggle off | 1 |  | 0840:0000 0940:0000 |
| input | SPDIF | lowcutFreq | probe | 0.75 | 224 Hz | 0821:00e0 0921:00e0 |
| input | SPDIF | lowcutFreq | restore | 0 | 20 Hz | 0821:0014 0921:0014 |
| input | SPDIF | lowcutGrade | probe | 0.75 | 18 dB/oct | 0822:0002 0922:0002 |
| input | SPDIF | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0822:0001 0922:0001 |
| input | SPDIF | lowcutEnable | toggle on | 1 |  | 0820:0001 0920:0001 |
| input | SPDIF | lowcutEnable | toggle off | 1 |  | 0820:0000 0920:0000 |
| input | SPDIF | compexpGain | probe | 0.75 | 15.0 dB | 0861:0096 0961:0096 |
| input | SPDIF | compexpGain | restore | 0.5 | 0.0 dB | 0861:0000 0961:0000 |
| input | SPDIF | compexpAttack | probe | 0.75 | 150 ms | 0862:0096 0962:0096 |
| input | SPDIF | compexpAttack | restore | 0.05 | 10 ms | 0862:000a 0962:000a |
| input | SPDIF | compexpRelease | probe | 0.75 | 774 ms | 0863:0306 0963:0306 |
| input | SPDIF | compexpRelease | restore | 0.2225 | 300 ms | 0863:012c 0963:012c |
| input | SPDIF | compTrsh | probe | 0.75 | -15.0 dB | 0864:ff6a 0964:ff6a |
| input | SPDIF | compTrsh | restore | 0.5 | -30.0 dB | 0864:fed4 0964:fed4 |
| input | SPDIF | compRatio | probe | 0.75 | 7.7 | 0865:004d 0965:004d |
| input | SPDIF | compRatio | restore | 0 | 1.0 | 0865:000a 0965:000a |
| input | SPDIF | expTrsh | probe | 0.75 | -47.5 dB | 0866:fe25 0966:fe25 |
| input | SPDIF | expTrsh | restore | 0.5652 | -60.0 dB | 0866:fda8 0966:fda8 |
| input | SPDIF | expRatio | probe | 0.75 | 7.7 | 0867:004d 0967:004d |
| input | SPDIF | expRatio | restore | 0 | 1.0 | 0867:000a 0967:000a |
| input | SPDIF | compexpEnable | toggle on | 1 |  | 0860:0001 0960:0001 |
| input | SPDIF | compexpEnable | toggle off | 1 |  | 0860:0000 0960:0000 |
| input | SPDIF | alevMaxgain | probe | 0.75 | 13.5 dB | 0881:0087 0981:0087 |
| input | SPDIF | alevMaxgain | restore | 0.3333 | 6.0 dB | 0881:003c 0981:003c |
| input | SPDIF | alevHeadroom | probe | 0.75 | 9.5 dB | 0882:005f 0982:005f |
| input | SPDIF | alevHeadroom | restore | 0.3333 | 6.0 dB | 0882:003c 0982:003c |
| input | SPDIF | alevRisetime | probe | 0.75 | 7.4 s | 0883:004a 0983:004a |
| input | SPDIF | alevRisetime | restore | 0.5 | 5.0 s | 0883:0032 0983:0032 |
| input | AS 1/2 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | AS 1/2 | eqType1 | restore | 1 | Shelf | (none) |
| input | AS 1/2 | eqGain1 | probe | 0.75 | 10.0 dB | 0a42:0064 0b42:0064 |
| input | AS 1/2 | eqGain1 | restore | 0.5 | 0.0 dB | 0a42:0000 0b42:0000 |
| input | AS 1/2 | eqFreq1 | probe | 0.75 | 3557 Hz | 0a43:0de5 0b43:0de5 |
| input | AS 1/2 | eqFreq1 | restore | 0.2007 | 80 Hz | 0a43:0050 0b43:0050 |
| input | AS 1/2 | eqQ1 | probe | 0.75 | 3.9 | 0a44:0027 0b44:0027 |
| input | AS 1/2 | eqQ1 | restore | 0.06977 | 1.0 | 0a44:000a 0b44:000a |
| input | AS 1/2 | eqGain2 | probe | 0.75 | 10.0 dB | 0a45:0064 0b45:0064 |
| input | AS 1/2 | eqGain2 | restore | 0.5 | 0.0 dB | 0a45:0000 0b45:0000 |
| input | AS 1/2 | eqFreq2 | probe | 0.75 | 3557 Hz | 0a46:0de5 0b46:0de5 |
| input | AS 1/2 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0a46:03e8 0b46:03e8 |
| input | AS 1/2 | eqQ2 | probe | 0.75 | 3.9 | 0a47:0027 0b47:0027 |
| input | AS 1/2 | eqQ2 | restore | 0.06977 | 1.0 | 0a47:000a 0b47:000a |
| input | AS 1/2 | eqType3 | probe | 0.75 | Low Pass | 0a48:0002 0b48:0002 |
| input | AS 1/2 | eqType3 | restore | 0.5 | Shelf | 0a48:0001 0b48:0001 |
| input | AS 1/2 | eqGain3 | probe | 0.75 | 10.0 dB | 0a49:0064 0b49:0064 |
| input | AS 1/2 | eqGain3 | restore | 0.5 | 0.0 dB | 0a49:0000 0b49:0000 |
| input | AS 1/2 | eqFreq3 | probe | 0.75 | 3557 Hz | 0a4a:0de5 0b4a:0de5 |
| input | AS 1/2 | eqFreq3 | restore | 0.7993 | 5000 Hz | 0a4a:1388 0b4a:1388 |
| input | AS 1/2 | eqQ3 | probe | 0.75 | 3.9 | 0a4b:0027 0b4b:0027 |
| input | AS 1/2 | eqQ3 | restore | 0.06977 | 1.0 | 0a4b:000a 0b4b:000a |
| input | AS 1/2 | eqEnable | toggle on | 1 |  | 0a40:0001 0b40:0001 |
| input | AS 1/2 | eqEnable | toggle off | 1 |  | 0a40:0000 0b40:0000 |
| input | AS 1/2 | lowcutFreq | probe | 0.75 | 224 Hz | 0a21:00e0 0b21:00e0 |
| input | AS 1/2 | lowcutFreq | restore | 0 | 20 Hz | 0a21:0014 0b21:0014 |
| input | AS 1/2 | lowcutGrade | probe | 0.75 | 18 dB/oct | 0a22:0002 0b22:0002 |
| input | AS 1/2 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0a22:0001 0b22:0001 |
| input | AS 1/2 | lowcutEnable | toggle on | 1 |  | 0a20:0001 0b20:0001 |
| input | AS 1/2 | lowcutEnable | toggle off | 1 |  | 0a20:0000 0b20:0000 |
| input | AS 1/2 | compexpGain | probe | 0.75 | 15.0 dB | 0a61:0096 0b61:0096 |
| input | AS 1/2 | compexpGain | restore | 0.5 | 0.0 dB | 0a61:0000 0b61:0000 |
| input | AS 1/2 | compexpAttack | probe | 0.75 | 150 ms | 0a62:0096 0b62:0096 |
| input | AS 1/2 | compexpAttack | restore | 0.05 | 10 ms | 0a62:000a 0b62:000a |
| input | AS 1/2 | compexpRelease | probe | 0.75 | 774 ms | 0a63:0306 0b63:0306 |
| input | AS 1/2 | compexpRelease | restore | 0.2225 | 300 ms | 0a63:012c 0b63:012c |
| input | AS 1/2 | compTrsh | probe | 0.75 | -15.0 dB | 0a64:ff6a 0b64:ff6a |
| input | AS 1/2 | compTrsh | restore | 0.5 | -30.0 dB | 0a64:fed4 0b64:fed4 |
| input | AS 1/2 | compRatio | probe | 0.75 | 7.7 | 0a65:004d 0b65:004d |
| input | AS 1/2 | compRatio | restore | 0 | 1.0 | 0a65:000a 0b65:000a |
| input | AS 1/2 | expTrsh | probe | 0.75 | -47.5 dB | 0a66:fe25 0b66:fe25 |
| input | AS 1/2 | expTrsh | restore | 0.5652 | -60.0 dB | 0a66:fda8 0b66:fda8 |
| input | AS 1/2 | expRatio | probe | 0.75 | 7.7 | 0a67:004d 0b67:004d |
| input | AS 1/2 | expRatio | restore | 0 | 1.0 | 0a67:000a 0b67:000a |
| input | AS 1/2 | compexpEnable | toggle on | 1 |  | 0a60:0001 0b60:0001 |
| input | AS 1/2 | compexpEnable | toggle off | 1 |  | 0a60:0000 0b60:0000 |
| input | AS 1/2 | alevMaxgain | probe | 0.75 | 13.5 dB | 0a81:0087 0b81:0087 |
| input | AS 1/2 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0a81:003c 0b81:003c |
| input | AS 1/2 | alevHeadroom | probe | 0.75 | 9.5 dB | 0a82:005f 0b82:005f |
| input | AS 1/2 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0a82:003c 0b82:003c |
| input | AS 1/2 | alevRisetime | probe | 0.75 | 7.4 s | 0a83:004a 0b83:004a |
| input | AS 1/2 | alevRisetime | restore | 0.5 | 5.0 s | 0a83:0032 0b83:0032 |
| input | ADAT 3/4 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | ADAT 3/4 | eqType1 | restore | 1 | Shelf | (none) |
| input | ADAT 3/4 | eqGain1 | probe | 0.75 | 10.0 dB | 0c42:0064 0d42:0064 |
| input | ADAT 3/4 | eqGain1 | restore | 0.5 | 0.0 dB | 0c42:0000 0d42:0000 |
| input | ADAT 3/4 | eqFreq1 | probe | 0.75 | 3557 Hz | 0c43:0de5 0d43:0de5 |
| input | ADAT 3/4 | eqFreq1 | restore | 0.2007 | 80 Hz | 0c43:0050 0d43:0050 |
| input | ADAT 3/4 | eqQ1 | probe | 0.75 | 3.9 | 0c44:0027 0d44:0027 |
| input | ADAT 3/4 | eqQ1 | restore | 0.06977 | 1.0 | 0c44:000a 0d44:000a |
| input | ADAT 3/4 | eqGain2 | probe | 0.75 | 10.0 dB | 0c45:0064 0d45:0064 |
| input | ADAT 3/4 | eqGain2 | restore | 0.5 | 0.0 dB | 0c45:0000 0d45:0000 |
| input | ADAT 3/4 | eqFreq2 | probe | 0.75 | 3557 Hz | 0c46:0de5 0d46:0de5 |
| input | ADAT 3/4 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0c46:03e8 0d46:03e8 |
| input | ADAT 3/4 | eqQ2 | probe | 0.75 | 3.9 | 0c47:0027 0d47:0027 |
| input | ADAT 3/4 | eqQ2 | restore | 0.06977 | 1.0 | 0c47:000a 0d47:000a |
| input | ADAT 3/4 | eqType3 | probe | 0.75 | Low Pass | 0c48:0002 0d48:0002 |
| input | ADAT 3/4 | eqType3 | restore | 0.5 | Shelf | 0c48:0001 0d48:0001 |
| input | ADAT 3/4 | eqGain3 | probe | 0.75 | 10.0 dB | 0c49:0064 0d49:0064 |
| input | ADAT 3/4 | eqGain3 | restore | 0.5 | 0.0 dB | 0c49:0000 0d49:0000 |
| input | ADAT 3/4 | eqFreq3 | probe | 0.75 | 3557 Hz | 0c4a:0de5 0d4a:0de5 |
| input | ADAT 3/4 | eqFreq3 | restore | 0.7993 | 5000 Hz | 0c4a:1388 0d4a:1388 |
| input | ADAT 3/4 | eqQ3 | probe | 0.75 | 3.9 | 0c4b:0027 0d4b:0027 |
| input | ADAT 3/4 | eqQ3 | restore | 0.06977 | 1.0 | 0c4b:000a 0d4b:000a |
| input | ADAT 3/4 | eqEnable | toggle on | 1 |  | 0c40:0001 0d40:0001 |
| input | ADAT 3/4 | eqEnable | toggle off | 1 |  | 0c40:0000 0d40:0000 |
| input | ADAT 3/4 | lowcutFreq | probe | 0.75 | 224 Hz | 0c21:00e0 0d21:00e0 |
| input | ADAT 3/4 | lowcutFreq | restore | 0 | 20 Hz | 0c21:0014 0d21:0014 |
| input | ADAT 3/4 | lowcutGrade | probe | 0.75 | 18 dB/oct | 0c22:0002 0d22:0002 |
| input | ADAT 3/4 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0c22:0001 0d22:0001 |
| input | ADAT 3/4 | lowcutEnable | toggle on | 1 |  | 0c20:0001 0d20:0001 |
| input | ADAT 3/4 | lowcutEnable | toggle off | 1 |  | 0c20:0000 0d20:0000 |
| input | ADAT 3/4 | compexpGain | probe | 0.75 | 15.0 dB | 0c61:0096 0d61:0096 |
| input | ADAT 3/4 | compexpGain | restore | 0.5 | 0.0 dB | 0c61:0000 0d61:0000 |
| input | ADAT 3/4 | compexpAttack | probe | 0.75 | 150 ms | 0c62:0096 0d62:0096 |
| input | ADAT 3/4 | compexpAttack | restore | 0.05 | 10 ms | 0c62:000a 0d62:000a |
| input | ADAT 3/4 | compexpRelease | probe | 0.75 | 774 ms | 0c63:0306 0d63:0306 |
| input | ADAT 3/4 | compexpRelease | restore | 0.2225 | 300 ms | 0c63:012c 0d63:012c |
| input | ADAT 3/4 | compTrsh | probe | 0.75 | -15.0 dB | 0c64:ff6a 0d64:ff6a |
| input | ADAT 3/4 | compTrsh | restore | 0.5 | -30.0 dB | 0c64:fed4 0d64:fed4 |
| input | ADAT 3/4 | compRatio | probe | 0.75 | 7.7 | 0c65:004d 0d65:004d |
| input | ADAT 3/4 | compRatio | restore | 0 | 1.0 | 0c65:000a 0d65:000a |
| input | ADAT 3/4 | expTrsh | probe | 0.75 | -47.5 dB | 0c66:fe25 0d66:fe25 |
| input | ADAT 3/4 | expTrsh | restore | 0.5652 | -60.0 dB | 0c66:fda8 0d66:fda8 |
| input | ADAT 3/4 | expRatio | probe | 0.75 | 7.7 | 0c67:004d 0d67:004d |
| input | ADAT 3/4 | expRatio | restore | 0 | 1.0 | 0c67:000a 0d67:000a |
| input | ADAT 3/4 | compexpEnable | toggle on | 1 |  | 0c60:0001 0d60:0001 |
| input | ADAT 3/4 | compexpEnable | toggle off | 1 |  | 0c60:0000 0d60:0000 |
| input | ADAT 3/4 | alevMaxgain | probe | 0.75 | 13.5 dB | 0c81:0087 0d81:0087 |
| input | ADAT 3/4 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0c81:003c 0d81:003c |
| input | ADAT 3/4 | alevHeadroom | probe | 0.75 | 9.5 dB | 0c82:005f 0d82:005f |
| input | ADAT 3/4 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0c82:003c 0d82:003c |
| input | ADAT 3/4 | alevRisetime | probe | 0.75 | 7.4 s | 0c83:004a 0d83:004a |
| input | ADAT 3/4 | alevRisetime | restore | 0.5 | 5.0 s | 0c83:0032 0d83:0032 |
| input | ADAT 5/6 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | ADAT 5/6 | eqType1 | restore | 1 | Shelf | (none) |
| input | ADAT 5/6 | eqGain1 | probe | 0.75 | 10.0 dB | 0e42:0064 0f42:0064 |
| input | ADAT 5/6 | eqGain1 | restore | 0.5 | 0.0 dB | 0e42:0000 0f42:0000 |
| input | ADAT 5/6 | eqFreq1 | probe | 0.75 | 3557 Hz | 0e43:0de5 0f43:0de5 |
| input | ADAT 5/6 | eqFreq1 | restore | 0.2007 | 80 Hz | 0e43:0050 0f43:0050 |
| input | ADAT 5/6 | eqQ1 | probe | 0.75 | 3.9 | 0e44:0027 0f44:0027 |
| input | ADAT 5/6 | eqQ1 | restore | 0.06977 | 1.0 | 0e44:000a 0f44:000a |
| input | ADAT 5/6 | eqGain2 | probe | 0.75 | 10.0 dB | 0e45:0064 0f45:0064 |
| input | ADAT 5/6 | eqGain2 | restore | 0.5 | 0.0 dB | 0e45:0000 0f45:0000 |
| input | ADAT 5/6 | eqFreq2 | probe | 0.75 | 3557 Hz | 0e46:0de5 0f46:0de5 |
| input | ADAT 5/6 | eqFreq2 | restore | 0.5663 | 1000 Hz | 0e46:03e8 0f46:03e8 |
| input | ADAT 5/6 | eqQ2 | probe | 0.75 | 3.9 | 0e47:0027 0f47:0027 |
| input | ADAT 5/6 | eqQ2 | restore | 0.06977 | 1.0 | 0e47:000a 0f47:000a |
| input | ADAT 5/6 | eqType3 | probe | 0.75 | Low Pass | 0e48:0002 0f48:0002 |
| input | ADAT 5/6 | eqType3 | restore | 0.5 | Shelf | 0e48:0001 0f48:0001 |
| input | ADAT 5/6 | eqGain3 | probe | 0.75 | 10.0 dB | 0e49:0064 0f49:0064 |
| input | ADAT 5/6 | eqGain3 | restore | 0.5 | 0.0 dB | 0e49:0000 0f49:0000 |
| input | ADAT 5/6 | eqFreq3 | probe | 0.75 | 3557 Hz | 0e4a:0de5 0f4a:0de5 |
| input | ADAT 5/6 | eqFreq3 | restore | 0.7993 | 5000 Hz | 0e4a:1388 0f4a:1388 |
| input | ADAT 5/6 | eqQ3 | probe | 0.75 | 3.9 | 0e4b:0027 0f4b:0027 |
| input | ADAT 5/6 | eqQ3 | restore | 0.06977 | 1.0 | 0e4b:000a 0f4b:000a |
| input | ADAT 5/6 | eqEnable | toggle on | 1 |  | 0e40:0001 0f40:0001 |
| input | ADAT 5/6 | eqEnable | toggle off | 1 |  | 0e40:0000 0f40:0000 |
| input | ADAT 5/6 | lowcutFreq | probe | 0.75 | 224 Hz | 0e21:00e0 0f21:00e0 |
| input | ADAT 5/6 | lowcutFreq | restore | 0 | 20 Hz | 0e21:0014 0f21:0014 |
| input | ADAT 5/6 | lowcutGrade | probe | 0.75 | 18 dB/oct | 0e22:0002 0f22:0002 |
| input | ADAT 5/6 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 0e22:0001 0f22:0001 |
| input | ADAT 5/6 | lowcutEnable | toggle on | 1 |  | 0e20:0001 0f20:0001 |
| input | ADAT 5/6 | lowcutEnable | toggle off | 1 |  | 0e20:0000 0f20:0000 |
| input | ADAT 5/6 | compexpGain | probe | 0.75 | 15.0 dB | 0e61:0096 0f61:0096 |
| input | ADAT 5/6 | compexpGain | restore | 0.5 | 0.0 dB | 0e61:0000 0f61:0000 |
| input | ADAT 5/6 | compexpAttack | probe | 0.75 | 150 ms | 0e62:0096 0f62:0096 |
| input | ADAT 5/6 | compexpAttack | restore | 0.05 | 10 ms | 0e62:000a 0f62:000a |
| input | ADAT 5/6 | compexpRelease | probe | 0.75 | 774 ms | 0e63:0306 0f63:0306 |
| input | ADAT 5/6 | compexpRelease | restore | 0.2225 | 300 ms | 0e63:012c 0f63:012c |
| input | ADAT 5/6 | compTrsh | probe | 0.75 | -15.0 dB | 0e64:ff6a 0f64:ff6a |
| input | ADAT 5/6 | compTrsh | restore | 0.5 | -30.0 dB | 0e64:fed4 0f64:fed4 |
| input | ADAT 5/6 | compRatio | probe | 0.75 | 7.7 | 0e65:004d 0f65:004d |
| input | ADAT 5/6 | compRatio | restore | 0 | 1.0 | 0e65:000a 0f65:000a |
| input | ADAT 5/6 | expTrsh | probe | 0.75 | -47.5 dB | 0e66:fe25 0f66:fe25 |
| input | ADAT 5/6 | expTrsh | restore | 0.5652 | -60.0 dB | 0e66:fda8 0f66:fda8 |
| input | ADAT 5/6 | expRatio | probe | 0.75 | 7.7 | 0e67:004d 0f67:004d |
| input | ADAT 5/6 | expRatio | restore | 0 | 1.0 | 0e67:000a 0f67:000a |
| input | ADAT 5/6 | compexpEnable | toggle on | 1 |  | 0e60:0001 0f60:0001 |
| input | ADAT 5/6 | compexpEnable | toggle off | 1 |  | 0e60:0000 0f60:0000 |
| input | ADAT 5/6 | alevMaxgain | probe | 0.75 | 13.5 dB | 0e81:0087 0f81:0087 |
| input | ADAT 5/6 | alevMaxgain | restore | 0.3333 | 6.0 dB | 0e81:003c 0f81:003c |
| input | ADAT 5/6 | alevHeadroom | probe | 0.75 | 9.5 dB | 0e82:005f 0f82:005f |
| input | ADAT 5/6 | alevHeadroom | restore | 0.3333 | 6.0 dB | 0e82:003c 0f82:003c |
| input | ADAT 5/6 | alevRisetime | probe | 0.75 | 7.4 s | 0e83:004a 0f83:004a |
| input | ADAT 5/6 | alevRisetime | restore | 0.5 | 5.0 s | 0e83:0032 0f83:0032 |
| input | ADAT 7/8 | eqType1 | probe | 0.75 | Shelf | (none) |
| input | ADAT 7/8 | eqType1 | restore | 1 | Shelf | (none) |
| input | ADAT 7/8 | eqGain1 | probe | 0.75 | 10.0 dB | 1042:0064 1142:0064 |
| input | ADAT 7/8 | eqGain1 | restore | 0.5 | 0.0 dB | 1042:0000 1142:0000 |
| input | ADAT 7/8 | eqFreq1 | probe | 0.75 | 3557 Hz | 1043:0de5 1143:0de5 |
| input | ADAT 7/8 | eqFreq1 | restore | 0.2007 | 80 Hz | 1043:0050 1143:0050 |
| input | ADAT 7/8 | eqQ1 | probe | 0.75 | 3.9 | 1044:0027 1144:0027 |
| input | ADAT 7/8 | eqQ1 | restore | 0.06977 | 1.0 | 1044:000a 1144:000a |
| input | ADAT 7/8 | eqGain2 | probe | 0.75 | 10.0 dB | 1045:0064 1145:0064 |
| input | ADAT 7/8 | eqGain2 | restore | 0.5 | 0.0 dB | 1045:0000 1145:0000 |
| input | ADAT 7/8 | eqFreq2 | probe | 0.75 | 3557 Hz | 1046:0de5 1146:0de5 |
| input | ADAT 7/8 | eqFreq2 | restore | 0.5663 | 1000 Hz | 1046:03e8 1146:03e8 |
| input | ADAT 7/8 | eqQ2 | probe | 0.75 | 3.9 | 1047:0027 1147:0027 |
| input | ADAT 7/8 | eqQ2 | restore | 0.06977 | 1.0 | 1047:000a 1147:000a |
| input | ADAT 7/8 | eqType3 | probe | 0.75 | Low Pass | 1048:0002 1148:0002 |
| input | ADAT 7/8 | eqType3 | restore | 0.5 | Shelf | 1048:0001 1148:0001 |
| input | ADAT 7/8 | eqGain3 | probe | 0.75 | 10.0 dB | 1049:0064 1149:0064 |
| input | ADAT 7/8 | eqGain3 | restore | 0.5 | 0.0 dB | 1049:0000 1149:0000 |
| input | ADAT 7/8 | eqFreq3 | probe | 0.75 | 3557 Hz | 104a:0de5 114a:0de5 |
| input | ADAT 7/8 | eqFreq3 | restore | 0.7993 | 5000 Hz | 104a:1388 114a:1388 |
| input | ADAT 7/8 | eqQ3 | probe | 0.75 | 3.9 | 104b:0027 114b:0027 |
| input | ADAT 7/8 | eqQ3 | restore | 0.06977 | 1.0 | 104b:000a 114b:000a |
| input | ADAT 7/8 | eqEnable | toggle on | 1 |  | 1040:0001 1140:0001 |
| input | ADAT 7/8 | eqEnable | toggle off | 1 |  | 1040:0000 1140:0000 |
| input | ADAT 7/8 | lowcutFreq | probe | 0.75 | 224 Hz | 1021:00e0 1121:00e0 |
| input | ADAT 7/8 | lowcutFreq | restore | 0 | 20 Hz | 1021:0014 1121:0014 |
| input | ADAT 7/8 | lowcutGrade | probe | 0.75 | 18 dB/oct | 1022:0002 1122:0002 |
| input | ADAT 7/8 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 1022:0001 1122:0001 |
| input | ADAT 7/8 | lowcutEnable | toggle on | 1 |  | 1020:0001 1120:0001 |
| input | ADAT 7/8 | lowcutEnable | toggle off | 1 |  | 1020:0000 1120:0000 |
| input | ADAT 7/8 | compexpGain | probe | 0.75 | 15.0 dB | 1061:0096 1161:0096 |
| input | ADAT 7/8 | compexpGain | restore | 0.5 | 0.0 dB | 1061:0000 1161:0000 |
| input | ADAT 7/8 | compexpAttack | probe | 0.75 | 150 ms | 1062:0096 1162:0096 |
| input | ADAT 7/8 | compexpAttack | restore | 0.05 | 10 ms | 1062:000a 1162:000a |
| input | ADAT 7/8 | compexpRelease | probe | 0.75 | 774 ms | 1063:0306 1163:0306 |
| input | ADAT 7/8 | compexpRelease | restore | 0.2225 | 300 ms | 1063:012c 1163:012c |
| input | ADAT 7/8 | compTrsh | probe | 0.75 | -15.0 dB | 1064:ff6a 1164:ff6a |
| input | ADAT 7/8 | compTrsh | restore | 0.5 | -30.0 dB | 1064:fed4 1164:fed4 |
| input | ADAT 7/8 | compRatio | probe | 0.75 | 7.7 | 1065:004d 1165:004d |
| input | ADAT 7/8 | compRatio | restore | 0 | 1.0 | 1065:000a 1165:000a |
| input | ADAT 7/8 | expTrsh | probe | 0.75 | -47.5 dB | 1066:fe25 1166:fe25 |
| input | ADAT 7/8 | expTrsh | restore | 0.5652 | -60.0 dB | 1066:fda8 1166:fda8 |
| input | ADAT 7/8 | expRatio | probe | 0.75 | 7.7 | 1067:004d 1167:004d |
| input | ADAT 7/8 | expRatio | restore | 0 | 1.0 | 1067:000a 1167:000a |
| input | ADAT 7/8 | compexpEnable | toggle on | 1 |  | 1060:0001 1160:0001 |
| input | ADAT 7/8 | compexpEnable | toggle off | 1 |  | 1060:0000 1160:0000 |
| input | ADAT 7/8 | alevMaxgain | probe | 0.75 | 13.5 dB | 1081:0087 1181:0087 |
| input | ADAT 7/8 | alevMaxgain | restore | 0.3333 | 6.0 dB | 1081:003c 1181:003c |
| input | ADAT 7/8 | alevHeadroom | probe | 0.75 | 9.5 dB | 1082:005f 1182:005f |
| input | ADAT 7/8 | alevHeadroom | restore | 0.3333 | 6.0 dB | 1082:003c 1182:003c |
| input | ADAT 7/8 | alevRisetime | probe | 0.75 | 7.4 s | 1083:004a 1183:004a |
| input | ADAT 7/8 | alevRisetime | restore | 0.5 | 5.0 s | 1083:0032 1183:0032 |
| playback | AN 1/2 | eqType1 | sweep | 0 | Bell | (none) |
| playback | AN 1/2 | eqType1 | sweep | 0.25 | Bell | (none) |
| playback | AN 1/2 | eqType1 | sweep | 0.5 | Shelf | (none) |
| playback | AN 1/2 | eqType1 | sweep | 0.75 | Shelf | (none) |
| playback | AN 1/2 | eqType1 | restore | 1 | Shelf | (none) |
| playback | AN 1/2 | eqGain1 | sweep | 0 | -20.0 dB | (none) |
| playback | AN 1/2 | eqGain1 | sweep | 0.25 | -10.0 dB | (none) |
| playback | AN 1/2 | eqGain1 | sweep | 0.75 | 10.0 dB | (none) |
| playback | AN 1/2 | eqGain1 | sweep | 1 | 20.0 dB | (none) |
| playback | AN 1/2 | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 1/2 | eqFreq1 | sweep | 0 | 20 Hz | (none) |
| playback | AN 1/2 | eqFreq1 | sweep | 0.25 | 112 Hz | (none) |
| playback | AN 1/2 | eqFreq1 | sweep | 0.5 | 632 Hz | (none) |
| playback | AN 1/2 | eqFreq1 | sweep | 0.75 | 3557 Hz | (none) |
| playback | AN 1/2 | eqFreq1 | sweep | 1 | 20000 Hz | (none) |
| playback | AN 1/2 | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | AN 1/2 | eqQ1 | sweep | 0 | 0.7 | (none) |
| playback | AN 1/2 | eqQ1 | sweep | 0.25 | 1.7 | (none) |
| playback | AN 1/2 | eqQ1 | sweep | 0.5 | 2.8 | (none) |
| playback | AN 1/2 | eqQ1 | sweep | 0.75 | 3.9 | (none) |
| playback | AN 1/2 | eqQ1 | sweep | 1 | 5.0 | (none) |
| playback | AN 1/2 | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 1/2 | eqGain2 | sweep | 0 | -20.0 dB | (none) |
| playback | AN 1/2 | eqGain2 | sweep | 0.25 | -10.0 dB | (none) |
| playback | AN 1/2 | eqGain2 | sweep | 0.75 | 10.0 dB | (none) |
| playback | AN 1/2 | eqGain2 | sweep | 1 | 20.0 dB | (none) |
| playback | AN 1/2 | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 1/2 | eqFreq2 | sweep | 0 | 20 Hz | (none) |
| playback | AN 1/2 | eqFreq2 | sweep | 0.25 | 112 Hz | (none) |
| playback | AN 1/2 | eqFreq2 | sweep | 0.5 | 632 Hz | (none) |
| playback | AN 1/2 | eqFreq2 | sweep | 0.75 | 3557 Hz | (none) |
| playback | AN 1/2 | eqFreq2 | sweep | 1 | 20000 Hz | (none) |
| playback | AN 1/2 | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | AN 1/2 | eqQ2 | sweep | 0 | 0.7 | (none) |
| playback | AN 1/2 | eqQ2 | sweep | 0.25 | 1.7 | (none) |
| playback | AN 1/2 | eqQ2 | sweep | 0.5 | 2.8 | (none) |
| playback | AN 1/2 | eqQ2 | sweep | 0.75 | 3.9 | (none) |
| playback | AN 1/2 | eqQ2 | sweep | 1 | 5.0 | (none) |
| playback | AN 1/2 | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 1/2 | eqType3 | sweep | 0 | Bell | (none) |
| playback | AN 1/2 | eqType3 | sweep | 0.25 | Shelf | (none) |
| playback | AN 1/2 | eqType3 | sweep | 0.75 | Low Pass | (none) |
| playback | AN 1/2 | eqType3 | sweep | 1 | Low Pass | (none) |
| playback | AN 1/2 | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | AN 1/2 | eqGain3 | sweep | 0 | -20.0 dB | (none) |
| playback | AN 1/2 | eqGain3 | sweep | 0.25 | -10.0 dB | (none) |
| playback | AN 1/2 | eqGain3 | sweep | 0.75 | 10.0 dB | (none) |
| playback | AN 1/2 | eqGain3 | sweep | 1 | 20.0 dB | (none) |
| playback | AN 1/2 | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 1/2 | eqFreq3 | sweep | 0 | 20 Hz | (none) |
| playback | AN 1/2 | eqFreq3 | sweep | 0.25 | 112 Hz | (none) |
| playback | AN 1/2 | eqFreq3 | sweep | 0.5 | 632 Hz | (none) |
| playback | AN 1/2 | eqFreq3 | sweep | 0.75 | 3557 Hz | (none) |
| playback | AN 1/2 | eqFreq3 | sweep | 1 | 20000 Hz | (none) |
| playback | AN 1/2 | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | AN 1/2 | eqQ3 | sweep | 0 | 0.7 | (none) |
| playback | AN 1/2 | eqQ3 | sweep | 0.25 | 1.7 | (none) |
| playback | AN 1/2 | eqQ3 | sweep | 0.5 | 2.8 | (none) |
| playback | AN 1/2 | eqQ3 | sweep | 0.75 | 3.9 | (none) |
| playback | AN 1/2 | eqQ3 | sweep | 1 | 5.0 | (none) |
| playback | AN 1/2 | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 1/2 | eqEnable | toggle on | 1 |  | (none) |
| playback | AN 1/2 | eqEnable | toggle off | 1 |  | (none) |
| playback | AN 1/2 | lowcutFreq | sweep | 0.25 | 45 Hz | (none) |
| playback | AN 1/2 | lowcutFreq | sweep | 0.5 | 100 Hz | (none) |
| playback | AN 1/2 | lowcutFreq | sweep | 0.75 | 224 Hz | (none) |
| playback | AN 1/2 | lowcutFreq | sweep | 1 | 500 Hz | (none) |
| playback | AN 1/2 | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | AN 1/2 | lowcutGrade | sweep | 0 | 6 dB/oct | (none) |
| playback | AN 1/2 | lowcutGrade | sweep | 0.25 | 12 dB/oct | (none) |
| playback | AN 1/2 | lowcutGrade | sweep | 0.5 | 18 dB/oct | (none) |
| playback | AN 1/2 | lowcutGrade | sweep | 0.75 | 18 dB/oct | (none) |
| playback | AN 1/2 | lowcutGrade | sweep | 1 | 24 dB/oct | (none) |
| playback | AN 1/2 | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | AN 1/2 | lowcutEnable | toggle on | 1 |  | (none) |
| playback | AN 1/2 | lowcutEnable | toggle off | 1 |  | (none) |
| playback | AN 1/2 | compexpGain | sweep | 0 | -30.0 dB | (none) |
| playback | AN 1/2 | compexpGain | sweep | 0.25 | -15.0 dB | (none) |
| playback | AN 1/2 | compexpGain | sweep | 0.75 | 15.0 dB | (none) |
| playback | AN 1/2 | compexpGain | sweep | 1 | 30.0 dB | (none) |
| playback | AN 1/2 | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 1/2 | compexpAttack | sweep | 0 | 0 ms | (none) |
| playback | AN 1/2 | compexpAttack | sweep | 0.25 | 50 ms | (none) |
| playback | AN 1/2 | compexpAttack | sweep | 0.5 | 100 ms | (none) |
| playback | AN 1/2 | compexpAttack | sweep | 0.75 | 150 ms | (none) |
| playback | AN 1/2 | compexpAttack | sweep | 1 | 200 ms | (none) |
| playback | AN 1/2 | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | AN 1/2 | compexpRelease | sweep | 0 | 100 ms | (none) |
| playback | AN 1/2 | compexpRelease | sweep | 0.25 | 325 ms | (none) |
| playback | AN 1/2 | compexpRelease | sweep | 0.5 | 550 ms | (none) |
| playback | AN 1/2 | compexpRelease | sweep | 0.75 | 774 ms | (none) |
| playback | AN 1/2 | compexpRelease | sweep | 1 | 999 ms | (none) |
| playback | AN 1/2 | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | AN 1/2 | compTrsh | sweep | 0 | -60.0 dB | (none) |
| playback | AN 1/2 | compTrsh | sweep | 0.25 | -45.0 dB | (none) |
| playback | AN 1/2 | compTrsh | sweep | 0.75 | -15.0 dB | (none) |
| playback | AN 1/2 | compTrsh | sweep | 1 | 0.0 dB | (none) |
| playback | AN 1/2 | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | AN 1/2 | compRatio | sweep | 0.25 | 3.2 | (none) |
| playback | AN 1/2 | compRatio | sweep | 0.5 | 5.5 | (none) |
| playback | AN 1/2 | compRatio | sweep | 0.75 | 7.7 | (none) |
| playback | AN 1/2 | compRatio | sweep | 1 | 10.0 | (none) |
| playback | AN 1/2 | compRatio | restore | 0 | 1.0 | (none) |
| playback | AN 1/2 | expTrsh | sweep | 0 | -99.0 dB | (none) |
| playback | AN 1/2 | expTrsh | sweep | 0.25 | -82.0 dB | (none) |
| playback | AN 1/2 | expTrsh | sweep | 0.5 | -64.5 dB | (none) |
| playback | AN 1/2 | expTrsh | sweep | 0.75 | -47.5 dB | (none) |
| playback | AN 1/2 | expTrsh | sweep | 1 | -30.0 dB | (none) |
| playback | AN 1/2 | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | AN 1/2 | expRatio | sweep | 0.25 | 3.2 | (none) |
| playback | AN 1/2 | expRatio | sweep | 0.5 | 5.5 | (none) |
| playback | AN 1/2 | expRatio | sweep | 0.75 | 7.7 | (none) |
| playback | AN 1/2 | expRatio | sweep | 1 | 10.0 | (none) |
| playback | AN 1/2 | expRatio | restore | 0 | 1.0 | (none) |
| playback | AN 1/2 | compexpEnable | toggle on | 1 |  | (none) |
| playback | AN 1/2 | compexpEnable | toggle off | 1 |  | (none) |
| playback | AN 1/2 | alevMaxgain | sweep | 0 | 0.0 dB | (none) |
| playback | AN 1/2 | alevMaxgain | sweep | 0.25 | 4.5 dB | (none) |
| playback | AN 1/2 | alevMaxgain | sweep | 0.5 | 9.0 dB | (none) |
| playback | AN 1/2 | alevMaxgain | sweep | 0.75 | 13.5 dB | (none) |
| playback | AN 1/2 | alevMaxgain | sweep | 1 | 18.0 dB | (none) |
| playback | AN 1/2 | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | AN 1/2 | alevHeadroom | sweep | 0 | 3.0 dB | (none) |
| playback | AN 1/2 | alevHeadroom | sweep | 0.25 | 5.0 dB | (none) |
| playback | AN 1/2 | alevHeadroom | sweep | 0.5 | 7.5 dB | (none) |
| playback | AN 1/2 | alevHeadroom | sweep | 0.75 | 9.5 dB | (none) |
| playback | AN 1/2 | alevHeadroom | sweep | 1 | 12.0 dB | (none) |
| playback | AN 1/2 | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | AN 1/2 | alevRisetime | sweep | 0 | 0.1 s | (none) |
| playback | AN 1/2 | alevRisetime | sweep | 0.25 | 2.5 s | (none) |
| playback | AN 1/2 | alevRisetime | sweep | 0.75 | 7.4 s | (none) |
| playback | AN 1/2 | alevRisetime | sweep | 1 | 9.9 s | (none) |
| playback | AN 1/2 | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| playback | AN 3/4 | eqType1 | probe | 0.75 | Shelf | (none) |
| playback | AN 3/4 | eqType1 | restore | 1 | Shelf | (none) |
| playback | AN 3/4 | eqGain1 | probe | 0.75 | 10.0 dB | (none) |
| playback | AN 3/4 | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 3/4 | eqFreq1 | probe | 0.75 | 3557 Hz | (none) |
| playback | AN 3/4 | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | AN 3/4 | eqQ1 | probe | 0.75 | 3.9 | (none) |
| playback | AN 3/4 | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 3/4 | eqGain2 | probe | 0.75 | 10.0 dB | (none) |
| playback | AN 3/4 | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 3/4 | eqFreq2 | probe | 0.75 | 3557 Hz | (none) |
| playback | AN 3/4 | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | AN 3/4 | eqQ2 | probe | 0.75 | 3.9 | (none) |
| playback | AN 3/4 | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 3/4 | eqType3 | probe | 0.75 | Low Pass | (none) |
| playback | AN 3/4 | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | AN 3/4 | eqGain3 | probe | 0.75 | 10.0 dB | (none) |
| playback | AN 3/4 | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 3/4 | eqFreq3 | probe | 0.75 | 3557 Hz | (none) |
| playback | AN 3/4 | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | AN 3/4 | eqQ3 | probe | 0.75 | 3.9 | (none) |
| playback | AN 3/4 | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 3/4 | eqEnable | toggle on | 1 |  | (none) |
| playback | AN 3/4 | eqEnable | toggle off | 1 |  | (none) |
| playback | AN 3/4 | lowcutFreq | probe | 0.75 | 224 Hz | (none) |
| playback | AN 3/4 | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | AN 3/4 | lowcutGrade | probe | 0.75 | 18 dB/oct | (none) |
| playback | AN 3/4 | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | AN 3/4 | lowcutEnable | toggle on | 1 |  | (none) |
| playback | AN 3/4 | lowcutEnable | toggle off | 1 |  | (none) |
| playback | AN 3/4 | compexpGain | probe | 0.75 | 15.0 dB | (none) |
| playback | AN 3/4 | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 3/4 | compexpAttack | probe | 0.75 | 150 ms | (none) |
| playback | AN 3/4 | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | AN 3/4 | compexpRelease | probe | 0.75 | 774 ms | (none) |
| playback | AN 3/4 | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | AN 3/4 | compTrsh | probe | 0.75 | -15.0 dB | (none) |
| playback | AN 3/4 | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | AN 3/4 | compRatio | probe | 0.75 | 7.7 | (none) |
| playback | AN 3/4 | compRatio | restore | 0 | 1.0 | (none) |
| playback | AN 3/4 | expTrsh | probe | 0.75 | -47.5 dB | (none) |
| playback | AN 3/4 | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | AN 3/4 | expRatio | probe | 0.75 | 7.7 | (none) |
| playback | AN 3/4 | expRatio | restore | 0 | 1.0 | (none) |
| playback | AN 3/4 | compexpEnable | toggle on | 1 |  | (none) |
| playback | AN 3/4 | compexpEnable | toggle off | 1 |  | (none) |
| playback | AN 3/4 | alevMaxgain | probe | 0.75 | 13.5 dB | (none) |
| playback | AN 3/4 | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | AN 3/4 | alevHeadroom | probe | 0.75 | 9.5 dB | (none) |
| playback | AN 3/4 | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | AN 3/4 | alevRisetime | probe | 0.75 | 7.4 s | (none) |
| playback | AN 3/4 | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| playback | AN 5/6 | eqType1 | probe | 0.75 |  | (none) |
| playback | AN 5/6 | eqType1 | restore | 1 |  | (none) |
| playback | AN 5/6 | eqGain1 | probe | 0.75 | 10.0 dB | (none) |
| playback | AN 5/6 | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 5/6 | eqFreq1 | probe | 0.75 | 3557 Hz | (none) |
| playback | AN 5/6 | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | AN 5/6 | eqQ1 | probe | 0.75 | 3.9 | (none) |
| playback | AN 5/6 | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 5/6 | eqGain2 | probe | 0.75 | 10.0 dB | (none) |
| playback | AN 5/6 | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 5/6 | eqFreq2 | probe | 0.75 | 3557 Hz | (none) |
| playback | AN 5/6 | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | AN 5/6 | eqQ2 | probe | 0.75 | 3.9 | (none) |
| playback | AN 5/6 | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 5/6 | eqType3 | probe | 0.75 | Low Pass | (none) |
| playback | AN 5/6 | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | AN 5/6 | eqGain3 | probe | 0.75 | 10.0 dB | (none) |
| playback | AN 5/6 | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 5/6 | eqFreq3 | probe | 0.75 | 3557 Hz | (none) |
| playback | AN 5/6 | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | AN 5/6 | eqQ3 | probe | 0.75 | 3.9 | (none) |
| playback | AN 5/6 | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | AN 5/6 | eqEnable | toggle on | 1 |  | (none) |
| playback | AN 5/6 | eqEnable | toggle off | 1 |  | (none) |
| playback | AN 5/6 | lowcutFreq | probe | 0.75 | 224 Hz | (none) |
| playback | AN 5/6 | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | AN 5/6 | lowcutGrade | probe | 0.75 | 18 dB/oct | (none) |
| playback | AN 5/6 | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | AN 5/6 | lowcutEnable | toggle on | 1 |  | (none) |
| playback | AN 5/6 | lowcutEnable | toggle off | 1 |  | (none) |
| playback | AN 5/6 | compexpGain | probe | 0.75 | 15.0 dB | (none) |
| playback | AN 5/6 | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | AN 5/6 | compexpAttack | probe | 0.75 | 150 ms | (none) |
| playback | AN 5/6 | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | AN 5/6 | compexpRelease | probe | 0.75 | 774 ms | (none) |
| playback | AN 5/6 | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | AN 5/6 | compTrsh | probe | 0.75 | -15.0 dB | (none) |
| playback | AN 5/6 | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | AN 5/6 | compRatio | probe | 0.75 | 7.7 | (none) |
| playback | AN 5/6 | compRatio | restore | 0 | 1.0 | (none) |
| playback | AN 5/6 | expTrsh | probe | 0.75 | -47.5 dB | (none) |
| playback | AN 5/6 | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | AN 5/6 | expRatio | probe | 0.75 | 7.7 | (none) |
| playback | AN 5/6 | expRatio | restore | 0 | 1.0 | (none) |
| playback | AN 5/6 | compexpEnable | toggle on | 1 |  | (none) |
| playback | AN 5/6 | compexpEnable | toggle off | 1 |  | (none) |
| playback | AN 5/6 | alevMaxgain | probe | 0.75 | 13.5 dB | (none) |
| playback | AN 5/6 | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | AN 5/6 | alevHeadroom | probe | 0.75 | 9.5 dB | (none) |
| playback | AN 5/6 | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | AN 5/6 | alevRisetime | probe | 0.75 | 7.4 s | (none) |
| playback | AN 5/6 | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| playback | PH 7/8 | eqType1 | probe | 0.75 | Shelf | (none) |
| playback | PH 7/8 | eqType1 | restore | 1 | Shelf | (none) |
| playback | PH 7/8 | eqGain1 | probe | 0.75 | 10.0 dB | (none) |
| playback | PH 7/8 | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | PH 7/8 | eqFreq1 | probe | 0.75 | 3557 Hz | (none) |
| playback | PH 7/8 | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | PH 7/8 | eqQ1 | probe | 0.75 | 3.9 | (none) |
| playback | PH 7/8 | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | PH 7/8 | eqGain2 | probe | 0.75 | 10.0 dB | (none) |
| playback | PH 7/8 | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | PH 7/8 | eqFreq2 | probe | 0.75 | 3557 Hz | (none) |
| playback | PH 7/8 | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | PH 7/8 | eqQ2 | probe | 0.75 | 3.9 | (none) |
| playback | PH 7/8 | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | PH 7/8 | eqType3 | probe | 0.75 | Low Pass | (none) |
| playback | PH 7/8 | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | PH 7/8 | eqGain3 | probe | 0.75 | 10.0 dB | (none) |
| playback | PH 7/8 | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | PH 7/8 | eqFreq3 | probe | 0.75 | 3557 Hz | (none) |
| playback | PH 7/8 | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | PH 7/8 | eqQ3 | probe | 0.75 | 3.9 | (none) |
| playback | PH 7/8 | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | PH 7/8 | eqEnable | toggle on | 1 |  | (none) |
| playback | PH 7/8 | eqEnable | toggle off | 1 |  | (none) |
| playback | PH 7/8 | lowcutFreq | probe | 0.75 | 224 Hz | (none) |
| playback | PH 7/8 | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | PH 7/8 | lowcutGrade | probe | 0.75 | 18 dB/oct | (none) |
| playback | PH 7/8 | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | PH 7/8 | lowcutEnable | toggle on | 1 |  | (none) |
| playback | PH 7/8 | lowcutEnable | toggle off | 1 |  | (none) |
| playback | PH 7/8 | compexpGain | probe | 0.75 | 15.0 dB | (none) |
| playback | PH 7/8 | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | PH 7/8 | compexpAttack | probe | 0.75 | 150 ms | (none) |
| playback | PH 7/8 | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | PH 7/8 | compexpRelease | probe | 0.75 | 774 ms | (none) |
| playback | PH 7/8 | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | PH 7/8 | compTrsh | probe | 0.75 | -15.0 dB | (none) |
| playback | PH 7/8 | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | PH 7/8 | compRatio | probe | 0.75 | 7.7 | (none) |
| playback | PH 7/8 | compRatio | restore | 0 | 1.0 | (none) |
| playback | PH 7/8 | expTrsh | probe | 0.75 | -47.5 dB | (none) |
| playback | PH 7/8 | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | PH 7/8 | expRatio | probe | 0.75 | 7.7 | (none) |
| playback | PH 7/8 | expRatio | restore | 0 | 1.0 | (none) |
| playback | PH 7/8 | compexpEnable | toggle on | 1 |  | (none) |
| playback | PH 7/8 | compexpEnable | toggle off | 1 |  | (none) |
| playback | PH 7/8 | alevMaxgain | probe | 0.75 | 13.5 dB | (none) |
| playback | PH 7/8 | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | PH 7/8 | alevHeadroom | probe | 0.75 | 9.5 dB | (none) |
| playback | PH 7/8 | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | PH 7/8 | alevRisetime | probe | 0.75 | 7.4 s | (none) |
| playback | PH 7/8 | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| playback | SPDIF | eqType1 | probe | 0.75 | Shelf | (none) |
| playback | SPDIF | eqType1 | restore | 1 | Shelf | (none) |
| playback | SPDIF | eqGain1 | probe | 0.75 | 10.0 dB | (none) |
| playback | SPDIF | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | SPDIF | eqFreq1 | probe | 0.75 | 3557 Hz | (none) |
| playback | SPDIF | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | SPDIF | eqQ1 | probe | 0.75 | 3.9 | (none) |
| playback | SPDIF | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | SPDIF | eqGain2 | probe | 0.75 | 10.0 dB | (none) |
| playback | SPDIF | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | SPDIF | eqFreq2 | probe | 0.75 | 3557 Hz | (none) |
| playback | SPDIF | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | SPDIF | eqQ2 | probe | 0.75 | 3.9 | (none) |
| playback | SPDIF | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | SPDIF | eqType3 | probe | 0.75 | Low Pass | (none) |
| playback | SPDIF | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | SPDIF | eqGain3 | probe | 0.75 | 10.0 dB | (none) |
| playback | SPDIF | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | SPDIF | eqFreq3 | probe | 0.75 | 3557 Hz | (none) |
| playback | SPDIF | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | SPDIF | eqQ3 | probe | 0.75 | 3.9 | (none) |
| playback | SPDIF | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | SPDIF | eqEnable | toggle on | 1 |  | (none) |
| playback | SPDIF | eqEnable | toggle off | 1 |  | (none) |
| playback | SPDIF | lowcutFreq | probe | 0.75 | 224 Hz | (none) |
| playback | SPDIF | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | SPDIF | lowcutGrade | probe | 0.75 | 18 dB/oct | (none) |
| playback | SPDIF | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | SPDIF | lowcutEnable | toggle on | 1 |  | (none) |
| playback | SPDIF | lowcutEnable | toggle off | 1 |  | (none) |
| playback | SPDIF | compexpGain | probe | 0.75 | 15.0 dB | (none) |
| playback | SPDIF | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | SPDIF | compexpAttack | probe | 0.75 | 150 ms | (none) |
| playback | SPDIF | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | SPDIF | compexpRelease | probe | 0.75 | 774 ms | (none) |
| playback | SPDIF | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | SPDIF | compTrsh | probe | 0.75 | -15.0 dB | (none) |
| playback | SPDIF | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | SPDIF | compRatio | probe | 0.75 | 7.7 | (none) |
| playback | SPDIF | compRatio | restore | 0 | 1.0 | (none) |
| playback | SPDIF | expTrsh | probe | 0.75 | -47.5 dB | (none) |
| playback | SPDIF | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | SPDIF | expRatio | probe | 0.75 | 7.7 | (none) |
| playback | SPDIF | expRatio | restore | 0 | 1.0 | (none) |
| playback | SPDIF | compexpEnable | toggle on | 1 |  | (none) |
| playback | SPDIF | compexpEnable | toggle off | 1 |  | (none) |
| playback | SPDIF | alevMaxgain | probe | 0.75 | 13.5 dB | (none) |
| playback | SPDIF | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | SPDIF | alevHeadroom | probe | 0.75 | 9.5 dB | (none) |
| playback | SPDIF | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | SPDIF | alevRisetime | probe | 0.75 | 7.4 s | (none) |
| playback | SPDIF | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| playback | AS 1/2 | eqType1 | probe | 0.75 | Shelf | (none) |
| playback | AS 1/2 | eqType1 | restore | 1 | Shelf | (none) |
| playback | AS 1/2 | eqGain1 | probe | 0.75 | 10.0 dB | (none) |
| playback | AS 1/2 | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | AS 1/2 | eqFreq1 | probe | 0.75 | 3557 Hz | (none) |
| playback | AS 1/2 | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | AS 1/2 | eqQ1 | probe | 0.75 | 3.9 | (none) |
| playback | AS 1/2 | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | AS 1/2 | eqGain2 | probe | 0.75 | 10.0 dB | (none) |
| playback | AS 1/2 | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | AS 1/2 | eqFreq2 | probe | 0.75 | 3557 Hz | (none) |
| playback | AS 1/2 | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | AS 1/2 | eqQ2 | probe | 0.75 | 3.9 | (none) |
| playback | AS 1/2 | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | AS 1/2 | eqType3 | probe | 0.75 | Low Pass | (none) |
| playback | AS 1/2 | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | AS 1/2 | eqGain3 | probe | 0.75 | 10.0 dB | (none) |
| playback | AS 1/2 | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | AS 1/2 | eqFreq3 | probe | 0.75 | 3557 Hz | (none) |
| playback | AS 1/2 | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | AS 1/2 | eqQ3 | probe | 0.75 | 3.9 | (none) |
| playback | AS 1/2 | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | AS 1/2 | eqEnable | toggle on | 1 |  | (none) |
| playback | AS 1/2 | eqEnable | toggle off | 1 |  | (none) |
| playback | AS 1/2 | lowcutFreq | probe | 0.75 | 224 Hz | (none) |
| playback | AS 1/2 | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | AS 1/2 | lowcutGrade | probe | 0.75 | 18 dB/oct | (none) |
| playback | AS 1/2 | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | AS 1/2 | lowcutEnable | toggle on | 1 |  | (none) |
| playback | AS 1/2 | lowcutEnable | toggle off | 1 |  | (none) |
| playback | AS 1/2 | compexpGain | probe | 0.75 | 15.0 dB | (none) |
| playback | AS 1/2 | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | AS 1/2 | compexpAttack | probe | 0.75 | 150 ms | (none) |
| playback | AS 1/2 | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | AS 1/2 | compexpRelease | probe | 0.75 | 774 ms | (none) |
| playback | AS 1/2 | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | AS 1/2 | compTrsh | probe | 0.75 | -15.0 dB | (none) |
| playback | AS 1/2 | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | AS 1/2 | compRatio | probe | 0.75 | 7.7 | (none) |
| playback | AS 1/2 | compRatio | restore | 0 | 1.0 | (none) |
| playback | AS 1/2 | expTrsh | probe | 0.75 | -47.5 dB | (none) |
| playback | AS 1/2 | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | AS 1/2 | expRatio | probe | 0.75 | 7.7 | (none) |
| playback | AS 1/2 | expRatio | restore | 0 | 1.0 | (none) |
| playback | AS 1/2 | compexpEnable | toggle on | 1 |  | (none) |
| playback | AS 1/2 | compexpEnable | toggle off | 1 |  | (none) |
| playback | AS 1/2 | alevMaxgain | probe | 0.75 | 13.5 dB | (none) |
| playback | AS 1/2 | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | AS 1/2 | alevHeadroom | probe | 0.75 | 9.5 dB | (none) |
| playback | AS 1/2 | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | AS 1/2 | alevRisetime | probe | 0.75 | 7.4 s | (none) |
| playback | AS 1/2 | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| playback | ADAT 3/4 | eqType1 | probe | 0.75 | Shelf | (none) |
| playback | ADAT 3/4 | eqType1 | restore | 1 | Shelf | (none) |
| playback | ADAT 3/4 | eqGain1 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 3/4 | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 3/4 | eqFreq1 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 3/4 | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | ADAT 3/4 | eqQ1 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 3/4 | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 3/4 | eqGain2 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 3/4 | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 3/4 | eqFreq2 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 3/4 | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | ADAT 3/4 | eqQ2 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 3/4 | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 3/4 | eqType3 | probe | 0.75 | Low Pass | (none) |
| playback | ADAT 3/4 | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | ADAT 3/4 | eqGain3 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 3/4 | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 3/4 | eqFreq3 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 3/4 | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | ADAT 3/4 | eqQ3 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 3/4 | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 3/4 | eqEnable | toggle on | 1 |  | (none) |
| playback | ADAT 3/4 | eqEnable | toggle off | 1 |  | (none) |
| playback | ADAT 3/4 | lowcutFreq | probe | 0.75 | 224 Hz | (none) |
| playback | ADAT 3/4 | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | ADAT 3/4 | lowcutGrade | probe | 0.75 | 18 dB/oct | (none) |
| playback | ADAT 3/4 | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | ADAT 3/4 | lowcutEnable | toggle on | 1 |  | (none) |
| playback | ADAT 3/4 | lowcutEnable | toggle off | 1 |  | (none) |
| playback | ADAT 3/4 | compexpGain | probe | 0.75 | 15.0 dB | (none) |
| playback | ADAT 3/4 | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 3/4 | compexpAttack | probe | 0.75 | 150 ms | (none) |
| playback | ADAT 3/4 | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | ADAT 3/4 | compexpRelease | probe | 0.75 | 774 ms | (none) |
| playback | ADAT 3/4 | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | ADAT 3/4 | compTrsh | probe | 0.75 | -15.0 dB | (none) |
| playback | ADAT 3/4 | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | ADAT 3/4 | compRatio | probe | 0.75 | 7.7 | (none) |
| playback | ADAT 3/4 | compRatio | restore | 0 | 1.0 | (none) |
| playback | ADAT 3/4 | expTrsh | probe | 0.75 | -47.5 dB | (none) |
| playback | ADAT 3/4 | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | ADAT 3/4 | expRatio | probe | 0.75 | 7.7 | (none) |
| playback | ADAT 3/4 | expRatio | restore | 0 | 1.0 | (none) |
| playback | ADAT 3/4 | compexpEnable | toggle on | 1 |  | (none) |
| playback | ADAT 3/4 | compexpEnable | toggle off | 1 |  | (none) |
| playback | ADAT 3/4 | alevMaxgain | probe | 0.75 | 13.5 dB | (none) |
| playback | ADAT 3/4 | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | ADAT 3/4 | alevHeadroom | probe | 0.75 | 9.5 dB | (none) |
| playback | ADAT 3/4 | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | ADAT 3/4 | alevRisetime | probe | 0.75 | 7.4 s | (none) |
| playback | ADAT 3/4 | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| playback | ADAT 5/6 | eqType1 | probe | 0.75 | Shelf | (none) |
| playback | ADAT 5/6 | eqType1 | restore | 1 | Shelf | (none) |
| playback | ADAT 5/6 | eqGain1 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 5/6 | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 5/6 | eqFreq1 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 5/6 | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | ADAT 5/6 | eqQ1 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 5/6 | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 5/6 | eqGain2 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 5/6 | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 5/6 | eqFreq2 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 5/6 | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | ADAT 5/6 | eqQ2 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 5/6 | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 5/6 | eqType3 | probe | 0.75 | Low Pass | (none) |
| playback | ADAT 5/6 | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | ADAT 5/6 | eqGain3 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 5/6 | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 5/6 | eqFreq3 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 5/6 | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | ADAT 5/6 | eqQ3 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 5/6 | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 5/6 | eqEnable | toggle on | 1 |  | (none) |
| playback | ADAT 5/6 | eqEnable | toggle off | 1 |  | (none) |
| playback | ADAT 5/6 | lowcutFreq | probe | 0.75 | 224 Hz | (none) |
| playback | ADAT 5/6 | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | ADAT 5/6 | lowcutGrade | probe | 0.75 | 18 dB/oct | (none) |
| playback | ADAT 5/6 | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | ADAT 5/6 | lowcutEnable | toggle on | 1 |  | (none) |
| playback | ADAT 5/6 | lowcutEnable | toggle off | 1 |  | (none) |
| playback | ADAT 5/6 | compexpGain | probe | 0.75 | 15.0 dB | (none) |
| playback | ADAT 5/6 | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 5/6 | compexpAttack | probe | 0.75 | 150 ms | (none) |
| playback | ADAT 5/6 | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | ADAT 5/6 | compexpRelease | probe | 0.75 | 774 ms | (none) |
| playback | ADAT 5/6 | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | ADAT 5/6 | compTrsh | probe | 0.75 | -15.0 dB | (none) |
| playback | ADAT 5/6 | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | ADAT 5/6 | compRatio | probe | 0.75 | 7.7 | (none) |
| playback | ADAT 5/6 | compRatio | restore | 0 | 1.0 | (none) |
| playback | ADAT 5/6 | expTrsh | probe | 0.75 | -47.5 dB | (none) |
| playback | ADAT 5/6 | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | ADAT 5/6 | expRatio | probe | 0.75 | 7.7 | (none) |
| playback | ADAT 5/6 | expRatio | restore | 0 | 1.0 | (none) |
| playback | ADAT 5/6 | compexpEnable | toggle on | 1 |  | (none) |
| playback | ADAT 5/6 | compexpEnable | toggle off | 1 |  | (none) |
| playback | ADAT 5/6 | alevMaxgain | probe | 0.75 | 13.5 dB | (none) |
| playback | ADAT 5/6 | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | ADAT 5/6 | alevHeadroom | probe | 0.75 | 9.5 dB | (none) |
| playback | ADAT 5/6 | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | ADAT 5/6 | alevRisetime | probe | 0.75 | 7.4 s | (none) |
| playback | ADAT 5/6 | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| playback | ADAT 7/8 | eqType1 | probe | 0.75 | Shelf | (none) |
| playback | ADAT 7/8 | eqType1 | restore | 1 | Shelf | (none) |
| playback | ADAT 7/8 | eqGain1 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 7/8 | eqGain1 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 7/8 | eqFreq1 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 7/8 | eqFreq1 | restore | 0.2007 | 80 Hz | (none) |
| playback | ADAT 7/8 | eqQ1 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 7/8 | eqQ1 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 7/8 | eqGain2 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 7/8 | eqGain2 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 7/8 | eqFreq2 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 7/8 | eqFreq2 | restore | 0.5663 | 1000 Hz | (none) |
| playback | ADAT 7/8 | eqQ2 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 7/8 | eqQ2 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 7/8 | eqType3 | probe | 0.75 | Low Pass | (none) |
| playback | ADAT 7/8 | eqType3 | restore | 0.5 | Shelf | (none) |
| playback | ADAT 7/8 | eqGain3 | probe | 0.75 | 10.0 dB | (none) |
| playback | ADAT 7/8 | eqGain3 | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 7/8 | eqFreq3 | probe | 0.75 | 3557 Hz | (none) |
| playback | ADAT 7/8 | eqFreq3 | restore | 0.7993 | 5000 Hz | (none) |
| playback | ADAT 7/8 | eqQ3 | probe | 0.75 | 3.9 | (none) |
| playback | ADAT 7/8 | eqQ3 | restore | 0.06977 | 1.0 | (none) |
| playback | ADAT 7/8 | eqEnable | toggle on | 1 |  | (none) |
| playback | ADAT 7/8 | eqEnable | toggle off | 1 |  | (none) |
| playback | ADAT 7/8 | lowcutFreq | probe | 0.75 | 224 Hz | (none) |
| playback | ADAT 7/8 | lowcutFreq | restore | 0 | 20 Hz | (none) |
| playback | ADAT 7/8 | lowcutGrade | probe | 0.75 | 18 dB/oct | (none) |
| playback | ADAT 7/8 | lowcutGrade | restore | 0.3333 | 12 dB/oct | (none) |
| playback | ADAT 7/8 | lowcutEnable | toggle on | 1 |  | (none) |
| playback | ADAT 7/8 | lowcutEnable | toggle off | 1 |  | (none) |
| playback | ADAT 7/8 | compexpGain | probe | 0.75 | 15.0 dB | (none) |
| playback | ADAT 7/8 | compexpGain | restore | 0.5 | 0.0 dB | (none) |
| playback | ADAT 7/8 | compexpAttack | probe | 0.75 | 150 ms | (none) |
| playback | ADAT 7/8 | compexpAttack | restore | 0.05 | 10 ms | (none) |
| playback | ADAT 7/8 | compexpRelease | probe | 0.75 | 774 ms | (none) |
| playback | ADAT 7/8 | compexpRelease | restore | 0.2225 | 300 ms | (none) |
| playback | ADAT 7/8 | compTrsh | probe | 0.75 | -15.0 dB | (none) |
| playback | ADAT 7/8 | compTrsh | restore | 0.5 | -30.0 dB | (none) |
| playback | ADAT 7/8 | compRatio | probe | 0.75 | 7.7 | (none) |
| playback | ADAT 7/8 | compRatio | restore | 0 | 1.0 | (none) |
| playback | ADAT 7/8 | expTrsh | probe | 0.75 | -47.5 dB | (none) |
| playback | ADAT 7/8 | expTrsh | restore | 0.5652 | -60.0 dB | (none) |
| playback | ADAT 7/8 | expRatio | probe | 0.75 | 7.7 | (none) |
| playback | ADAT 7/8 | expRatio | restore | 0 | 1.0 | (none) |
| playback | ADAT 7/8 | compexpEnable | toggle on | 1 |  | (none) |
| playback | ADAT 7/8 | compexpEnable | toggle off | 1 |  | (none) |
| playback | ADAT 7/8 | alevMaxgain | probe | 0.75 | 13.5 dB | (none) |
| playback | ADAT 7/8 | alevMaxgain | restore | 0.3333 | 6.0 dB | (none) |
| playback | ADAT 7/8 | alevHeadroom | probe | 0.75 | 9.5 dB | (none) |
| playback | ADAT 7/8 | alevHeadroom | restore | 0.3333 | 6.0 dB | (none) |
| playback | ADAT 7/8 | alevRisetime | probe | 0.75 | 7.4 s | (none) |
| playback | ADAT 7/8 | alevRisetime | restore | 0.5 | 5.0 s | (none) |
| output | AN 1/2 | eqType1 | sweep | 0 | Bell | 1241:0000 1341:0000 |
| output | AN 1/2 | eqType1 | sweep | 0.25 | Bell | (none) |
| output | AN 1/2 | eqType1 | sweep | 0.5 | Shelf | 1241:0001 1341:0001 |
| output | AN 1/2 | eqType1 | sweep | 0.75 | Shelf | (none) |
| output | AN 1/2 | eqType1 | restore | 1 | Shelf | (none) |
| output | AN 1/2 | eqGain1 | sweep | 0 | -20.0 dB | 1242:ff38 1342:ff38 |
| output | AN 1/2 | eqGain1 | sweep | 0.25 | -10.0 dB | 1242:ff9c 1342:ff9c |
| output | AN 1/2 | eqGain1 | sweep | 0.75 | 10.0 dB | 1242:0064 1342:0064 |
| output | AN 1/2 | eqGain1 | sweep | 1 | 20.0 dB | 1242:00c8 1342:00c8 |
| output | AN 1/2 | eqGain1 | restore | 0.5 | 0.0 dB | 1242:0000 1342:0000 |
| output | AN 1/2 | eqFreq1 | sweep | 0 | 20 Hz | 1243:0014 1343:0014 |
| output | AN 1/2 | eqFreq1 | sweep | 0.25 | 112 Hz | 1243:0070 1343:0070 |
| output | AN 1/2 | eqFreq1 | sweep | 0.5 | 632 Hz | 1243:0278 1343:0278 |
| output | AN 1/2 | eqFreq1 | sweep | 0.75 | 3557 Hz | 1243:0de5 1343:0de5 |
| output | AN 1/2 | eqFreq1 | sweep | 1 | 20000 Hz | 1243:4e20 1343:4e20 |
| output | AN 1/2 | eqFreq1 | restore | 0.2007 | 80 Hz | 1243:0050 1343:0050 |
| output | AN 1/2 | eqQ1 | sweep | 0 | 0.7 | 1244:0007 1344:0007 |
| output | AN 1/2 | eqQ1 | sweep | 0.25 | 1.7 | 1244:0011 1344:0011 |
| output | AN 1/2 | eqQ1 | sweep | 0.5 | 2.8 | 1244:001c 1344:001c |
| output | AN 1/2 | eqQ1 | sweep | 0.75 | 3.9 | 1244:0027 1344:0027 |
| output | AN 1/2 | eqQ1 | sweep | 1 | 5.0 | 1244:0032 1344:0032 |
| output | AN 1/2 | eqQ1 | restore | 0.06977 | 1.0 | 1244:000a 1344:000a |
| output | AN 1/2 | eqGain2 | sweep | 0 | -20.0 dB | 1245:ff38 1345:ff38 |
| output | AN 1/2 | eqGain2 | sweep | 0.25 | -10.0 dB | 1245:ff9c 1345:ff9c |
| output | AN 1/2 | eqGain2 | sweep | 0.75 | 10.0 dB | 1245:0064 1345:0064 |
| output | AN 1/2 | eqGain2 | sweep | 1 | 20.0 dB | 1245:00c8 1345:00c8 |
| output | AN 1/2 | eqGain2 | restore | 0.5 | 0.0 dB | 1245:0000 1345:0000 |
| output | AN 1/2 | eqFreq2 | sweep | 0 | 20 Hz | 1246:0014 1346:0014 |
| output | AN 1/2 | eqFreq2 | sweep | 0.25 | 112 Hz | 1246:0070 1346:0070 |
| output | AN 1/2 | eqFreq2 | sweep | 0.5 | 632 Hz | 1246:0278 1346:0278 |
| output | AN 1/2 | eqFreq2 | sweep | 0.75 | 3557 Hz | 1246:0de5 1346:0de5 |
| output | AN 1/2 | eqFreq2 | sweep | 1 | 20000 Hz | 1246:4e20 1346:4e20 |
| output | AN 1/2 | eqFreq2 | restore | 0.5663 | 1000 Hz | 1246:03e8 1346:03e8 |
| output | AN 1/2 | eqQ2 | sweep | 0 | 0.7 | 1247:0007 1347:0007 |
| output | AN 1/2 | eqQ2 | sweep | 0.25 | 1.7 | 1247:0011 1347:0011 |
| output | AN 1/2 | eqQ2 | sweep | 0.5 | 2.8 | 1247:001c 1347:001c |
| output | AN 1/2 | eqQ2 | sweep | 0.75 | 3.9 | 1247:0027 1347:0027 |
| output | AN 1/2 | eqQ2 | sweep | 1 | 5.0 | 1247:0032 1347:0032 |
| output | AN 1/2 | eqQ2 | restore | 0.06977 | 1.0 | 1247:000a 1347:000a |
| output | AN 1/2 | eqType3 | sweep | 0 | Bell | 1248:0000 1348:0000 |
| output | AN 1/2 | eqType3 | sweep | 0.25 | Shelf | 1248:0001 1348:0001 |
| output | AN 1/2 | eqType3 | sweep | 0.75 | Low Pass | 1248:0002 1348:0002 |
| output | AN 1/2 | eqType3 | sweep | 1 | Low Pass | (none) |
| output | AN 1/2 | eqType3 | restore | 0.5 | Shelf | 1248:0001 1348:0001 |
| output | AN 1/2 | eqGain3 | sweep | 0 | -20.0 dB | 1249:ff38 1349:ff38 |
| output | AN 1/2 | eqGain3 | sweep | 0.25 | -10.0 dB | 1249:ff9c 1349:ff9c |
| output | AN 1/2 | eqGain3 | sweep | 0.75 | 10.0 dB | 1249:0064 1349:0064 |
| output | AN 1/2 | eqGain3 | sweep | 1 | 20.0 dB | 1249:00c8 1349:00c8 |
| output | AN 1/2 | eqGain3 | restore | 0.5 | 0.0 dB | 1249:0000 1349:0000 |
| output | AN 1/2 | eqFreq3 | sweep | 0 | 20 Hz | 124a:0014 134a:0014 |
| output | AN 1/2 | eqFreq3 | sweep | 0.25 | 112 Hz | 124a:0070 134a:0070 |
| output | AN 1/2 | eqFreq3 | sweep | 0.5 | 632 Hz | 124a:0278 134a:0278 |
| output | AN 1/2 | eqFreq3 | sweep | 0.75 | 3557 Hz | 124a:0de5 134a:0de5 |
| output | AN 1/2 | eqFreq3 | sweep | 1 | 20000 Hz | 124a:4e20 134a:4e20 |
| output | AN 1/2 | eqFreq3 | restore | 0.7993 | 5000 Hz | 124a:1388 134a:1388 |
| output | AN 1/2 | eqQ3 | sweep | 0 | 0.7 | 124b:0007 134b:0007 |
| output | AN 1/2 | eqQ3 | sweep | 0.25 | 1.7 | 124b:0011 134b:0011 |
| output | AN 1/2 | eqQ3 | sweep | 0.5 | 2.8 | 124b:001c 134b:001c |
| output | AN 1/2 | eqQ3 | sweep | 0.75 | 3.9 | 124b:0027 134b:0027 |
| output | AN 1/2 | eqQ3 | sweep | 1 | 5.0 | 124b:0032 134b:0032 |
| output | AN 1/2 | eqQ3 | restore | 0.06977 | 1.0 | 124b:000a 134b:000a |
| output | AN 1/2 | eqEnable | toggle on | 1 |  | 1240:0001 1340:0001 |
| output | AN 1/2 | eqEnable | toggle off | 1 |  | 1240:0000 1340:0000 |
| output | AN 1/2 | lowcutFreq | sweep | 0.25 | 45 Hz | 1221:002d 1321:002d |
| output | AN 1/2 | lowcutFreq | sweep | 0.5 | 100 Hz | 1221:0064 1321:0064 |
| output | AN 1/2 | lowcutFreq | sweep | 0.75 | 224 Hz | 1221:00e0 1321:00e0 |
| output | AN 1/2 | lowcutFreq | sweep | 1 | 500 Hz | 1221:01f4 1321:01f4 |
| output | AN 1/2 | lowcutFreq | restore | 0 | 20 Hz | 1221:0014 1321:0014 |
| output | AN 1/2 | lowcutGrade | sweep | 0 | 6 dB/oct | 1222:0000 1322:0000 |
| output | AN 1/2 | lowcutGrade | sweep | 0.25 | 12 dB/oct | 1222:0001 1322:0001 |
| output | AN 1/2 | lowcutGrade | sweep | 0.5 | 18 dB/oct | 1222:0002 1322:0002 |
| output | AN 1/2 | lowcutGrade | sweep | 0.75 | 18 dB/oct | (none) |
| output | AN 1/2 | lowcutGrade | sweep | 1 | 24 dB/oct | 1222:0003 1322:0003 |
| output | AN 1/2 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 1222:0001 1322:0001 |
| output | AN 1/2 | lowcutEnable | toggle on | 1 |  | 1220:0001 1320:0001 |
| output | AN 1/2 | lowcutEnable | toggle off | 1 |  | 1220:0000 1320:0000 |
| output | AN 1/2 | compexpGain | sweep | 0 | -30.0 dB | 1261:fed4 1361:fed4 |
| output | AN 1/2 | compexpGain | sweep | 0.25 | -15.0 dB | 1261:ff6a 1361:ff6a |
| output | AN 1/2 | compexpGain | sweep | 0.75 | 15.0 dB | 1261:0096 1361:0096 |
| output | AN 1/2 | compexpGain | sweep | 1 | 30.0 dB | 1261:012c 1361:012c |
| output | AN 1/2 | compexpGain | restore | 0.5 | 0.0 dB | 1261:0000 1361:0000 |
| output | AN 1/2 | compexpAttack | sweep | 0 | 0 ms | 1262:0000 1362:0000 |
| output | AN 1/2 | compexpAttack | sweep | 0.25 | 50 ms | 1262:0032 1362:0032 |
| output | AN 1/2 | compexpAttack | sweep | 0.5 | 100 ms | 1262:0064 1362:0064 |
| output | AN 1/2 | compexpAttack | sweep | 0.75 | 150 ms | 1262:0096 1362:0096 |
| output | AN 1/2 | compexpAttack | sweep | 1 | 200 ms | 1262:00c8 1362:00c8 |
| output | AN 1/2 | compexpAttack | restore | 0.05 | 10 ms | 1262:000a 1362:000a |
| output | AN 1/2 | compexpRelease | sweep | 0 | 100 ms | 1263:0064 1363:0064 |
| output | AN 1/2 | compexpRelease | sweep | 0.25 | 325 ms | 1263:0145 1363:0145 |
| output | AN 1/2 | compexpRelease | sweep | 0.5 | 550 ms | 1263:0226 1363:0226 |
| output | AN 1/2 | compexpRelease | sweep | 0.75 | 774 ms | 1263:0306 1363:0306 |
| output | AN 1/2 | compexpRelease | sweep | 1 | 999 ms | 1263:03e7 1363:03e7 |
| output | AN 1/2 | compexpRelease | restore | 0.2225 | 300 ms | 1263:012c 1363:012c |
| output | AN 1/2 | compTrsh | sweep | 0 | -60.0 dB | 1264:fda8 1364:fda8 |
| output | AN 1/2 | compTrsh | sweep | 0.25 | -45.0 dB | 1264:fe3e 1364:fe3e |
| output | AN 1/2 | compTrsh | sweep | 0.75 | -15.0 dB | 1264:ff6a 1364:ff6a |
| output | AN 1/2 | compTrsh | sweep | 1 | 0.0 dB | 1264:0000 1364:0000 |
| output | AN 1/2 | compTrsh | restore | 0.5 | -30.0 dB | 1264:fed4 1364:fed4 |
| output | AN 1/2 | compRatio | sweep | 0.25 | 3.2 | 1265:0020 1365:0020 |
| output | AN 1/2 | compRatio | sweep | 0.5 | 5.5 | 1265:0037 1365:0037 |
| output | AN 1/2 | compRatio | sweep | 0.75 | 7.7 | 1265:004d 1365:004d |
| output | AN 1/2 | compRatio | sweep | 1 | 10.0 | 1265:0064 1365:0064 |
| output | AN 1/2 | compRatio | restore | 0 | 1.0 | 1265:000a 1365:000a |
| output | AN 1/2 | expTrsh | sweep | 0 | -99.0 dB | 1266:fc22 1366:fc22 |
| output | AN 1/2 | expTrsh | sweep | 0.25 | -82.0 dB | 1266:fccc 1366:fccc |
| output | AN 1/2 | expTrsh | sweep | 0.5 | -64.5 dB | 1266:fd7b 1366:fd7b |
| output | AN 1/2 | expTrsh | sweep | 0.75 | -47.5 dB | 1266:fe25 1366:fe25 |
| output | AN 1/2 | expTrsh | sweep | 1 | -30.0 dB | 1266:fed4 1366:fed4 |
| output | AN 1/2 | expTrsh | restore | 0.5652 | -60.0 dB | 1266:fda8 1366:fda8 |
| output | AN 1/2 | expRatio | sweep | 0.25 | 3.2 | 1267:0020 1367:0020 |
| output | AN 1/2 | expRatio | sweep | 0.5 | 5.5 | 1267:0037 1367:0037 |
| output | AN 1/2 | expRatio | sweep | 0.75 | 7.7 | 1267:004d 1367:004d |
| output | AN 1/2 | expRatio | sweep | 1 | 10.0 | 1267:0064 1367:0064 |
| output | AN 1/2 | expRatio | restore | 0 | 1.0 | 1267:000a 1367:000a |
| output | AN 1/2 | compexpEnable | toggle on | 1 |  | 1260:0001 1360:0001 |
| output | AN 1/2 | compexpEnable | toggle off | 1 |  | 1260:0000 1360:0000 |
| output | AN 1/2 | alevMaxgain | sweep | 0 | 0.0 dB | 1281:0000 1381:0000 |
| output | AN 1/2 | alevMaxgain | sweep | 0.25 | 4.5 dB | 1281:002d 1381:002d |
| output | AN 1/2 | alevMaxgain | sweep | 0.5 | 9.0 dB | 1281:005a 1381:005a |
| output | AN 1/2 | alevMaxgain | sweep | 0.75 | 13.5 dB | 1281:0087 1381:0087 |
| output | AN 1/2 | alevMaxgain | sweep | 1 | 18.0 dB | 1281:00b4 1381:00b4 |
| output | AN 1/2 | alevMaxgain | restore | 0.3333 | 6.0 dB | 1281:003c 1381:003c |
| output | AN 1/2 | alevHeadroom | sweep | 0 | 3.0 dB | 1282:001e 1382:001e |
| output | AN 1/2 | alevHeadroom | sweep | 0.25 | 5.0 dB | 1282:0032 1382:0032 |
| output | AN 1/2 | alevHeadroom | sweep | 0.5 | 7.5 dB | 1282:004b 1382:004b |
| output | AN 1/2 | alevHeadroom | sweep | 0.75 | 9.5 dB | 1282:005f 1382:005f |
| output | AN 1/2 | alevHeadroom | sweep | 1 | 12.0 dB | 1282:0078 1382:0078 |
| output | AN 1/2 | alevHeadroom | restore | 0.3333 | 6.0 dB | 1282:003c 1382:003c |
| output | AN 1/2 | alevRisetime | sweep | 0 | 0.1 s | 1283:0001 1383:0001 |
| output | AN 1/2 | alevRisetime | sweep | 0.25 | 2.5 s | 1283:0019 1383:0019 |
| output | AN 1/2 | alevRisetime | sweep | 0.75 | 7.4 s | 1283:004a 1383:004a |
| output | AN 1/2 | alevRisetime | sweep | 1 | 9.9 s | 1283:0063 1383:0063 |
| output | AN 1/2 | alevRisetime | restore | 0.5 | 5.0 s | 1283:0032 1383:0032 |
| output | AN 3/4 | eqType1 | probe | 0.75 | Shelf | (none) |
| output | AN 3/4 | eqType1 | restore | 1 | Shelf | (none) |
| output | AN 3/4 | eqGain1 | probe | 0.75 | 10.0 dB | 1442:0064 1542:0064 |
| output | AN 3/4 | eqGain1 | restore | 0.5 | 0.0 dB | 1442:0000 1542:0000 |
| output | AN 3/4 | eqFreq1 | probe | 0.75 | 3557 Hz | 1443:0de5 1543:0de5 |
| output | AN 3/4 | eqFreq1 | restore | 0.2007 | 80 Hz | 1443:0050 1543:0050 |
| output | AN 3/4 | eqQ1 | probe | 0.75 | 3.9 | 1444:0027 1544:0027 |
| output | AN 3/4 | eqQ1 | restore | 0.06977 | 1.0 | 1444:000a 1544:000a |
| output | AN 3/4 | eqGain2 | probe | 0.75 | 10.0 dB | 1445:0064 1545:0064 |
| output | AN 3/4 | eqGain2 | restore | 0.5 | 0.0 dB | 1445:0000 1545:0000 |
| output | AN 3/4 | eqFreq2 | probe | 0.75 | 3557 Hz | 1446:0de5 1546:0de5 |
| output | AN 3/4 | eqFreq2 | restore | 0.5663 | 1000 Hz | 1446:03e8 1546:03e8 |
| output | AN 3/4 | eqQ2 | probe | 0.75 | 3.9 | 1447:0027 1547:0027 |
| output | AN 3/4 | eqQ2 | restore | 0.06977 | 1.0 | 1447:000a 1547:000a |
| output | AN 3/4 | eqType3 | probe | 0.75 | Low Pass | 1448:0002 1548:0002 |
| output | AN 3/4 | eqType3 | restore | 0.5 | Shelf | 1448:0001 1548:0001 |
| output | AN 3/4 | eqGain3 | probe | 0.75 | 10.0 dB | 1449:0064 1549:0064 |
| output | AN 3/4 | eqGain3 | restore | 0.5 | 0.0 dB | 1449:0000 1549:0000 |
| output | AN 3/4 | eqFreq3 | probe | 0.75 | 3557 Hz | 144a:0de5 154a:0de5 |
| output | AN 3/4 | eqFreq3 | restore | 0.7993 | 5000 Hz | 144a:1388 154a:1388 |
| output | AN 3/4 | eqQ3 | probe | 0.75 | 3.9 | 144b:0027 154b:0027 |
| output | AN 3/4 | eqQ3 | restore | 0.06977 | 1.0 | 144b:000a 154b:000a |
| output | AN 3/4 | eqEnable | toggle on | 1 |  | 1440:0001 1540:0001 |
| output | AN 3/4 | eqEnable | toggle off | 1 |  | 1440:0000 1540:0000 |
| output | AN 3/4 | lowcutFreq | probe | 0.75 | 224 Hz | 1421:00e0 1521:00e0 |
| output | AN 3/4 | lowcutFreq | restore | 0 | 20 Hz | 1421:0014 1521:0014 |
| output | AN 3/4 | lowcutGrade | probe | 0.75 | 18 dB/oct | 1422:0002 1522:0002 |
| output | AN 3/4 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 1422:0001 1522:0001 |
| output | AN 3/4 | lowcutEnable | toggle on | 1 |  | 1420:0001 1520:0001 |
| output | AN 3/4 | lowcutEnable | toggle off | 1 |  | 1420:0000 1520:0000 |
| output | AN 3/4 | compexpGain | probe | 0.75 | 15.0 dB | 1461:0096 1561:0096 |
| output | AN 3/4 | compexpGain | restore | 0.5 | 0.0 dB | 1461:0000 1561:0000 |
| output | AN 3/4 | compexpAttack | probe | 0.75 | 150 ms | 1462:0096 1562:0096 |
| output | AN 3/4 | compexpAttack | restore | 0.05 | 10 ms | 1462:000a 1562:000a |
| output | AN 3/4 | compexpRelease | probe | 0.75 | 774 ms | 1463:0306 1563:0306 |
| output | AN 3/4 | compexpRelease | restore | 0.2225 | 300 ms | 1463:012c 1563:012c |
| output | AN 3/4 | compTrsh | probe | 0.75 | -15.0 dB | 1464:ff6a 1564:ff6a |
| output | AN 3/4 | compTrsh | restore | 0.5 | -30.0 dB | 1464:fed4 1564:fed4 |
| output | AN 3/4 | compRatio | probe | 0.75 | 7.7 | 1465:004d 1565:004d |
| output | AN 3/4 | compRatio | restore | 0 | 1.0 | 1465:000a 1565:000a |
| output | AN 3/4 | expTrsh | probe | 0.75 | -47.5 dB | 1466:fe25 1566:fe25 |
| output | AN 3/4 | expTrsh | restore | 0.5652 | -60.0 dB | 1466:fda8 1566:fda8 |
| output | AN 3/4 | expRatio | probe | 0.75 | 7.7 | 1467:004d 1567:004d |
| output | AN 3/4 | expRatio | restore | 0 | 1.0 | 1467:000a 1567:000a |
| output | AN 3/4 | compexpEnable | toggle on | 1 |  | 1460:0001 1560:0001 |
| output | AN 3/4 | compexpEnable | toggle off | 1 |  | 1460:0000 1560:0000 |
| output | AN 3/4 | alevMaxgain | probe | 0.75 | 13.5 dB | 1481:0087 1581:0087 |
| output | AN 3/4 | alevMaxgain | restore | 0.3333 | 6.0 dB | 1481:003c 1581:003c |
| output | AN 3/4 | alevHeadroom | probe | 0.75 | 9.5 dB | 1482:005f 1582:005f |
| output | AN 3/4 | alevHeadroom | restore | 0.3333 | 6.0 dB | 1482:003c 1582:003c |
| output | AN 3/4 | alevRisetime | probe | 0.75 | 7.4 s | 1483:004a 1583:004a |
| output | AN 3/4 | alevRisetime | restore | 0.5 | 5.0 s | 1483:0032 1583:0032 |
| output | AN 5/6 | eqType1 | probe | 0.75 | Shelf | (none) |
| output | AN 5/6 | eqType1 | restore | 1 | Shelf | (none) |
| output | AN 5/6 | eqGain1 | probe | 0.75 | 10.0 dB | 1642:0064 1742:0064 |
| output | AN 5/6 | eqGain1 | restore | 0.5 | 0.0 dB | 1642:0000 1742:0000 |
| output | AN 5/6 | eqFreq1 | probe | 0.75 | 3557 Hz | 1643:0de5 1743:0de5 |
| output | AN 5/6 | eqFreq1 | restore | 0.2007 | 80 Hz | 1643:0050 1743:0050 |
| output | AN 5/6 | eqQ1 | probe | 0.75 | 3.9 | 1644:0027 1744:0027 |
| output | AN 5/6 | eqQ1 | restore | 0.06977 | 1.0 | 1644:000a 1744:000a |
| output | AN 5/6 | eqGain2 | probe | 0.75 | 10.0 dB | 1645:0064 1745:0064 |
| output | AN 5/6 | eqGain2 | restore | 0.5 | 0.0 dB | 1645:0000 1745:0000 |
| output | AN 5/6 | eqFreq2 | probe | 0.75 | 3557 Hz | 1646:0de5 1746:0de5 |
| output | AN 5/6 | eqFreq2 | restore | 0.5663 | 1000 Hz | 1646:03e8 1746:03e8 |
| output | AN 5/6 | eqQ2 | probe | 0.75 | 3.9 | 1647:0027 1747:0027 |
| output | AN 5/6 | eqQ2 | restore | 0.06977 | 1.0 | 1647:000a 1747:000a |
| output | AN 5/6 | eqType3 | probe | 0.75 | Low Pass | 1648:0002 1748:0002 |
| output | AN 5/6 | eqType3 | restore | 0.5 | Shelf | 1648:0001 1748:0001 |
| output | AN 5/6 | eqGain3 | probe | 0.75 | 10.0 dB | 1649:0064 1749:0064 |
| output | AN 5/6 | eqGain3 | restore | 0.5 | 0.0 dB | 1649:0000 1749:0000 |
| output | AN 5/6 | eqFreq3 | probe | 0.75 | 3557 Hz | 164a:0de5 174a:0de5 |
| output | AN 5/6 | eqFreq3 | restore | 0.7993 | 5000 Hz | 164a:1388 174a:1388 |
| output | AN 5/6 | eqQ3 | probe | 0.75 | 3.9 | 164b:0027 174b:0027 |
| output | AN 5/6 | eqQ3 | restore | 0.06977 | 1.0 | 164b:000a 174b:000a |
| output | AN 5/6 | eqEnable | toggle on | 1 |  | 1640:0001 1740:0001 |
| output | AN 5/6 | eqEnable | toggle off | 1 |  | 1640:0000 1740:0000 |
| output | AN 5/6 | lowcutFreq | probe | 0.75 | 224 Hz | 1621:00e0 1721:00e0 |
| output | AN 5/6 | lowcutFreq | restore | 0 | 20 Hz | 1621:0014 1721:0014 |
| output | AN 5/6 | lowcutGrade | probe | 0.75 | 18 dB/oct | 1622:0002 1722:0002 |
| output | AN 5/6 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 1622:0001 1722:0001 |
| output | AN 5/6 | lowcutEnable | toggle on | 1 |  | 1620:0001 1720:0001 |
| output | AN 5/6 | lowcutEnable | toggle off | 1 |  | 1620:0000 1720:0000 |
| output | AN 5/6 | compexpGain | probe | 0.75 | 15.0 dB | 1661:0096 1761:0096 |
| output | AN 5/6 | compexpGain | restore | 0.5 | 0.0 dB | 1661:0000 1761:0000 |
| output | AN 5/6 | compexpAttack | probe | 0.75 | 150 ms | 1662:0096 1762:0096 |
| output | AN 5/6 | compexpAttack | restore | 0.05 | 10 ms | 1662:000a 1762:000a |
| output | AN 5/6 | compexpRelease | probe | 0.75 | 774 ms | 1663:0306 1763:0306 |
| output | AN 5/6 | compexpRelease | restore | 0.2225 | 300 ms | 1663:012c 1763:012c |
| output | AN 5/6 | compTrsh | probe | 0.75 | -15.0 dB | 1664:ff6a 1764:ff6a |
| output | AN 5/6 | compTrsh | restore | 0.5 | -30.0 dB | 1664:fed4 1764:fed4 |
| output | AN 5/6 | compRatio | probe | 0.75 | 7.7 | 1665:004d 1765:004d |
| output | AN 5/6 | compRatio | restore | 0 | 1.0 | 1665:000a 1765:000a |
| output | AN 5/6 | expTrsh | probe | 0.75 | -47.5 dB | 1666:fe25 1766:fe25 |
| output | AN 5/6 | expTrsh | restore | 0.5652 | -60.0 dB | 1666:fda8 1766:fda8 |
| output | AN 5/6 | expRatio | probe | 0.75 | 7.7 | 1667:004d 1767:004d |
| output | AN 5/6 | expRatio | restore | 0 | 1.0 | 1667:000a 1767:000a |
| output | AN 5/6 | compexpEnable | toggle on | 1 |  | 1660:0001 1760:0001 |
| output | AN 5/6 | compexpEnable | toggle off | 1 |  | 1660:0000 1760:0000 |
| output | AN 5/6 | alevMaxgain | probe | 0.75 | 13.5 dB | 1681:0087 1781:0087 |
| output | AN 5/6 | alevMaxgain | restore | 0.3333 | 6.0 dB | 1681:003c 1781:003c |
| output | AN 5/6 | alevHeadroom | probe | 0.75 | 9.5 dB | 1682:005f 1782:005f |
| output | AN 5/6 | alevHeadroom | restore | 0.3333 | 6.0 dB | 1682:003c 1782:003c |
| output | AN 5/6 | alevRisetime | probe | 0.75 | 7.4 s | 1683:004a 1783:004a |
| output | AN 5/6 | alevRisetime | restore | 0.5 | 5.0 s | 1683:0032 1783:0032 |
| output | Main | eqType1 | probe | 0.75 | Shelf | (none) |
| output | Main | eqType1 | restore | 1 | Shelf | (none) |
| output | Main | eqGain1 | probe | 0.75 | 10.0 dB | 1842:0064 1942:0064 |
| output | Main | eqGain1 | restore | 0.5 | 0.0 dB | 1842:0000 1942:0000 |
| output | Main | eqFreq1 | probe | 0.75 | 3557 Hz | 1843:0de5 1943:0de5 |
| output | Main | eqFreq1 | restore | 0.2007 | 80 Hz | 1843:0050 1943:0050 |
| output | Main | eqQ1 | probe | 0.75 | 3.9 | 1844:0027 1944:0027 |
| output | Main | eqQ1 | restore | 0.06977 | 1.0 | 1844:000a 1944:000a |
| output | Main | eqGain2 | probe | 0.75 | 10.0 dB | 1845:0064 1945:0064 |
| output | Main | eqGain2 | restore | 0.5 | 0.0 dB | 1845:0000 1945:0000 |
| output | Main | eqFreq2 | probe | 0.75 | 3557 Hz | 1846:0de5 1946:0de5 |
| output | Main | eqFreq2 | restore | 0.5663 | 1000 Hz | 1846:03e8 1946:03e8 |
| output | Main | eqQ2 | probe | 0.75 | 3.9 | 1847:0027 1947:0027 |
| output | Main | eqQ2 | restore | 0.06977 | 1.0 | 1847:000a 1947:000a |
| output | Main | eqType3 | probe | 0.75 | Low Pass | 1848:0002 1948:0002 |
| output | Main | eqType3 | restore | 0.5 | Shelf | 1848:0001 1948:0001 |
| output | Main | eqGain3 | probe | 0.75 | 10.0 dB | 1849:0064 1949:0064 |
| output | Main | eqGain3 | restore | 0.5 | 0.0 dB | 1849:0000 1949:0000 |
| output | Main | eqFreq3 | probe | 0.75 | 3557 Hz | 184a:0de5 194a:0de5 |
| output | Main | eqFreq3 | restore | 0.7993 | 5000 Hz | 184a:1388 194a:1388 |
| output | Main | eqQ3 | probe | 0.75 | 3.9 | 184b:0027 194b:0027 |
| output | Main | eqQ3 | restore | 0.06977 | 1.0 | 184b:000a 194b:000a |
| output | Main | eqEnable | toggle on | 1 |  | 1840:0001 1940:0001 |
| output | Main | eqEnable | toggle off | 1 |  | 1840:0000 1940:0000 |
| output | Main | lowcutFreq | probe | 0.75 | 224 Hz | 1821:00e0 1921:00e0 |
| output | Main | lowcutFreq | restore | 0 | 20 Hz | 1821:0014 1921:0014 |
| output | Main | lowcutGrade | probe | 0.75 | 18 dB/oct | 1822:0002 1922:0002 |
| output | Main | lowcutGrade | restore | 0.3333 | 12 dB/oct | 1822:0001 1922:0001 |
| output | Main | lowcutEnable | toggle on | 1 |  | 1820:0001 1920:0001 |
| output | Main | lowcutEnable | toggle off | 1 |  | 1820:0000 1920:0000 |
| output | Main | compexpGain | probe | 0.75 | 15.0 dB | 1861:0096 1961:0096 |
| output | Main | compexpGain | restore | 0.5 | 0.0 dB | 1861:0000 1961:0000 |
| output | Main | compexpAttack | probe | 0.75 | 150 ms | 1862:0096 1962:0096 |
| output | Main | compexpAttack | restore | 0.05 | 10 ms | 1862:000a 1962:000a |
| output | Main | compexpRelease | probe | 0.75 | 774 ms | 1863:0306 1963:0306 |
| output | Main | compexpRelease | restore | 0.2225 | 300 ms | 1863:012c 1963:012c |
| output | Main | compTrsh | probe | 0.75 | -15.0 dB | 1864:ff6a 1964:ff6a |
| output | Main | compTrsh | restore | 0.5 | -30.0 dB | 1864:fed4 1964:fed4 |
| output | Main | compRatio | probe | 0.75 | 7.7 | 1865:004d 1965:004d |
| output | Main | compRatio | restore | 0 | 1.0 | 1865:000a 1965:000a |
| output | Main | expTrsh | probe | 0.75 | -47.5 dB | 1866:fe25 1966:fe25 |
| output | Main | expTrsh | restore | 0.5652 | -60.0 dB | 1866:fda8 1966:fda8 |
| output | Main | expRatio | probe | 0.75 | 7.7 | 1867:004d 1967:004d |
| output | Main | expRatio | restore | 0 | 1.0 | 1867:000a 1967:000a |
| output | Main | compexpEnable | toggle on | 1 |  | 1860:0001 1960:0001 |
| output | Main | compexpEnable | toggle off | 1 |  | 1860:0000 1960:0000 |
| output | Main | alevMaxgain | probe | 0.75 | 13.5 dB | 1881:0087 1981:0087 |
| output | Main | alevMaxgain | restore | 0.3333 | 6.0 dB | 1881:003c 1981:003c |
| output | Main | alevHeadroom | probe | 0.75 | 9.5 dB | 1882:005f 1982:005f |
| output | Main | alevHeadroom | restore | 0.3333 | 6.0 dB | 1882:003c 1982:003c |
| output | Main | alevRisetime | probe | 0.75 | 7.4 s | 1883:004a 1983:004a |
| output | Main | alevRisetime | restore | 0.5 | 5.0 s | 1883:0032 1983:0032 |
| output | SPDIF | eqType1 | probe | 0.75 | Shelf | (none) |
| output | SPDIF | eqType1 | restore | 1 | Shelf | (none) |
| output | SPDIF | eqGain1 | probe | 0.75 | 10.0 dB | 1a42:0064 1b42:0064 |
| output | SPDIF | eqGain1 | restore | 0.5 | 0.0 dB | 1a42:0000 1b42:0000 |
| output | SPDIF | eqFreq1 | probe | 0.75 | 3557 Hz | 1a43:0de5 1b43:0de5 |
| output | SPDIF | eqFreq1 | restore | 0.2007 | 80 Hz | 1a43:0050 1b43:0050 |
| output | SPDIF | eqQ1 | probe | 0.75 | 3.9 | 1a44:0027 1b44:0027 |
| output | SPDIF | eqQ1 | restore | 0.06977 | 1.0 | 1a44:000a 1b44:000a |
| output | SPDIF | eqGain2 | probe | 0.75 | 10.0 dB | 1a45:0064 1b45:0064 |
| output | SPDIF | eqGain2 | restore | 0.5 | 0.0 dB | 1a45:0000 1b45:0000 |
| output | SPDIF | eqFreq2 | probe | 0.75 | 3557 Hz | 1a46:0de5 1b46:0de5 |
| output | SPDIF | eqFreq2 | restore | 0.5663 | 1000 Hz | 1a46:03e8 1b46:03e8 |
| output | SPDIF | eqQ2 | probe | 0.75 | 3.9 | 1a47:0027 1b47:0027 |
| output | SPDIF | eqQ2 | restore | 0.06977 | 1.0 | 1a47:000a 1b47:000a |
| output | SPDIF | eqType3 | probe | 0.75 | Low Pass | 1a48:0002 1b48:0002 |
| output | SPDIF | eqType3 | restore | 0.5 | Shelf | 1a48:0001 1b48:0001 |
| output | SPDIF | eqGain3 | probe | 0.75 | 10.0 dB | 1a49:0064 1b49:0064 |
| output | SPDIF | eqGain3 | restore | 0.5 | 0.0 dB | 1a49:0000 1b49:0000 |
| output | SPDIF | eqFreq3 | probe | 0.75 | 3557 Hz | 1a4a:0de5 1b4a:0de5 |
| output | SPDIF | eqFreq3 | restore | 0.7993 | 5000 Hz | 1a4a:1388 1b4a:1388 |
| output | SPDIF | eqQ3 | probe | 0.75 | 3.9 | 1a4b:0027 1b4b:0027 |
| output | SPDIF | eqQ3 | restore | 0.06977 | 1.0 | 1a4b:000a 1b4b:000a |
| output | SPDIF | eqEnable | toggle on | 1 |  | 1a40:0001 1b40:0001 |
| output | SPDIF | eqEnable | toggle off | 1 |  | 1a40:0000 1b40:0000 |
| output | SPDIF | lowcutFreq | probe | 0.75 | 224 Hz | 1a21:00e0 1b21:00e0 |
| output | SPDIF | lowcutFreq | restore | 0 | 20 Hz | 1a21:0014 1b21:0014 |
| output | SPDIF | lowcutGrade | probe | 0.75 | 18 dB/oct | 1a22:0002 1b22:0002 |
| output | SPDIF | lowcutGrade | restore | 0.3333 | 12 dB/oct | 1a22:0001 1b22:0001 |
| output | SPDIF | lowcutEnable | toggle on | 1 |  | 1a20:0001 1b20:0001 |
| output | SPDIF | lowcutEnable | toggle off | 1 |  | 1a20:0000 1b20:0000 |
| output | SPDIF | compexpGain | probe | 0.75 | 15.0 dB | 1a61:0096 1b61:0096 |
| output | SPDIF | compexpGain | restore | 0.5 | 0.0 dB | 1a61:0000 1b61:0000 |
| output | SPDIF | compexpAttack | probe | 0.75 | 150 ms | 1a62:0096 1b62:0096 |
| output | SPDIF | compexpAttack | restore | 0.05 | 10 ms | 1a62:000a 1b62:000a |
| output | SPDIF | compexpRelease | probe | 0.75 | 774 ms | 1a63:0306 1b63:0306 |
| output | SPDIF | compexpRelease | restore | 0.2225 | 300 ms | 1a63:012c 1b63:012c |
| output | SPDIF | compTrsh | probe | 0.75 | -15.0 dB | 1a64:ff6a 1b64:ff6a |
| output | SPDIF | compTrsh | restore | 0.5 | -30.0 dB | 1a64:fed4 1b64:fed4 |
| output | SPDIF | compRatio | probe | 0.75 | 7.7 | 1a65:004d 1b65:004d |
| output | SPDIF | compRatio | restore | 0 | 1.0 | 1a65:000a 1b65:000a |
| output | SPDIF | expTrsh | probe | 0.75 | -47.5 dB | 1a66:fe25 1b66:fe25 |
| output | SPDIF | expTrsh | restore | 0.5652 | -60.0 dB | 1a66:fda8 1b66:fda8 |
| output | SPDIF | expRatio | probe | 0.75 | 7.7 | 1a67:004d 1b67:004d |
| output | SPDIF | expRatio | restore | 0 | 1.0 | 1a67:000a 1b67:000a |
| output | SPDIF | compexpEnable | toggle on | 1 |  | 1a60:0001 1b60:0001 |
| output | SPDIF | compexpEnable | toggle off | 1 |  | 1a60:0000 1b60:0000 |
| output | SPDIF | alevMaxgain | probe | 0.75 | 13.5 dB | 1a81:0087 1b81:0087 |
| output | SPDIF | alevMaxgain | restore | 0.3333 | 6.0 dB | 1a81:003c 1b81:003c |
| output | SPDIF | alevHeadroom | probe | 0.75 | 9.5 dB | 1a82:005f 1b82:005f |
| output | SPDIF | alevHeadroom | restore | 0.3333 | 6.0 dB | 1a82:003c 1b82:003c |
| output | SPDIF | alevRisetime | probe | 0.75 | 7.4 s | 1a83:004a 1b83:004a |
| output | SPDIF | alevRisetime | restore | 0.5 | 5.0 s | 1a83:0032 1b83:0032 |
| output | AS 1/2 | eqType1 | probe | 0.75 | Shelf | (none) |
| output | AS 1/2 | eqType1 | restore | 1 | Shelf | (none) |
| output | AS 1/2 | eqGain1 | probe | 0.75 | 10.0 dB | 1c42:0064 1d42:0064 |
| output | AS 1/2 | eqGain1 | restore | 0.5 | 0.0 dB | 1c42:0000 1d42:0000 |
| output | AS 1/2 | eqFreq1 | probe | 0.75 | 3557 Hz | 1c43:0de5 1d43:0de5 |
| output | AS 1/2 | eqFreq1 | restore | 0.2007 | 80 Hz | 1c43:0050 1d43:0050 |
| output | AS 1/2 | eqQ1 | probe | 0.75 | 3.9 | 1c44:0027 1d44:0027 |
| output | AS 1/2 | eqQ1 | restore | 0.06977 | 1.0 | 1c44:000a 1d44:000a |
| output | AS 1/2 | eqGain2 | probe | 0.75 | 10.0 dB | 1c45:0064 1d45:0064 |
| output | AS 1/2 | eqGain2 | restore | 0.5 | 0.0 dB | 1c45:0000 1d45:0000 |
| output | AS 1/2 | eqFreq2 | probe | 0.75 | 3557 Hz | 1c46:0de5 1d46:0de5 |
| output | AS 1/2 | eqFreq2 | restore | 0.5663 | 1000 Hz | 1c46:03e8 1d46:03e8 |
| output | AS 1/2 | eqQ2 | probe | 0.75 | 3.9 | 1c47:0027 1d47:0027 |
| output | AS 1/2 | eqQ2 | restore | 0.06977 | 1.0 | 1c47:000a 1d47:000a |
| output | AS 1/2 | eqType3 | probe | 0.75 | Low Pass | 1c48:0002 1d48:0002 |
| output | AS 1/2 | eqType3 | restore | 0.5 | Shelf | 1c48:0001 1d48:0001 |
| output | AS 1/2 | eqGain3 | probe | 0.75 | 10.0 dB | 1c49:0064 1d49:0064 |
| output | AS 1/2 | eqGain3 | restore | 0.5 | 0.0 dB | 1c49:0000 1d49:0000 |
| output | AS 1/2 | eqFreq3 | probe | 0.75 | 3557 Hz | 1c4a:0de5 1d4a:0de5 |
| output | AS 1/2 | eqFreq3 | restore | 0.7993 | 5000 Hz | 1c4a:1388 1d4a:1388 |
| output | AS 1/2 | eqQ3 | probe | 0.75 | 3.9 | 1c4b:0027 1d4b:0027 |
| output | AS 1/2 | eqQ3 | restore | 0.06977 | 1.0 | 1c4b:000a 1d4b:000a |
| output | AS 1/2 | eqEnable | toggle on | 1 |  | 1c40:0001 1d40:0001 |
| output | AS 1/2 | eqEnable | toggle off | 1 |  | 1c40:0000 1d40:0000 |
| output | AS 1/2 | lowcutFreq | probe | 0.75 | 224 Hz | 1c21:00e0 1d21:00e0 |
| output | AS 1/2 | lowcutFreq | restore | 0 | 20 Hz | 1c21:0014 1d21:0014 |
| output | AS 1/2 | lowcutGrade | probe | 0.75 | 18 dB/oct | 1c22:0002 1d22:0002 |
| output | AS 1/2 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 1c22:0001 1d22:0001 |
| output | AS 1/2 | lowcutEnable | toggle on | 1 |  | 1c20:0001 1d20:0001 |
| output | AS 1/2 | lowcutEnable | toggle off | 1 |  | 1c20:0000 1d20:0000 |
| output | AS 1/2 | compexpGain | probe | 0.75 | 15.0 dB | 1c61:0096 1d61:0096 |
| output | AS 1/2 | compexpGain | restore | 0.5 | 0.0 dB | 1c61:0000 1d61:0000 |
| output | AS 1/2 | compexpAttack | probe | 0.75 | 150 ms | 1c62:0096 1d62:0096 |
| output | AS 1/2 | compexpAttack | restore | 0.05 | 10 ms | 1c62:000a 1d62:000a |
| output | AS 1/2 | compexpRelease | probe | 0.75 | 774 ms | 1c63:0306 1d63:0306 |
| output | AS 1/2 | compexpRelease | restore | 0.2225 | 300 ms | 1c63:012c 1d63:012c |
| output | AS 1/2 | compTrsh | probe | 0.75 | -15.0 dB | 1c64:ff6a 1d64:ff6a |
| output | AS 1/2 | compTrsh | restore | 0.5 | -30.0 dB | 1c64:fed4 1d64:fed4 |
| output | AS 1/2 | compRatio | probe | 0.75 | 7.7 | 1c65:004d 1d65:004d |
| output | AS 1/2 | compRatio | restore | 0 | 1.0 | 1c65:000a 1d65:000a |
| output | AS 1/2 | expTrsh | probe | 0.75 | -47.5 dB | 1c66:fe25 1d66:fe25 |
| output | AS 1/2 | expTrsh | restore | 0.5652 | -60.0 dB | 1c66:fda8 1d66:fda8 |
| output | AS 1/2 | expRatio | probe | 0.75 | 7.7 | 1c67:004d 1d67:004d |
| output | AS 1/2 | expRatio | restore | 0 | 1.0 | 1c67:000a 1d67:000a |
| output | AS 1/2 | compexpEnable | toggle on | 1 |  | 1c60:0001 1d60:0001 |
| output | AS 1/2 | compexpEnable | toggle off | 1 |  | 1c60:0000 1d60:0000 |
| output | AS 1/2 | alevMaxgain | probe | 0.75 | 13.5 dB | 1c81:0087 1d81:0087 |
| output | AS 1/2 | alevMaxgain | restore | 0.3333 | 6.0 dB | 1c81:003c 1d81:003c |
| output | AS 1/2 | alevHeadroom | probe | 0.75 | 9.5 dB | 1c82:005f 1d82:005f |
| output | AS 1/2 | alevHeadroom | restore | 0.3333 | 6.0 dB | 1c82:003c 1d82:003c |
| output | AS 1/2 | alevRisetime | probe | 0.75 | 7.4 s | 1c83:004a 1d83:004a |
| output | AS 1/2 | alevRisetime | restore | 0.5 | 5.0 s | 1c83:0032 1d83:0032 |
| output | ADAT 3/4 | eqType1 | probe | 0.75 | Shelf | (none) |
| output | ADAT 3/4 | eqType1 | restore | 1 | Shelf | (none) |
| output | ADAT 3/4 | eqGain1 | probe | 0.75 | 10.0 dB | 1e42:0064 1f42:0064 |
| output | ADAT 3/4 | eqGain1 | restore | 0.5 | 0.0 dB | 1e42:0000 1f42:0000 |
| output | ADAT 3/4 | eqFreq1 | probe | 0.75 | 3557 Hz | 1e43:0de5 1f43:0de5 |
| output | ADAT 3/4 | eqFreq1 | restore | 0.2007 | 80 Hz | 1e43:0050 1f43:0050 |
| output | ADAT 3/4 | eqQ1 | probe | 0.75 | 3.9 | 1e44:0027 1f44:0027 |
| output | ADAT 3/4 | eqQ1 | restore | 0.06977 | 1.0 | 1e44:000a 1f44:000a |
| output | ADAT 3/4 | eqGain2 | probe | 0.75 | 10.0 dB | 1e45:0064 1f45:0064 |
| output | ADAT 3/4 | eqGain2 | restore | 0.5 | 0.0 dB | 1e45:0000 1f45:0000 |
| output | ADAT 3/4 | eqFreq2 | probe | 0.75 | 3557 Hz | 1e46:0de5 1f46:0de5 |
| output | ADAT 3/4 | eqFreq2 | restore | 0.5663 | 1000 Hz | 1e46:03e8 1f46:03e8 |
| output | ADAT 3/4 | eqQ2 | probe | 0.75 | 3.9 | 1e47:0027 1f47:0027 |
| output | ADAT 3/4 | eqQ2 | restore | 0.06977 | 1.0 | 1e47:000a 1f47:000a |
| output | ADAT 3/4 | eqType3 | probe | 0.75 | Low Pass | 1e48:0002 1f48:0002 |
| output | ADAT 3/4 | eqType3 | restore | 0.5 | Shelf | 1e48:0001 1f48:0001 |
| output | ADAT 3/4 | eqGain3 | probe | 0.75 | 10.0 dB | 1e49:0064 1f49:0064 |
| output | ADAT 3/4 | eqGain3 | restore | 0.5 | 0.0 dB | 1e49:0000 1f49:0000 |
| output | ADAT 3/4 | eqFreq3 | probe | 0.75 | 3557 Hz | 1e4a:0de5 1f4a:0de5 |
| output | ADAT 3/4 | eqFreq3 | restore | 0.7993 | 5000 Hz | 1e4a:1388 1f4a:1388 |
| output | ADAT 3/4 | eqQ3 | probe | 0.75 | 3.9 | 1e4b:0027 1f4b:0027 |
| output | ADAT 3/4 | eqQ3 | restore | 0.06977 | 1.0 | 1e4b:000a 1f4b:000a |
| output | ADAT 3/4 | eqEnable | toggle on | 1 |  | 1e40:0001 1f40:0001 |
| output | ADAT 3/4 | eqEnable | toggle off | 1 |  | 1e40:0000 1f40:0000 |
| output | ADAT 3/4 | lowcutFreq | probe | 0.75 | 224 Hz | 1e21:00e0 1f21:00e0 |
| output | ADAT 3/4 | lowcutFreq | restore | 0 | 20 Hz | 1e21:0014 1f21:0014 |
| output | ADAT 3/4 | lowcutGrade | probe | 0.75 | 18 dB/oct | 1e22:0002 1f22:0002 |
| output | ADAT 3/4 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 1e22:0001 1f22:0001 |
| output | ADAT 3/4 | lowcutEnable | toggle on | 1 |  | 1e20:0001 1f20:0001 |
| output | ADAT 3/4 | lowcutEnable | toggle off | 1 |  | 1e20:0000 1f20:0000 |
| output | ADAT 3/4 | compexpGain | probe | 0.75 | 15.0 dB | 1e61:0096 1f61:0096 |
| output | ADAT 3/4 | compexpGain | restore | 0.5 | 0.0 dB | 1e61:0000 1f61:0000 |
| output | ADAT 3/4 | compexpAttack | probe | 0.75 | 150 ms | 1e62:0096 1f62:0096 |
| output | ADAT 3/4 | compexpAttack | restore | 0.05 | 10 ms | 1e62:000a 1f62:000a |
| output | ADAT 3/4 | compexpRelease | probe | 0.75 | 774 ms | 1e63:0306 1f63:0306 |
| output | ADAT 3/4 | compexpRelease | restore | 0.2225 | 300 ms | 1e63:012c 1f63:012c |
| output | ADAT 3/4 | compTrsh | probe | 0.75 | -15.0 dB | 1e64:ff6a 1f64:ff6a |
| output | ADAT 3/4 | compTrsh | restore | 0.5 | -30.0 dB | 1e64:fed4 1f64:fed4 |
| output | ADAT 3/4 | compRatio | probe | 0.75 | 7.7 | 1e65:004d 1f65:004d |
| output | ADAT 3/4 | compRatio | restore | 0 | 1.0 | 1e65:000a 1f65:000a |
| output | ADAT 3/4 | expTrsh | probe | 0.75 | -47.5 dB | 1e66:fe25 1f66:fe25 |
| output | ADAT 3/4 | expTrsh | restore | 0.5652 | -60.0 dB | 1e66:fda8 1f66:fda8 |
| output | ADAT 3/4 | expRatio | probe | 0.75 | 7.7 | 1e67:004d 1f67:004d |
| output | ADAT 3/4 | expRatio | restore | 0 | 1.0 | 1e67:000a 1f67:000a |
| output | ADAT 3/4 | compexpEnable | toggle on | 1 |  | 1e60:0001 1f60:0001 |
| output | ADAT 3/4 | compexpEnable | toggle off | 1 |  | 1e60:0000 1f60:0000 |
| output | ADAT 3/4 | alevMaxgain | probe | 0.75 | 13.5 dB | 1e81:0087 1f81:0087 |
| output | ADAT 3/4 | alevMaxgain | restore | 0.3333 | 6.0 dB | 1e81:003c 1f81:003c |
| output | ADAT 3/4 | alevHeadroom | probe | 0.75 | 9.5 dB | 1e82:005f 1f82:005f |
| output | ADAT 3/4 | alevHeadroom | restore | 0.3333 | 6.0 dB | 1e82:003c 1f82:003c |
| output | ADAT 3/4 | alevRisetime | probe | 0.75 | 7.4 s | 1e83:004a 1f83:004a |
| output | ADAT 3/4 | alevRisetime | restore | 0.5 | 5.0 s | 1e83:0032 1f83:0032 |
| output | ADAT 5/6 | eqType1 | probe | 0.75 | Shelf | (none) |
| output | ADAT 5/6 | eqType1 | restore | 1 | Shelf | (none) |
| output | ADAT 5/6 | eqGain1 | probe | 0.75 | 10.0 dB | 2042:0064 2142:0064 |
| output | ADAT 5/6 | eqGain1 | restore | 0.5 | 0.0 dB | 2042:0000 2142:0000 |
| output | ADAT 5/6 | eqFreq1 | probe | 0.75 | 3557 Hz | 2043:0de5 2143:0de5 |
| output | ADAT 5/6 | eqFreq1 | restore | 0.2007 | 80 Hz | 2043:0050 2143:0050 |
| output | ADAT 5/6 | eqQ1 | probe | 0.75 | 3.9 | 2044:0027 2144:0027 |
| output | ADAT 5/6 | eqQ1 | restore | 0.06977 | 1.0 | 2044:000a 2144:000a |
| output | ADAT 5/6 | eqGain2 | probe | 0.75 | 10.0 dB | 2045:0064 2145:0064 |
| output | ADAT 5/6 | eqGain2 | restore | 0.5 | 0.0 dB | 2045:0000 2145:0000 |
| output | ADAT 5/6 | eqFreq2 | probe | 0.75 | 3557 Hz | 2046:0de5 2146:0de5 |
| output | ADAT 5/6 | eqFreq2 | restore | 0.5663 | 1000 Hz | 2046:03e8 2146:03e8 |
| output | ADAT 5/6 | eqQ2 | probe | 0.75 | 3.9 | 2047:0027 2147:0027 |
| output | ADAT 5/6 | eqQ2 | restore | 0.06977 | 1.0 | 2047:000a 2147:000a |
| output | ADAT 5/6 | eqType3 | probe | 0.75 | Low Pass | 2048:0002 2148:0002 |
| output | ADAT 5/6 | eqType3 | restore | 0.5 | Shelf | 2048:0001 2148:0001 |
| output | ADAT 5/6 | eqGain3 | probe | 0.75 | 10.0 dB | 2049:0064 2149:0064 |
| output | ADAT 5/6 | eqGain3 | restore | 0.5 | 0.0 dB | 2049:0000 2149:0000 |
| output | ADAT 5/6 | eqFreq3 | probe | 0.75 | 3557 Hz | 204a:0de5 214a:0de5 |
| output | ADAT 5/6 | eqFreq3 | restore | 0.7993 | 5000 Hz | 204a:1388 214a:1388 |
| output | ADAT 5/6 | eqQ3 | probe | 0.75 | 3.9 | 204b:0027 214b:0027 |
| output | ADAT 5/6 | eqQ3 | restore | 0.06977 | 1.0 | 204b:000a 214b:000a |
| output | ADAT 5/6 | eqEnable | toggle on | 1 |  | 2040:0001 2140:0001 |
| output | ADAT 5/6 | eqEnable | toggle off | 1 |  | 2040:0000 2140:0000 |
| output | ADAT 5/6 | lowcutFreq | probe | 0.75 | 224 Hz | 2021:00e0 2121:00e0 |
| output | ADAT 5/6 | lowcutFreq | restore | 0 | 20 Hz | 2021:0014 2121:0014 |
| output | ADAT 5/6 | lowcutGrade | probe | 0.75 | 18 dB/oct | 2022:0002 2122:0002 |
| output | ADAT 5/6 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 2022:0001 2122:0001 |
| output | ADAT 5/6 | lowcutEnable | toggle on | 1 |  | 2020:0001 2120:0001 |
| output | ADAT 5/6 | lowcutEnable | toggle off | 1 |  | 2020:0000 2120:0000 |
| output | ADAT 5/6 | compexpGain | probe | 0.75 | 15.0 dB | 2061:0096 2161:0096 |
| output | ADAT 5/6 | compexpGain | restore | 0.5 | 0.0 dB | 2061:0000 2161:0000 |
| output | ADAT 5/6 | compexpAttack | probe | 0.75 | 150 ms | 2062:0096 2162:0096 |
| output | ADAT 5/6 | compexpAttack | restore | 0.05 | 10 ms | 2062:000a 2162:000a |
| output | ADAT 5/6 | compexpRelease | probe | 0.75 | 774 ms | 2063:0306 2163:0306 |
| output | ADAT 5/6 | compexpRelease | restore | 0.2225 | 300 ms | 2063:012c 2163:012c |
| output | ADAT 5/6 | compTrsh | probe | 0.75 | -15.0 dB | 2064:ff6a 2164:ff6a |
| output | ADAT 5/6 | compTrsh | restore | 0.5 | -30.0 dB | 2064:fed4 2164:fed4 |
| output | ADAT 5/6 | compRatio | probe | 0.75 | 7.7 | 2065:004d 2165:004d |
| output | ADAT 5/6 | compRatio | restore | 0 | 1.0 | 2065:000a 2165:000a |
| output | ADAT 5/6 | expTrsh | probe | 0.75 | -47.5 dB | 2066:fe25 2166:fe25 |
| output | ADAT 5/6 | expTrsh | restore | 0.5652 | -60.0 dB | 2066:fda8 2166:fda8 |
| output | ADAT 5/6 | expRatio | probe | 0.75 | 7.7 | 2067:004d 2167:004d |
| output | ADAT 5/6 | expRatio | restore | 0 | 1.0 | 2067:000a 2167:000a |
| output | ADAT 5/6 | compexpEnable | toggle on | 1 |  | 2060:0001 2160:0001 |
| output | ADAT 5/6 | compexpEnable | toggle off | 1 |  | 2060:0000 2160:0000 |
| output | ADAT 5/6 | alevMaxgain | probe | 0.75 | 13.5 dB | 2081:0087 2181:0087 |
| output | ADAT 5/6 | alevMaxgain | restore | 0.3333 | 6.0 dB | 2081:003c 2181:003c |
| output | ADAT 5/6 | alevHeadroom | probe | 0.75 | 9.5 dB | 2082:005f 2182:005f |
| output | ADAT 5/6 | alevHeadroom | restore | 0.3333 | 6.0 dB | 2082:003c 2182:003c |
| output | ADAT 5/6 | alevRisetime | probe | 0.75 | 7.4 s | 2083:004a 2183:004a |
| output | ADAT 5/6 | alevRisetime | restore | 0.5 | 5.0 s | 2083:0032 2183:0032 |
| output | ADAT 7/8 | eqType1 | probe | 0.75 | Shelf | (none) |
| output | ADAT 7/8 | eqType1 | restore | 1 |  | (none) |
| output | ADAT 7/8 | eqGain1 | probe | 0.75 | 10.0 dB | 2242:0064 2342:0064 |
| output | ADAT 7/8 | eqGain1 | restore | 0.5 | 0.0 dB | 2242:0000 2342:0000 |
| output | ADAT 7/8 | eqFreq1 | probe | 0.75 | 3557 Hz | 2243:0de5 2343:0de5 |
| output | ADAT 7/8 | eqFreq1 | restore | 0.2007 | 80 Hz | 2243:0050 2343:0050 |
| output | ADAT 7/8 | eqQ1 | probe | 0.75 | 3.9 | 2244:0027 2344:0027 |
| output | ADAT 7/8 | eqQ1 | restore | 0.06977 | 1.0 | 2244:000a 2344:000a |
| output | ADAT 7/8 | eqGain2 | probe | 0.75 | 10.0 dB | 2245:0064 2345:0064 |
| output | ADAT 7/8 | eqGain2 | restore | 0.5 | 0.0 dB | 2245:0000 2345:0000 |
| output | ADAT 7/8 | eqFreq2 | probe | 0.75 | 3557 Hz | 2246:0de5 2346:0de5 |
| output | ADAT 7/8 | eqFreq2 | restore | 0.5663 | 1000 Hz | 2246:03e8 2346:03e8 |
| output | ADAT 7/8 | eqQ2 | probe | 0.75 | 3.9 | 2247:0027 2347:0027 |
| output | ADAT 7/8 | eqQ2 | restore | 0.06977 | 1.0 | 2247:000a 2347:000a |
| output | ADAT 7/8 | eqType3 | probe | 0.75 | Low Pass | 2248:0002 2348:0002 |
| output | ADAT 7/8 | eqType3 | restore | 0.5 | Shelf | 2248:0001 2348:0001 |
| output | ADAT 7/8 | eqGain3 | probe | 0.75 | 10.0 dB | 2249:0064 2349:0064 |
| output | ADAT 7/8 | eqGain3 | restore | 0.5 | 0.0 dB | 2249:0000 2349:0000 |
| output | ADAT 7/8 | eqFreq3 | probe | 0.75 | 3557 Hz | 224a:0de5 234a:0de5 |
| output | ADAT 7/8 | eqFreq3 | restore | 0.7993 | 5000 Hz | 224a:1388 234a:1388 |
| output | ADAT 7/8 | eqQ3 | probe | 0.75 | 3.9 | 224b:0027 234b:0027 |
| output | ADAT 7/8 | eqQ3 | restore | 0.06977 | 1.0 | 224b:000a 234b:000a |
| output | ADAT 7/8 | eqEnable | toggle on | 1 |  | 2240:0001 2340:0001 |
| output | ADAT 7/8 | eqEnable | toggle off | 1 |  | 2240:0000 2340:0000 |
| output | ADAT 7/8 | lowcutFreq | probe | 0.75 | 224 Hz | 2221:00e0 2321:00e0 |
| output | ADAT 7/8 | lowcutFreq | restore | 0 | 20 Hz | 2221:0014 2321:0014 |
| output | ADAT 7/8 | lowcutGrade | probe | 0.75 | 18 dB/oct | 2222:0002 2322:0002 |
| output | ADAT 7/8 | lowcutGrade | restore | 0.3333 | 12 dB/oct | 2222:0001 2322:0001 |
| output | ADAT 7/8 | lowcutEnable | toggle on | 1 |  | 2220:0001 2320:0001 |
| output | ADAT 7/8 | lowcutEnable | toggle off | 1 |  | 2220:0000 2320:0000 |
| output | ADAT 7/8 | compexpGain | probe | 0.75 | 15.0 dB | 2261:0096 2361:0096 |
| output | ADAT 7/8 | compexpGain | restore | 0.5 | 0.0 dB | 2261:0000 2361:0000 |
| output | ADAT 7/8 | compexpAttack | probe | 0.75 | 150 ms | 2262:0096 2362:0096 |
| output | ADAT 7/8 | compexpAttack | restore | 0.05 | 10 ms | 2262:000a 2362:000a |
| output | ADAT 7/8 | compexpRelease | probe | 0.75 | 774 ms | 2263:0306 2363:0306 |
| output | ADAT 7/8 | compexpRelease | restore | 0.2225 | 300 ms | 2263:012c 2363:012c |
| output | ADAT 7/8 | compTrsh | probe | 0.75 | -15.0 dB | 2264:ff6a 2364:ff6a |
| output | ADAT 7/8 | compTrsh | restore | 0.5 | -30.0 dB | 2264:fed4 2364:fed4 |
| output | ADAT 7/8 | compRatio | probe | 0.75 | 7.7 | 2265:004d 2365:004d |
| output | ADAT 7/8 | compRatio | restore | 0 | 1.0 | 2265:000a 2365:000a |
| output | ADAT 7/8 | expTrsh | probe | 0.75 | -47.5 dB | 2266:fe25 2366:fe25 |
| output | ADAT 7/8 | expTrsh | restore | 0.5652 | -60.0 dB | 2266:fda8 2366:fda8 |
| output | ADAT 7/8 | expRatio | probe | 0.75 | 7.7 | 2267:004d 2367:004d |
| output | ADAT 7/8 | expRatio | restore | 0 | 1.0 | 2267:000a 2367:000a |
| output | ADAT 7/8 | compexpEnable | toggle on | 1 |  | 2260:0001 2360:0001 |
| output | ADAT 7/8 | compexpEnable | toggle off | 1 |  | 2260:0000 2360:0000 |
| output | ADAT 7/8 | alevMaxgain | probe | 0.75 | 13.5 dB | 2281:0087 2381:0087 |
| output | ADAT 7/8 | alevMaxgain | restore | 0.3333 | 6.0 dB | 2281:003c 2381:003c |
| output | ADAT 7/8 | alevHeadroom | probe | 0.75 | 9.5 dB | 2282:005f 2382:005f |
| output | ADAT 7/8 | alevHeadroom | restore | 0.3333 | 6.0 dB | 2282:003c 2382:003c |
| output | ADAT 7/8 | alevRisetime | probe | 0.75 | 7.4 s | 2283:004a 2383:004a |
| output | ADAT 7/8 | alevRisetime | restore | 0.5 | 5.0 s | 2283:0032 2383:0032 |
