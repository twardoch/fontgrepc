## SpecStory

## Debugging Fontgrepc Panic Error (12/03/2025, 15:57:59)

_**User**_

time ./target/release/fontgrepc find -u U+0105 /Users/adam/Documents/f/f_lib/


thread 'main' panicked at /Users/adam/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clap_builder-4.5.32/src/parser/error.rs:32:9:
Mismatch between definition and access of `codepoints`. Could not downcast to TypeId(0x3d1bd3e92f06a91ea48c1fb40922450b), need to downcast to TypeId(0xecbbfeeffa82ce478234f46ae5ae2f29)

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

real	0m0.036s
user	0m0.017s
sys	0m0.023s

---

_**Assistant**_

I'll help you debug this panic in the `fontgrepc` tool. This looks like a type mismatch error in the clap argument parser, specifically related to the `codepoints` argument.

Let me first explore the codebase to understand the structure and the issue better.

---

_****_

<details>
            <summary>Listed current directory • **13** results</summary>
        
| Name |
|-------|
| 📁 `.git` |
| 📄 `TODO.md` |
| 📄 `Cargo.lock` |
| 📄 `Cargo.toml` |
| 📄 `README.md` |
| 📄 `.DS_Store` |
| 📁 `target` |
| 📁 `.specstory` |
| 📄 `.gitignore` |
| 📄 `CC0-1.0` |
| 📄 `LICENSE` |
| 📁 `src` |
| 📁 `.github` |

</details>

---

_****_

Let's look at the source code structure:

---

_****_

<details>
            <summary>Listed directory src • **8** results</summary>
        
| Name |
|-------|
| 📄 `cache.rs` |
| 📄 `lib.rs` |
| 📄 `cli.rs` |
| 📄 `query.rs` |
| 📄 `main.rs` |
| 📄 `utils.rs` |
| 📄 `matchers.rs` |
| 📄 `font.rs` |

</details>

---

_****_

Now, let's look at the CLI code since the error is related to the clap argument parser:

---

_****_

Read file: undefined

---

_****_

Let's look at the `parse_codepoints` function since it's related to the error:

---

_****_

Read file: undefined

---

_****_

Let's see the rest of the `parse_codepoint` function:

---

_****_

Read file: undefined

---

_****_

Now let's look at the `SearchArgs` struct in more detail:

---

_****_

Read file: undefined

---

_****_

Let's see the full definition of the codepoints argument:

---

_****_

Read file: undefined

---

_****_

Let's also check the main.rs file to understand how the CLI is being used:

---

_****_

Read file: undefined

---

_****_

Now, let's look at the query.rs file to understand how the codepoints are used:

---

_****_

Read file: undefined

---

_****_

Let's look at the matchers.rs file to understand how the CodepointsMatcher works:

---

_****_

Read file: undefined

---

_****_

Let's look at the CodepointsMatcher implementation:

---

_****_

Read file: undefined

---

_****_

Based on the error message and the code I've examined, I can identify the issue. The error is related to a type mismatch in the clap argument parser for the `codepoints` parameter. The error message indicates that there's a mismatch between the definition and access of the `codepoints` field.

The error is:
```
Mismatch between definition and access of `codepoints`. Could not downcast to TypeId(0x3d1bd3e92f06a91ea48c1fb40922450b), need to downcast to TypeId(0xecbbfeeffa82ce478234f46ae5ae2f29)
```

This suggests that there's a type mismatch between how the `codepoints` argument is defined in the clap parser and how it's being accessed in the code.

Let's fix this issue by modifying the `codepoints` argument in the `SearchArgs` struct:

---

_****_



---

_****_

Let's try a different approach. The issue might be in how the `parse_codepoints` function is defined or used. Let's modify the `parse_codepoints` function:

---

_****_

```diff
  
  /// Parse codepoints from strings
- pub fn parse_codepoints(input: &str) -> Result<Vec<char>> {
+ pub fn parse_codepoints(input: &str) -> std::result::Result<Vec<char>, FontgrepcError> {
      let mut result = Vec::new();
  
```

---

_****_

Let's try a different approach. The issue might be in how the `codepoints` argument is defined in the `SearchArgs` struct. Let's modify it to use a different approach for parsing:

---

_****_

```diff
          short = 'u',
          long,
-         value_parser = parse_codepoints,
+         value_delimiter = ',',
          help = "Unicode codepoints or ranges to search for (e.g., U+0041-U+005A,U+0061)",
          long_help = "Comma-separated list of Unicode codepoints or ranges to search for. \
                      - Single character: A"
      )]
-     pub codepoints: Vec<char>,
+     pub codepoints_str: Vec<String>,
  
      /// Text to check for support
                  tables: args.tables.clone(),
                  name_patterns: args.name.clone(),
-                 codepoints: args.codepoints.clone(),
+                 codepoints: args.codepoints_str.iter().map(|s| s.parse::<char>().unwrap()).collect(),
                  charset: args.text.clone().unwrap_or_default(),
              };
```

---

_****_

The changes look good, but we need to update the code that uses the `codepoints_str` field to properly parse the codepoints. Let's modify the `execute` function to handle the new field:

---

_****_

Read file: undefined

---

_****_

The changes made by the apply model are incorrect. The `codepoints_str` field can't be directly parsed as `char` values because it might contain ranges or Unicode codepoint notation. Let's fix this:

---

_****_

```diff
                  tables: args.tables.clone(),
                  name_patterns: args.name.clone(),
-                 codepoints: args.codepoints_str.iter().map(|s| s.parse::<char>().unwrap()).collect(),
+                 codepoints: parse_codepoints_from_strings(&args.codepoints_str)?,
                  charset: args.text.clone().unwrap_or_default(),
              };
          .ok_or_else(|| FontgrepcError::Parse(format!("Invalid Unicode codepoint: U+{:04X}", cp)))
  }
+ 
+ /// Parse codepoints from strings
+ pub fn parse_codepoints_from_strings(input: &[String]) -> Result<Vec<char>> {
+     let mut result = Vec::new();
+ 
+     for item in input {
+         result.extend(parse_codepoints(item)?);
+     }
+ 
+     Ok(result)
+ }
  
```

