# Spark installation

## イントロ
Sparkに関する知見がないので、触ろうと思った。

## Sparkとは
基本はHadoopの上にあるin-memory database。Hadoopは実行結果はファイルに保管されるが、SparkやPrestoをはじめとするIn-Memory Databaseはメモリに保管される。そのために、難しい処理をしようとすると落ちるので、好ましくない。

## Environment
* RedHat Enterprise
* 検証用に過ぎないので、t2.medium

## 手順
### SSH keyを発行し、公開鍵をAWSに登録する
1. 分析サーバーの `teraterm`を立ち上げて、 `設定` →　`SSH鍵生成` を押す
2. 出てくる ssh keyの生成に関する画面が出てくるので、 `生成` ボタンを押す
3. `$HOME`以下に`ssh`ディレクトリを作成して、公開鍵と秘密鍵を**両方とも**保存する。保存する際に名前を付けなければならないが、秘密鍵はそのままの`id_rsa`、公開鍵は自分のものとわかるようにリネームしておくこと
4. AWSのタブから `Services` → `EC2` をクリックする
5. `Key Pairs`をクリックし、鍵一覧画面が立ち上がったら `import Key Pair`を押す
6. `Load Public Key from File` のボタンを押し、公開鍵を選択する

### インスタンスの立ち上げ
1. `EC2` -> `Launch Instances` のボタンを押す
2. `Red Hat Enterprise Linux 7.4 (HVM), SSD Volume Type` を選択する
3. インスタンスのスペックが表示されるので、適切なスペックを選択し `Next: Configure Instance Details` を押す
4. `Number of Instances` に必要な台数を記し、 `Review and Launch`ボタンを押す
5. `Launch`ボタンを押した後に出てくるModalで自身のSSH Keyを選択し、インスタンスを作成する。

### インスタンスへの接続
1. Security Groupに設定されている環境で分析環境からのアクセスを拒絶している場合があるので、分析環境のGlobal IPアドレスのアクセスを許可する
2. 接続方法をSSH、ユーザー名を `ec2-user` or `root`、SSHキーを自分の秘密鍵に設定して接続する。

### installation
1. yumパッケージの更新とreboot
	* `sudo yum update`
	* `sudo yum install vim wget`

2. Sparkのrepository登録
	* `wget http://archive.cloudera.com/cdh5/redhat/7/x86_64/cdh/cloudera-cdh5.repo`
	* `sudo mv cloudera-cdh5.repo /etc/yum.repo.d`
	* `sudo yum install yum-utils createrepo`
	* `cd /etc/yum.repo.d`
	* `sudo reposync -r cloudera-cdh5`
	* `cd -`

3. Sparkのインストール
	* 必要なものをインストールする
	* `sudo yum install spark-core.noarch`
```
[ec2-user@ip-xxx.xxx.xxx.xxx ~]$ sudo yum search spark
Loaded plugins: amazon-id, rhui-lb, search-disabled-repos
============================= N/S matched: spark ==============================
hue-spark.x86_64 : A UI for Spark on Hue
spark-history-server.noarch : History Server for Spark
spark-master.noarch : Server for Spark master
spark-python.noarch : Python client for Spark
spark-worker.noarch : Server for Spark worker
spark-core.noarch : Lightning-Fast Cluster Computing
```

## 注意点
### SSH keyに関して
利用者視点では、同一SSH Keyを他のサーバーでも用いる方が楽。
