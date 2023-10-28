#!/bin/bash
sudo auditctl -a exit,always  -F dir=/home/maciek/box -F perm=w -F key=WRITE &&
sudo auditctl -a exit,always  -F dir=/home/maciek/box -F perm=r -F key=READ  &&
sudo auditctl -l