---

_****_

Now we need to fix the `parse_codepoints` function to work with the new approach:

---

_****_

```diff
  
  /// Parse codepoints from strings
- pub fn parse_codepoints(input: &str) -> std::result::Result<Vec<char>, FontgrepcError> {
+ pub fn parse_codepoints(input: &str) -> Result<Vec<char>> {
      let mut result = Vec::new();
  
```

---

_****_

Let's run the command to see if our changes fixed the issue:

---

_****_

```bash
cd /Users/adam/Developer/vcs/github.twardoch/dev2/fontgrepc && cargo build --release
```

```
   Compiling fontgrepc v1.0.4 (/Users/adam/Developer/vcs/github.twardoch/dev2/f
ontgrepc)
   Finished `release` profile [optimized] target(s) in 37.01s

```

---

_****_

Now let's test the command that was failing:

---

_****_

```bash
time ./target/release/fontgrepc find -u U+0105 /Users/adam/Documents/f/f_lib/
```

```
adam@Hamlet fontgrepc % 
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/DomCasualCom/DomCasualCom-
Diagonal.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCZapfChanceryW02/ITCZapf
ChanceryW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
BoldIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/DownTownW02/DownTownW02.tt
f
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnzoOffcPro/EnzoOffcPro-Li
ghtItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnviroW02/EnviroW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/InfoDisplayOffcPro/InfoDis
playOffcPro-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GaramondW02/GaramondW02-It
alic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-Condensed.
ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-Compressed
Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Bo
ld.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansW02/HarmoniaSa
nsW02-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaPro/GivensAnt
iquaPro-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurekaSansScOffcPro/Eureka
SansScOffcPro-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyModernMTW02/GoudyMode
rnMTW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
Book.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Bo
ldOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnzoOffcPro/EnzoOffcPro-Bl
ack.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/DreamlandW02/DreamlandW02.
ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Li
ght.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GaramondW02/GaramondW02-It
alicAlt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Li
ghtOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-CondensedI
talic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaPro/GivensAnt
iquaPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurekaSansScOffcPro/Eureka
SansScOffcPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansW02/HarmoniaSa
nsW02-BlackItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-LightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
BookIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Ob
lique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaPro/GivensAnt
iquaPro-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnzoOffcPro/EnzoOffcPro-Bl
ackItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCZapfChanceryW02/ITCZapf
ChanceryW02-Demi.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Ro
man.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyModernMTW02/GoudyMode
rnMTW02-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-Demi.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Th
in.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GaramondW02/GaramondW02.tt
f
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnzoOffcPro/EnzoOffcPro-Me
dium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurekaScOffcPro/EurekaScOf
fcPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansW02/HarmoniaSa
nsW02-BoldCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaW02/GivensAnt
iquaW02-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
ExtdBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Th
inOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-DemiConden
sed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaPro/GivensAnt
iquaPro-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyOldStyleMTW02/GoudyOl
dStyleMTW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnzoOffcPro/EnzoOffcPro-Bo
ld.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-35
Thin.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GarthGraphicW02/GarthGraph
icW02-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-35
ThinOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansW02/HarmoniaSa
nsW02-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaW02/GivensAnt
iquaW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileCandyLTCom/Eurost
ileCandyLTCom-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-MediumItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-45
Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
ExtdBoldIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/DropinkLTW02/DropinkLTW02.
ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-65
BoldOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-DemiConden
sedIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyOldStyleMTW02/GoudyOl
dStyleMTW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-MediumCond
ensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnzoOffcPro/EnzoOffcPro-Me
diumItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCZapfChanceryW02/ITCZapf
ChanceryW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GarthGraphicW02/GarthGraph
icW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaW02/GivensAnt
iquaW02-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
ExtdBook.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnzoOffcPro/EnzoOffcPro-Th
in.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileCandyW02/Eurostil
eCandyW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextLTCom/Eurosti
leNextLTCom-LightExt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-75
Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-DemiItalic
.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-Heavy.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GrafikoCom/GrafikoCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaPro/GivensAnt
iquaPro-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/F2FOCRBczykCom/F2FOCRBczyk
Com-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-MediumCond
ensedIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EnzoOffcPro/EnzoOffcPro-Th
inItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GarthGraphicW02/GarthGraph
icW02-BoldCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextLTCom/Eurosti
leNextLTCom-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-Thin.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-HeavyItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
ExtdBookIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaW02/GivensAnt
iquaW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-45
LightOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EonAgeATT/EonAgeATT.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/F2FOCRBczykW02/F2FOCRBczyk
W02-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileCandyW02/Eurostil
eCandyW02-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-75
BlackOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextLTCom/Eurosti
leNextLTCom-SmBd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GrafikoW02/GrafikoW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GarthGraphicW02/GarthGraph
icW02-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-MediumItal
ic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FacsimileCom/FacsimileCom.
ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-ThinItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextLTCom/Eurosti
leNextLTCom-SmBdExt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
ExtdMedium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansW02/HarmoniaSa
nsW02-Condensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaW02/GivensAnt
iquaW02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GrafiloneLTW02/GrafiloneLT
W02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileCandyW02/Eurostil
eCandyW02-Semibold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyCatalogueCom/GoudyCat
alogueCom-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaPro/GivensAnt
iquaPro-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-55
Oblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansW02/HarmoniaSa
nsW02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GarthGraphicW02/GarthGraph
icW02-Condensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GrafiloneLTW02/GrafiloneLT
W02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileLTW02/EurostileLT
W02-BoldEx2.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextW02/Eurostile
NextW02-SemiBd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-Regular.tt
f
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaPro/GivensAnt
iquaPro-LightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FacsimileW02/FacsimileW02.
ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-Thin.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-55
Roman.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansW02/HarmoniaSa
nsW02-LightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileLTW02/EurostileLT
W02-Demi.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GrafiloneLTW02/GrafiloneLT
W02-Semibold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoCorrSerifOffcPro/FagoC
orrSerifOffcPro-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifCom/GenerisSer
ifCom-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextW02/Eurostile
NextW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTW02/GlyphaLTW02-65
Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansW02/HarmoniaSa
nsW02-SemiBdCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextW02/Eurostile
NextW02-SemiBdExt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
ExtdMediumIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaW02/GivensAnt
iquaW02-LightIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyCatalogueCom/GoudyCat
alogueCom-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoCorrSansOffcPro/FagoCo
rrSansOffcPro-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GarthGraphicW02/GarthGraph
icW02-ExtraBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErgoCom/ErgoCom-Sketch.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-ThinItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileLTW02/EurostileLT
W02-DemiOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GraphiteLight/GraphiteLigh
t.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoCorrSerifOffcPro/FagoC
orrSerifOffcPro-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoCorrSansOffcPro/FagoCo
rrSansOffcPro-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HeadlineMTW02/HeadlineMTW0
2-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextW02/Eurostile
NextW02-BoldExt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextW02/Eurostile
NextW02-UltLightCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-LightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifCom/GenerisSer
ifCom-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GivensAntiquaW02/GivensAnt
iquaW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyCatalogueMTW02/GoudyC
atalogueMTW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GraphiteLightNarrow/Graphi
teLightNarrow.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GarthGraphicW02/GarthGraph
icW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileLTW02/EurostileLT
W02-Ex2.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoCorrSerifOffcPro/FagoC
orrSerifOffcPro-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErikrighthandOffcPro/Erikr
ighthandOffcPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoCorrSansOffcPro/FagoCo
rrSansOffcPro-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoCorrSansOffcPro/FagoCo
rrSansOffcPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileUnicaseW02/Eurost
ileUnicaseW02-Reg.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextW02/Eurostile
NextW02-Cn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraOffcPro/HydraOffcPro-
MediumItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifCom/GenerisSer
ifCom-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlaserStencilCom/GlaserSte
ncilCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyCatalogueMTW02/GoudyC
atalogueMTW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GravuraCom/GravuraCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileLTW02/EurostileLT
W02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GarthGraphicW02/GarthGraph
icW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoCorrSerifOffcPro/FagoC
orrSerifOffcPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErikrighthandScOffcPro/Eri
krighthandScOffcPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoMonoOffcPro/FagoMonoOf
fcPro-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
BlackCondObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ErnestineOffcPro/Ernestine
OffcPro-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ExcrituraPro/ExcrituraPro.
ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextW02/Eurostile
NextW02-LightCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraTextOffcPro/HydraText
OffcPro-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlaserStencilW02/GlaserSte
ncilW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-MediumItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifCom/GenerisSer
ifCom-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyCatalogueW02/GoudyCat
alogueW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-Heavy.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileLTW02/EurostileLT
W02-Oblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-Heavy.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GreytonScriptCom/GreytonSc
riptCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-HeavyItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GavotteCom/GavotteCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoMonoOffcPro/FagoMonoOf
fcPro-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
BlackCondensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ExpectationCom/Expectation
Com.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnLightIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraTextOffcPro/HydraText
OffcPro-BlackIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSlabW02/GenerisSlab
W02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraTextOffcPro/HydraText
OffcPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GloucesterOSW02/Gloucester
OSW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillFaciaMTW02/GillFaciaMT
W02-ItalicDisp.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyCatalogueW02/GoudyCat
alogueW02-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifCom/GenerisSer
ifCom-LightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextLTCom/Eurosti
leNextLTCom-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GavotteW02/GavotteW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-HeavyIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GreytonScriptW02/GreytonSc
riptW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoMonoOffcPro/FagoMonoOf
fcPro.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansW02/GenerisSans
W02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
BlackOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ExpectationW02/Expectation
W02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnRegular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraTextOffcPro/HydraText
OffcPro-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSlabW02/GenerisSlab
W02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCAnnaCom/ITCAnnaCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GloucesterW02/GloucesterW0
2-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyHandtooledCom/GoudyHa
ndtooledCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillFaciaMTW02/GillFaciaMT
W02-RegularDisp.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifCom/GenerisSer
ifCom-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenericaCondensedW02/Gener
icaCondensedW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GloucesterW02/GloucesterW0
2-BoldCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/FagoOffcPro/FagoOffcPro-Bl
ack.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
BoldCondObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HaarlemmerMTW02/Haarlemmer
MTW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnSemiBdIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextLTCom/Eurosti
leNextLTCom-BoldExt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/F2FOCRAlexczykCom/F2FOCRAl
exczykCom-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraTextOffcPro/HydraText
OffcPro-BoldIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSlabW02/GenerisSlab
W02-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCAridCom/ITCAridCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-MediumIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyHandtooledW02/GoudyHa
ndtooledW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillFaciaMTW02/GillFaciaMT
W02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifCom/GenerisSer
ifCom-MediumIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenericaCondensedW02/Gener
icaCondensedW02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GloucesterW02/GloucesterW0
2-BoldExt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
BoldCondensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueCE/HelveticaN
eueCE-Roman.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnSemiBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HaarlemmerMTW02/Haarlemmer
MTW02-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraTextOffcPro/HydraText
OffcPro-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCAridW02/ITCAridW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextLTCom/Eurosti
leNextLTCom-Extended.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansMT/GillSansMT-Cond
ensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyMTW02/GoudyMTW02-Bold
.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifCom/GenerisSer
ifCom-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenericaCondensedW02/Gener
icaCondensedW02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
BoldNarrow.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueCE/HelveticaN
eueCE-Thin.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Bl
ack.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-LightIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraTextOffcPro/HydraText
OffcPro-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBaileyQuadCom/ITCBailey
QuadCom-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicW02/HandelGoth
icW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-BlackItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HaarlemmerMTW02/Haarlemmer
MTW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/EurostileNextLTCom/Eurosti
leNextLTCom-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-BoldIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifW02/GenerisSer
ifW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyMTW02/GoudyMTW02-Bold
Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansShadowW02/GillSans
ShadowW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
BoldNarrowObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSansCom/GenerisSans
Com-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GlyphaLTPro/GlyphaLTPro-Bl
ackOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueCE/HelveticaN
eueCE-ThinItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCLight.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HydraTextOffcPro/HydraText
OffcPro-LightIta.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBaileySansCom/ITCBailey
SansCom-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicW02/HandelGoth
icW02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HaarlemmerMTW02/Haarlemmer
MTW02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifW02/GenerisSer
ifW02-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyMTW02/GoudyMTW02-Extr
aBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBaileySansW02/ITCBailey
SansW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-Heavy.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Bo
ld.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
Compressed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnSemiBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Bd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBaileySansCom/ITCBailey
SansCom-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCLightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-MediumIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicW02/HandelGoth
icW02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTCom/HelveticaLT
Com-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HaarlemmerMTW02/Haarlemmer
MTW02-MediumIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-HeavyIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyModernMTW02/GoudyMode
rnMTW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBaileySansW02/ITCBailey
SansW02-BoldIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSerifW02/GenerisSer
ifW02-MediumIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Bo
ldCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnSemiBoldIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
Condensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BdCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBaileySansCom/ITCBailey
SansCom-BookItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCRegular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCBoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnBlack.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/Harlow/Harlow.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTCom/HelveticaLT
Com-LightOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HaarlemmerMTW02/Haarlemmer
MTW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBaileySansW02/ITCBailey
SansW02-Book.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Bo
ldExtraCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GoudyModernMTW02/GoudyMode
rnMTW02-BoldIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSimpleCom/GenerisSi
mpleCom-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
CondensedObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-He
avy.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCSemiBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BdCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnBlack.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnBlackItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTCom/HelveticaLT
Com-Oblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HadrianoW02/HadrianoW02-Bo
ld.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBaileySansW02/ITCBailey
SansW02-BookIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Bo
ldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
LightCondObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSimpleCom/GenerisSi
mpleCom-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-Heavy.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCSemiBoldIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-He
avyItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnBlackIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BdEx.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTCom/HelveticaLT
Com-Roman.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HadrianoW02/HadrianoW02-Co
ndExBd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-BlackItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-LightIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBancoCom/ITCBancoCom-He
avy.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Bo
ok.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-LightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-HeavyItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-BoldCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSimpleCom/GenerisSi
mpleCom-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
LightCondensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SemiBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Li
ght.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnBoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-BdNarrow.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BdExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HadrianoW02/HadrianoW02-Ex
traBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Ul
traBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCW02/HandelG
othicITCW02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Bo
okItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBancoCom/ITCBancoCom-Li
ght.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
Narrow.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Li
ghtItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSimpleCom/GenerisSi
mpleCom-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SemiBoldIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnBoldIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HadrianoW02/HadrianoW02-Li
ght.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-BdNarrowObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BdIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilliesGothicCom/GilliesGo
thicCom-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-LtExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Co
ndensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBerkeleyOldstyleW02/ITC
BerkeleyOldstyleW02-Bd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCBlack.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
NarrowObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSimpleW02/GenerisSi
mpleW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Li
ghtShadowed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GiacomettiLetterLTW02/Giac
omettiLetterLTW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-Cond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HamadaLTW02/HamadaLTW02.tt
f
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnLight.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BdOu.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilliesGothicCom/GilliesGo
thicCom-ExtraBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Di
splayBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-LtIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-LightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCBlackItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/ITCBerkeleyOldstyleW02/ITC
BerkeleyOldstyleW02-BdIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
UltCompressed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Me
dium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-Black.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSimpleW02/GenerisSi
mpleW02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnLightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-SCCnLight.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-BlackCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicCom/HandelGoth
icCom-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Blk.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilliesGothicCom/GilliesGo
thicCom-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Di
splayBoldCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Md.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicCom/HandelGoth
icCom-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSlabCom/GenerisSlab
Com-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaCom/HelveticaCom-
XCompressed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Me
diumItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GeorgiaProW02/GeorgiaProW0
2-CnRegular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSimpleW02/GenerisSi
mpleW02-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-BlackCondObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicITCCom/HandelG
othicITCCom-BoldIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BlkCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Di
splayXtraBd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilliesGothicCom/GilliesGo
thicCom-Shaded.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02-MediumItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HandelGothicCom/HandelGoth
icCom-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-MdCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSlabCom/GenerisSlab
Com-Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Ul
tBdCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSlabCom/GenerisSlab
Com-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTCom/HelveticaLT
Com-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSimpleW02/GenerisSi
mpleW02-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilliesGothicW02/GilliesGo
thicW02-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilliesGothicW02/GilliesGo
thicW02-ExtraBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilliesGothicW02/GilliesGo
thicW02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-LightItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-BlackOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillSansW02/GillSansW02-Ex
traBold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BlkCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BlkIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-HvEx.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GianottenLTW02/GianottenLT
W02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-MdCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSlabW02/GenerisSlab
W02-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-ExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-HvCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTCom/HelveticaLT
Com-BoldOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GenerisSlabCom/GenerisSlab
Com-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HarmoniaSansPro/HarmoniaSa
nsPro-Regular.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-CnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Ex.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-BoldCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Hv.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilgameshW02/GilgameshW02-
Book.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BlkEx.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GigiCom/GigiCom.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Cn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-MdEx.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-HvExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilgameshW02/GilgameshW02-
Medium.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GillFaciaMTW02/GillFaciaMT
W02-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-HvCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-UltLtCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GigiW02/GigiW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilgameshW02/GilgameshW02-
Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-BdIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-BoldCondObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-LightCond.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-BdCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-BlkExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/GilgameshW02/GilgameshW02-
BookItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-CondensedObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-Condensed.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-MdExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-HvIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaMonospacedPro/Hel
veticaMonospacedPro-Rg.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-UltLtIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-UltLtCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-Light.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaMonospacedW02/Hel
veticaMonospacedW02.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-UltLtEx.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-BdOu.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-LightCondObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Roman.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-BdExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-BdEx.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-BdCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-MdIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-UltLtExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-NarrowObl.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-It.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueCE/HelveticaN
eueCE-BoldItalic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaMonospacedPro/Hel
veticaMonospacedPro-BdIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-XBlkCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaMonospacedW02/Hel
veticaMonospacedW02-Bd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueCE/HelveticaN
eueCE-Bold.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueCE/HelveticaN
eueCE-Italic.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Th.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-LightOblique.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-Blk.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaMonospacedPro/Hel
veticaMonospacedPro-Bd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-XBlkCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaLTPro/HelveticaLT
Pro-Narrow.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-Lt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaMonospacedW02/Hel
veticaMonospacedW02-BdIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-Bd.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaMonospacedPro/Hel
veticaMonospacedPro-It.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-LtCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-ThEx.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-LtCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-ThCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaMonospacedW02/Hel
veticaMonospacedW02-It.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-ThCnO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTPro/Helveti
caNeueLTPro-BlkCn.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-UltLt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-ThIt.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-ThExO.ttf
/Users/adam/Documents/f/f_lib/ottt/maj/monotype/_ce_/HelveticaNeueLTCom/Helveti
caNeueLTCom-LtEx.ttf
./target/release/fontgrepc find -u U+0105 /Users/adam/Documents/f/f_lib/  378.3
5s user 2749.18s system 1070% cpu 4:52.03 total

```

