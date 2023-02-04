10 CLS:PRINT "Program to calculate the dimensions of a Moxon Rectangle."
20 PRINT "All equations correlated to NEC antenna modeling software for wire diameters"
30 PRINT "     from 1E-5 to 1E-2 wavelengths."
40 PRINT "L. B. Cebik, W4RNL"
50 INPUT "Enter Desired Frequency in MHz:";F
60 PRINT "Select Units for Wire Diameter in 1. Inches, 2. Millimeters, 3. Wavelengths"
70 INPUT "Choose 1. or 2. or 3.";U
80 IF U>3 THEN 60
90 INPUT "Enter Wire Diameter in your Selected Units";WD
100 IF U=1 THEN WLI=11802.71/F:DW=WD/WLI
110 IF U=2 THEN WLI=299792.5/F:DW=WD/WLI
120 IF U=3 THEN DW=WD
130 PRINT "Wire Diameter in Wavelengths:";DW
140 D1=.4342945*LOG(DW)
150 IF D1<-6 THEN 160 ELSE 170
160 PRINT "Wire diameter less than 1E-6 wavelengths: results uncertain."
170 IF D1>-2 THEN 180 ELSE 190
180 PRINT "Wire diameter greater than 1E-2 wavelengths: results uncertain."
190 AA=-.0008571428571#:AB=-.009571428571#:AC=.3398571429#
200 A=(AA*(D1^2))+(AB*D1)+AC
210 BA=-.002142857143#:BB=-.02035714286#:BC=.008285714286#
220 B=(BA*(D1^2))+(BB*D1)+BC
230 CA=.001809523381#:CB=.01780952381#:CC=.05164285714#
240 C=(CA*(D1^2))+(CB*D1)+CC
241 DA=.001:DB=.07178571429#
242 D=(DA*D1)+DB
243 E=(B+C)+D
250 PRINT "Moxon Dimensions in Wavelengths:"
260 PRINT "A = ";A
270 PRINT "B = ";B
280 PRINT "C = ";C
290 PRINT "D = ";D
295 PRINT "E = ";E
299 WF=983.5592/F:WFI=WF*12:PRINT "Wavelength: =";WF;"Feet or ";WFI;"Inches"
300 PRINT "Dimensions in Feet and Inches"
301 PRINT "A = ";A*WF;"Feet or ";A*WFI;"Inches"
302 PRINT "B = ";B*WF;"Feet or ";B*WFI;"Inches"
303 PRINT "C = ";C*WF;"Feet or ";C*WFI;"Inches"
304 PRINT "D = ";D*WF;"Feet or ";D*WFI;"Inches"
305 PRINT "E = ";E*WF;"Feet or ";E*WFI;"Inches"
350 INPUT "Another Value = 1, Stop = 2: ";P
360 IF P=1 THEN 10 ELSE 370
370 END
