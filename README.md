## pic2text

Convert pictures to ascii art.

The symbols to use can be specified on the command line. Symbols are ranked by estimating the 'brightness' of each symbol - this is done by looking up the symbol in a small bitmap font and counting how many bits are set (see [src/font8x8.rs](src/font8x8.rs)). 

![cat](baimou.jpg)

```
cargo run -- -h

Usage: pic2text [OPTIONS] <IMAGE>

Arguments:
  <IMAGE>  image

Options:
  -i, --i        invert
  -w, --w <W>    max picture width in characters [default: 80]
  -u, --u <U>    each char in string [default: ]
  -h, --help     Print help
  -V, --version  Print version
```

Scale image to 80 pixels wide & map each pixel to one of the 7 symbols in " _a2WM+" 
```
cargo run -- -u " _a2WM+" -w 80 baimou.jpg

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++aaa+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++++++++++++++++22222a+++++++++++++++++++++++++++++++++++++++++++++++++++++++++
+++++++++++++++++a22222a++++++++++++++++++++++++++++++++++++++++aaa++++++a++++++
++++++++++++++++++22222Wa++++++++++++++++++++++++++++++++++++a22WW2a+++++++++++_
+++++++++++++++++a22a222Wa++++++++++++++++++++++++++++++++++a2W222W2+++++++++++_
++++++++++++++++222aa2222Wa++++++++++++++++++++++++++++++++2W22222W2++++++++++_+
++++++++++++++++22aaaa2222Wa+++++++++++++++++++++++++++++a2W222222W2++++++++++_+
++++++++++++++++22a+++aa222Wa+++++++++++++++++++++++++++aW2222222222+++++++++__+
_+++++++++++++++22a++++aa222Wa+++++++++++++++++++++++++2W222222222W2+aa++++++__+
__++++++++++++++22+++___+a222Wa+++++++++++++++++++++++2W2222222aa222+aa++++++__+
__++++++++++++++22aaa_ __+a22WWa++++++++++++++++++++a2W222222aaaa222aaa++++++__a
+ _+++++++++++++22aaa+___+aa22WW2++++++++++++++++++a2W2222aaaaaaa222+aa++++++__a
a_ _++++++++++++a2a++++__++a22W2Wa++_________+++++2WWW222aaa++a2222a+aa+++++___a
a+__+++++++++++++22a+++++++a22WWW2a++_+_______+_+2WWWW2222aa++a2222aa+a++++__ _a
________________+2222a_++aaa22WWWW2++_++__+_+++a2WWWW2222aaaaaaa22aa+++++++__ +a
_________________22222a_+aa+a2222a+_______+__++2WWWWW222222aaaa22W++aa++++__  +a
_________________222a22+++_________  __++++____a222222222222aa22W2aaaa++++__  +2
_________________222a+a+ _    _______+++++a++___+++++a+222222222W2+a+++++__   +2
_________________22222a__     _____+++aa2+aaa+___++++__a+a222222W2++++++___   +2
_________________2W2222+__     ____+a++a2a_aaa++_+____+__a222222W+++++++____  +2
________________ +WW2a+__      ___++++aaaa+a+a+______++_+a22W22W2+_+++++___   a2
+_________________2WWa_      ____+++aaaa++aa+++_______++++2W222W2__________   a2
++____________    aW2+___  __++++aa+a2a2a+aa+aa++++++++a++22WWW2+__________   a2
+++____________   a22+__  __+a++a2a+a222aa2aa2aaaa++++++aaa2WWWa___________   a2
++______________  +a+__ __++aa+a22a+a222222aa22a+aaa+++++aaa2W2+________  __  a2
++_____________   _______+aaa++a22aa2222222aa22aaaaaa++++a+a222+_______   _   +2
++_____________   _  ___+aa+ _++a22222222222222a++aaaa++++a+a2a________       +2
++____________   __  __+22+ _WW2_a222222222222a+a+_+aaa++++++++______ _       _2
++___________      ___a222a a22W__22222222222a+2W2+ aaaa++++++______  _       _2
++____________     ___+a222__aaa _22222222222+_22W2 +a22a++++_ ____    __      2
++___________      ____a2aa2___  +22222222222+ _22__aa222a+a+_____             a
+++__________      ____+22aa2aaaa2222222222222+_+_+aaa22aa++______             +
+++_________       ____+a2aaaaaa222222222WWW2222aa2a+a22a++_______   _         _
++___________      _____+222a+aa2W222W22W22W22aaa+++a22aa++______              _
+++_________       _____++2222222W2222++WW2WW22a+aaa22a++++______              _
+++_________      _______++22222222+__  +2WW222222222aa+++a+_____               
+++_______      ________ _++a22222+  __  +2222222222a++__+aa+____               
+++________   _________    _+2222_ _ __ _++a2222222a++___+2aa+____              
++++_______   __________    +aa2+  _______ _2222a2+______+222a_____             
++++_______  __________     ++++   ___ _+_  _aaaa_  _____+2222+_ __             
+++++__________________     _+++    _____    _+a+   _____+2222a___              
+++++_______ ___________     _+_     __      _++   ______+a2222+__              
++++++___________________      __    __     _+_       ___aaaa22+_               
++++++___________________                __        __   _aa2aaaa_               
a+++++_________+_________                _         ______a22222a_               
a++++++______+++_________                         _  ___+a+a2a2a_               
aa++++++++___+++________                             ___+aaaaa22_               
aa+a+____++__++++_______                          ______aaa2aaa2+               
aa+++____+++++++_______        _                  _____+aaaaaaaa+               
aa+++_____++++++_______      _____               _____+aaaaaaaaa+               
aaa+++____++++++______  _   ________            ______+aaaaaaaa2_               
aaaa+++____++++__+____  _  ___________         ______++aaaaaaaaa_ _             
aaaaa++++__++++_________________________    _______++aaaaaaaaa2a_               
aaaaa+++++__++++__________________________________+aaaaaaaaaaa2+                
```