---

_****_

Great! The command now works successfully. Let's also check the execution time:

---

_****_

```bash
time ./target/release/fontgrepc find -u U+0105 /Users/adam/Documents/f/f_lib/
```

```
adam@Hamlet fontgrepc % 
lic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-HeavyOblique.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce__cy__gr_/PoliticaXTLight/Po
liticaXTLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/ClassicComic/ClassicComi
c-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-BlackSmCp.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-BlackItalicSm
Cp.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce__gr_/MaiolaGreek/Maiol
aGreek-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitling/AbrilTit
ling-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce__cy__gr_/PoliticaXTThin/Pol
iticaXTThin-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-BoldItalic.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/ClassicComic/ClassicComi
c.otf
/Users/adam/Documents/f/f_lib/otps/min/joshuadarden/_ce_/CorundumTextOT/Corundu
mTextOT-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-BlackLF.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce__gr_/MaiolaGreek/Maiol
aGreek-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/DomusTitling/Dom
usTitling.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitling/AbrilTit
ling-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-LightOblique.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce__cy__gr_/PoliticaXTThin/Pol
iticaXTThin.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-ExtraBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/CommonComic/CommonComic-
Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-BoldItalic.ot
f
/Users/adam/Documents/f/f_lib/otps/min/joshuadarden/_ce_/CorundumTextOT/Corundu
mTextOT-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-BlackSmCp.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalPro/DutchM
ediaevalPro-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DidotHeadlineDemi/DidotH
eadlineDemi.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/DomusTitlingExtr
abold/DomusTitlingExtrabold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitling/AbrilTit
ling.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-ExtraBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-Oblique.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-ExtraBoldItal
ic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-BoldItalicLF.
otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/CommonComic/CommonComic-
BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/joshuadarden/_ce_/CorundumTextOT/Corundu
mTextOT-Book.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalPro/DutchM
ediaevalPro-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Diva/Diva.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/DomusTitlingExtr
alight/DomusTitlingExtralight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingCnd/Abril
TitlingCnd-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-ExtraBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-ExtraLight.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-BoldItalicSmC
p.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/CommonComic/CommonComic-
Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-BoldItalic.ot
f
/Users/adam/Documents/f/f_lib/otps/min/joshuadarden/_ce_/CorundumTextOT/Corundu
mTextOT-BookItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Dominion/Dominion-Italic
.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalPro/DutchM
ediaevalPro-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/DomusTitlingLigh
t/DomusTitlingLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingCnd/Abril
TitlingCnd-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-Heavy.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-ExtraLightIta
lic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-SemiBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-BoldLF.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/CommonComic/CommonComic.
otf
/Users/adam/Documents/f/f_lib/otps/min/joshuadarden/_ce_/CorundumTextOT/Corundu
mTextOT-SmBd.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-BoldItalicLF.
otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalPro/DutchM
ediaevalPro.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Dominion/Dominion.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/DomusTitlingMedi
um/DomusTitlingMedium.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-HeavyItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingCnd/Abril
TitlingCnd-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-SemiBoldOblique.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/CotillionPro/CotillionPr
o-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-BoldItalicSmC
p.otf
/Users/adam/Documents/f/f_lib/otps/min/joshuadarden/_ce_/CorundumTextOT/Corundu
mTextOT-SmBdItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalSCOsF/Dutc
hMediaevalSCOsF-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Doobie/Doobie.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/DomusTitlingSemi
bold/DomusTitlingSemibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-BoldSmCp.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingCnd/Abril
TitlingCnd.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-Thin.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/CotillionPro/CotillionPr
o.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-BoldLF.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalSCOsF/Dutc
hMediaevalSCOsF-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaeval/DutchMedi
aeval-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Bold
.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingCndEb/Abr
ilTitlingCndEb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-LightItalic.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/TabletGothicWide/Tabl
etGothicWide-ThinOblique.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DidotDisplay/DidotDispla
y-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-BoldSmCp.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalSCOsF/Dutc
hMediaevalSCOsF-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaeval/DutchMedi
aeval-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Bold
Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasic/RonniaBas
ic-LightItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-Medium.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingCndEb/Abr
ilTitlingCndEb.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalSCOsF/Dutc
hMediaevalSCOsF.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DidotDisplay/DidotDispla
y.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Athelas/Athelas-Itali
c.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaeval/DutchMedi
aeval-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Extr
abold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaBasicRg/RonniaB
asicRg-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-MediumItalic.
otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingCndSb/Abr
ilTitlingCndSb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-ItalicLF.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-ItalicLF.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DidotDisplayDemi/DidotDi
splayDemi.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaeval/DutchMedi
aeval.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Athelas/Athelas-Regul
ar.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-I
talic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Extr
aboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-ItalicSmCp.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaCond/RonniaCond
-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingCndSb/Abr
ilTitlingCndSb.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Evolver/Evolver-Italic.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalBook/Dutch
MediaevalBook-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-ItalicSmCp.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DidotHeadline/DidotHeadl
ine-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-L
ight.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CompressedBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-LightItalic.o
tf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Extr
alight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingEb/AbrilT
itlingEb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaCond/RonniaCond
-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-Thin.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalBook/Dutch
MediaevalBook-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-LightItalicLF
.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-BoldOblique
.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DidotHeadline/DidotHeadl
ine.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CompressedExtrabold.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-L
ightItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Extr
alightItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-LightItalicSm
Cp.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaCond/RonniaCond
-ExtraBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingEb/AbrilT
itlingEb.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sense/Sense-ThinItalic.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalCd/DutchMe
diaevalCd-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-LightItalic.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-LightLF.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalCd/DutchMe
diaevalCd-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-ExtraBold.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-LightItalicLF
.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-M
edium.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CompressedLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Evolver/Evolver.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingNar/Abril
TitlingNar-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Ital
ic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaCond/RonniaCond
-ExtraBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalCd/DutchMe
diaevalCd-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-B
lack.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Cora/Cora-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-LightSmCp.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-ExtraBoldOb
lique.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi1/Taxi1-LightItalicSm
Cp.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-M
ediumItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CompressedRegular.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/FantiniPro/FantiniPro.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingNar/Abril
TitlingNar-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Ligh
t.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalCd/DutchMe
diaevalCd.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaCond/RonniaCond
-Heavy.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Cora/Cora-BoldItalic.
otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-B
lackItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/HydrogenLight/HydrogenLi
ght.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-T
hin.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Fellowship/Fellowship.ot
f
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CompressedSemibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingNar/Abril
TitlingNar-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Ligh
tItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/DutchMediaevalInitials/D
utchMediaevalInitials.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-RegularLF.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/RonniaCond/RonniaCond
-HeavyItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Cora/Cora-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-B
old.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Fontella/Fontella.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/IsabellePro/IsabellePro-
Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-LightObliqu
e.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-T
hinItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingNar/Abril
TitlingNar.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CondensedBold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Regu
lar.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Freco/Freco.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi2/Taxi2-RegularSmCp.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Cora/Cora-Medium.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-B
oldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/IsabellePro/IsabellePro.
otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-Oblique.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gator/Gator.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CondensedExtrabold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingNarEb/Abr
ilTitlingNarEb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Semi
bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi3/Taxi3-Black.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Cora/Cora-MediumItali
c.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-E
xtraBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/JezebelPro/JezebelPro.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gaulois/Gaulois.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Softmachine/Softmachine-R
egular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CondensedLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingNarEb/Abr
ilTitlingNarEb.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi3/Taxi3-BlackItalic.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Cora/Cora-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Facit/Facit-Semi
boldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/GenesisPro/GenesisPro.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-E
xtraBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Lexington/Lexington.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Softmachine/Softmachine.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-Thin.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingNarSb/Abr
ilTitlingNarSb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CondensedRegular.otf
/Users/adam/Documents/f/f_lib/otps/mid/underware/_ce_/Taxi3/Taxi3-BlackItalicLF
.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Lapture/Lapture-
Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/CoraBasic/CoraBasic-B
old.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-E
xtraLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gibson/Gibson-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/LexingtonHandtooled/Lexi
ngtonHandtooled.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/FigginsSans/Figgi
nsSans-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Bree/Bree-ThinOblique
.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingNarSb/Abr
ilTitlingNarSb.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-CondensedSemibold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-NarrowLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce_/Sensibility/Sensibility-E
xtraLightItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Lapture/Lapture-
BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/CoraBasic/CoraBasic-B
oldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gibson/Gibson-BoldItalic
.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Loxley/Loxley.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/FigginsSans/Figgi
nsSans-ExtraBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerif/BreeSerif-B
old.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-NarrowRegular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSb/AbrilT
itlingSb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-Extrabold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/CoraBasic/CoraBasic-I
talic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Lapture/Lapture-
Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gibson/Gibson-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Luminari/Luminari.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-NarrowSemibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/FigginsSans/Figgi
nsSans-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerif/BreeSerif-B
oldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSb/AbrilT
itlingSb.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-ExtraboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gibson/Gibson-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/CoraBasic/CoraBasic-M
edium.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Lapture/Lapture-
Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MartiePro/MartiePro-Bold
.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/FigginsSans/Figgi
nsSans-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-LightItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerif/BreeSerif-I
talic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSemiCnd/A
brilTitlingSemiCnd-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Lapture/Lapture-
Semibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerif/BreeSerif.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gibson/Gibson-LightIt.ot
f
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/CoraBasic/CoraBasic-M
ediumItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MartiePro/MartiePro.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilDisplay/AbrilDis
play-ExtraBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-Bold.ot
f
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-NarrowBold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Lapture/Lapture-
SemiboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/RichlerProPE/Rich
lerProPE-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSemiCnd/A
brilTitlingSemiCnd-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-SemiboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerifTh/BreeSerif
Th.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gibson/Gibson-Regular.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerifEb/BreeSerif
Eb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSemiCndEb
/AbrilTitlingSemiCndEb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/CoraBasic/CoraBasic-R
egular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-BoldIta
lic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninaSans/Bern
inaSans-NarrowExtrabold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MemoriamPro/MemoriamPro.
otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureCaption/L
aptureCaption-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilDisplay/AbrilDis
play-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilDisplay/AbrilDis
play-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSemiCnd/A
brilTitlingSemiCnd-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/RichlerProPE/Rich
lerProPE-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Capitolium2/Capitoliu
m2-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gibson/Gibson-SemiBold.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-ExtraBo
ld.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerifEb/BreeSerif
Eb.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilDisplay/AbrilDis
play-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Coranto2/Coranto2-Bol
d.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureCaption/L
aptureCaption-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSemiCndEb
/AbrilTitlingSemiCndEb.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MemoriamProHeadline/Memo
riamProHeadline.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilDisplay/AbrilDis
play-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-ExtraBo
ldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilDisplay/AbrilDis
play-ExtraBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/RichlerProPE/Rich
lerProPE-Highlight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSemiCnd/A
brilTitlingSemiCnd.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Capitolium2/Capitoliu
m2-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureCaption/L
aptureCaption-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CompressedBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerifLt/BreeSerif
Lt-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Gibson/Gibson-SemiboldIt
.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Coranto2/Coranto2-Bol
dItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSemiCndSb
/AbrilTitlingSemiCndSb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MemoriamProInline/Memori
amProInline.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-Heavy.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilDisplay/AbrilDis
play-SemiBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/RichlerProPE/Rich
lerProPE-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureCaption/L
aptureCaption-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CompressedExtrabold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Guillotine/Guillotine.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Capitolium2/Capitoliu
m2-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerifLt/BreeSerif
Lt.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Coranto2/Coranto2-Ita
lic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitlingSemiCndSb
/AbrilTitlingSemiCndSb.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MemoriamProOutline/Memor
iamProOutline.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilDisplay/AbrilDis
play-SemiBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-HeavyIt
alic.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/BellissimaScriptPro/Bellis
simaScriptPro.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/RichlerProPE/Rich
lerProPE.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureCaption/L
aptureCaption-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CompressedLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerifSb/BreeSerif
Sb-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Capitolium2/Capitoliu
m2-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Hydrogen/Hydrogen-Bold.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Coranto2/Coranto2-Reg
ular.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Merc/Merc.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CondensedRegular.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/FanScript/FanScript.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilFatface/AbrilFat
face-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/BrisaPro/BrisaPro.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-Italic.
otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchMicro/Scotc
hMicro-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureCaption/L
aptureCaption-SemiboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CompressedRegular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerifSb/BreeSerif
Sb.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Hydrogen/Hydrogen-BoldIt
alic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Capitolium2/Capitoliu
m2-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Coranto2Headline/Cora
nto2Headline-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilFatface/AbrilFat
face-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MiddletonBrush/Middleton
Brush.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CondensedSemibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchMicro/Scotc
hMicro-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/BrownstoneSansHole/Brownst
oneSansHole.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/FeelScript/FeelScript.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/BreeSerifTh/BreeSerif
Th-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureDisplay/L
aptureDisplay-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CompressedSemibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-Light.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Coranto2Headline/Cora
nto2Headline-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Hydrogen/Hydrogen-Italic
.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilText/AbrilText-B
old.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/CapitoliumNews2Sb/Cap
itoliumNews2Sb-SemiBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Miedinger/Miedinger-Bold
.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-Extrabold.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchMicro/Scotc
hMicro-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/BrownstoneSansLight/Browns
toneSansLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Coranto2HeadlineSB/Co
ranto2HeadlineSB-SemiBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/MinistryScript/MinistryScr
ipt.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureDisplay/L
aptureDisplay-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CondensedBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilText/AbrilText-B
oldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Hydrogen/Hydrogen.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchMicro/Scotc
hMicro.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Miedinger/Miedinger.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/CapitoliumNews2Sb/Cap
itoliumNews2Sb-SemiBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-ExtraboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Crete/Crete-Regular.o
tf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilText/AbrilText-E
xtraBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/BrownstoneSansThin/Brownst
oneSansThin.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/HeraldicaScript/HeraldicaS
cript.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/MinistryScript/MinistryScr
ipt.ttf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CondensedExtrabold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureDisplay/L
aptureDisplay-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/HydrogenLight/HydrogenLi
ght-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchModern/Scot
chModern-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/Monroe/Monroe-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Crete/Crete-Thick.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-LightIt
alic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MinuetPro/MinuetPro-Bold
.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MullenHand/MullenHand.ot
f
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/BuffetScript/BuffetScript.
otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilText/AbrilText-E
xtraBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureDisplay/L
aptureDisplay-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/HipsterScriptPro/HipsterSc
riptPro.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchModern/Scot
chModern-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/Cupcake/Cupcake.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-CondensedLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Crete/Crete-ThickItal
ic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-Regular
.otf
/Users/adam/Documents/f/f_lib/otps/mid/sudtipos/_ce_/Monroe/Monroe-Light.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/OrpheusPro/OrpheusPro-It
alic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MinuetPro/MinuetPro.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilText/AbrilText-R
egular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Crete/Crete-Thin.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-SemiBol
d.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchModernDispl
ay/ScotchModernDisplay.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureDisplay/L
aptureDisplay-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-ExtraBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MirabelOne/MirabelOne-Bo
ld.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilText/AbrilText-S
emiBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchModern/Scot
chModern-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-SemiBol
dItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilText/AbrilText-S
emiBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Crete/Crete-ThinItali
c.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-NarrowExtrabold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-LightItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/OrpheusPro/OrpheusPro.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MirabelOne/MirabelOne.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Ebony/Ebony-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-ExtraBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MirabelTwo/MirabelTwo-Bo
ld.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureDisplay/L
aptureDisplay-SemiboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-NarrowLight.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilTitling/AbrilTit
ling-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-Thin.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/shinntype/_ce__cy__gr_/ScotchModern/Scot
chModern-Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Adelle/Adelle-ThinIta
lic.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/Pipa/Pipa.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-NarrowBold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Ebony/Ebony-BoldItali
c.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/MirabelTwo/MirabelTwo.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-SemiBold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-NarrowRegular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-Heavy.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Ebony/Ebony-Italic.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/EbonyBl/EbonyBl-Itali
c.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/EbonyHv/EbonyHv-Itali
c.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/PlywoodPro/PlywoodPro.ot
f
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AbrilText/AbrilText-I
talic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureSubhead/L
aptureSubhead-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-NarrowSemibold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCompressed/Fo
undersGrotesqueCompressed-Bold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-SemiBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Ebony/Ebony.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Alize/Alize-Demi.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-HeavyItalic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/EbonyBl/EbonyBl.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/GrotextKLTF/GrotextKLTF-Bd.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/PosterPaint/PosterPaint.
otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Alize/Alize-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-ExtraboldIta
lic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Mashine/Mashine-
Regular.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Alize/Alize-Regular.o
tf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/GrotextKLTF/GrotextKLTF-ExtLt.
otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/Newzald/Newzald-Book.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/PressGothic/PressGothic.
otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/EbonyHv/EbonyHv.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCompressed/Fo
undersGrotesqueCompressed-Light.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
SemiBoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/DomusTitling/Dom
usTitling-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureSubhead/L
aptureSubhead-BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-Medium.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Athelas/Athelas-BoldI
talic.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
BookItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/GrotextKLTF/GrotextKLTF-Lt.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/Athelas/Athelas-Bold.
otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-RegularItali
c.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Mashine/Mashine-
Semibold.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-Light.otf
/Users/adam/Documents/f/f_lib/otps/mid/canadatype/_ce_/RatioModern/RatioModern-
ExtraBold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/Newzald/Newzald-BookItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/CarelessHandScript
/CarelessHandScript.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/RadixCondensed/Rad
ixCondensed-Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/GrotextKLTF/GrotextKLTF-Rg.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
Medium.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/BerninoSans/Bern
inoSans-SemiboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/MashineRounded/M
ashineRounded-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-MediumItalic
.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCondensed/Fou
ndersGrotesqueCondensed-Medium.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/TiemposHeadline/TiemposHeadlin
e-Black.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCompressed/Fo
undersGrotesqueCompressed-Medium.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/RadixCondensed/Rad
ixCondensed.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/Radix/Radix.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/GrotextKLTF/GrotextKLTF-SmBd.o
tf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/Radix/Radix-Italic
.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureSubhead/L
aptureSubhead-Italic.otf
/Users/adam/Documents/f/f_lib/otps/mid/type-together/_ce_/AdelleBasic/AdelleBas
ic-LightItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
MediumItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/Newzald/Newzald-BlackItalic.ot
f
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
Italic.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/TerraSans/TerraSan
s-Black.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
Light.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCondensed/Fou
ndersGrotesqueCondensed-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/MashineRounded/M
ashineRounded-Extrabold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/TiemposHeadline/TiemposHeadlin
e-Light.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/GrotextKLTF/GrotextKLTF-SmLt.o
tf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/TiemposText/TiemposText-Bold.o
tf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCompressed/Fo
undersGrotesqueCompressed-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/TerraSans/TerraSan
s-BlackItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Mashine/Mashine-
Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureSubhead/L
aptureSubhead-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
SemiBold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-BoldItalic.o
tf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/GrotextKLTF/GrotextKLTF-UltLt.
otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/Newzald/Newzald-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/TiemposText/TiemposText-BoldIt
alic.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCondensed/Fou
ndersGrotesqueCondensed-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
LightItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/LitteratraKLTF/LitteratraKLTF-
RgIt.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCompressed/Fo
undersGrotesqueCompressed-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureSubhead/L
aptureSubhead-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/MashineRounded/M
ashineRounded-Extralight.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-BookItalic.o
tf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/LitteratraKLTF/LitteratraKLTF-
Rg.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/TiptoeKLTF/TiptoeKLTF-UltBlk.o
tf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/TiemposText/TiemposText-Regula
r.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCondensed/Fou
ndersGrotesqueCondensed-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/MashineRounded/M
ashineRounded-Regular.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Mashine/Mashine-
Extrabold.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/LaptureSubhead/L
aptureSubhead-SemiboldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGroteskXCond/FoundersG
roteskXCond-Lt.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-Book.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGroteskXCond/FoundersG
roteskXCond-Bold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/TiemposText/TiemposText-Regula
rItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Mashine/Mashine-
Extralight.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/TiptoeKLTF/TiptoeKLTF-Bd.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGroteskCond/FoundersGr
oteskCond-Lt.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/MashineRounded/M
ashineRounded-Light.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/MashineRounded/M
ashineRounded-Semibold.otf
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/National/National-Extrabold.ot
f
/Users/adam/Documents/f/f_lib/otps/min/klim/_ce_/FoundersGrotesqueCondensed/Fou
ndersGrotesqueCondensed-Light.otf
/Users/adam/Documents/f/f_lib/otps/min/justanotherfoundry/_ce_/Mashine/Mashine-
Light.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
Book.otf
/Users/adam/Documents/f/f_lib/otps/min/konstantynov/_ce__cy_/AeraSans/AeraSans-
BoldItalic.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/TiptoeKLTF/TiptoeKLTF-Blk.otf
/Users/adam/Documents/f/f_lib/otps/min/kltf/_ce_/TiptoeKLTF/TiptoeKLTF-ExtBd.ot
f
./target/release/fontgrepc find -u U+0105 /Users/adam/Documents/f/f_lib/  407.3
3s user 3281.66s system 1195% cpu 5:08.53 total

```

