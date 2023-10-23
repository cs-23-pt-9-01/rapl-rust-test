#!/bin/bash

HOSTNAME="seff_jr"
IP="192.168.0.5"

# Send entire results dir to Raspberry Pi
# Uses -i ~/.ssh/id_rsa for public key use, rather than password auth
scp /results/* -i /home/seff/.ssh/id_rsa $HOSTNAME@$IP:/home/$HOSTNAME/results/ 
#scp -r /home/seff/rapl-interface/results $HOSTNAME@$IP:/home/$HOSTNAME/