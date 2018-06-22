# rust-raytrace
## abstract
[三葉レイちゃん](https://twitter.com/mitsuba_rei)のこちらの動画  
[【レイトレ】CG技術系バーチャルYoutuber、レイトレーシングしてみた](https://www.youtube.com/watch?v=4XeJEDuhyPs&t=379s)  
[【レイトレ】レイトレーシングで鏡面反射と屈折](https://www.youtube.com/watch?v=hzeT48zUx1M)  
を受講して、Rustでレイトレーシングを実装しました！

## result
サンプリング 3000, 反射回数 20  
![result-3000-20](img/png/result1-3000-20.png)

サンプリング 2000, 反射回数 20  
![result-2000-20](img/png/result2-2000-20.png)

サンプリング 5000, 反射回数 50  
![result-5000-50](img/png/result3-5000-50.png)

Normal, Reflectance, Depth
![result-dnc](img/png/result1-DepthNormalColor.png)

Normal, Reflectance
![result-nc](img/png/result1-NormalColor.png)

Depth
![result-d](img/png/result1-Depth.png)