#!/bin/bash

NODE_IP=$1

echo Copying folders...
scp -oStrictHostKeyChecking=no -v -r -i IOENTest.txt ~/scripts ubuntu@$NODE_IP:~
scp -v -i IOENTest.txt ~/nectar-agent/docker-compose.yml ubuntu@$NODE_IP:~

echo Connecting to node $NODE_IP
ssh -v -i IOENTest.txt ubuntu@$NODE_IP << HERE

# Install docker
sudo apt-get update
sudo apt-get install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg-agent \
    software-properties-common
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository \
   "deb [arch=amd64] https://download.docker.com/linux/ubuntu \
   $(lsb_release -cs) \
   stable"
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io
sudo apt-get install gnupg2 pass

# Install docker-compose
sudo curl -L "https://github.com/docker/compose/releases/download/1.27.1/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo usermod -aG docker $USER
sudo chmod +x /usr/local/bin/docker-compose

# Install node
curl -sL https://deb.nodesource.com/setup_12.x | sudo -E bash -
sudo apt install nodejs -y

# Install newman
sudo chown -R $USER /usr/lib/node_modules
sudo npm i -g newman

# Install bc
sudo apt-get install bc -y

HERE

# Login with docker
# This is a fake docker account created only for the purpose of being able to pull the images from the remote nodes
ssh -v -i IOENTest.txt ubuntu@$NODE_IP "docker login -u ioen-tech -p xxxxx"