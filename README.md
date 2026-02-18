## pic2text

Convert pictures to ascii art.


### Usage

![cat](baimou.jpg)

```text
cargo run -- -h

Usage: pic2text [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  image

Options:
  -i, --invert             invert grayscale
  -a, --aratio <ARATIO>    aspect ratio (character height to width) [default: 2]
  -w, --width <WIDTH>      max picture width in characters [default: 80]
  -s, --symbols <SYMBOLS>  Custom character set (ordered from dark to bright)
      --ascii              Use ASCII character set (default)
      --braille            Use Braille character set for high resolution
  -h, --help               Print help
  -V, --version            Print version
```

### Comparison of Modes

| Mode | Symbols | Tonal Range | Use Case |
|------|---------|-------------|----------|
| `--ascii` (default) | ~38 | High | Classic ASCII art, best tonal gradation |
| `--symbols` | Custom | Variable | Artistic control, specific aesthetics |
| `--braille` | 9 | Low | Clean geometric patterns, minimalist look |


### Ascii Mode (`--ascii`)

Map to ascii range (default). Note that many symbols have the same 'brightness' - only one representative symbol per brightness is used. 
```
cargo run -- -w 80 baimou.jpg

777xx7777xxxxxxxxxx77xxxxxxxxxxxxxxxxxx777777fffffff777xxxxxx777xxxssxxss7xtsx7f
f77xx7777xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx77777ffffff777xxxxxxxx77xxxxsxssxxstsxff
f77x77777xxxxxxxxxxsx77x7xxxxxxxxxxxxxxx777777fff77777xxxxxxxxxxxxxxxxsx7stsx7fC
ff777xxxxxxxxxxxx/,;:;!fxx7xxxxxxxxxxxxxxxx777ff77777xxxxxxxxx7ff77xsxsxxttsx7fC
fff777xxxxxxxxxxx!:;;:.;x7xxsxxxxxxxxxxxxxxx77777777xxxxsxxs77t/;:/sssx7s!ssx7CC
fff777xxxxxxxx7xxi:;=;: .i7x7xxxxxxxxxxxxxxxx777777xxxxxss77i,     /xxxstttsx7C3
Cff77777xxxxxxxx,.;i+;:,. ;x7xxxxxxxxxxxxxxx777777xxxxxxx7+,  .,,, ,f7sttstsxf3C
ffff77777xxx7xxx.,iss!/;:, .!7xxxxxxxxxxxxxx77xxxx7xxsx7i,  ..,,,: :fsttttss7Cq7
dCCCf77777xxxx7x.,!sffft/;,  ;x7xxxxxxxxxxx77xxxxxxx77!,  ..,,::;; :7i!tttsxf3qs
&XCC7f777xxx7xx7,:!+tX&X3s/:. ,!7xx77xxxxx7xxxxxxxx7t:  .,,:;;///; ;ti!tttsxfqd!
7&Xf7777777777xf;,+!!CdAXCt;,.  :x7xx7f7f7777xxxxxx/   .:;=+!ti+=: =t!!ttts73Xdi
=x&XCf77777777x7!.;!77x7fCx+;.   ,tfCqqqddqqq333f+.   .,:;/!xx/;;,,!tt!tts7Cd&q+
dqdXXXdddddddddq7  ,,+CCs!!i;.    :xfq33XdCdCCx/,     ,:;=++ii+/;.+!tttttxCdADC/
XXXXXXXhhhhhhhh&d..:;,:f7t7CC77t!3XA&hXCX37Ahdf: ...,.,,,,;=/+:,  !!!!txxCdh&D7=
qddXXhXXXXXhhhXAq..,;!+7A&DRDD&DAAXhX3xs!xi7CXXdfss7ttx;::,,::,. ,t!tts7Cdh&&Dx;
ddXhhAhXhhhhhAAAh.  ..;3ADDDDD&AAXq7sx!/;t7!!t3qX3qqqhdCf+,.,:;, +xttsxfqXhA&Ds;
qdXXXXXXhhAAAAAhDx  .;7hA&DRDDDAAAfftxt/i!7!x!fqXddXX3Cdfi, .., .CqCf7f3dhA&DD!;
C3dddXddXhA&AA&&&D=  ih&&DD&h3C3ftsCi+i;!s++fttf33q3CCfsxs;     !dXXXddqdh&&@Di;
CC3qddXXhhAAAA&&DDt.;3XAD&hqx!tt/;!f+;;:=/;+!=+i!!ttxxxttii;   /hXXXhhhhhAA&DDi:
fC33qdXXhhAAA&&&DD3Cd&A&hqx+/tx!:,+i;:,,;::/i;;+ti+!txxfx!!i. .CAhhAAhA&&&A&DRt:
fC33qdXhhhAAAA&DDDADD&Aqx!ixXx+si:;::,,,,,,;;;;tx7!!t!xffx!x+;/dAAAAAAA&D&&DDR7:
fC3qqdXhhAAA&DD&&&&&&X7+:;tRx ..qf;,.,.....,;/s+:/3qi/+s77ssfxCAAAA&&&A&D&&D&Dd:
CC3qddXhhAA&&&DDDDDAX3t=:=;3diiiRh;:,.... .::xX;. =D!/,,=txtsqAAAA&&&&&&&&D&&D&;
CC3qddXhhAA&&&DDDDD&Xdq+:/+;!ff3qt,::.,,. .,.+Xd!/3x/i;:;ittdAAAAA&&&&&DDDDDDDDt
CC3qddhhAA&&&DDDDDD&AhdC/:/!+//=,. :,.,,.   ,:;/ts+it/;;i!sXAAAAA&&&&&&DDDDDDDDq
CC3qddXhAA&&&DDDDRDAhAXdf/,:/++;. ..  ;;     ,/!t!ss;:=itx3hAA&A&&&&&&DDDDDDDDDA
CC3qqdXXhA&&DDDDDDAAhhhhdCt;,...,.:tCq&As.    .:::;;;+!sCf!xdAAA&&&&&&DDDDDD&DDD
fC3qqddXhA&&DDDAAhdhhXA&DAd7i:,,.iqDR&A&Dx:: ....,:;!xCXXC/+!ChAA&&&&&DDDDDDD&DD
fCC3qddXhAA&DDAhXddXhhh&DDD&x!i;tNDXAhhhd3Aq:.:;;;s3XhAhX7;;;iq&&AA&DDDDDDDDDD&D
7fCC3qqdhhA&&AhXXhhhAAA&DDDRd7f7DNRhXXAqq&@R&7t+tXDD&Ahdd7;;::sh&A&DDDDDDDDDDDD&
x7ffC3qdXhhA&AAhhhAhAAh&DDDDRA3qDNNND3q&@@@RRdxf&DD&AAAAAx==;:;Ch&DDDDDDDDDDDDDD
sxfffC3qdXhAAhXXdXAhAAAAADDDDRD&DD@@RA&DD&&DAqhDDDDA&&&&Xi+//=;!hDDDDDDDDDDDDDDD
tsx7fCqqddXhhXqCqhhhhhAA&&DDDDDDDDR@@RDDD&&RDRRDDDDAAA&&d!=;;=;+XD&DDDDDDDDDDDDD
!tsxxfCCqqXXqCfCqXXXhAA&&DDDDD&&DDDDDDDDRRR@@DDDDD&&&&hAfi!+==;;qD&DDDDDDDDDDDDD
iit!fq3337fq3f7f3XXhhhA&DDDDDDD&DDDDRRRRRRR@@RDDD&&&AAhd!//=++/;CDDDDDDDDDDDDDDD
i!sxCqddXCttx77f3XhhhA&&&DDDD&&&&DDDDDDR@@@@RRDDD&AAAhdx++//==//CDD&&DDDDDDDDDDD
+itx73qddXfsxxfC3qdXh&&&&DD&&&&A&&&&DDDDRRRRDDD&&AAA&hfi/==///==3D&DDDDDDDDDDDDD
=/i!s7f33dX7x7f3q3qXA&&&&&&&&AA&AAAAAA&&DDDDD&&AAhAXfst+//==//=/XD&DDDDDDDDDDDDD
=/++!tx7f3dqxxf3qqqXAAA&AAAAAhhhdddXdXhAA&&&AAAXhX3ti+i+=/////=!AD&DDDDDDDDDDDDD
```


