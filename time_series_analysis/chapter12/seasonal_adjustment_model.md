---
title: "seasonal_adjustment_model"
author: "Kousuke Kikuchi"
date: "September 29, 2015"
output: html_document
---

# 12. 季節調整モデル
- ある一定の周期で現れるパターンSnを季節成分（seasonal components）という
- p期ごとに周期があるときに、Sn = Sn-pとなるので、時間遅れオペレータBを使うと

$$
x_{n} = F_{n}x_{n} + G_{n}v_{n}\\           
(1-B^p)S}_{n} = 0
$$