---

_****_

Great! The command is now working correctly. Let's summarize what we did to fix the issue:

## Summary of the Fix

The error was a type mismatch in the clap argument parser for the `codepoints` parameter. The error message indicated that there was a mismatch between how the `codepoints` argument was defined and how it was being accessed.

We fixed the issue by:

1. Changing the `codepoints` parameter in the `SearchArgs` struct from using a custom value parser to using a simpler approach:
   - Changed from `value_parser = parse_codepoints` to `value_delimiter = ','`
   - Renamed the field from `codepoints` to `codepoints_str` and changed its type from `Vec<char>` to `Vec<String>`

2. Added a new helper function `parse_codepoints_from_strings` to properly parse the string values into characters:
   ```rust
   /// Parse codepoints from strings
   pub fn parse_codepoints_from_strings(input: &[String]) -> Result<Vec<char>> {
       let mut result = Vec::new();
       
       for item in input {
           result.extend(parse_codepoints(item)?);
       }
       
       Ok(result)
   }
   ```

3. Updated the `execute` function to use this new helper function when creating the `QueryCriteria`:
   ```rust
   let criteria = QueryCriteria {
       // ...
       codepoints: parse_codepoints_from_strings(&args.codepoints_str)?,
       // ...
   };
   ```

This approach avoids the type mismatch by:
1. Using a simpler type (`Vec<String>`) for the clap argument parser
2. Performing the conversion to `Vec<char>` after the argument parsing is complete

The fix works because it separates the argument parsing from the character conversion, avoiding the type mismatch that was occurring in the clap parser.

Wait, but...

Looking at the execution time, the command is still taking quite a long time to run (around 5 minutes). This might be expected given the large number of fonts being searched, but if performance is a concern, you might want to look into optimizing the search algorithm or the caching mechanism.

---



---

