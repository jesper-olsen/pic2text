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
  -i, --i        invert
  -r, --r <R>    aspect ratio (character height to width) [default: 2]
  -w, --w <W>    max picture width in characters [default: 80]
  -u, --u <U>    each char in string [default: ]
  -h, --help     Print help
  -V, --version  Print version
```

Scale image to 80 pixels wide, set character aspect ratio to 2, map each pixel to one of the 7 symbols in " _a2WM+", and invert: 
```
cargo run -- -u " _a2WM+" -w 80 -r 2 -i baimou.jpg

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++++++++++++++++a2222a+++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++++++++++++++++a222222+++++++++++++++++++++++++++++++++++++++a22a++++++a++++++
+++++++++++++++++a2222222a++++++++++++++++++++++++++++++++++a22WWWWa+++++++++++_
++++++++++++++++222aa2222W2+++++++++++++++++++++++++++++++a2W22222W2++++++++++_+
++++++++++++++++22a++aa222W2a+++++++++++++++++++++++++++a2WW222222W2++++++++++_+
_+++++++++++++++22a+++++a222W2++++++++++++++++++++++++a2WW22222222W2+aa++++++__+
 _++++++++++++++22aa+____+a22W2a+++++++++++++++++++++2WW222222aaa222+aa++++++__a
+ _+++++++++++++22aaa+___++222WW2++++++++++++++++++a2W2222aaa+aaa22a+aa++++++__a
a+ _+++++++++++++22a+++++++a22WWW2+++________++_+a2WWW2222aa++a2222aa+a+++++_ _a
________________+2222a+++aaa22WWWW2++_+___+_+++a2WWWW2222aaaaaaa22aaa++++++__ +a
_________________222222+++++++++a______+_++___+2W2222222222aaa2222aaaa++++__  +a
_________________2222aa+__    _ _____+++a+a++___+++++++222222222W2+a+++++__   +2
_________________2W2222__     _____+++aa2++aaa+__+_____++a222222Wa++++++____  +2
________________ +WW22+___     ___++a+aaaa+a+a+_______+_+a22222W2+_+++++___   a2
+________________ 2WWa___  __+++++++aaa2a+aa+++++__+++++++2WW2WWa__________   a2
+++_____________  a22___  __+a++a2a+a2222a2aaaaaaa+++++++aa2WWWa____________  a2
++______________  ++___ __+aa++a22aa2222222aa22aaaaa+++++aaa2W2+________  _   +2
++_____________   _  ___+aa+_+a+a22222222222222a++aaaa++++a+a2a________       +2
++___________      _ _+a22+ +W22_+22222222222a+a2a__aaa++++++++______ _       _2
++____________     ___+2222__aaa _22222222222+_22Wa aa22aa+++_______   _       2
++___________      ____a2aa2a++__a22222222222a__aa_+aa222a++_______            +
+++_________       ____+a2aaaaa22222222222WW222a++aaaa22aa+_______   _         _
+++__________      _____+a22aaa22W222W22WW2WW2aaaa++222a+++_______             _
+++_________      _______++22222222++___+2WW222222222aa+++a+______              
+++________    ________  __+a2222a_   _  +2222222222a++__+aaa+____              
++++_______   __________   _+aa2+  ______+__222222++_____+222a_____             
+++++______  ___________    _+++   ______   _++a+_   ____+2222+____             
+++++___________________     _+_     __      _++   ______+aa222+__              
++++++___________________      _     _   _  ___    _ ____aaaaa2a_               
a+++++_________+_________                _         ______aa2222a_               
aa++++++_____+++_________                         _  ___+aaaaa22_               
aa+a+____++__+++________                         _______aaaaaaa2+               
aa+++____+++++++_______       __                  _____+aaaa2aaa+               
aa++++____++++++______  _   _______             ______+aaaaaaaa2+               
aaaa++++___++++__+_____ _  ____________       ______+++aaaaaaa2a_ _             
aaaaa++++___+++___________________________________++aaaaaaaaaa2a_               
```

Map to ascii range (default). Note that some symbols have the same 'brightness' - only one representative symbol per brightness is used. 
```
cargo run -- -w 80 baimou.jpg

77xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx7777fff7777xxxxsxxxxxxxsssssstsxxttx7f
77xxxxxxxxxxxxxxxsxxssxxxxxxxxxxxxxxxxxxxxx777fff7777xxxsssxxxxxxsssssssxxstsx7f
77xxxxxxxxxxxxxxxxsss77xxxxxxxxxxxxxxxxxxxxx777f7777xxxxsssxxxxxsssssssxxs!tsxfC
7777xxxxxxxxxxsxx/,::;!fxsxxxxxxxxxxxxxxxxxxx77777xxxxxsssxxsx7777xssssxxttss7fC
f777xxxxxxxxxxssxi:;:,.;x7xssssxxxxxxxxxxxxxx7777xxxxxssssssx7t/;:/sssxxt!tsx7fC
ff77xxxxxxxxxxxxsi::;;: .i7sxsxsxxxxxxxxxxxxxx777xxxxxssssxxi,     /xsxs!ttsx7CC
fff777xxxxxxxsxx,.;+/;:,. ;x7ssssssxxxxxxxxxxxxxxxssxsssxx+,  .,,, ,7xs!ttttxf3C
f7f77xxxxxxxxxxs.,isti=;,. .!xsssssxxxxxxxxxxxxxxxxsssxx+,  ..,,,: :7st!ttts7Cq7
dfff77xxxxxxxxxx.,!sff7t/;,  :xxsxxxxxxxxxxxxxxxxxssx7!,  ..,,::;: :xi!tttsxf3qs
&XCC7f7xxxxxxxx7,:!+tXAXCs=:. ,!7xxxxxsssxxxxxxxxxs7t:  .,,::;=//; :ti!tttsxfqd!
x&Xf777777xxxxxf;,+!!fdhXf!;,   :s7xx7f777777xxxxxs=   .,;=+!ti/=: ;!!!ttts7CXdi
=x&dff777xxxxxx7t.:!x7sxffs+;.   ,!fC33qdd333CC3f/.   .,:;/!xx/;;..i!!!ttsxCdAq/
q3dXddddqqdddqqq7  ,,+fCsiii;.    :s73CCddfqffx=.     ,:;;+++i+/;.+!!ttttxfdh&C/
ddddXXXXXhhXhXXAd..:;,:fxt7ff77t!CdhAXdCdCxhXqf:  ..,.,,,,:=/+:,  !!!!tsxfdh&D7;
qqddXXXXXdXXhhXhq..,;!+xAA&DD&A&hhXXXCst!xixfddq7ts7tts;,:,,::,. ,!!tttxCqh&&Dx;
dddXhhXXXXXXXAhhh.  ..;3ADDDDDAhhd3xtx!/;txii!C3dC333Xdff+,.,:;, +xttsxfqXhA&Ds:
3qddXXXXXhhhhhhh&x  .;xXAADDDD&hhhff!x!/i!xisi73dqqdd3Cqfi, ...  CqCf7fCqXhADD!;
f3qdddddXXhAAAAAA&;  iXAA&&AhCfC7tsf+/+;it+/7t!7C33Cffftst;     idXXXddqdhA&@Di;
ffC3qddXXXhhhhAA&&! ;3Xh&AX3s!t!/:!7/;;,;/:+!=+ii!ttsss!ti+;   =XXXXhhhhhAAADDi:
fCC33ddXXXhhhAAAD&CfdAh&h3s//tsi:,+i;:,,:,:/i;;+!i/i!ss7x!!i. .ChhhAAhAA&AA&&Dt:
fCC33qXXXhhhhhA&D&hDDAh3s!isdx/si:;::,,,,,,;;:;!x7!i!!s7fs!x+;/dhhhAAAA&&A&DDD7:
fCC3qdXXXhhhA&&&&AAA&dx/:;tDx ..q7;..,.....,;=t+:/33i/+t77ss7sfAAAAAA&A&DA&D&Dd:
fCC3qdXXhhhAAA&&DD&hd3t;:;;3diiiRh;,,.... .,:xX;. =D!/,,=!stsqAhAAAAA&&AA&D&&D&;
fCC3qdXXhhAAA&&DDDDAXq3+:=/:!ff3q!,::.,,  .,./Xq!/3s=+;:;+t!qAAAAAA&&A&&&&&D&DDt
fCC3qqXhhhAAA&DDDDDAhXqf=,=i/==;,. :,..,    ,::/ts+i!/;;+!sdhhhAAAA&&A&&DD&&D&D3
fCC3qqdXhAAAA&&&DD&hhhdqf/,:=//;. ..  ;;     ./i!!st;:;+tsCXhhAAAAA&&&&&DDDDDDDh
ffC3qqdXhAAA&D&&&&hhXXXXdCt;,..., :!C3AAs.    .:::;:;+!sff!xqhAAAAA&&&&&DDDD&DDD
ffC3qqddXhAA&D&AhXqXXXhA&hd7i:,,.+qDR&A&Ds:: ....,:;!xCdXf/+iChAAAAA&&&DDDDDD&DD
7fCC3qqdXhh&&&hXddddXXXADDDAxi+;t@DXAhAAdCAq,.::;;tCdXAXdx;:;iqAAAAA&&&DDDDDDD&D
x7ffC3qdXXh&&hXXdXXhhhhADDDRqx7xDNRhhhAdq&RDA7t+tXDD&hhqdx;;::shAAA&D&DDDDDDDDD&
s77ffC3dXXhAAhhXXhhhhhhA&DD&DhC3&NNNDqq&@@RRDqsfAD&Ahhhhhx==;:;fhA&D&&DDDDDDDDDD
ts7fff3qddhAhXdddXhhhhhhh&DDDD&A&DR@DA&DDAADh3XD&&&AAAAAd++/==;!X&&D&&DDDDDDDDDD
!txx7f33qqXhXd3f3XhXXhhhA&DD&D&DDDDRRDDD&A&DDRDD&&&hhAAAd!=;;;;/d&&&D&DDDDDDDDDD
i!tsx7fC3qdXqf7fqdddXhhAA&DD&D&A&&DDDDDDDDDR@DD&D&A&&Ahhf+i+==;;3&&&&&&DDDDDDDDD
ii!!7q3C37f3C7x7CdXXXhhA&DDD&&&&&&DDDDDDRDDRRDD&&AAAAhhd!/==++/;f&&D&&DDDDDDDDDD
i!sxCqdddCttxxx73dXXhhAA&D&&&AAAA&&DDDDDRRRRDDDD&&AhhXdx+///;=/=f&&&&&&DDDDDDDDD
+itx7C3ddX7ssxfC33dXhA&&A&&&AAAhAAAA&&DDDDDDDD&&AhhhAX7i/==//==;C&&DD&&&DDDDDDDD
=/+!t7fC3dd7xxfC3C3dhAAAAAAAAhhAhhhhhAA&&&&&&&AhhhhXfst//====/;/d&AD&&&&&DDDDDDD
;=/+itx7f3d3xx7C33qdhhhAAAhhhXXXdqdddXhhAAAAAAhXhXCt+++/=//===;!A&&&&&&&&&&DDDD&
```


