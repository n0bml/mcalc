BASIC program and explanatory text for L. B. Cebik, W4RNL, "Having a Field Day
with the Moxon Rectangle," QST, Jun 2000.

# Designing a Moxon Rectangle

The dimensions shown in the main body of the companion article are all
calculated for the center of each HF band except as specially noted.  As well,
they all use #14 bare copper wire.  You may want to change the design
frequency or the wire diameter.

The accompanying BASIC program allows you to design a Moxon Rectangle having a
feedpoint impedance of 52 to 55 ohms for any HF frequency using any wire
diameter ranging from 10-6 to 10-2 wavelengths, that is, from smaller than #18
wire at 160 meters to larger than one inch at 2 meters.  Simply specify the wire
diameter in inches, millimeters, or wavelengths when the input requests it, and
the dimensions of the Moxon will emerge.  The dimensions use the letter
designations that appear in Figure 1 of the referenced article.

```basic
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
```

The constants in the listing use far greater precision than required by most
builders.  However, when using a computer to calculate for me, my personal
preference is to use the highest level of precision internally and save any
rounding off activity for the output.  This little program uses no error
trapping.  You can embellish it as you please, including adding a routine to
accept wire sizes in AWG.

The calculations of the dimensions are based on regression analysis of a
base-line data set derived from optimized models of Moxon rectangles meeting
certain limits of gain, F/B, and feedpoint impedance.  The program appears to
produce results that are accurate within less than 1% relative to models
optimized for random frequencies and wire sizes.  The results assume perfectly
conducting wire.  Hence, the use of small wire sizes (relative to a wavelength)
will add up to about 2 ohms to the feedpoint resistance and decrease gain
by up to about 0.25 dB from the idealized models.  None of these deviations
from perfection should affect construction or operation.

A full account of the technique used to derive the program will appear in
a forthcoming issue of [antenneX](http://www.antennex.com) on-line magazine.  The
program will also be added to the HAMCALC suite of GW BASIC electronics
utility programs available from George Murphy, VE3ERP.  Those having access to
NEC-Win Plus, a NEC-2 antenna modeling software package available from Nittany
Scientific, can simplify the process of deriving dimensions and checking the
resultant model.  The model-by-equation facility of the spreadsheet input system
permitted me to transfer the design equations directly into a model, which the
user can set for any desired design frequency. The output will include both the
dimensions and a standard NEC-2 calculation of the antenna pattern and source
impedance, with options for changing any of variables, including the wire
conductivity, size, etc.  A copy of the MOXGENE8.NWP file is available among the
examples at the [NEC-Win](http://www.nittany-scientific.com) Web site.
