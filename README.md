## pic2text

Convert pictures to ascii art.

The symbols to use can be specified on the command line. Symbols are ranked by estimating the 'brightness' of each symbol - this is done by looking up the symbol in a small bitmap font and counting bits (see [src/font8x8.rs](src/font8x8.rs)). 

![cat](baimou.jpg)

```
cargo run -- -h

Usage: pic2text [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  image

Options:
  -i, --invert             invert grayscale
  -a, --aratio <ARATIO>    aspect ratio (character height to width) [default: 2]
  -w, --width <WIDTH>      max picture width in characters [default: 80]
  -s, --symbols <SYMBOLS>  each char in string [default: ]
  -h, --help               Print help
  -V, --version            Print version
```

Map to ascii range (default). Note that some symbols have the same 'brightness' - only one representative symbol per brightness is used. 
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


Scale image to 80 pixels wide, set character aspect ratio to 2, map each pixel to one of the 9 Braille symbols.
When symbols are explicitly specified, the program assumes that they are already sorted in intensity order and that
intensity varies linearly. 

```
cargo run -- --symbols "⠀⠁⠃⠇⠏⠟⠿⣿" --width 80 --aratio 2  baimou.jpg


⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠏⠏⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏
⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠏⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏
⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏
⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠃⠁⠁⠁⠃⠇⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏
⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠁⠁⠁⠁ ⠃⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠃⠃⠁⠃⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏
⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠃⠁⠁⠃⠃⠁ ⠁⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠃⠁     ⠃⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏
⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠁⠁⠃⠃⠃⠃⠁⠁⠁ ⠃⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠃⠁  ⠁⠁⠁⠁ ⠁⠏⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠏
⠏⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠁⠁⠃⠇⠇⠇⠃⠃⠁⠁ ⠁⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠃⠁   ⠁⠁⠁⠁⠁ ⠁⠏⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠇
⠏⠏⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠁⠁⠇⠇⠏⠏⠏⠇⠃⠃⠁  ⠁⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠁    ⠁⠁⠁⠁⠃⠁ ⠁⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠏⠇
⠟⠏⠏⠏⠇⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠁⠁⠇⠃⠇⠟⠟⠟⠏⠇⠃⠁⠁ ⠁⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠁  ⠁⠁⠁⠁⠁⠃⠃⠃⠃⠃ ⠁⠇⠃⠇⠇⠇⠇⠇⠇⠏⠏⠏⠇
⠇⠟⠟⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠏⠃⠁⠃⠇⠇⠏⠏⠟⠟⠏⠇⠃⠁   ⠁⠇⠇⠇⠇⠇⠏⠇⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠃   ⠁⠁⠃⠃⠃⠇⠇⠃⠃⠃⠁ ⠃⠇⠇⠇⠇⠇⠇⠇⠇⠏⠟⠏⠃
⠃⠇⠟⠏⠏⠏⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠁⠁⠇⠇⠇⠇⠇⠏⠏⠇⠃⠁    ⠁⠇⠏⠏⠏⠏⠏⠏⠏⠏⠏⠏⠏⠏⠏⠏⠃    ⠁⠁⠁⠃⠃⠇⠇⠇⠃⠃⠃⠁⠁⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠟⠏⠃
⠏⠏⠏⠟⠏⠟⠏⠏⠏⠏⠏⠏⠏⠏⠏⠏⠇  ⠁⠁⠃⠏⠏⠇⠇⠇⠇⠁     ⠁⠇⠏⠏⠏⠏⠏⠏⠏⠏⠏⠏⠇⠃⠁     ⠁⠁⠃⠃⠃⠃⠃⠃⠃⠃⠃ ⠃⠇⠇⠇⠇⠇⠇⠇⠏⠏⠟⠟⠏⠃
⠏⠟⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏ ⠁⠁⠃⠁⠁⠏⠇⠇⠇⠏⠏⠇⠇⠇⠇⠏⠟⠟⠟⠟⠏⠏⠏⠏⠇⠟⠟⠏⠏⠁    ⠁⠁⠁⠁⠁⠁⠁⠃⠃⠃⠁⠁  ⠇⠇⠇⠇⠇⠇⠇⠏⠏⠟⠟⠟⠇⠃
⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏  ⠁⠃⠇⠃⠇⠟⠟⠟⠿⠿⠟⠟⠟⠟⠟⠟⠟⠟⠏⠇⠇⠇⠇⠇⠇⠏⠟⠏⠏⠏⠇⠇⠇⠇⠇⠇⠃⠁⠁⠁⠁⠁⠁⠁⠁ ⠁⠇⠇⠇⠇⠇⠇⠏⠏⠟⠟⠟⠟⠇⠃
⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟    ⠁⠃⠏⠟⠟⠿⠿⠿⠟⠟⠟⠟⠟⠏⠇⠇⠇⠇⠃⠃⠇⠇⠇⠇⠇⠏⠏⠟⠏⠏⠏⠏⠟⠏⠏⠏⠃⠁ ⠁⠁⠃⠁ ⠃⠇⠇⠇⠇⠇⠏⠏⠟⠟⠟⠟⠟⠇⠁
⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠇   ⠃⠇⠟⠟⠟⠟⠿⠿⠟⠟⠟⠟⠟⠏⠏⠇⠇⠇⠃⠇⠇⠇⠇⠇⠇⠏⠏⠏⠏⠏⠟⠟⠏⠏⠏⠏⠃⠁   ⠁  ⠏⠏⠏⠏⠇⠏⠏⠏⠟⠟⠟⠟⠟⠇⠃
⠏⠏⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠃  ⠃⠟⠟⠟⠟⠟⠟⠟⠏⠏⠏⠏⠇⠇⠏⠃⠃⠃⠃⠇⠇⠃⠃⠏⠇⠇⠏⠏⠏⠏⠏⠏⠏⠏⠇⠇⠇⠃     ⠇⠏⠟⠟⠟⠏⠏⠏⠏⠟⠟⠟⠿⠟⠇⠃
⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠇ ⠃⠏⠟⠟⠟⠟⠟⠏⠇⠇⠇⠇⠃⠁⠇⠏⠃⠃⠃⠁⠃⠃⠁⠃⠇⠃⠃⠇⠇⠇⠇⠇⠇⠇⠇⠇⠇⠃⠃⠃   ⠃⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠃⠁
⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠏⠏⠟⠟⠟⠟⠏⠇⠃⠃⠇⠇⠇⠁⠁⠃⠇⠃⠁⠁⠁⠃⠁⠁⠃⠃⠃⠃⠃⠇⠇⠃⠇⠇⠇⠇⠏⠇⠇⠇⠃⠁  ⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠇⠁
⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠇⠇⠇⠇⠏⠇⠃⠇⠇⠁⠃⠁⠁⠁⠁⠁⠁⠁⠁⠃⠃⠁⠃⠇⠇⠇⠇⠇⠇⠇⠇⠏⠏⠇⠇⠇⠃⠃⠃⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠇⠁
⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠇⠃⠁⠃⠇⠿⠇   ⠏⠏⠃⠁⠁⠁ ⠁ ⠁ ⠁⠃⠃⠇⠃⠁⠃⠏⠏⠇⠃⠃⠇⠇⠇⠇⠇⠏⠇⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠁
⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠇⠃⠁⠃⠃⠏⠏⠇⠇⠇⠿⠟⠃⠁⠁⠁⠁⠁   ⠁⠁⠇⠟⠃  ⠃⠟⠇⠃⠁⠁⠃⠇⠇⠇⠇⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠃
⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠏⠃⠁⠃⠃⠃⠇⠏⠏⠏⠏⠇⠁⠁⠁⠁⠁⠁   ⠁ ⠃⠟⠏⠇⠃⠏⠇⠃⠃⠃⠁⠃⠃⠇⠇⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠇
⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠟⠟⠟⠟⠟⠏⠏⠃⠁⠃⠇⠃⠃⠃⠃⠁  ⠁⠁ ⠁⠁    ⠁⠁⠁⠃⠇⠇⠃⠇⠇⠃⠃⠃⠃⠇⠇⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏
⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠿⠟⠟⠟⠟⠏⠏⠏⠃⠁⠁⠃⠃⠃⠃⠁ ⠁⠁  ⠃⠃     ⠁⠃⠇⠇⠇⠇⠇⠃⠁⠃⠃⠇⠇⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠏⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠏⠇⠃⠁⠁⠁⠁⠁ ⠁⠇⠏⠏⠟⠟⠇⠁     ⠁⠁⠁⠃⠁⠃⠃⠇⠇⠏⠏⠇⠇⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠏⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠟⠟⠟⠟⠟⠟⠟⠏⠇⠇⠁⠁⠁⠁⠃⠏⠿⠿⠟⠟⠟⠟⠇⠁⠁ ⠁   ⠁⠁⠃⠇⠇⠏⠏⠟⠏⠃⠃⠇⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠏⠏⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠏⠏⠏⠟⠟⠟⠟⠟⠟⠿⠿⠟⠇⠇⠃⠃⠇⠿⠟⠏⠟⠟⠟⠟⠏⠏⠟⠏⠁ ⠁⠁⠃⠃⠇⠏⠟⠟⠟⠟⠏⠇⠃⠁⠃⠇⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠿⠟⠟⠟
⠇⠏⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠿⠏⠇⠏⠇⠿⠿⠿⠟⠟⠟⠟⠏⠏⠟⠿⠿⠟⠇⠇⠃⠇⠟⠟⠟⠟⠟⠟⠏⠏⠇⠃⠃⠁⠁⠇⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠇⠇⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠟⠏⠏⠟⠿⠿⠿⠟⠏⠏⠟⠿⠿⠿⠿⠿⠏⠇⠏⠟⠿⠟⠟⠟⠟⠟⠟⠟⠇⠃⠃⠃⠁⠃⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠟
⠇⠇⠏⠏⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠿⠟⠟⠟⠿⠿⠿⠿⠟⠟⠿⠟⠟⠟⠟⠟⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠃⠃⠃⠃⠃⠃⠇⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠇⠇⠇⠇⠏⠏⠏⠏⠏⠏⠟⠟⠟⠟⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠟⠿⠿⠿⠿⠟⠟⠟⠟⠟⠿⠿⠿⠿⠿⠟⠟⠟⠟⠟⠟⠟⠟⠏⠇⠃⠁⠃⠃⠃⠃⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠇⠇⠇⠇⠇⠏⠏⠏⠏⠏⠏⠟⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠿⠿⠿⠿⠿⠿⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠃⠇⠃⠃⠃⠃⠃⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠇⠇⠇⠇⠏⠏⠏⠏⠏⠇⠏⠏⠏⠏⠇⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠟⠟⠟⠟⠟⠟⠟⠟⠏⠇⠃⠃⠃⠃⠃⠃⠃⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠃⠇⠇⠇⠏⠏⠏⠏⠏⠏⠇⠇⠇⠇⠇⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠟⠟⠟⠟⠟⠟⠟⠏⠇⠃⠃⠃⠃⠃⠃⠃⠃⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠃⠃⠇⠇⠇⠏⠏⠏⠏⠏⠏⠇⠇⠇⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠿⠿⠿⠿⠿⠿⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠃⠃⠃⠃⠃⠃⠃⠃⠃⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠃⠃⠃⠇⠇⠇⠏⠏⠏⠏⠏⠇⠇⠇⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠇⠇⠃⠃⠃⠃⠃⠃⠃⠃⠃⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
⠃⠃⠃⠃⠇⠇⠇⠇⠏⠏⠏⠏⠇⠇⠏⠏⠏⠏⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠏⠏⠟⠏⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠏⠇⠃⠃⠃⠃⠃⠃⠃⠃⠃⠃⠃⠇⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟⠟
```

