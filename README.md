# rnapkin: drawing RNA secondary structure with style
## Usage
rnapkin accepts a file containing secondary structure and optionally sequence and a name.
For example:
```
# name has to start with > 
# you can add .png / .svg to request specific output; though .svg is default
# the name can be overwritten with -o flag
>remarkable molecule
UUAUAGGCGAUGGAGUUCGCCAUAAACGCUGCUUAGCUAAUGACUCCUACCAGUAUCACUACUGGUAGGAGUCUAUUUUUUU
.....(((((......)))))......(((....)))....((((((((((((((....)))))))))))))).........
```
The input format is not that rigid; you can have multiline sequences and structures.
They can be even neatly aligned and mixed like this:

```text
>fantastic molecule
AATATAATAGGAACACTCATATAATCGCGTGGATATGGCACGCAAGTTTCTACCGGGCAC
..........(..(.((((.((((..(((((.......)))))..........((((((.
CGTAAATGTCCGACTATGGGTGAGCAATGGAACCGCACGTGTACGGTTTTTTGTGATATC
......)))))).....((((((((((((((((((........))))))...........
AGCATTGCTTGCTCTTTATTTGAGCGGGCAATGCTTTTTTTA
..)))))))))))).)))).)))).)..).............
```

let's say the file above is called *guanineribo*, one could then run napkin thus:
```
rnapkin guanineribo
```
surely rnapkin would respond:
```
drawn: "fantastic_molecule.svg"
```
and this scalable vector graphic would be produced:

<p align="center">
 <img src="https://github.com/ukmrs/rnapkin/blob/main/gallery/fantastic_molecule.svg" height="750"/>
</p>


I quite like it, but if that looks upside down to you, you can tell rnapkin
to mirror it by --mx flag. Matter of fact you can apply arbitrary rotation 
with -a / --angle <degrees> option or, of course, mirror along y axis with --my.

![](https://github.com/ukmrs/gallery/blob/main/rnapkin/v0.2.0/angmirrordemo0.png)

color themes can be changed by -t option as demonstrated; a config file allowing to define custom color themes
is planned though unimplemented!()


## Installing
I plan to offer precompiled binaries but for now
you'll need rust. Easiest way to get it is with [rustup](https://rustup.rs) :crab:

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
