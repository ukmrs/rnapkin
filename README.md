# rnapkin: drawing RNA secondary structure with style
## Usage
rnapkin accepts a file containing secondary structure and optionally sequence and a name.
For example you could have this marvellous RNA molecule sitting peacefully
in a file called "guaniners"
```text
>fantastic guanine riboswitch
AAUAUAAUAGGAACACUCAUAUAAUCGCGUGGAUAUGGCACGCAAGUUUCUACCGGGCAC
..........(..(.((((.((((..(((((.......)))))..........((((((.
CGUAAAUGUCCGACUAUGGGUGAGCAAUGGAACCGCACGUGUACGGUUUUUUGUGAUAUC
......)))))).....((((((((((((((((((........))))))...........
AGCAUUGCUUGCUCUUUAUUUGAGCGGGCAAUGCUUUUUUUA
..)))))))))))).)))).)))).)..).............
```
Then, if you wish to visualize it, you could invoke rnapkin thus:
```
rnapkin guaniners
```
Surely rnapkin would respond with the name of a file it has just drawn to:
```
fantastic_guanine_riboswitch.svg
```
And this scalable vector graphic would be produced:
<p align="center">
 <img src="https://raw.githubusercontent.com/ukmrs/gallery/main/rnapkin/v0.3.0/guaniners.svg" height="750"/>
</p>

*I* happen to quite enjoy the outcome, so *I* would say:
```
that's pretty neat
```
Your mileage may vary though.

## rotating and flipping
If you'd like to see this or any other RNA molecule upside-down, tilted or what have you, there are
some options listed below that you can use and combine:
```text
-a / --angle <DEGREES> | starting angle / boils down to clockwise rotation
--mx                   | **m**irror along **x** axis / aka vertical flip
--my                   | **m**irror along **y** axis / aka horizontal flip
```
<p align="center">
 <img src="https://raw.githubusercontent.com/ukmrs/gallery/main/rnapkin/v0.3.0/angle_mirror_demo.png" />
</p>

color themes can be changed by -t option as demonstrated; a config file allowing to define custom color themes
is planned though unimplemented!()

## Installing
I plan to offer precompiled binaries but for now
you'll need **rust**. Easiest way to acquire **rust** is via [rustup](https://rustup.rs) :crab:

### Anywhere
```bash
cargo install rnapkin
```
### WSL
Fontconfig is the default Fontmanagement utility on Linux and Unix but WSL may not have them installed;
```bash
sudo apt-get install libfontconfig libfontconfig1-dev
cargo install rnapkin
```

## input file
input file is quite **flexible**; it should contain secondary_structure and optionally
name and sequence. Name has to start with ">" and can be overwritten with -o flag
which has priority. Here are some variations of valid input files:

### simple one

```text
# you can add .png to the name to request png instead of svg
@ the same of course can be achieved with -o flag.
* this is a comment btw: any symbol other than ">.()" works
>simple molecule.png
((((((((((..((((((.........))))))......).((((((.......))))))..)))))))))
CGCUUCAUAUAAUCCUAAUGAUAUGGUUUGGGAGUUUCUACCAAGAGCCUUAAACUCUUGAUUAUGAAGUG
```

### only secondary structure

```text
.........(((..((((((...((((((((.....((((((((((...)))))).....
(((((((...))))))).))))(((.....)))...)))).)))).))))))..)))..(
(((.(((((..(((......))).)))))..))))(((((((((((((....))))))))
))))).....
```

### multiline
You've seen it already; sequence and secondary structure can be separate,
mixed and aligned, everything should work.

## DIY
using -p / --points flag you can make rnapkin print calculated coordinates
of nucleotide bubbles (with 0.5 unit radius). You can then plot it
yourself if you need to do something specific;

If you happen to clone the repository, there is an example python
script using **matplotlib** than you can pipe the input to
You can also combine -p flag with --mx --my and -a


```bash
cargo run -- atelier/example_inputs/guaniners -p | atelier/plot.py
```

## rnapkin name
The wordsmithing proccess was arduous. It involved
googling "words starting with na" and looking for anything drawing related.
Once the word was found, unparalled strength was employed to slap it on top of "rna"
ultimately creating this glorious amalgamation.
### why it kinda makes sense:
You ever heard of all those physicists, mathematicians and the like, scribbling formulas on the
back of a napkin ~~or a book margin~~? There is even a [wikipedia page](https://en.wikipedia.org/wiki/Back-of-the-envelope_calculation) about it.

It doesn't take much mental gymnastic to imagine a biologist frantically scrambling together
rna structure on a napkin. I am currently working on baiting my biologist 
friend into heated rna debate while in close proximity to abundant napkin source
in order to produce a proof of concept.
