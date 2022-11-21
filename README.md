# rnapkin: drawing RNA secondary structure in style
## Usage
rnapkin accepts a file containing secondary structure and optionally sequence and a name.
For Example:
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

let say file the above is called *guanineribo* then one could run napkin thus:
```
rnapkin guanineribo
```
surely rnapkin would respond like this:
```
drawn: "fantastic_molecule.svg"
```
and this scalable vector graphic would be produced:

color themes can be changed by -t flag; a config file allowing to define custom color themes
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
