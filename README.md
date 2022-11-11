# EdgeApproximate

## Environment Setup
The tools we will use for big data processing are Spark and Kafka running on a Ubuntu VM. We will use the following tools to setup the environment:
- Java 8
- Scala 2.11.8
- Spark 2.2.0
- Kafka 2.7.0
- Intellij IDEA
- Anaconda 3.
- Jupyter Notebook

### Install VirtualBox and Create a Ubuntu VM

- Download VirtualBox from [here](https://www.virtualbox.org/wiki/Downloads)
- Download Ubuntu 22.04 from [here](https://ubuntu.com/download/desktop)

### Install Java, Spark, and Scala

- First, if you haven't done it, run `sudo apt-get update` and `sudo apt-get upgrade`
- 
- Install *Java*
  - `sudo apt-get install openjdk-8-jdk`
  - if `echo $JAVA_HOME` returns empty`
    - add `export JAVA_HOME='/usr/lib/jvm/java-8-openjdk-amd64'` to *.bashrc*
    - then run `source ~/.bashrc`
    - run `java -version` to check if it's installed correctly

- Install *Spark*
  - download spark-hadoop 2.2.0 from [here](https://archive.apache.org/dist/spark/spark-2.2.0/spark-2.2.0-bin-hadoop2.7.tgz)
  - create a folder `mkdir ~/work`
  - extract spark-hadoop 2.2.0
    - `tar xvf spark-2.2.0-bin-hadoop2.7.tgz`
  - move spark to `work` folder
      - `mv spark-2.2.0-bin-hadoop2.7 ~/work`
  - add to *.bashrc*
    - get your *username* by running `whoami`
    - `export SPARK_HOME='/home/[username]/work/spark-2.2.0-bin-hadoop2.7'`
    - and `export PATH=$SPARK_HOME/bin:$PATH`
    - then run `source ~/.bashrc`

- Install *Scala*
  - go to [here](https://www.scala-lang.org/download/2.11.8.html) and download scala-2.11.8.tgz
  - extract scala-2.11.8
    - `tar xvf scala-2.11.8.tgz`
  - move scala to `work` folder
    - `mv scala-2.11.8 ~/work`
  - if `which scala` returns empty
    - add to *.bashrc*
      - get your *username* by running `whoami`
      - `export SCALA_HOME='/home/[username]/work/scala-2.11.8'`
      - and `export PATH=$SCALA_HOME/bin:$PATH`
    - then run `source ~/.bashrc`
    - run `scala -version` to check if it's installed correctly

### Download Intellij Idea
Download Intellij Idea from [here](https://www.jetbrains.com/idea/download/#section=linux)

### Install Kafka
- Create Kafka user
```Bash
sudo useradd kafka -m
sudo passwd kafka
sudo adduser kafka sudo
sudo getent group sudo
```
- Switch to Kafka user
  - `su -l kafka`
- Install Kafka
```Bash
sudo apt install curl
mkdir ~/Downloads
curl "https://archive.apache.org/dist/kafka/2.7.0/kafka_2.12-2.7.0.tgz" -o ~/Downloads/kafka.tgz
mkdir ~/kafka && cd ~/kafka
tar -xvzf ~/Downloads/kafka.tgz --strip 1  
```
- to the bottom of *~/kafka/config/server.properties*
  - add `delete.topic.enable = true`
- edit */etc/systemd/system/zookeeper.service*
```
[Unit]
Description=Apache Zookeeper server
Documentation=http://zookeeper.apache.org
Requires=network.target remote-fs.target
After=network.target remote-fs.target

[Service]
Type=simple
ExecStart=/home/kafka/kafka/bin/zookeeper-server-start.sh /home/kafka/kafka/configzookeeper.properties
ExecStop=/home/kafka/kafka/bin/zookeeper-server-stop.sh
Restart=on-abnormal

[Install]
WantedBy=multi-user.target
```
- edit */etc/systemd/system/kafka.service*
 ```
[Unit]
Requires=zookeeper.service
After=zookeeper.service

[Service]
Type=simple
User=kafka
ExecStart=/bin/sh -c '/home/kafka/kafka/bin/kafka-server-start.sh /home/kafka/kafka/config/properties > /home/kafka/kafka/kafka.log 2>&1'
ExecStop=/home/kafka/kafka/bin/kafka-server-stop.sh
Restart=on-abnormal

[Install]
WantedBy=multi-user.target
```
- start kafka with `sudo systemctl start kafka`
- To check if kafka is running, run `sudo systemctl status kafka`
- (Optional) To start kafka on boot
  - `sudo journalctl -u kafka`
  - `sudo systemctl enable kafka`
- (Info) To stop kafka
  - `sudo systemctl stop kafka`

### Install Jupyter
> Important: Switch back to your user (`su -l [username]`)
Install *Anaconda3* (version: Anaconda3-2020.02-Linux-x86_64.sh)
- Download Anaconda3 from [here](https://repo.anaconda.com/archive/)
- Install prerequisites
  - `sudo 	apt-get install libgl1-mesa-glx libegl1-mesa libxrandr2 libxrandr2 libxss1 libxcursor1 libxcomposite1 libasound2 libxi6 libxtst6`
  - Install Anaconda3
    - `bash ~/Downloads/Anaconda3-2020.02-Linux-x86_64.sh`
    - accept the license and install to default location
    - choose to let conda modify your *.bashrc*
      - if you didn't choose this option, run
        - `eval "$(/home/[username]/anaconda3/bin/conda shell.[your_shell_name] hook)"`
          - to check your username, run `whoami`
          - to check your shell name, run `echo $SHELL` (e.g. bash)
        - then run `conda init`
    - `source ~/.bashrc`
    - run `conda config --set auto_activate_base False`
    - *Jupyter* comes with Anaconda3 (run `jupyter notebook` to start it)
> Info: To activate conda environment, run `conda activate [environment_name]`.
> The default environment is called `base`, which you can omit.
> Without activating the environment, you can't use `jupyter`.

#### Install Sparkmagic
Install sparkmagic
- run `pip install sparkmagic`
- make sure ipywidgets is correctly installed
  - run `jupyter nbextension enable --py --sys-prefix widgetsnbextension`
- to check your sparkmagic version, run `pip show sparkmagic`
- go to the folder shown in the output of the above command beside `Location:`
  - e.g. `/home/[username]/anaconda3/lib/python3.7/site-packages/`
- then run the following commands 
```
sudo ~/anaconda3/bin/jupyter-kernelspec install ./sparkmagic/kernels/sparkkernel
sudo ~/anaconda3/bin/jupyter-kernelspec install ./sparkmagic/kernels/pysparkkernel
sudo ~/anaconda3/bin/jupyter-kernelspec install ./sparkmagic/kernels/sparkrkernel
```
- Install *Livy*
  - download from [here](https://livy.incubator.apache.org/download/)
  - `unzip ~/Downloads/livy-[version...].zip`
  > To start Livy, run `~/apache-livy[version...]/bin/livy-server start`

