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

Scale image to 80 pixels wide, set character aspect ratio to 2 and map each pixel to one of the 7 symbols in " _a2WM+" 
```
cargo run -- -u " _a2WM+" -w 80 -r 2 baimou.jpg

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++++++++++++++++a2222a+++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++++++++++++++++a222222+++++++++++++++++++++++++++++++++++++++a22a++++++a++++++
+++++++++++++++++a22a2222a++++++++++++++++++++++++++++++++++a22WWWWa+++++++++++_
++++++++++++++++222aa2222W2+++++++++++++++++++++++++++++++a2W22222W2++++++++++_+
++++++++++++++++22a++aa222W2a+++++++++++++++++++++++++++a2WW222222W2++++++++++_+
_+++++++++++++++22a+++++a222W2++++++++++++++++++++++++a2WW22222222W2+aa++++++__+
 _++++++++++++++22aa+____+a22W2a+++++++++++++++++++++2WW222222aaa222+aa++++++__a
+ _+++++++++++++22aaa+___++222WW2++++++++++++++++++a2W2222aaa+aaa22a+aa++++++__a
a+ _+++++++++++++22a+++++++a22W2W2+++_________+_+a2WWW2222aa++a2222a++a+++++_ _a
________________+2222a+++aaa22WWWW2++_+___+_+++a2WWWW2222aaaaaaa22a++++++++__ +a
_________________222222+++++++++a______+_++___+222222222222aaa2222aaaa++++__  +a
_________________2222aa+__    _ _____+++a+a++___+++++++222222222W2+a+++++__   +2
_________________2W2222__     _____+++aa2++aaa+__+_____++a222222Wa++++++____  +2
________________ +WW22+__      ___+++++aaa+a+a+_______+_+a22222W2+_++++____   a2
+________________ 2WWa__   __+++++++aaa2a+aa+++++__+++++++2WW2WWa__________   a2
+++____________   +22___  __+a++a2a+a222aa2aaaaaaa+++++++aa2WWWa____________  a2
++______________  ++___ __+aa++a22aa2222222aa22aaaaa+++++aaa2W2+________  _   +2
++_____________   _  ___+aa+_+a+a22222222222222a++aaaa++++a+a2a________       +2
++___________      _ _+a22+ +W22_+22222222222a+a2a__aaa++++++++______ _       _2
++___________      ___+2222__aaa _22222222222+_22Wa aa22aa+++______    _       2
++___________      ____a2aa2a++__+22222222222a__aa_+aa222a++______             +
++__________       ____+a2aaaaa22222222222WW222a++aaaa22aa+_______   _         _
+++_________       _____+a22aaa22W222W22WW2WW2aa+a++222a+++______              _
+++_________      _______++22222222++___+2WW222222222aa+++a+_____               
+++________    ________  __+a2222a_   _  +2222222222a++__+aaa+___               
++++_______   __________   _+aa2+  ______+__222222+______+222a_____             
+++++______  __________     _+++   ______   _++a+_   ____+2222+_ __             
+++++___________________     _+_     __      _++    _____+aa222+__              
++++++___________________            _   _  ___    _  ___aaaaa2a_               
++++++_________+_________                _         ______aa22a2a_               
aa++++++_____+++_________                         _  ___+aaaaa22_               
aa+a+____++__+++________                          ______aaaaaaa2+               
aa+++____+++++++_______       __                  _____+aaaa2aaa+               
aa+++_____++++++______  _   _______             ______+aaaaaaaa2+               
aaaa+++____++++__+_____ _  ____________       ______+++aaaaaaaaa_ _             
aaaaa++++___+++____________________________________+aaaaaaaaaa2a_               
```