Invert the image - map to ascii range (default)
```
cargo run -- -i -w 80 baimou.jpg

zzxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxzzzuuuuuzzzzxxxxxxxxxxxxvvvvvvvxx}}xzu
zzxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxzzzzuuuzzzzxxxxxxxxxxxxxvvvvvvvxv}vxzu
zzxxxxxxxxxxxxxxxxvvvxxxxxxxxxxxxxxxxxxxxxzzzzuuuzzzzxxxxvxxxxxxxxvvvvvvxvv}vxzu
zzxxxzxxxxxxxxxxxxzxxxxxxxxxxxxxxxxxxxxxxxzzzzuuuzzzzxxxvvxxxxxxxxvvvvvxxv}}vzuT
zzzxxxxxxxxxxxxxxv\\ixzzxxxxxxxxxxxxxxxxxxxxzzzuzzzzxxxxxxxxvxxxxvvvvvvxxv?}vzuT
uzzzxxxxxxxxxxvvx~._``>uxxxvxvxxxxxxxxxxxxxxzzzzzzzxxxxxvvxxxxxzzxxvvvvxx}}vvzuT
uuzzxxxxxxxxxvvvx>_~~_ \uxzxvvvxxxxxxxxxxxxxxzzzzzxxxxxxvvxvvxz}>|?vvvxx}?}vxzuT
uuzzzxxxxxxxzxxxxv__~~_ |zvvvvxxxxxxxxxxxxxxxzzzzxxxxxvvvvvvz?`    ?vvxx}?}vxzuT
uuzzzxxxxxxxxxxxv|_~|~_. |zvzxxvxxxxxxxxxxxxxxzzzxxxxxvvvvxx|   .. ~zxxv?}}vxzT3
uuuzzzxxxxxxxvxx`.~>\~_`. \zxxxxvvvxxxxxxxxxxxxxxxvvxxxxvx}`  .``. _zxv}}}}}xu3T
uzuzzzxxxxxxxxxv.`>??\~_`. \zvvvvvvxxxxxxxxxxxxxxxxvxvxvxi.  .```` `zx}?}}}vxT3u
uzuuzxxxxxxxxxxx.`ivv}>~_`  >xvvvxxxxxxxxxxxxxxxxxxxvvvx~  ..``___ _z}}}}}vvz3yz
guuuzzxxxxxxxxxx.`ivuuz?|_`. |xvvxxxxxxxxxxxxxxxxxxxxxx_  ..``_`__ _z??}}}vxu3yv
Q3Tuzzzxxxxxxxxx._v?zyy3xi~`  >zxxxxxxxxxxxxxxxxxxvvz}. ...``_~~\~ _vi?}}}vxuyg}
QwTTzuzxxxxxxxxz``>>ikZw3x>_`  \zxxzzxxvxxzzxxxxxxxz>  ..`_~~|\>|_ ~?i?}}}vxuggi
zZguzzzzzxxxxxxz~`>iiukQwT?~`.  ~xv}}vxxzxxxxxxxxvzi   .__|>i?i>\_ ~}??}}}vzTkgi
|3Z3zzzzzzzzzzxui`\vxxx33uv~_    \TT3yg3yy3y3uuzzx~   .`_|\ivx>~~` i}??}}}xuyQg>
\}QwTuzzzzxxxzxzv.`ivzvvxuvi~.    \zu3T3gg3333T3u_    ._~~|?xv|~~.`??}?}}vz3k&y\
gygkkkgggggggyyyx  ``?3uv??i~.    .}xTuTyyuyuTxi.     `_~\i>>>>\~.\?}}}}}xugw&T\
kkggkkkkkkkwwwkQg..`..iyx\i}i~~``>Tykwg3wyzQyT}       ```_~|\i>~` }}??vvvzykQZz|
yggkkkgkkwkkkkkQg.`~>_.vzu3gwkwg3ww&&gyuTzvkwky>```_~_````_~\>`  .????}vzTgQ&Zz~
yyyggkkkggkwwwgwy. `|x?x&w&ZZ&wQwwgkkTxv}xixukgyzvvu}?}~.```_``. `}?}}vxTgw&&Zx~
ygkkwwkkkkkkwQwwg.   `iyQZZZZ&Qwwgguzzi>~}i??x3gkTTuukki}\`.._~` ~v}}vvz3kw&&Zv~
gygkwwkkkwwkkwQwQ` ..._TwQZZZZQwwgTx?x}\~?3i?iTTyTyyykTk3>`.`_~` }zvvvxTykwQ&Z}_
3ygkkgkkkkwwQQww&x  .~ukw&ZRZZZwwQzu}x?\i?x?viz3gyyggTT3z>`.  . .TyTuzuTykQQZZ?~
u3gggkggkwwQQQQQww`  \k&&&Z&Zwgk3TzT??i|}}i\z}v3ggyy33uuv}_ ... _gkkg333gwQ&BZ?~
uT3yggggkwwQQQQ&&&i .vkkQ&&w3xvx}|?u\~\_>v\>z??}xzTzxzziv}~.    zkkkkkkgkwQ&RZi~
uuT3gggkkwwwwQQ&&&? ~Tgw&&w3v?}}>~?u>~~_||_>i~>iii?}xvv}?>i~   |kkkkwwwwwQQQZZi_
uT33yggkkwwwwwQwZ&zizQw&Qgu}??v?_`i}|__`~__>i~~>}i>i}xvxvii>.  uwkwQQwQQQ&QQ&Z?_
uTT3ygkkkwwwQQQ&ZZgkQQQwyu?|~}}>~`|\~```_`_|>~_\??>??vxux???` `TwwwQQwQQ&&Q&ZZ}_
uTT3ygkkkwwwwwQZZ&wZZQw3v?ivQ3?x>_~__````.`~~_~?vv\i?ivzuvixi~\gQwQQQQQ&&Q&ZZZz_
uTT33gkkkwwQQQ&&QQQ&&w3}~~}Z3  _3>~..`.....`_~>}?ukv?>?xuz}vx}xwQQQQQ&Q&ZQ&Z&Z3_
uTT3ygkkwwwwQ&&&&&&wQTi~`~iZi._ gk~`.`.....`~>u~  x&>||i}xv}zxyQQQQQQ&Q&&&&&&Zk_
uTT3ygkkwwwQQQ&ZZZ&wg3}\~~_gw\~>Zg~`_.... ._`xg_` ~Q}\``|?xv}3QwQQQQ&&&QQ&Z&&Z&~
uTT3ygkkwwQQQ&&ZZZ&QgyT|_\|_3QkZRT`_```.  .`.v&3_ 3g~>~`~>}iTQQQQQQ&&&&&&&ZZ&ZZi
uTT3ygkwwwQQQ&&ZZZZQwg3?_~i|_|\|\~.__..`.  .._uyTyu|>>~~|i?zQwwQQQQ&&Q&&&Z&ZZZZx
uTT3ygkwwwQQ&&ZZZZZQQwyu\`|?i\|\`. ``.`_    `_.~>\~?}\~~i}}kwQQQQQQ&&Q&ZZZ&&Z&Zy
uTT3ygkkwQQQQ&&ZZZ&Qkwkgx~`~i??\. ._. ``     _ii?vux\~_|?}TwwwQQQQ&&&&&&ZZZZZZZw
uTT3yygkwQQQ&&&&ZZ&wwwgyTv_``__`` `..`vx      ~??>i>__i}vzuywQQQQQ&&&&&ZZZZZ&ZZQ
uuT3yygkwQQQ&ZZ&Z&wkwwwwyTv~`...`._vkw&&z.  .  ```__~i?}TzixywQQQQ&&&&&ZZZZZ&ZZZ
uuT3yyggwQQ&&Z&&QkywkgwQ&gTv>_```_zZZQQQZ? `... .__~?}vyku>i}ywQQQ&&&&&ZZZZZZ&ZZ
uuT33yggkwQ&&&Qkkkgkkkw&ZZ&3}~`_`y&QZQwQwz}\ ..```~\}uwwkz~|\xkQQQQ&&&&ZZZZZZ&&Z
zuTT3yygkww&Z&QkggggkkkQZZZ&x??~zWZkwwwwg3&k_._~|~vgQQwkkx~_~i3QQQQ&&ZZZZZZZZZ&Z
zuuTTyyykww&&wkkggkkwwwQZZZRTvz}&WZgwQZwTkRZyii>>T&ZQwkyyx~__~zQQQQ&ZZZZZZZZZZZ&
xzuuT3ygkkwQQwkkkwQwwwwQZ&ZZQuuTZWWZwgg3wWRZRwx?TZ&&QQwkkz~~~_igQQQ&ZZZZZZZZZZZ&
xzzuT33gkkwQ&QwkkkkkwwkQZZZ&RQT3ZWWWRyyRWRRRZyvTZZZQwwQQwx|~~~~uwQ&Z&ZZZZZZZZZZZ
vxuuuu3ygkwQwggggkwwwwwwwZZZZZ&wQZRRZwQZZZZZwx3&&&Z&&QwQkii>\~~}k&&ZZZZZZZZZZZZZ
}vzzzT3yyykwwkyyykwwwwkwQ&ZZZZZZZZRBRZZZ&wkQ&ZRZ&&&wQQ&&g>|~|||ik&&&&ZZZZZZZZZZZ
?}vxzT33yykwkg3u3kwwkkwQQ&Z&&Z&ZZ&ZRRRZZ&QZBZZZZZZZwwwQQg?~~~~~\g&&ZZZZZZZZZZZZZ
??vxxzuTyygkyTuuykkgwwQQQ&ZZ&&&&&&ZZ&&&&ZRZRRZZZZ&Q&&QwQT?}|~|~|yZ&&&ZZZZZZZZZZZ
ii}vxTuuTT3gyuzu3ygkkkwQQZZZZZ&Q&&ZZZZZZRZZRBZZ&&&Q&QQwgv||?i\~~3Z&&&ZZZZZZZZZZZ
ii}?uyy3yxz3TzxzTgkkkwwQ&ZZZ&&&&&&ZZZZZZZRRRZZZZ&QQQQwwyi>\~\i>~u&&Z&ZZZZZZZZZZZ
i?vvTygggu}vxxxzTgkkwwQQ&ZZ&&&QQ&&ZZZZZZRRRRRZZZ&&QQwkku>\\\~~\\u&&&QZZZZZZZZZZZ
>?vzu3ygkgv}vxzu3gkkwQQ&Q&&&&QQQQQ&&&ZZZZRRRZZZ&&QQwQwTi\\\\||||TZ&&ZZZZZZZZZZZZ
\>?vzT3ggkuvxxuT33ykwQ&&Q&&QQQQwQQQQQ&ZZZZZZZZ&QQwQwQkz?\~||\\|~T&&ZZ&ZZZZZZZZZZ
|\i?vuT33ggxxzuT3T3kwQ&&Q&&QQQwQQwQQQQ&&ZZZZ&&QwwwQw3xv\\\~||\||g&QZ&&&ZZZZZZZZZ
~\>i?vzuTykuxxz333ygwQQQQQQQwwwQkkkwkwwQQ&&&QQQkwwkT}ii\|\\||\~>w&&&Z&&&&ZZZZZZZ
|\\>i?xxuTyyxxzT33ygwQwQQwwwwkkkyyygygwwQQQQQwwkkgu?>\>>|\|||\~}Q&&&&&&&&ZZZZZZ&
```


