#!/usr/bin/env bash
# Author: Gilles Biagomba

# Asking the user for the project name
echo "What is your project name?"
read prj

# Asking the user for their username
echo "What is your git username?"
echo "I need this for your project"
read usrnm

# Creating workspace and readme file
mkdir /opt/$prj/
touch /opt/$prj/README.md
echo "# First commit" > README.md

# Creating git project
cd /opt/$prj/
git init --bare
git add README.md
git commit -m "First commit"
git tag -a "v1.0.0" -m "version 1.0.0"
git remote add origin https://github.com/$usrnm/$prj.git
git push -u origin master --tags