### Custom Mode 

Map each pixel to one of a given number of symbols. It is assumed that the symbols are already sorted in order of brighness (black to white). The brighness range of a symbol can be expanded by repeating the symbol.

Note that while there are many unicode symbols that are potentially useful, they don't always mix well,
because the widths may vary.

```
cargo run -- --symbols " .:░▒▓█" --width 80 --aratio 2 baimou.jpg

░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░:░░░::░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░:░░░░
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░::░░░░
░░░░░░░░░░░░░░░░░:....:░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░:::░░░░
░░░░░░░░░░░░░░░░░:.... .░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░::..:░:░░░:::░░░░░
░░░░░░░░░░░░░░░░░:.....  :░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░:.     :░░░░:::░░░░░
░░░░░░░░░░░░░░░░. .::...  .░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░:.   ... .░░░:::::░░░░
░░░░░░░░░░░░░░░░ .:░:::...  :░░░░░░░░░░░░░░░░░░░░░░░░░░░:.    .... .░░:::::░░░░░
▒░░░░░░░░░░░░░░░ .:░░░░::..  .░░░░░░░░░░░░░░░░░░░░░░░░:.    ...... .░:::::░░░░▒:
▒▒░░░░░░░░░░░░░░..:::▒▒▒░░:.  .:░░░░░░░░░░░░░░░░░░░░:.   .....:::. .:::::::░░▒▒:
░▒▒░░░░░░░░░░░░░..:::░▒▒▒░:..   .░░░░░░░░░░░░░░░░░░:    ..:::::::. :::::::░░░▒▒:
:░▒▒░░░░░░░░░░░░: .:░░░░░░░:.    .:░░░░▒▒▒░░░░░░░:     ...::░░:....:::::::░░▒▒▒:
▒░▒▒▒▒▒▒▒▒▒▒▒▒▒▒░  ..:░░░:::.     .░░░░░▒▒░▒░░░:.     ...:::::::. :::::::░░▒▒▒░:
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  ....░░:░░░░░::░▒▒▒▒▒░▒░░▒▒▒░.    . .....:::..  :::::░░░▒▒▒▓░.
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  ..::░▒▒▒▓▓▓▒▒▒▒▒▒▒░░░:░:░░▒▒▒░░░░::░........  .:::::░░▒▒▒▒▓░.
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒     .░▒▓▓▓▓▓▒▒▒▒▒░:░::.:░:::░▒▒░░░░▒▒░░:. .... :░:::░░▒▒▒▒▒▒░.
░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░   .░▒▒▒▓▓▓▓▓▒▒▒░░:░::::░:░:░░▒▒▒▒▒░░▒░:.   .  ░░░░░░░▒▒▒▒▓▓:.
░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒.  :▒▒▒▒▓▒▒░░░░:░░:::.:░::░::░░░░░░░░:░:.     :▒▒▒▒▒▒▒▒▒▒▒▓▓:.
░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒: .░▒▒▒▒▒▒░::::.:░:...::.:::::::::░░░::::.   :▒▒▒▒▒▒▒▒▒▒▒▒▒▓:.
░░░░░▒▒▒▒▒▒▒▒▒▒▒▓▒░░▒▒▒▒▒░░:::░:..::.......::..::::::░░░░:::   ░▒▒▒▒▒▒▒▒▒▒▒▒▒▓:.
░░░░░▒▒▒▒▒▒▒▒▒▒▒▓▒▒▓▓▒▒▒░::░▒░:░:..............:░░::::░░░░:░:.:▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓░.
░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░:..:▓░   ▒░.. .     ..:::.:░░:::░░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▒.
░░░░▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▒▒░:....░▒:::▓▒...      ..░▒.  :▓::..::░:░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▒.
░░░░▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▒▒▒░:.::.:░░░▒:... ..   . :▒▒::░░::...:::▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓:
░░░░▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▒▒▒▒░:.::::::.  .. ..    ...::░::::..::░▒▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▓▒▓░
░░░░▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒░:..:::.      ..     .::::░:...::░░▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▒▓▓▒
░░░░▒▒▒▒▒▒▒▒▒▓▒▒▓▒▒▒▒▒▒▒▒░:..   . .:░░▒▒░      ......::░░░:░▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▒▓▓▒
░░░░▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▒▒▒▒▒▒░:... :▒▓▓▒▒▒▓░..     ...:░░▒▒░:::░▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▒▒▓
░░░░░▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▒▒▒▒▓▓▓▒░::.:▓▒▒▒▒▒▒▒░▒▒. ....░░▒▒▒▒▒░...:▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒
░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▒░░░▓▓▓▒▒▒▒▒░▒▓▓▒░:::▒▓▓▒▒▒▒▒░....░▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▒▒
░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▒▓▒░▒▓▓▓▓▒░░▒▓▓▓▓▓▒░░▒▓▒▒▒▒▒▒▒░::...░▒▒▒▓▒▒▓▓▓▓▓▓▓▓▓▒
░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▒▒▓▓▓▓▒▒▓▓▒▒▓▒░▒▓▒▒▒▒▒▒▒▒▒:::::.:▒▒▒▓▒▒▓▓▓▓▓▓▓▓▓▓
::░░░░░░▒▒▒▒▒▒▒░░▒▒▒▒▒▒▒▒▒▓▓▒▓▒▓▓▓▓▓▓▓▓▓▒▒▒▓▓▓▓▓▓▓▒▒▒▒▒▒▒::....:▒▒▒▒▓▒▓▓▓▓▓▓▓▓▓▓
::░░░░░░░▒▒▒▒░░░▒▒▒▒▒▒▒▒▒▒▓▓▒▓▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒░:::::..░▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓
::::░▒░░░░░░░░░░░▒▒▒▒▒▒▒▒▓▓▓▓▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒:::::::.░▒▒▓▒▒▓▓▓▓▓▓▓▓▓▓
::░░░░▒▒▒░::░░░░░▒▒▒▒▒▒▒▒▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒░::::.:::░▒▒▒▒▒▒▓▓▓▓▓▓▓▓▓
:::░░░░▒▒▒░░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓▒▒▒▒▒▒▒▒░::::::::.░▒▒▓▓▒▒▒▓▓▓▓▓▓▓▓
:::::░░░░▒▒░░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▓▒▒▒▒▒▒▒▒▒░░::::::::::▒▒▒▓▒▒▒▒▒▒▒▓▓▓▒▒
::::::░░░░▒░░░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░:::::::::::.:▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
```


