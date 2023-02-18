## Time Series Analysis
時系列分析入門をRで行う
c.f. http://www.amazon.co.jp/%E7%B5%8C%E6%B8%88%E3%83%BB%E3%83%95%E3%82%A1%E3%82%A4%E3%83%8A%E3%83%B3%E3%82%B9%E3%83%87%E3%83%BC%E3%82%BF%E3%81%AE%E8%A8%88%E9%87%8F%E6%99%82%E7%B3%BB%E5%88%97%E5%88%86%E6%9E%90-%E7%B5%B1%E8%A8%88%E3%83%A9%E3%82%A4%E3%83%96%E3%83%A9%E3%83%AA%E3%83%BC-%E6%B2%96%E6%9C%AC-%E7%AB%9C%E7%BE%A9/dp/4254127928/

# Dependency
TSSS: http://jasp.ism.ac.jp/ism/TSSS/

## How to install TSSS

# 1. install gfortran-4.8, which is a dependency of TSSS
curl -O http://r.research.att.com/libs/gfortran-4.8.2-darwin13.tar.bz2
sudo tar fvxz gfortran-4.8.2-darwin13.tar.bz2 -C /
# 2. save this file
http://jasp.ism.ac.jp/ism/TSSS/TSSS_1.0.1.tar.gz
# 3. install TSSS on R console
install.packages("path/to/TSSS.tar.gz", repos=NULL)
library(TSSS)
