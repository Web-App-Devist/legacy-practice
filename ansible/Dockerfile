
# Use the official image as a parent image
FROM ubuntu

# Update the system, install OpenSSH Server, and set up users
RUN apt-get update && apt-get upgrade -y && \
  apt-get install -y openssh-server && apt-get install sudo -y && apt-get install sshpass -y && \
  apt-get install software-properties-common -y && apt-get install curl -y && apt-get install stow -y && \
  add-apt-repository ppa:git-core/ppa -y && apt-get update -y && apt-get install git -y

# Create user and set password for user and root user
RUN  useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1000 ubuntu && \
  echo 'ubuntu:BuDTuT@143' | chpasswd && \
  echo 'root:BuDTuT@143' | chpasswd

# Set up configuration for SSH
RUN mkdir /var/run/sshd && \
  sed -i 's/PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config && \
  sed 's@session\s*required\s*pam_loginuid.so@session optional pam_loginuid.so@g' -i /etc/pam.d/sshd && \
  echo "export VISIBLE=now" >> /etc/profile

# Expose the SSH port
EXPOSE 22

# Run SSH
CMD ["/usr/sbin/sshd", "-D"]


