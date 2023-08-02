## pic2text

Convert pictures to ascii art.

The symbols to use can be specified on the command line. Symbols are ranked by estimating the 'brightness' of each symbol - this is done by looking up the symbol in a small bitmap font and counting how many bits are set (see ![src/font8x8.rs](src/font8x8.rs)). 

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

Scale image to 50 pixels wide & map each pixel to one of the 7 symbols in " _a2WM+" 
```
cargo run -- -u " _a2WM+" -w 50 baimou.jpg

++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++++++++++++++++++++++++++++++++++++++++
++++++++++++a+++++++++++++++++++++++++++++++++++++
+++++++++++222++++++++++++++++++++++++++++++++++++
+++++++++++2222+++++++++++++++++++++++++aa++++++++
+++++++++++2222a++++++++++++++++++++++a2W2++++++++
++++++++++a2a222a++++++++++++++++++++2W22Wa++++++_
++++++++++22aa222+++++++++++++++++++2W222Wa++++++_
++++++++++2a+a22W2+++++++++++++++++2W22222a+++++_+
_+++++++++2a+++22Wa+++++++++++++++aW222222a+++++_+
_+++++++++2a+__+222++++++++++++++aW2222aa2aa++++_+
__++++++++2aa_ _a2W2++++++++++++aW222aaaa2aa++++_+
+_++++++++2a++__+22W2+++++++++++2W22aa+a22a+++++_+
a__+++++++a2+++++a2WWa________+2WW22a++222a++++__+
__________a22++++a2WW2+++__+++2WW222aaaa22+++++__a
_________ +222++a+a22+____+_+2WWW2222aa22a+a++___a
__________+2a2a_______ _+++__+22a2222a22Wa+a++_ _a
__________+W2a+   _____+aaa+___+++a222222a+++__ _a
_________ +W22+    ___++aa+a+__+___+22222++++__ _a
___________222__   ___++aa++++___+_+22222_+++__ _2
+_________ 2Wa    ___++aa+a+++____++2W2Wa______ _2
+________  aW+__ _++aa+22aa+aa++++++a2W2_______ _2
++________ +2__ _+++2a+2222aaaaa+++aa2W2_______ _2
+_________ _+___+aaa2aa2222a2a+aa+++a2Wa_____ _ _2
++_______  _ __+a+_+2222222a2a+aa++++a2+____     a
++_______  _ _+2+_22+22222222a+_+a+++++_____     a
+_______    _+22+_22_a222222+aW+_2a++++___       a
+________   __a22_++ a222222++W2_22a++____       +
++______    __+2a2+__2222222a_a+aa2aa+___        +
++______    __+2aa22222222W22a+aaa2a+____        _
++______    ___a2a+a22222WWW2aa++22a+____        _
++______    ___+22222222+W2W2aaa22a++____         
++_____    _____+22222__ +W222222a++a+___         
++_____  ______ _+222_ _ _222222a+__aa___         
++_____  ______  +a2+ _____a222a+___a2+___        
+++____ _______  _++  _ __ _aaa_ ___a2a_ _        
+++____________   ++  ___   _a_ ____a22+ _        
+++____________   __   _    ++  ____a22+_         
++++____________       _    __     +aa2a_         
++++____________          _     __ +222a_         
a+++_____+______                 __+a22a_         
a+++++__++_____                  __+aaa2_         
aa+__++_++_____                ____aaaaa_         
a++___++++_____    _           ___+aaaaa_         
a++___++++____    ___         ____+aaaa2_         
aa++___+++____   ______      ____+aaaaaa_         
aaa++__+++______ ________    ___+aaaaaaa_         
aaa+++_+++_____________________+aaaaaaaa          
```

