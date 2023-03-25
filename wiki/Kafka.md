## Make Kafka Commands Available to Your Preferred User
The programs that uses Kafka may need access to the `/home/kafka/kafka/bin` binary files in order to perform some of their operations.
To do so, follow these steps (as `kafka` user):
- Install `acl` by running 
  ```
  sudo apt update
  sudo apt install acl
  ```
- Run
  ```Bash
  sudo groupadd kafka-users
  sudo usermod -a -G kafka-users USERNAME
  sudo chgrp -R kafka-users /home/kafka/kafka/bin
  sudo setfacl -R -m g:kafka-users:rx /home/kafka/kafka/bin
  sudo setfacl -R -m user:USERNAME:rx ~kafka
  ```
- Add `/home/kafka/kafka/bin` to PATH variable (e.g. by editing `/etc/environment` appending `:/home/kafka/kafka/bin` to the end of the PATH variable).