Invert the image - map to ascii range (default). Note that some symbols have same 'brightness' - only one representative symbol per brightness is used. 
```
cargo run -- -i -w 80 baimou.jpg

zzxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxzzzzuuuzzzzzxxxxxxxxxxxxvvvvvvvxx}}xzu
zzxxxxxxxxxxxxxxxxxxxvxxxxxxxxxxxxxxxxxxxxzzzzuuuzzzzxxxxvxxxxxxxxvvvvvvxxv}vxzu
zzxxxzxxxxxxxxxxxxvvvzzxxxxxxxxxxxxxxxxxxxxzzzuuzzzzzxxxxvxxxxxxxvvvvvvxxv}}vzuT
uzzzxxxxxxxxxxvxx\`__~?uxxxxxxxxxxxxxxxxxxxxzzzzzzzxxxxxvxxxxxzzzzxvvvvxx}}vvzuT
uuzzxxxxxxxxxxvvxi_~~_.~xzxvvvvxxxxxxxxxxxxxxzzzzzxxxxxxvvvvzz}\~_\vvvxx}?}vxzuT
uuzzzxxxxxxxxxxxvi__~~_ .izvxxxvxxxxxxxxxxxxxxzzzxxxxxvvvvxxi`     \xvxv}}}vxzT3
uuuzzzxxxxxxxxxx`.~>\~_`. ~xzvxxvvvxxxxxxxxxxxxxxxxvxxvvxx>`  .``` _zxv}}}}vxu3T
uzuuzzxxxxxxxxxx.`iv}i|~_` .?xvvvvxxxxxxxxxxxxxxxxxxvvxx>`  ..```_ _zv}}}}vvzTyz
guuuzzzxxxxxxxxx._?vuuz}\~`  _xxvxxxxxxxxxxxxxxxxxvvzz?`  ..``__~_ _xi?}}}vxu3yv
&kTTzuzxxxxxxxxz`_?>}kQkTv|_. `?zxxxxxxvxxxxxxxxxxxz}_  .``__~|\\~ _}i?}}}vxuyg?
x&kuzzzzzzxxxxxu~`>??ugwku}~`.  _vzxxzuzuzzzzxxxxxx\   .`~|>?}i>|_ |}??}}}vzTkgi
|x&kuuzzzzxxxxxz}._?xzvxuux>~.   `}uT33ygg333TT3u\.   .`_~\?xx\~~`.????}}vxTg&y>
y3gkgggggygggyyyz  ``>TTviii~.    _vz3T3gguyuTx|.     `_~|>>>i>\~.>??}}}}xugQ&T\
gkggkkkkwwwkwkkQg.._~`_uz}zuTzz}?TgwQkgTgTxwkyu_ ...`.````~|\>_`  ????}vxugw&Zz~
yyggkkkkkkkkwwkwy..`~?>xQQ&ZZ&Q&wQkkkTx}?xixuggyz}vz}}v~`_``__`. `??}}vxTgw&&Zx~
gggkwwkkkkkkkQwww.  ..~3QZZZZZQwwgyx}x?\~}xii?T3gT333wguu>`.`_~` >x}}vxuykwQ&Zv_
3ygkkkkkkwwwwwww&x  .~xkQQZZZZ&wwwuu?x?\i?xiviz3gyygg3Tyui` ..`  TyTuzuTykQQZZ?~
u3ygggggkwwQQQQQQ&~  iwQQ&&QwTuTz}vui\>~iv>\z}}zT33TuTu}v}~     igkkkggygwQ&BZi~
uuT3gggkkwwwwwQQ&&?.~3kw&&k3x?}?\_?z>~~`~\_>?|>ii?}}vvv?}i>~   |kkkkwwwwwQQQZZi_
uT333gkkkwwwQQQQZ&TugQw&w3v>\}xi_`>i~_``~`_\i~~>?i>i}vxzx??i` .TwwwQQwQQ&&Q&&Z}_
uTT33gkkkwwwQQQ&Z&wZZQw3v?ivgx\vi_~__``````~~_~?xz?i??vzuv?x>~\gwwQQQQQ&&Q&ZZZz_
uTT3ygkkkwwwQ&&&&&QQ&gx\_~}Zx ..yz~..`.....`~|}>_\33i\>vzzvvzvuQQQQQQ&Q&Z&&Z&Zg_
uTT3ygkkwwwQQQ&&ZZ&wg3}~_~~3giiiRw~``.... .__xk~. |Z?\``|?v}vyQQQQQQ&&&QQ&Z&&Z&~
uTT3ygkkwwQQQ&&ZZZZQky3>_\\~?uu3y?`__.``. .`.\ky?\3v|>~_~i}}yQQQQQQ&&&&&&&&ZZZZ}
uTT3ygkwwwQQQ&ZZZZZQQkyu|_|i\\|~`. _`..`    `__\}v>i?\~~>?vkQwQQQQ&&&Q&ZZZ&&Z&Z3
uTT3ygkkwQQQQ&&&ZZ&wwwggu\`_|\\~. ..  ~~     .\???v}~_~>}vTkwQQQQQ&&&&&ZZZZZZZZQ
uuT3yygkwQQQ&ZZ&Z&wwwwkkgT}~`...` _?T3QQv.    .___~_~>?vuu?xywQQQQ&&&&&ZZZZZ&ZZZ
uuT3yyggwQQ&&Z&QwkywkkwQ&wgzi_``.iyZR&Q&Zv__ ....`_~?xTgku\>?TwQQQ&&&&&ZZZZZZ&ZZ
zuTT3yygkwQ&Z&QkggggkkkQZZZQxii~}BZkQwQQgTQy`.__~~vTgkQkgx~_~iyQQQQ&&&ZZZZZZZZ&Z
xzuuT3ygkkw&&wkkkkwwwwwQZZZRyxzxZWRQwwQgy&RZQz}>}kZZ&wwygx~~__vwQQQ&ZZZZZZZZZZZ&
xzuuuT3gkkwQQQwkkwwwwwwQ&ZZ&ZwT3&WWWZyy&BBRRZyvuQZ&Qwwwwwx||~_~uwQ&Z&ZZZZZZZZZZZ
}vzuuu3yggwQwkgggkwwwwwww&ZZZZZQ&ZRBZQ&ZZQ&Zw3kZ&&&Q&QQQg>>\||~?k&&Z&ZZZZZZZZZZZ
?}xxzT33yykwkg3T3kwwwwwQQ&ZZ&Z&ZZZZRRZZZ&Q&ZZRZZ&Z&wwQQQg?|~~~~>g&&ZZZZZZZZZZZZZ
i?vvxuuT3ygkyTzuygkgkwQQQ&ZZ&Z&Q&&ZZZZZZZRZRBZZ&Z&Q&&Qwwu>i>||~~3Z&&&ZZZZZZZZZZZ
ii}?zy333zu33zxzTgkkkwwQ&ZZZ&&&&&&ZZZZZZRRZRRZZ&&QQQQwwg?\||>>\~T&&Z&ZZZZZZZZZZZ
i?vxTyggkT}}xxxu3gkkwwQQ&Z&&&QQQQ&&ZZZZZRRRRZZZZ&&Qwwkgx>\\\~|\|u&&&&ZZZZZZZZZZZ
>i}xzTyggkzvvxuT33gkwQ&&Q&&&QQQQQQQ&&&ZZZZZZZZ&&QwwwQkzi\||\\||~T&&ZZ&ZZZZZZZZZZ
|\>?}zuT3ggzxxu33T3gwQQ&Q&QQQQwQwwwQwQQ&&ZZ&&&Qwwwwkuv}\\|||\\~\g&QZ&&&&ZZZZZZZZ
|\\>i}xzu3g3xxzT33ygwQwQQQwwwkkkgygggkwwQQQQQQwkwkT}>>i\|\\||\~?Q&&&&&&&&ZZZZZZZ
```