### Braille Mode (`--braille`)

Uses Braille Unicode characters (U+2800-U+28FF) sorted by dot density. 
Provides 9 distinct brightness levels (0-8 dots) with monospace alignment.

**Note**: While Braille characters render correctly in terminals, some web browsers 
may show a visually jagged right edge due to font rendering. This is cosmetic only - 
the character spacing is correct and the image proportions are preserved.

```
cargo run -- --braille baimou.jpg

⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⢎⢎⢎⡒⡒⢎⢎⢎
⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⡒⢎⢎⢎⢎
⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⢎⢎⢎⡒⡒⢎⢎⢎⢎
⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢐⠂⢐⢐⢐⡒⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⡒⡒⢎⢎⢎⢎
⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⠂⢐⢐⠂⠂⢐⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⢎⢎⢎⢎⢎⡒⢐⢐⢐⢐⢎⡒⢎⢎⢎⡒⡒⡒⢎⢎⢎⢎⢎
⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⠂⢐⢐⢐⠂ ⠂⡒⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⠂     ⢐⢎⢎⢎⢎⡒⡒⡒⢎⢎⢎⢎⣲
⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⠂⠂⢐⡒⡒⢐⢐⠂⠂ ⢐⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⠂  ⠂⠂⠂⠂ ⠂⢎⢎⢎⡒⡒⡒⡒⡒⢎⢎⣲⢎
⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⠂⠂⡒⡒⡒⡒⢐⢐⠂⠂ ⠂⡒⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⠂  ⠂⠂⠂⠂⠂⠂ ⠂⢎⢎⡒⡒⡒⡒⡒⡒⢎⢎⣲⢎
⣲⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⠂⠂⡒⢎⢎⢎⢎⡒⢐⢐⠂  ⢐⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⠂  ⠂⠂⠂⠂⠂⠂⢐⢐ ⢐⢎⡒⡒⡒⡒⡒⢎⢎⢎⣲⣲⡒
⣛⣲⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⠂⠂⡒⡒⡒⣲⣛⣲⣲⡒⢐⠂⠂ ⠂⡒⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⠂  ⠂⠂⠂⢐⢐⢐⢐⡒⢐⢐ ⢐⡒⡒⡒⡒⡒⡒⡒⢎⢎⣲⣲⡒
⢎⣛⣲⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢐⠂⡒⡒⡒⢎⣲⣛⣲⢎⡒⢐⠂   ⢐⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢐   ⠂⠂⢐⢐⡒⡒⡒⡒⡒⢐⠂ ⢐⡒⡒⡒⡒⡒⡒⢎⢎⢎⣲⣲⡒
⢐⢎⣛⣲⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⢎⡒⠂⢐⡒⢎⢎⢎⢎⢎⢎⢎⡒⢐    ⠂⡒⢎⢎⣲⣲⣲⣲⣲⣲⣲⣲⣲⢎⣲⢎⡒⠂   ⠂⠂⢐⢐⢐⡒⢎⢎⢐⢐⢐⠂⠂⡒⡒⡒⡒⡒⡒⡒⢎⢎⣲⣛⣲⡒
⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⢎  ⠂⠂⡒⢎⢎⢎⡒⡒⡒⢐⠂    ⠂⢎⢎⣲⣲⣲⣲⣲⢎⣲⢎⢎⢎⢐⠂     ⠂⠂⢐⢐⡒⡒⡒⡒⡒⢐⢐⠂⡒⡒⡒⡒⡒⡒⡒⢎⢎⣲⣛⣛⢎⢐
⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣛⣲ ⠂⢐⢐⠂⠂⢎⢎⡒⢎⢎⢎⢎⢎⡒⡒⣲⣲⣛⣛⣲⣲⢎⣲⣲⢎⣛⣲⣲⢎⠂  ⠂⠂⠂⠂⠂⠂⠂⠂⢐⢐⢐⡒⢐⠂  ⡒⡒⡒⡒⡒⢎⢎⢎⣲⣛⣛⣛⢎⢐
⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣛⣛⣲⣛⣲  ⠂⢐⡒⡒⢎⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⣲⢎⢎⡒⡒⢎⡒⢎⢎⣲⣲⣲⢎⡒⢎⢎⡒⡒⢎⢐⠂⠂⠂⠂⠂⠂⠂⠂ ⠂⡒⡒⡒⡒⡒⢎⢎⣲⣲⣛⣛⣛⢎⢐
⣲⣲⣲⣲⣛⣛⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣲⠂   ⠂⢐⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⢎⡒⢎⡒⡒⢐⡒⢎⡒⡒⡒⣲⣲⣲⣲⣲⣲⣲⣲⣲⢎⢎⡒⠂ ⠂⢐⢐⠂ ⡒⢎⡒⡒⡒⢎⢎⣲⣲⣲⣛⣛⣛⡒⢐
⣲⣲⣲⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⢎   ⢐⢎⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⢎⢎⡒⢎⡒⢐⡒⡒⢎⡒⢎⡒⢎⣲⣲⣲⣲⣲⣲⣲⢎⣲⢎⡒⠂  ⠂⠂  ⢎⣲⢎⢎⢎⢎⣲⣲⣲⣛⣛⣛⣛⡒⢐
⢎⣲⣲⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⢐  ⡒⣛⣛⣛⣛⣛⣛⣲⣲⢎⣲⢎⡒⢎⢎⡒⡒⡒⢐⡒⡒⡒⡒⢎⡒⡒⢎⢎⣲⣲⣲⢎⢎⢎⡒⢎⡒⢐     ⡒⣲⣲⣲⣲⣲⣲⣲⣲⣲⣛⣛⣷⣛⡒⢐
⢎⢎⣲⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⡒ ⢐⣲⣲⣛⣛⣛⣲⣲⢎⡒⡒⡒⢐⢐⡒⢎⡒⢐⢐⠂⢐⢐⢐⡒⡒⢐⡒⡒⡒⡒⡒⡒⢎⢎⢎⡒⡒⡒⡒⢐   ⢐⣲⣲⣲⣲⣛⣛⣛⣲⣛⣛⣛⣛⣛⣛⡒⢐
⢎⢎⣲⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣲⢎⣲⣛⣛⣛⣛⣲⢎⡒⡒⡒⢎⡒⢐⠂⡒⡒⢐⠂⠂⠂⢐⠂⠂⢐⡒⢐⢐⡒⡒⡒⡒⡒⡒⢎⢎⢎⢎⡒⡒⡒⠂ ⠂⢎⣛⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⡒⢐
⢎⢎⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⢎⡒⡒⢎⣲⢎⡒⡒⡒⠂⢐⠂⢐⠂⠂⠂⠂⠂⠂⢐⢐⢐⢐⡒⢎⢎⡒⡒⡒⡒⢎⢎⢎⢎⡒⢎⡒⢐⢐⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⢎⢐
⢎⢎⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⢎⡒⢐⢐⡒⣛⢎  ⠂⣲⢎⢐⠂⠂⠂⠂⠂⠂⠂⠂⠂⢐⢐⡒⡒⠂⢐⣲⣲⡒⢐⡒⡒⢎⢎⢎⢎⢎⢎⢎⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⠂
⢎⢎⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⡒⢐⢐⢐⢐⣲⣲⡒⡒⡒⣛⣲⢐⠂⠂⠂⠂⠂⠂ ⠂⠂⠂⢎⣲⢐  ⢐⣛⡒⢐⠂⠂⢐⡒⢎⡒⡒⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⢐
⢎⢎⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⣲⡒⠂⢐⡒⢐⡒⢎⢎⣲⣲⡒⠂⠂⠂⠂⠂⠂  ⠂⠂ ⡒⣲⣲⡒⡒⣲⢎⢐⡒⢐⢐⢐⡒⡒⡒⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⡒
⢎⢎⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⢎⢐⠂⢐⡒⡒⢐⢐⢐⠂  ⠂⠂ ⠂⠂    ⠂⠂⢐⡒⡒⢎⡒⡒⡒⢐⢐⢐⡒⡒⢎⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲
⢎⢎⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⢎⢐⠂⢐⢐⡒⡒⢐⠂ ⠂⠂  ⢐⢐     ⠂⢐⡒⡒⡒⢎⡒⢐⢐⢐⡒⡒⢎⢎⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⢎⢎⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⣲⣲⣲⢎⡒⢐⠂⠂⠂⠂⠂ ⢐⡒⢎⣲⣛⣛⢎⠂    ⠂⠂⠂⢐⢐⢐⢐⡒⡒⢎⢎⢎⡒⢎⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⢎⢎⢎⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⣲⣲⣲⣛⣛⣛⣛⣲⢎⡒⢐⠂⠂⠂⡒⣲⣛⣷⣛⣛⣛⣛⢎⠂⠂ ⠂  ⠂⠂⢐⢐⡒⢎⢎⣲⣲⢎⢐⡒⡒⢎⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⢎⢎⢎⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣲⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⢎⡒⡒⢐⡒⣷⣛⣲⣛⣲⣛⣛⣲⢎⣛⣲⠂⠂⠂⢐⢐⢐⢎⣲⣲⣲⣛⣲⣲⢎⢐⢐⢐⡒⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⢎⢎⢎⢎⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣷⣲⢎⢎⢎⣛⣷⣛⣛⣲⣲⣛⣲⣲⣛⣷⣛⣛⢎⡒⡒⡒⣲⣛⣛⣛⣛⣲⣲⣲⢎⢐⢐⢐⢐⢎⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⢎⢎⢎⢎⢎⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣷⣛⣲⣲⣛⣷⣷⣷⣛⣲⣲⣛⣷⣷⣷⣷⣛⣲⢎⢎⣛⣛⣛⣛⣛⣛⣛⣛⣛⢎⢐⢐⢐⢐⢐⢎⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⡒⢎⢎⢎⢎⢎⣲⣲⣲⣲⣲⣛⣛⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣷⣛⣛⣛⣛⣷⣷⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⡒⡒⢐⢐⢐⢐⡒⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⡒⡒⢎⢎⢎⢎⣲⣲⣲⣲⣲⣛⣲⣲⣲⢎⣲⣲⣛⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣷⣷⣷⣛⣛⣛⣛⣛⣛⣛⣷⣷⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⡒⢐⢐⢐⢐⢐⡒⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⡒⡒⢎⢎⢎⢎⢎⢎⣲⣲⣲⣲⣲⢎⢎⢎⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣷⣷⣷⣷⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⢎⡒⡒⡒⢐⢐⢐⢐⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⡒⡒⡒⡒⢎⣲⣲⣲⣲⢎⢎⣲⣲⢎⢎⢎⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣷⣷⣛⣷⣷⣷⣷⣷⣷⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⡒⢐⢐⢐⡒⡒⢐⢐⢎⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⡒⡒⡒⢎⢎⣲⣲⣲⣲⢎⡒⡒⢎⢎⢎⢎⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣷⣷⣷⣷⣷⣷⣛⣛⣛⣛⣛⣛⣛⣲⣲⢎⡒⡒⡒⢐⢐⢐⢐⢐⢎⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⡒⡒⡒⢎⢎⢎⣲⣲⣲⣲⢎⡒⢎⢎⢎⢎⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⢎⡒⢐⢐⢐⢐⢐⢐⢐⢐⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⢐⢐⡒⡒⡒⢎⢎⣲⣲⣲⣲⢎⢎⢎⢎⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣛⣲⢎⢎⡒⡒⢐⢐⢐⢐⢐⢐⢐⢐⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
⢐⢐⡒⡒⡒⡒⢎⢎⢎⣲⣲⣲⢎⢎⢎⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⣲⣲⣲⣲⣲⣲⣲⣛⣛⣛⣛⣛⣛⣛⣛⣛⣲⣲⣲⣲⡒⡒⡒⡒⡒⢐⢐⢐⢐⢐⢐⢐⡒⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛⣛
```

### How It Works

1. **Symbol Selection**: Choose character set (ASCII, Braille, or custom)
2. **Density Calculation**: For ASCII/Braille, bitmap font determines "brightness" 
3. **Brightness Mapping**: Each character maps to a grayscale intensity (0-255)
4. **Image Processing**: 
   - Resize image to target width (accounting for character aspect ratio)
   - Convert each pixel to grayscale
   - Map to nearest character by brightness

For braille the brightness is easy to calulate - known number of dots in the symbol (0-9).

For the ascii character set, brightness is calculated by looking up the individual characters in a small font ([font8x8-rs](https://github.com/jesper-olsen/font8x8-rs)) and counting bits. This is only approximate because the terminal or the browser used for rendering the result will likely be using a different font.


