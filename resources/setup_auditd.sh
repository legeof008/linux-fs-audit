#!/bin/bash
sudo auditctl -a exit,always  -F dir=$AU_FS_DIR -F perm=w -F key=WRITE &&
sudo auditctl -a exit,always  -F dir=$AU_FS_DIR -F perm=r -F key=READ  &&
sudo auditctl -l
