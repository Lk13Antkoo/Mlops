# 1. Install skypilot:
'''bash
sudo pip install skypilot
pip install cffi --upgrade
#==== check for credentials AWS, make sure you configure aws by running: aws configure
pip install skypilot[aws]
sky check
'''
The result:if aws is connected.
🎉 Enabled clouds 🎉
  ✔ AWS

# 2. Run
'''bash
sky launch -c lorax-cluster lorax.yaml
'''
