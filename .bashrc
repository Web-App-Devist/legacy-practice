#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

alias ls='ls --color=auto'
alias grep='grep --color=auto'
PS1='[\u@\h \W]\$ '
eval "$(/home/sussan/.local/bin/mise activate bash)"
. "$HOME/.cargo/env"

export PATH="$HOME/.local/nvim/bin:$PATH"

export NEOVIM_PROFILE_PATH="neovim"
alias nvim-jdhao="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/jdhao/ nvim"
alias nvim-xero="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/xero/ nvim"
alias nvim-astro="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/astrovim/ nvim"
alias nvim-josean="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/josean/ nvim"
alias nvim-ecosse3="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/ecosse3/ nvim"
alias nvim-rafi="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/rafi/ nvim"
alias nvim-sf="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/sandheep-first/ nvim"
alias nvim-s="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/sandheep/ nvim"
alias lz="NVIM_APPNAME=$NEOVIM_PROFILE_PATH/lazyvim/ nvim"

eval "$(zoxide init --cmd cd bash)"