Invert the image - map to ascii range (default)
```
cargo run -- -i -w 50 baimou.jpg


zxxzxxxxxxxxxxxxxxxxxxxxxxzzuuuzzzxxxxxxxxvvvxx}xu
zzxzxxxxxxxxxxxxxxxxxxxxxxzzuuuzzzxxxxxxxxvvvxv}xu
zzxzxxxxxxxxxxxxxxxxxxxxxxzzzuuzzxxxxxxxxxvvvx}vzu
zzxzxxxxxxx}?zzxxxxxxxxxxxzzzuzzzxxxxxxxxvvvxx}vzT
uzzxxxxxxx}``~zxxxxxxxxxxxxzzuzzxxxvxxxzzxvvxv}vzT
uzzxxxxxxx}~~`_zxvvxxxxxxxxxzzzzxxxvxvz}|ivvx}}xzT
uuzxxxxxxxz~~~.\zvxxxxxxxxxxzzzxxxvvvz|   }xx}}xzT
uuzzxxxxxz|_|~` ?zxxxxxxxxxxzzzxxxvvz~ .. izv}}xu3
uuzzxxxxxv.~?|_.`xxxvxxxxxxxzxxxvxxx~ .`` >z}}}vT3
uuuzxxxxxv`>vi~_ ~zvxxxxxxxxxxxxxvz~ .``_.>x}}vx3T
3uuzzzxxxx`\zu}~` \xvxxxxxxxxxxxxz| ..`__.>}}}vzyu
wTuzzxxxxx`ivgyx~. vzxzxxxzxxxxvz> .``_||.\?}}vzyz
wyzzzzxxxx`|>g&yi_ .vxxvvxxxxxxzi  `_\\>|.\?}}}zkx
uQzzzzzzzu~|}zgg}~. _zxuTTuuzxx}. ._\?}>~.i}}}vTwx
\wyzzzxxxu\_xxvux|.  ?3yykyy33z.  `_|vv~_`?}}}xyQv
3gkgyyyy3y\ _vu}}\.  .xTTy3Tuv`   _~>ii>_~}}}vukQ?
kkkkwwwww&} ``u}i}|_~ukwykTkT_   .``~\i~.\}?vx3QQi
ygkkkkwkkQ}.>~>3gQQwkQ&guuxgwx_~|__`_|~  \}?vug&Q>
ygkkkkkwwQ} `iu&ZZQwwgyu???v3g3zzu?_`._` ?}}x3wZQ\
ggwwkwwkw&v ..v&ZZZQk3vv\|xi}yyT3k3z_.~`.x}vugw&w|
ygkkkkwwwQg  ~3QZZZQQ3vx>ix}}TyykTyz` . _yzxTgwZQ~
TgkkgkQQQw&~ >&&&Zwk3ux?\}i}vuggy3ux_ . >wg33gQRQ~
T3gggkwQQ&Z? xwQ&gvxi?v~~i>}??xuzz}v|   3wkwkkQZw~
uTygkkwwwQZv_yQ&kz}}~iv~_~~i|>iivvv?i_ ~QkwwQQQ&w~
u33gkkwQQQZyTQQkx\i?`|>_`__>~|}i?xzv?~ ?QwQwQ&QZQ~
uT3gkwwwQ&ZQZQkz>x3v~_~````|_\vi?}zx}?`zQwQQ&&&Z&|
uT3gkwwQQ&&Q&Qz~}Q`_x~.`..._~?v3}>xuvx}yQQQQ&Z&ZZ>
u3ygkwwQ&&&&QT~`}y..k>....._}> }3~\vxxuwQQ&&&&&&Zi
T3ygkwQQQZZZw3>~~gxxZ\``` .`zx ~y~`|v}gQQQ&&&&Z&Rx
uTygkwQQ&ZZZwg}~\~Tg3__`` ..?Q>T?\~|?zQQQ&&&&&ZZRT
uTygwwQQ&ZZZQku_\>~~`._.`. .`>z>i>~>vkQQQ&&&&Z&ZZg
uTygkwQQ&ZZ&wwgi`ivi..` `   _\>v}~~?uQwQQ&&&ZZZZZQ
uTygkwQQ&&ZZwwgu~`__...~v   .i?i~_?xukQQQ&&&ZZZZZ&
uTygkwQ&ZZ&wwwwgx_..`_gQZv  ..`_~ivT?zwQQ&&&ZZZZZZ
uT3ygwQ&&QwgwkQ&gv_``3RQ&k_...._\vggi>3QQ&&&ZZZZZZ
uT3ygwQ&&wggkkQZZT>_xZwQQgy| __|ukQg|~vQQQ&ZZZZZZZ
zTTygkQ&QkgkkwQZRkv}ZZk&Q3Zk|\ikZwg3|_\w&Q&ZZZZZZ&
zuT3gkwQwkkwwwQZZ&uuRWQkg&BRk?3ZQQwy\~~T&QZZZZZZZZ
xzu3ykwQQkkwwwwZZZQ3ZWBg&WRRTuZZQwQg\~_}Q&ZZZZZZZZ
vzuuygwQgggwwwwQZZZ&&RZQZZ&&3QZ&&&&T>>~>wZZZZZZZZZ
}xzTyykwk3ywwwwQZZZZZZBRZ&QZZZZ&wQ&T~~~>wZZZZZZZZZ
?vxu3ykk3ugkkwQQZZ&&ZZZZ&ZZRZZZ&&QQT>~~|kZZZZZZZZZ
i}xuuTyguuygkwQ&ZZZ&&ZZZZRZBZZ&&&Qw}>i|~gZ&ZZZZZZZ
i?xyyTz3zz3kkwQ&ZZ&&&ZZZZRRRZZ&QQwgi||i|3Z&ZZZZZZZ
i}zyggvvxzykkwQ&Z&&Q&&ZZZRRRZZ&QwwT>>||\yZ&ZZZZZZZ
>}z3gkzvxu3gkQ&&&&QQQ&&ZZZRZZ&QwQw}\\\|~yZZZZZZZZZ
\ivT3g3xzT3ykQ&&&QQQQQQ&ZZZZZQwQwT?\|\\|gZZ&ZZZZZZ
|>?xu3gzxT33wQQQ&QwQwwwwQ&&&&wwwu?>|\\|\w&ZZZZZZZZ
|\i}xuyTxT3ywQQQwwkkyggkwQQQQkwTi>>|\\|i&&&ZZZZZZZ
```


