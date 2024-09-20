#!/usr/bin/env bash
# Author: Gilles Biagomba

# Asking the user for the project name
echo "What is your project name?"
read prj

# Asking the user for their username
echo "What is your git username?"
echo "I need this for your project"
read usrnm

# Asking the user for their real name
echo "What is your git username?"
echo "I need this for your project"
read name

# Asking the user for their email
echo "What is your git username?"
echo "I need this for your project"
read email

# Creating workspace and readme file
mkdir /opt/$prj/
touch /opt/$prj/README.md
echo "# First commit" > README.md

# Creating git project
cd /opt/$prj/
git init --bare
git add README.md
git config --global user.name "$name"
git config --global user.email $email
git commit -m "First commit"
git tag -a "v1.0.0" -m "version 1.0.0"
git remote add origin https://github.com/$usrnm/$prj.git
git push -u origin master --tags
