# rust-raytrace
## abstract
[三葉レイちゃん](https://twitter.com/mitsuba_rei)のこちらの動画
[【レイトレ】CG技術系バーチャルYoutuber、レイトレーシングしてみた](https://www.youtube.com/watch?v=4XeJEDuhyPs&t=379s)
を受講して、Rustでレイトレーシングを実装しました！

## result
spp = 1000, reflectance_num = 10
![result-1000-10](result-1000-10.png)

spp = 5000, reflectance_num = 10
![result-5000-10](result-5000-10.png)

spp = 3000, reflectance_num = 20
![result2-3000-20](result-3000-20.png)

Normal, Reflectance, Depth
![result2-dnc](result-DepthNormalColor.png)

Normal, Reflectance
![result2-nc](result-NormalColor.png)

Depth
![result2-d](result-Depth.png)