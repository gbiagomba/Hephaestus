# Author: Gilles Biagomba
# Program: Git_Updater.sh
# Description: This script was design to update my Git repo for me.\n
# 	       TBH I wrote this out of pure laziness, 
#              and not wanting to have to enter the below commands everytime (^_^). \n

git add *
echo "Why are you updating your git profile?"
read comment
git commit -m "$comment"
git push origin master

unset comment
